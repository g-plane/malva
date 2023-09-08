use super::DocGen;
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl DocGen for BracketBlock<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let mut docs = itertools::intersperse(
            self.value.iter().map(|value| value.doc(ctx)),
            Doc::softline(),
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

        self.left
            .doc(ctx)
            .append(match ctx.options.operator_linebreak {
                OperatorLineBreak::Before => Doc::softline().nest(ctx.indent_width),
                OperatorLineBreak::After => Doc::space(),
            })
            .append(self.op.doc(ctx))
            .append(match ctx.options.operator_linebreak {
                OperatorLineBreak::Before => Doc::space(),
                OperatorLineBreak::After => Doc::softline().nest(ctx.indent_width),
            })
            .append(self.right.doc(ctx))
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
            ComponentValue::Url(url) => url.doc(ctx),
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

        let mut args = Vec::with_capacity(self.args.len() * 2);
        args.push(Doc::line_or_nil());
        let mut iter = self.args.iter();
        if let Some(first) = iter.next() {
            args.push(first.doc(ctx));
        }
        iter.for_each(|value| {
            if !matches!(
                value,
                ComponentValue::Delimiter(Delimiter {
                    kind: DelimiterKind::Comma | DelimiterKind::Semicolon,
                    ..
                })
            ) {
                args.push(Doc::line_or_space());
            }
            args.push(value.doc(ctx));
        });
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
        use crate::config::HexCase;

        let hex = match ctx.options.hex_case {
            HexCase::Ignore => format!("#{}", self.raw),
            HexCase::Lower => format!("#{}", self.raw.to_ascii_lowercase()),
            HexCase::Upper => format!("#{}", self.raw.to_ascii_uppercase()),
        };
        Doc::text(hex)
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
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text(self.raw)
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
                        Doc::softline(),
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
