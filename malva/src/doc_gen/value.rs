use super::{
    helpers,
    str::{format_str, CssStrRawFormatter},
    DocGen,
};
use crate::{ctx::Ctx, state::State};
use aho_corasick::PatternID;
use raffia::{ast::*, token::TokenWithSpan, Spanned};
use std::{borrow::Cow, mem};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for BracketBlock<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = itertools::intersperse(
            self.value.iter().map(|value| value.doc(ctx, state)),
            Doc::soft_line(),
        )
        .collect::<Vec<_>>();
        docs.insert(0, Doc::text("["));
        docs.push(Doc::text("]"));
        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for Calc<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
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
                .append(Doc::line_or_nil())
                .append(self.left.doc(ctx, state))
                .nest(ctx.indent_width)
                .append(Doc::line_or_nil())
                .append(Doc::text(")"))
        } else {
            self.left.doc(ctx, state)
        };

        let right = if let (
            // a * (b + c)
            // a * (b - c)
            // a / (b + c)
            // a / (b - c)
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
        // a + (b - c)
        | (
            CalcOperatorKind::Plus,
            ComponentValue::Calc(Calc {
                op:
                    CalcOperator {
                        kind: CalcOperatorKind::Minus,
                        ..
                    },
                ..
            }),
        )
        // a - (b + c)
        // a - (b - c)
        | (
            CalcOperatorKind::Minus,
            ComponentValue::Calc(Calc {
                op:
                    CalcOperator {
                        kind: CalcOperatorKind::Plus | CalcOperatorKind::Minus,
                        ..
                    },
                ..
            }),
        )
        // a * (b / c)
        | (
            CalcOperatorKind::Multiply,
            ComponentValue::Calc(Calc {
                op:
                    CalcOperator {
                        kind: CalcOperatorKind::Division,
                        ..
                    },
                ..
            }),
        )
        // a / (b * c)
        // a / (b / c)
        | (
            CalcOperatorKind::Division,
            ComponentValue::Calc(Calc {
                op:
                    CalcOperator {
                        kind: CalcOperatorKind::Multiply | CalcOperatorKind::Division,
                        ..
                    },
                ..
            }),
        ) = (&self.op.kind, &*self.right)
        {
            Doc::text("(")
                .append(Doc::line_or_nil())
                .append(self.right.doc(ctx, state))
                .nest(ctx.indent_width)
                .append(Doc::line_or_nil())
                .append(Doc::text(")"))
        } else {
            self.right.doc(ctx, state)
        };

        left.append(helpers::format_operator_prefix_space(ctx))
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.left.span().end, self.op.span.start),
            ))
            .append(self.op.doc(ctx, state))
            .append(helpers::format_operator_suffix_space(ctx))
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.op.span.end, self.right.span().start),
            ))
            .append(right)
            .group()
    }
}

impl<'s> DocGen<'s> for CalcOperator {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        Doc::text(match self.kind {
            CalcOperatorKind::Plus => "+",
            CalcOperatorKind::Minus => "-",
            CalcOperatorKind::Multiply => "*",
            CalcOperatorKind::Division => "/",
        })
    }
}

impl<'s> DocGen<'s> for ComponentValue<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            ComponentValue::BracketBlock(bracket_block) => bracket_block.doc(ctx, state),
            ComponentValue::Calc(calc) => calc.doc(ctx, state),
            ComponentValue::Dimension(dimension) => dimension.doc(ctx, state),
            ComponentValue::Delimiter(delimiter) => delimiter.doc(ctx, state),
            ComponentValue::Function(function) => function.doc(ctx, state),
            ComponentValue::HexColor(hex_color) => hex_color.doc(ctx, state),
            ComponentValue::IdSelector(id_selector) => id_selector.doc(ctx, state),
            ComponentValue::ImportantAnnotation(important) => important.doc(ctx, state),
            ComponentValue::InterpolableIdent(interpolable_ident) => {
                interpolable_ident.doc(ctx, state)
            }
            ComponentValue::InterpolableStr(interpolable_str) => interpolable_str.doc(ctx, state),
            ComponentValue::LayerName(layer_name) => layer_name.doc(ctx, state),
            ComponentValue::LessBinaryOperation(less_binary_operation) => {
                less_binary_operation.doc(ctx, state)
            }
            ComponentValue::LessCondition(less_condition) => less_condition.doc(ctx, state),
            ComponentValue::LessDetachedRuleset(less_detached_ruleset) => {
                less_detached_ruleset.doc(ctx, state)
            }
            ComponentValue::LessEscapedStr(less_escaped_str) => less_escaped_str.doc(ctx, state),
            ComponentValue::LessJavaScriptSnippet(less_js_snippet) => {
                less_js_snippet.doc(ctx, state)
            }
            ComponentValue::LessList(less_list) => less_list.doc(ctx, state).nest(ctx.indent_width),
            ComponentValue::LessMixinCall(less_mixin_call) => less_mixin_call.doc(ctx, state),
            ComponentValue::LessNamespaceValue(less_namespace_value) => {
                less_namespace_value.doc(ctx, state)
            }
            ComponentValue::LessNegativeValue(less_negative_value) => {
                less_negative_value.doc(ctx, state)
            }
            ComponentValue::LessParenthesizedOperation(less_parenthesized_operation) => {
                less_parenthesized_operation.doc(ctx, state)
            }
            ComponentValue::LessPercentKeyword(less_percent_keyword) => {
                less_percent_keyword.doc(ctx, state)
            }
            ComponentValue::LessPropertyVariable(less_property_variable) => {
                less_property_variable.doc(ctx, state)
            }
            ComponentValue::LessVariable(less_variable) => less_variable.doc(ctx, state),
            ComponentValue::LessVariableVariable(less_variable_variable) => {
                less_variable_variable.doc(ctx, state)
            }
            ComponentValue::Number(number) => number.doc(ctx, state),
            ComponentValue::Percentage(percentage) => percentage.doc(ctx, state),
            ComponentValue::Ratio(ratio) => ratio.doc(ctx, state),
            ComponentValue::SassArbitraryArgument(sass_arbitrary_arg) => {
                sass_arbitrary_arg.doc(ctx, state)
            }
            ComponentValue::SassBinaryExpression(sass_binary_expr) => {
                sass_binary_expr.doc(ctx, state)
            }
            ComponentValue::SassKeywordArgument(sass_keyword_arg) => {
                sass_keyword_arg.doc(ctx, state)
            }
            ComponentValue::SassList(sass_list) => sass_list.doc(ctx, state),
            ComponentValue::SassMap(sass_map) => sass_map.doc(ctx, state),
            ComponentValue::SassQualifiedName(sass_qualified_name) => {
                sass_qualified_name.doc(ctx, state)
            }
            ComponentValue::SassNestingDeclaration(sass_nesting_decl) => {
                sass_nesting_decl.doc(ctx, state)
            }
            ComponentValue::SassParenthesizedExpression(sass_parenthesized_expr) => {
                sass_parenthesized_expr.doc(ctx, state)
            }
            ComponentValue::SassParentSelector(sass_parent_selector) => {
                sass_parent_selector.doc(ctx, state)
            }
            ComponentValue::SassUnaryExpression(sass_unary_expr) => sass_unary_expr.doc(ctx, state),
            ComponentValue::SassVariable(sass_variable) => sass_variable.doc(ctx, state),
            ComponentValue::TokenWithSpan(token_with_span) => token_with_span.doc(ctx, state),
            ComponentValue::UnicodeRange(unicode_range) => unicode_range.doc(ctx, state),
            ComponentValue::Url(url) => url.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for Delimiter {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        match self.kind {
            DelimiterKind::Comma => Doc::text(","),
            DelimiterKind::Solidus => Doc::text("/"),
            DelimiterKind::Semicolon => Doc::text(";"),
        }
    }
}

impl<'s> DocGen<'s> for Dimension<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
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
            DimensionKind::Unknown => self.unit.doc(ctx, state),
        };
        self.value.doc(ctx, state).append(unit)
    }
}

impl<'s> DocGen<'s> for Function<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = Vec::with_capacity(4);
        docs.push(self.name.doc(ctx, state));
        docs.push(Doc::text("("));

        let mut pos = self.name.span().end;
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

        fn format_group<'s>(
            group: &[ComponentValue<'s>],
            pos: &mut usize,
            separator: Doc<'s>,
            ctx: &Ctx<'_, 's>,
            state: &State,
        ) -> Doc<'s> {
            Doc::list(
                itertools::intersperse(
                    group.iter().map(|arg| {
                        let arg_span = arg.span();
                        Doc::list(
                            ctx.end_spaced_comments(ctx.get_comments_between(
                                mem::replace(pos, arg_span.end),
                                arg_span.start,
                            ))
                            .collect(),
                        )
                        .append(arg.doc(ctx, state))
                    }),
                    separator,
                )
                .collect(),
            )
        }

        let separator = if args_groups.len() == 1 {
            Doc::line_or_space()
        } else {
            Doc::space()
        };
        arg_docs.extend(itertools::intersperse(
            args_groups.iter().map(|group| {
                if let [group @ .., ComponentValue::Delimiter(
                    delimiter @ Delimiter {
                        kind: DelimiterKind::Comma | DelimiterKind::Semicolon,
                        span: delimiter_span,
                    },
                )] = group
                {
                    format_group(group, &mut pos, separator.clone(), ctx, state)
                        .concat(ctx.start_spaced_comments(ctx.get_comments_between(
                            mem::replace(&mut pos, delimiter_span.end),
                            delimiter_span.start,
                        )))
                        .append(delimiter.doc(ctx, state))
                } else {
                    format_group(group, &mut pos, separator.clone(), ctx, state)
                }
            }),
            helpers::get_smart_linebreak(
                self.span.start,
                &self.args,
                ctx.options.function_args_prefer_single_line,
                ctx,
            ),
        ));

        let mut has_last_line_comment = false;
        arg_docs.extend(ctx.start_spaced_comments_without_last_hard_line(
            ctx.get_comments_between(pos, self.span.end),
            &mut has_last_line_comment,
        ));

        docs.push(
            Doc::list(arg_docs)
                .nest(ctx.indent_width)
                .append(if has_last_line_comment {
                    Doc::hard_line()
                } else {
                    Doc::line_or_nil()
                })
                .group(),
        );

        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for FunctionName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            FunctionName::Ident(ident) => ident.doc(ctx, state),
            FunctionName::LessFormatFunction(less_format_fn) => less_format_fn.doc(ctx, state),
            FunctionName::LessListFunction(less_list_fn) => less_list_fn.doc(ctx, state),
            FunctionName::SassQualifiedName(sass_qualified_name) => {
                sass_qualified_name.doc(ctx, state)
            }
        }
    }
}

impl<'s> DocGen<'s> for Ident<'s> {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        Doc::text(self.raw)
    }
}

impl<'s> DocGen<'s> for HexColor<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        Doc::text(format_hex_raw(self.raw, ctx))
    }
}

impl<'s> DocGen<'s> for InterpolableIdent<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            InterpolableIdent::Literal(literal) => literal.doc(ctx, state),
            InterpolableIdent::SassInterpolated(sass_interpolated) => {
                sass_interpolated.doc(ctx, state)
            }
            InterpolableIdent::LessInterpolated(less_interpolated) => {
                less_interpolated.doc(ctx, state)
            }
        }
    }
}

impl<'s> DocGen<'s> for InterpolableIdentStaticPart<'s> {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        Doc::text(self.raw)
    }
}

impl<'s> DocGen<'s> for InterpolableStr<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            InterpolableStr::Literal(literal) => literal.doc(ctx, state),
            InterpolableStr::SassInterpolated(sass_interpolated) => {
                sass_interpolated.doc(ctx, state)
            }
            InterpolableStr::LessInterpolated(less_interpolated) => {
                less_interpolated.doc(ctx, state)
            }
        }
    }
}

impl<'s> DocGen<'s> for InterpolableUrlStaticPart<'s> {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        Doc::text(self.raw)
    }
}

impl<'s> DocGen<'s> for Number<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        Doc::text(format_number_raw(self.raw, ctx))
    }
}

impl<'s> DocGen<'s> for Percentage<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.value.doc(ctx, state).append(Doc::text("%"))
    }
}

impl<'s> DocGen<'s> for Ratio<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.numerator
            .doc(ctx, state)
            .append(Doc::text("/"))
            .append(self.denominator.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for Str<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        Doc::text(format_str(
            self.raw,
            CssStrRawFormatter::new(self.raw),
            is_preferred_quote_allowed(self.raw, ctx),
            ctx,
        ))
    }
}

impl<'s> DocGen<'s> for TokenWithSpan<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
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
            Token::Dimension(dimension) => {
                let unit_name = dimension.unit.raw;
                let unit = if unit_name.eq_ignore_ascii_case("px")
                    || unit_name.eq_ignore_ascii_case("em")
                    || unit_name.eq_ignore_ascii_case("rem")
                    || unit_name.eq_ignore_ascii_case("ex")
                    || unit_name.eq_ignore_ascii_case("rex")
                    || unit_name.eq_ignore_ascii_case("cap")
                    || unit_name.eq_ignore_ascii_case("rcap")
                    || unit_name.eq_ignore_ascii_case("ch")
                    || unit_name.eq_ignore_ascii_case("rch")
                    || unit_name.eq_ignore_ascii_case("ic")
                    || unit_name.eq_ignore_ascii_case("ric")
                    || unit_name.eq_ignore_ascii_case("lh")
                    || unit_name.eq_ignore_ascii_case("rlh")
                    || unit_name.eq_ignore_ascii_case("vw")
                    || unit_name.eq_ignore_ascii_case("vh")
                    || unit_name.eq_ignore_ascii_case("vi")
                    || unit_name.eq_ignore_ascii_case("vb")
                    || unit_name.eq_ignore_ascii_case("vmin")
                    || unit_name.eq_ignore_ascii_case("vmax")
                    || unit_name.eq_ignore_ascii_case("lvw")
                    || unit_name.eq_ignore_ascii_case("lvh")
                    || unit_name.eq_ignore_ascii_case("lvi")
                    || unit_name.eq_ignore_ascii_case("lvb")
                    || unit_name.eq_ignore_ascii_case("lvmin")
                    || unit_name.eq_ignore_ascii_case("lvmax")
                    || unit_name.eq_ignore_ascii_case("svw")
                    || unit_name.eq_ignore_ascii_case("svh")
                    || unit_name.eq_ignore_ascii_case("svi")
                    || unit_name.eq_ignore_ascii_case("svb")
                    || unit_name.eq_ignore_ascii_case("vmin")
                    || unit_name.eq_ignore_ascii_case("vmax")
                    || unit_name.eq_ignore_ascii_case("dvw")
                    || unit_name.eq_ignore_ascii_case("dvh")
                    || unit_name.eq_ignore_ascii_case("dvi")
                    || unit_name.eq_ignore_ascii_case("dvb")
                    || unit_name.eq_ignore_ascii_case("dvmin")
                    || unit_name.eq_ignore_ascii_case("dvmax")
                    || unit_name.eq_ignore_ascii_case("cm")
                    || unit_name.eq_ignore_ascii_case("mm")
                    || unit_name.eq_ignore_ascii_case("Q")
                    || unit_name.eq_ignore_ascii_case("in")
                    || unit_name.eq_ignore_ascii_case("pc")
                    || unit_name.eq_ignore_ascii_case("pt")
                    || unit_name.eq_ignore_ascii_case("deg")
                    || unit_name.eq_ignore_ascii_case("grad")
                    || unit_name.eq_ignore_ascii_case("rad")
                    || unit_name.eq_ignore_ascii_case("turn")
                    || unit_name.eq_ignore_ascii_case("s")
                    || unit_name.eq_ignore_ascii_case("ms")
                    || unit_name.eq_ignore_ascii_case("dpi")
                    || unit_name.eq_ignore_ascii_case("dpcm")
                    || unit_name.eq_ignore_ascii_case("dppx")
                    || unit_name.eq_ignore_ascii_case("fr")
                {
                    Cow::from(unit_name.to_ascii_lowercase())
                } else if unit_name.eq_ignore_ascii_case("Hz") {
                    Cow::from("Hz")
                } else if unit_name.eq_ignore_ascii_case("kHz") {
                    Cow::from("kHz")
                } else {
                    Cow::from(unit_name)
                };
                Doc::text(format!(
                    "{}{unit}",
                    format_number_raw(dimension.value.raw, ctx)
                ))
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
            Token::Number(number) => Doc::text(number.raw),
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
            Token::Str(str) => Doc::text(format_str(
                str.raw,
                CssStrRawFormatter::new(str.raw),
                is_preferred_quote_allowed(str.raw, ctx),
                ctx,
            )),
            Token::StrTemplate(..) => unreachable!(),
            Token::Tilde(..) => Doc::text("~"),
            Token::TildeEqual(..) => Doc::text("~="),
            Token::UrlRaw(..) | Token::UrlTemplate(..) => unreachable!(),
        }
    }
}

impl<'s> DocGen<'s> for UnicodeRange<'s> {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        let mut s = format!("U+{}", self.start_raw);
        if let Some(end_raw) = self.end_raw {
            s.push('-');
            s.push_str(end_raw);
        }
        s.make_ascii_uppercase();
        Doc::text(s)
    }
}

impl<'s> DocGen<'s> for Url<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = Vec::with_capacity(3);
        docs.push(Doc::text(format!(
            "{}(",
            self.name.raw.to_ascii_lowercase()
        )));

        let mut args = Vec::with_capacity(1);
        if let Some(value) = &self.value {
            args.push(value.doc(ctx, state));

            if !self.modifiers.is_empty() {
                args.push(Doc::line_or_space());
                args.append(
                    &mut itertools::intersperse(
                        self.modifiers
                            .iter()
                            .map(|modifier| modifier.doc(ctx, state)),
                        Doc::soft_line(),
                    )
                    .collect(),
                );
            }
        }

        docs.push(Doc::list(args).group().nest(ctx.indent_width));
        docs.push(Doc::text(")"));

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for UrlModifier<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            UrlModifier::Ident(ident) => ident.doc(ctx, state),
            UrlModifier::Function(function) => function.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for UrlRaw<'s> {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        Doc::text(self.raw)
    }
}

impl<'s> DocGen<'s> for UrlValue<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            UrlValue::Raw(raw) => raw.doc(ctx, state),
            UrlValue::SassInterpolated(sass_interpolated) => sass_interpolated.doc(ctx, state),
            UrlValue::Str(str) => str.doc(ctx, state),
            UrlValue::LessEscapedStr(less_escaped_str) => less_escaped_str.doc(ctx, state),
        }
    }
}

fn format_hex_raw(raw: &str, ctx: &Ctx) -> String {
    use crate::config::{HexCase, HexColorLength};

    let chars = raw.chars().collect::<Vec<_>>();
    let mut hex = if chars.iter().all(|c| c.is_ascii_hexdigit()) {
        match (chars.as_slice(), &ctx.options.hex_color_length) {
            ([c1, c2, c3], Some(HexColorLength::Long)) => {
                format!("#{c1}{c1}{c2}{c2}{c3}{c3}")
            }
            ([c1, c2, c3, c4], Some(HexColorLength::Long)) => {
                format!("#{c1}{c1}{c2}{c2}{c3}{c3}{c4}{c4}")
            }
            ([c1, c2, c3, c4, c5, c6], Some(HexColorLength::Short))
                if c1.eq_ignore_ascii_case(c2)
                    && c3.eq_ignore_ascii_case(c4)
                    && c5.eq_ignore_ascii_case(c6) =>
            {
                format!("#{c1}{c3}{c5}")
            }
            ([c1, c2, c3, c4, c5, c6, c7, c8], Some(HexColorLength::Short))
                if c1.eq_ignore_ascii_case(c2)
                    && c3.eq_ignore_ascii_case(c4)
                    && c5.eq_ignore_ascii_case(c6)
                    && c7.eq_ignore_ascii_case(c8) =>
            {
                format!("#{c1}{c3}{c5}{c7}")
            }
            _ => format!("#{raw}"),
        }
    } else {
        format!("#{raw}")
    };

    match ctx.options.hex_case {
        HexCase::Ignore => {}
        HexCase::Lower => {
            hex.make_ascii_lowercase();
        }
        HexCase::Upper => {
            hex.make_ascii_uppercase();
        }
    };

    hex
}

fn format_number_raw<'s>(raw: &'s str, ctx: &Ctx<'_, 's>) -> Cow<'s, str> {
    let number = raw.strip_suffix('.').unwrap_or(raw);
    #[allow(clippy::collapsible_else_if)]
    let number: Cow<_> = if ctx.options.omit_number_leading_zero {
        if let Some(number) = number.strip_prefix("0.") {
            format!(".{number}").into()
        } else if let Some(number) = number.strip_prefix("-0.") {
            format!("-.{number}").into()
        } else if let Some(number) = number.strip_prefix("+0.") {
            format!("+.{number}").into()
        } else {
            number.into()
        }
    } else {
        if let Some(number) = number.strip_prefix('.') {
            format!("0.{number}").into()
        } else if let Some(number) = number.strip_prefix("-.") {
            format!("-0.{number}").into()
        } else if let Some(number) = number.strip_prefix("+.") {
            format!("+0.{number}").into()
        } else {
            number.into()
        }
    };

    if let Some((coefficient, exponent)) = number.split_once(['e', 'E']) {
        format!(
            "{}e{exponent}",
            coefficient.strip_suffix('.').unwrap_or(coefficient)
        )
        .into()
    } else {
        number
    }
}

fn is_preferred_quote_allowed(raw: &str, ctx: &Ctx) -> bool {
    use super::str::{AC_DOUBLE_QUOTES, AC_SINGLE_QUOTES};
    use crate::config::Quotes;

    match ctx.options.quotes {
        Quotes::AlwaysDouble | Quotes::AlwaysSingle => false,
        Quotes::PreferDouble => {
            let pattern_id = PatternID::must(2);
            AC_DOUBLE_QUOTES
                .find_iter(raw)
                .any(|mat| mat.pattern() == pattern_id)
        }
        Quotes::PreferSingle => {
            let pattern_id = PatternID::must(2);
            AC_SINGLE_QUOTES
                .find_iter(raw)
                .any(|mat| mat.pattern() == pattern_id)
        }
    }
}
