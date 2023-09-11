use super::DocGen;
use crate::ctx::Ctx;
use raffia::{ast::*, token::TokenWithSpan};
use std::borrow::Cow;
use tiny_pretty::Doc;

impl DocGen for BracketBlock<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let mut docs = itertools::intersperse(
            self.value.iter().map(|value| value.doc(ctx)),
            Doc::soft_line(),
        )
        .collect::<Vec<_>>();
        docs.insert(0, Doc::text("["));
        docs.push(Doc::text("]"));
        Doc::list(docs)
    }
}

impl DocGen for Calc<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        use crate::config::OperatorLineBreak;

        let left = if let (
            ComponentValue::Calc(Calc {
                op:
                    CalcOperator {
                        kind: CalcOperatorKind::Plus | CalcOperatorKind::Minus,
                        ..
                    },
                ..
            }),
            CalcOperatorKind::Multiply | CalcOperatorKind::Division,
        ) = (&*self.left, &self.op.kind)
        {
            Doc::text("(")
                .append(self.left.doc(ctx))
                .append(Doc::text(")"))
        } else {
            self.left.doc(ctx)
        };

        let right = if let (
            CalcOperatorKind::Multiply | CalcOperatorKind::Division,
            ComponentValue::Calc(Calc {
                op:
                    CalcOperator {
                        kind: CalcOperatorKind::Plus | CalcOperatorKind::Minus,
                        ..
                    },
                ..
            }),
        )
        | (
            CalcOperatorKind::Plus | CalcOperatorKind::Minus,
            ComponentValue::Calc(Calc {
                op:
                    CalcOperator {
                        kind: CalcOperatorKind::Minus,
                        ..
                    },
                ..
            }),
        )
        | (
            CalcOperatorKind::Multiply | CalcOperatorKind::Division,
            ComponentValue::Calc(Calc {
                op:
                    CalcOperator {
                        kind: CalcOperatorKind::Division,
                        ..
                    },
                ..
            }),
        ) = (&self.op.kind, &*self.right)
        {
            Doc::text("(")
                .append(self.right.doc(ctx))
                .append(Doc::text(")"))
        } else {
            self.right.doc(ctx)
        };

        left.append(match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => Doc::soft_line().nest(ctx.indent_width),
            OperatorLineBreak::After => Doc::space(),
        })
        .append(self.op.doc(ctx))
        .append(match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => Doc::space(),
            OperatorLineBreak::After => Doc::soft_line().nest(ctx.indent_width),
        })
        .append(right)
    }
}

impl DocGen for CalcOperator {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text(match self.kind {
            CalcOperatorKind::Plus => "+",
            CalcOperatorKind::Minus => "-",
            CalcOperatorKind::Multiply => "*",
            CalcOperatorKind::Division => "/",
        })
    }
}

impl DocGen for ComponentValue<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            ComponentValue::BracketBlock(bracket_block) => bracket_block.doc(ctx),
            ComponentValue::Calc(calc) => calc.doc(ctx),
            ComponentValue::Dimension(dimension) => dimension.doc(ctx),
            ComponentValue::Delimiter(delimiter) => delimiter.doc(ctx),
            ComponentValue::Function(function) => function.doc(ctx),
            ComponentValue::HexColor(hex_color) => hex_color.doc(ctx),
            ComponentValue::IdSelector(id_selector) => id_selector.doc(ctx),
            ComponentValue::ImportantAnnotation(important) => important.doc(ctx),
            ComponentValue::InterpolableIdent(interpolable_ident) => interpolable_ident.doc(ctx),
            ComponentValue::InterpolableStr(interpolable_str) => interpolable_str.doc(ctx),
            ComponentValue::Number(number) => number.doc(ctx),
            ComponentValue::Percentage(percentage) => percentage.doc(ctx),
            ComponentValue::Ratio(ratio) => ratio.doc(ctx),
            ComponentValue::SassNestingDeclaration(sass_nesting_decl) => sass_nesting_decl.doc(ctx),
            ComponentValue::SassParenthesizedExpression(sass_parenthesized_expr) => {
                sass_parenthesized_expr.doc(ctx)
            }
            ComponentValue::TokenWithSpan(token_with_span) => token_with_span.doc(ctx),
            ComponentValue::Url(url) => url.doc(ctx),
            ComponentValue::UnicodeRange(unicode_range) => unicode_range.doc(ctx),
            _ => todo!(),
        }
    }
}

impl DocGen for Delimiter {
    fn doc(&self, _: &Ctx) -> Doc {
        match self.kind {
            DelimiterKind::Comma => Doc::text(","),
            DelimiterKind::Solidus => Doc::text("/"),
            DelimiterKind::Semicolon => Doc::text(";"),
        }
    }
}

impl DocGen for Dimension<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let unit = match self.kind {
            DimensionKind::Length => {
                if self.unit.name.eq_ignore_ascii_case("Q") {
                    Doc::text("Q")
                } else {
                    Doc::text(self.unit.raw.to_ascii_lowercase())
                }
            }
            DimensionKind::Angle
            | DimensionKind::Duration
            | DimensionKind::Resolution
            | DimensionKind::Flex => Doc::text(self.unit.raw.to_ascii_lowercase()),
            DimensionKind::Frequency => {
                if self.unit.name.eq_ignore_ascii_case("Hz") {
                    Doc::text("Hz")
                } else if self.unit.name.eq_ignore_ascii_case("kHz") {
                    Doc::text("kHz")
                } else {
                    Doc::text(self.unit.raw.to_ascii_lowercase())
                }
            }
            DimensionKind::Unknown => self.unit.doc(ctx),
        };
        self.value.doc(ctx).append(unit)
    }
}

impl DocGen for Function<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let mut docs = Vec::with_capacity(4);
        docs.push(self.name.doc(ctx));
        docs.push(Doc::text("("));

        let mut arg_docs = Vec::with_capacity(self.args.len() * 2);
        arg_docs.push(Doc::line_or_nil());

        let args_groups = self
            .args
            .split_inclusive(|arg| {
                matches!(
                    arg,
                    ComponentValue::Delimiter(Delimiter {
                        kind: DelimiterKind::Comma | DelimiterKind::Semicolon,
                        ..
                    })
                )
            })
            .collect::<Vec<_>>();

        let separator = if args_groups.len() == 1 {
            Doc::line_or_space()
        } else {
            Doc::space()
        };
        arg_docs.extend(itertools::intersperse(
            args_groups.iter().map(|group| {
                if let Some(ComponentValue::Delimiter(
                    delimiter @ Delimiter {
                        kind: DelimiterKind::Comma | DelimiterKind::Semicolon,
                        ..
                    },
                )) = group.last()
                {
                    Doc::list(
                        itertools::intersperse(
                            group.iter().take(group.len() - 1).map(|arg| arg.doc(ctx)),
                            separator.clone(),
                        )
                        .collect(),
                    )
                    .append(delimiter.doc(ctx))
                } else {
                    Doc::list(
                        itertools::intersperse(
                            group.iter().map(|arg| arg.doc(ctx)),
                            separator.clone(),
                        )
                        .collect(),
                    )
                }
            }),
            Doc::line_or_space(),
        ));
        docs.push(
            Doc::list(arg_docs)
                .nest(ctx.indent_width)
                .append(Doc::line_or_nil())
                .group(),
        );

        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for FunctionName<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            FunctionName::Ident(ident) => ident.doc(ctx),
            FunctionName::LessFormatFunction(less_format_fn) => less_format_fn.doc(ctx),
            FunctionName::LessListFunction(less_list_fn) => less_list_fn.doc(ctx),
            FunctionName::SassQualifiedName(sass_qualified_name) => sass_qualified_name.doc(ctx),
        }
    }
}

impl DocGen for Ident<'_> {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text(self.raw)
    }
}

impl DocGen for HexColor<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::text(format_hex_raw(self.raw, ctx))
    }
}

impl DocGen for InterpolableIdent<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            InterpolableIdent::Literal(literal) => literal.doc(ctx),
            _ => todo!(),
        }
    }
}

impl DocGen for InterpolableStr<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            InterpolableStr::Literal(literal) => literal.doc(ctx),
            _ => todo!(),
        }
    }
}

impl DocGen for Number<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::text(format_number_raw(self.raw, ctx))
    }
}

impl DocGen for Percentage<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        self.value.doc(ctx).append(Doc::text("%"))
    }
}

impl DocGen for Ratio<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        self.numerator
            .doc(ctx)
            .append(Doc::text("/"))
            .append(self.denominator.doc(ctx))
    }
}

impl DocGen for Str<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        use crate::config::Quotes;

        let (left, right) = self.raw.split_at(1);
        let inner = &right[0..right.len() - 1];

        match ctx.options.quotes {
            Quotes::AlwaysDouble => {
                if left == "\"" {
                    Doc::text(self.raw)
                } else {
                    Doc::text(format!("\"{}\"", inner.replace('"', "\\\"")))
                }
            }
            Quotes::AlwaysSingle => {
                if left == "\'" {
                    Doc::text(self.raw)
                } else {
                    Doc::text(format!("'{}'", inner.replace('\'', "\\'")))
                }
            }
            Quotes::PreferDouble => {
                if left == "\"" || inner.contains("\\\"") {
                    Doc::text(self.raw)
                } else {
                    Doc::text(format!("\"{inner}\""))
                }
            }
            Quotes::PreferSingle => {
                if left == "\'" || inner.contains("\\\'") {
                    Doc::text(self.raw)
                } else {
                    Doc::text(format!("'{inner}'"))
                }
            }
        }
    }
}

impl<'s> DocGen for TokenWithSpan<'s> {
    fn doc(&self, ctx: &Ctx) -> Doc<'s> {
        use raffia::token::Token;

        match &self.token {
            Token::Ampersand(..) => Doc::text("&"),
            Token::Asterisk(..) => Doc::text("*"),
            Token::AsteriskEqual(..) => Doc::text("*="),
            Token::At(..) => Doc::text("@"),
            Token::AtKeyword(at_keyword) => Doc::text(format!("@{}", at_keyword.ident.raw)),
            Token::AtLBraceVar(at_lbrace_var) => {
                Doc::text(format!("@{}{}{}", '{', at_lbrace_var.ident.raw, '}'))
            }
            Token::BacktickCode(backtick_code) => Doc::text(format!("`{}`", backtick_code.raw)),
            Token::Bar(..) => Doc::text("|"),
            Token::BarBar(..) => Doc::text("||"),
            Token::BarEqual(..) => Doc::text("|="),
            Token::CaretEqual(..) => Doc::text("^="),
            Token::Cdc(..) | Token::Cdo(..) => unreachable!(),
            Token::Colon(..) => Doc::text(":"),
            Token::ColonColon(..) => Doc::text("::"),
            Token::Comma(..) => Doc::text(","),
            Token::Dedent(..) => unreachable!(),
            Token::Dimension(..) => {
                todo!()
            }
            Token::DollarEqual(..) => Doc::text("$="),
            Token::DollarLBraceVar(dollar_lbrace_var) => {
                Doc::text(format!("${}{}{}", '{', dollar_lbrace_var.ident.raw, '}'))
            }
            Token::DollarVar(dollar_var) => Doc::text(format!("${}", dollar_var.ident.raw)),
            Token::Dot(..) => Doc::text("."),
            Token::DotDotDot(..) => Doc::text("..."),
            Token::Eof(..) => unreachable!(),
            Token::Equal(..) => Doc::text("="),
            Token::EqualEqual(..) => Doc::text("=="),
            Token::Exclamation(..) => Doc::text("!"),
            Token::ExclamationEqual(..) => Doc::text("!="),
            Token::GreaterThan(..) => Doc::text(">"),
            Token::GreaterThanEqual(..) => Doc::text(">="),
            Token::Hash(hash) => Doc::text(format_hex_raw(hash.raw, ctx)),
            Token::HashLBrace(..) => Doc::text("#{"),
            Token::Ident(ident) => Doc::text(ident.raw),
            Token::Indent(..) => unreachable!(),
            Token::LBrace(..) => Doc::text("{"),
            Token::LBracket(..) => Doc::text("["),
            Token::LessThan(..) => Doc::text("<"),
            Token::LessThanEqual(..) => Doc::text("<="),
            Token::Linebreak(..) => unreachable!(),
            Token::LParen(..) => Doc::text("("),
            Token::Minus(..) => Doc::text("-"),
            Token::Number(number) => Doc::text(format_number_raw(number.raw, ctx)),
            Token::NumberSign(..) => Doc::text("#"),
            Token::Percent(..) => Doc::text("%"),
            Token::Percentage(percentage) => {
                Doc::text(format!("{}%", format_number_raw(percentage.value.raw, ctx)))
            }
            Token::Plus(..) => Doc::text("+"),
            Token::PlusUnderscore(..) => Doc::text("+_"),
            Token::Question(..) => Doc::text("?"),
            Token::RBrace(..) => Doc::text("}"),
            Token::RBracket(..) => Doc::text("]"),
            Token::RParen(..) => Doc::text(")"),
            Token::Semicolon(..) => Doc::text(";"),
            Token::Solidus(..) => Doc::text("/"),
            Token::Str(str) => todo!(),
            Token::StrTemplate(..) => todo!(),
            Token::Tilde(..) => Doc::text("~"),
            Token::TildeEqual(..) => Doc::text("~="),
            Token::UrlRaw(..) | Token::UrlTemplate(..) => unreachable!(),
        }
    }
}

impl DocGen for UnicodeRange<'_> {
    fn doc(&self, _: &Ctx) -> Doc {
        let mut s = format!("U+{}", self.start_raw);
        if let Some(end_raw) = self.end_raw {
            s.push('-');
            s.push_str(end_raw);
        }
        s.make_ascii_uppercase();
        Doc::text(s)
    }
}

impl DocGen for Url<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let mut docs = Vec::with_capacity(3);
        docs.push(Doc::text("url("));

        let mut args = Vec::with_capacity(1);
        if let Some(value) = &self.value {
            args.push(Doc::line_or_nil());
            args.push(value.doc(ctx));

            if !self.modifiers.is_empty() {
                args.push(Doc::line_or_space());
                args.append(
                    &mut itertools::intersperse(
                        self.modifiers.iter().map(|modifier| modifier.doc(ctx)),
                        Doc::soft_line(),
                    )
                    .collect(),
                );
            }
        }

        docs.push(
            Doc::list(args)
                .nest(ctx.indent_width)
                .append(Doc::line_or_nil())
                .group(),
        );
        docs.push(Doc::text(")"));

        Doc::list(docs)
    }
}

impl DocGen for UrlModifier<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            UrlModifier::Ident(ident) => ident.doc(ctx),
            UrlModifier::Function(function) => function.doc(ctx),
        }
    }
}

impl DocGen for UrlRaw<'_> {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text(self.raw)
    }
}

impl DocGen for UrlValue<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            UrlValue::Raw(raw) => raw.doc(ctx),
            UrlValue::SassInterpolated(sass_interpolated) => todo!(),
            UrlValue::Str(str) => str.doc(ctx),
            UrlValue::LessEscapedStr(less_escaped_str) => less_escaped_str.doc(ctx),
        }
    }
}

fn format_hex_raw(raw: &str, ctx: &Ctx) -> String {
    use crate::config::HexCase;

    match ctx.options.hex_case {
        HexCase::Ignore => format!("#{}", raw),
        HexCase::Lower => format!("#{}", raw.to_ascii_lowercase()),
        HexCase::Upper => format!("#{}", raw.to_ascii_uppercase()),
    }
}

fn format_number_raw<'a, 's>(raw: &'s str, ctx: &'a Ctx) -> Cow<'s, str> {
    let number: Cow<_> = if ctx.options.omit_zero_before_dot {
        if let Some(raw) = raw.strip_prefix("0.") {
            format!(".{raw}").into()
        } else if let Some(raw) = raw.strip_suffix("-0.") {
            format!("-.{raw}").into()
        } else if let Some(raw) = raw.strip_prefix("+0.") {
            format!("+.{raw}").into()
        } else {
            raw.into()
        }
    } else {
        if let Some(raw) = raw.strip_prefix('.') {
            format!("0.{raw}").into()
        } else if let Some(raw) = raw.strip_suffix("-.") {
            format!("-0.{raw}").into()
        } else if let Some(raw) = raw.strip_prefix("+.") {
            format!("+0.{raw}").into()
        } else {
            raw.into()
        }
    };

    if let Some((coefficient, exponent)) = number.split_once('E') {
        format!("{coefficient}e{exponent}").into()
    } else {
        number
    }
}
