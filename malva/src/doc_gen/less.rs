use super::{
    helpers,
    str::{
        format_str, InterpolatedFirstStrRawFormatter, InterpolatedLastStrRawFormatter,
        InterpolatedMidStrRawFormatter,
    },
    DocGen,
};
use crate::ctx::Ctx;
use raffia::{ast::*, Spanned};
use std::{iter, mem};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for LessBinaryCondition<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.left
            .doc(ctx)
            .append(helpers::format_operator_prefix_space(ctx))
            .concat(ctx.end_spaced_comments(self.left.span().end, self.op.span.start))
            .append(self.op.doc(ctx))
            .append(helpers::format_operator_suffix_space(ctx))
            .concat(ctx.end_spaced_comments(self.op.span.end, self.right.span().start))
            .append(self.right.doc(ctx))
            .group()
    }
}

impl<'s> DocGen<'s> for LessBinaryConditionOperator {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        match self.kind {
            LessBinaryConditionOperatorKind::GreaterThan => Doc::text(">"),
            LessBinaryConditionOperatorKind::GreaterThanOrEqual => Doc::text(">="),
            LessBinaryConditionOperatorKind::LessThan => Doc::text("<"),
            LessBinaryConditionOperatorKind::LessThanOrEqual => Doc::text("<="),
            LessBinaryConditionOperatorKind::Equal => Doc::text("="),
            LessBinaryConditionOperatorKind::EqualOrGreaterThan => Doc::text("=>"),
            LessBinaryConditionOperatorKind::EqualOrLessThan => Doc::text("=<"),
            LessBinaryConditionOperatorKind::And => Doc::text("and"),
            LessBinaryConditionOperatorKind::Or => Doc::text("or"),
        }
    }
}

impl<'s> DocGen<'s> for LessCondition<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            LessCondition::Binary(binary) => binary.doc(ctx),
            LessCondition::Negated(negated) => negated.doc(ctx),
            LessCondition::Parenthesized(parenthesized) => parenthesized.doc(ctx),
            LessCondition::Value(value) => value.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for LessDetachedRuleset<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.block.doc(ctx)
    }
}

impl<'s> DocGen<'s> for LessEscapedStr<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("~").append(self.str.doc(ctx))
    }
}

impl<'s> DocGen<'s> for LessExtend<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let selector = self.selector.doc(ctx);
        if self.all.is_some() {
            selector.append(Doc::text(" all"))
        } else {
            selector
        }
    }
}

impl<'s> DocGen<'s> for LessExtendList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        helpers::format_comma_separated_list(
            &self.elements,
            &self.comma_spans,
            self.span.start,
            Doc::space(),
            ctx,
        )
    }
}

impl<'s> DocGen<'s> for LessImportOptions<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        helpers::format_parenthesized(
            helpers::format_comma_separated_list_with_trailing(
                &self.names,
                &self.comma_spans,
                self.span.start,
                Doc::line_or_space(),
                ctx,
            ),
            self.names
                .last()
                .map(|name| name.span.end)
                .unwrap_or(self.span.start),
            self.span.end,
            ctx,
        )
    }
}

impl<'s> DocGen<'s> for LessImportPrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = Vec::with_capacity(3);
        docs.push(self.href.doc(ctx));
        let mut pos = self.href.span().end;

        docs.push(Doc::line_or_space());
        docs.extend(ctx.end_spaced_comments(
            mem::replace(&mut pos, self.options.span.end),
            self.options.span.start,
        ));
        docs.push(self.options.doc(ctx));

        if let Some(media) = &self.media {
            docs.push(Doc::line_or_space());
            docs.extend(ctx.end_spaced_comments(pos, media.span.start));
            docs.push(media.doc(ctx));
        }

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for LessFormatFunction {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("%")
    }
}

impl<'s> DocGen<'s> for LessInterpolatedIdent<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            self.elements
                .iter()
                .map(|element| match element {
                    LessInterpolatedIdentElement::Static(s) => s.doc(ctx),
                    LessInterpolatedIdentElement::Variable(variable) => variable.doc(ctx),
                    LessInterpolatedIdentElement::Property(property) => property.doc(ctx),
                })
                .collect(),
        )
    }
}

impl<'s> DocGen<'s> for LessInterpolatedStr<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        if let [LessInterpolatedStrElement::Static(first), mid @ .., LessInterpolatedStrElement::Static(last)] =
            &self.elements[..]
        {
            let allow_prefer = is_preferred_quote_allowed(self, ctx);

            let mut docs = Vec::with_capacity(self.elements.len());
            docs.push(Doc::text(format_str(
                first.raw,
                InterpolatedFirstStrRawFormatter::new(first.raw),
                allow_prefer,
                ctx,
            )));
            docs.extend(mid.iter().map(|element| match element {
                LessInterpolatedStrElement::Static(s) => Doc::text(format_str(
                    s.raw,
                    InterpolatedMidStrRawFormatter::new(s.raw),
                    allow_prefer,
                    ctx,
                )),
                LessInterpolatedStrElement::Variable(variable) => variable.doc(ctx),
                LessInterpolatedStrElement::Property(property) => property.doc(ctx),
            }));
            docs.push(Doc::text(format_str(
                last.raw,
                InterpolatedLastStrRawFormatter::new(last.raw),
                allow_prefer,
                ctx,
            )));
            Doc::list(docs)
        } else {
            unreachable!()
        }
    }
}

impl<'s> DocGen<'s> for LessJavaScriptSnippet<'s> {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        let code = Doc::text("`")
            .concat(itertools::intersperse(
                self.raw
                    .split('\n')
                    .map(|s| Doc::text(s.strip_suffix('\r').unwrap_or(s))),
                Doc::empty_line(),
            ))
            .append(Doc::text("`"));
        if self.escaped {
            Doc::text("~").append(code)
        } else {
            code
        }
    }
}

impl<'s> DocGen<'s> for LessList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        helpers::format_values_list(&self.elements, self.comma_spans.as_deref(), &self.span, ctx)
    }
}

impl<'s> DocGen<'s> for LessListFunction {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("~")
    }
}

impl<'s> DocGen<'s> for LessLookup<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        if let Some(name) = &self.name {
            let name_span = name.span();
            Doc::text("[")
                .concat(ctx.end_spaced_comments(self.span.start, name_span.start))
                .append(name.doc(ctx))
                .concat(ctx.start_spaced_comments(name_span.end, self.span.end))
                .append(Doc::text("]"))
        } else {
            Doc::text("[")
                .concat(ctx.end_spaced_comments(self.span.start, self.span.end))
                .append(Doc::text("]"))
        }
    }
}

impl<'s> DocGen<'s> for LessLookupName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            LessLookupName::LessVariable(less_variable) => less_variable.doc(ctx),
            LessLookupName::LessVariableVariable(less_variable_variable) => {
                less_variable_variable.doc(ctx)
            }
            LessLookupName::LessPropertyVariable(less_property_variable) => {
                less_property_variable.doc(ctx)
            }
            LessLookupName::Ident(ident) => ident.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for LessLookups<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            self.lookups
                .iter()
                .scan(self.span.start, |pos, lookup| {
                    Some(
                        ctx.start_spaced_comments(
                            mem::replace(pos, lookup.span.end),
                            lookup.span.start,
                        )
                        .chain(iter::once(lookup.doc(ctx))),
                    )
                })
                .flatten()
                .collect(),
        )
    }
}

impl<'s> DocGen<'s> for LessMixinArgument<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            LessMixinArgument::Named(named) => named.doc(ctx),
            LessMixinArgument::Value(value) => value.doc(ctx),
            LessMixinArgument::Variadic(variadic) => variadic.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for LessMixinCalleeChild<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        if let Some(combinator) = &self.combinator {
            combinator
                .doc(ctx)
                .append(Doc::space())
                .concat(ctx.end_spaced_comments(combinator.span.end, self.name.span().start))
                .append(self.name.doc(ctx))
        } else {
            self.name.doc(ctx)
        }
    }
}

impl<'s> DocGen<'s> for LessMixinName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            LessMixinName::ClassSelector(class_selector) => class_selector.doc(ctx),
            LessMixinName::IdSelector(id_selector) => id_selector.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for LessMixinNamedArgument<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.name
            .doc(ctx)
            .concat(ctx.start_spaced_comments(self.name.span().end, self.colon_span.start))
            .append(Doc::text(": "))
            .concat(ctx.end_spaced_comments(self.colon_span.end, self.value.span().start))
            .append(self.value.doc(ctx))
    }
}

impl<'s> DocGen<'s> for LessMixinNamedParameter<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let name = self.name.doc(ctx);
        if let Some(value) = &self.value {
            name.concat(ctx.start_spaced_comments(self.name.span().end, value.span.start))
                .append(value.doc(ctx))
        } else {
            name
        }
    }
}

impl<'s> DocGen<'s> for LessMixinParameter<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            LessMixinParameter::Named(named) => named.doc(ctx),
            LessMixinParameter::Unnamed(unnamed) => unnamed.doc(ctx),
            LessMixinParameter::Variadic(variadic) => variadic.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for LessMixinNamedParameterDefaultValue<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text(": ")
            .concat(ctx.end_spaced_comments(self.colon_span.end, self.value.span().start))
            .append(self.value.doc(ctx))
    }
}

impl<'s> DocGen<'s> for LessMixinParameterName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            LessMixinParameterName::Variable(variable) => variable.doc(ctx),
            LessMixinParameterName::PropertyVariable(property_variable) => {
                property_variable.doc(ctx)
            }
        }
    }
}

impl<'s> DocGen<'s> for LessMixinUnnamedParameter<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.value.doc(ctx)
    }
}

impl<'s> DocGen<'s> for LessMixinVariadicArgument<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.name.doc(ctx).append(Doc::text("..."))
    }
}

impl<'s> DocGen<'s> for LessMixinVariadicParameter<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        if let Some(name) = &self.name {
            name.doc(ctx).append(Doc::text("..."))
        } else {
            Doc::text("...")
        }
    }
}

impl<'s> DocGen<'s> for LessNegatedCondition<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let condition_span = self.condition.span();
        Doc::text("not").append(helpers::format_parenthesized(
            Doc::list(
                ctx.end_spaced_comments(self.span.start, condition_span.start)
                    .collect(),
            )
            .append(self.condition.doc(ctx)),
            condition_span.end,
            self.span.end,
            ctx,
        ))
    }
}

impl<'s> DocGen<'s> for LessParenthesizedCondition<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let condition_span = self.condition.span();
        helpers::format_parenthesized(
            Doc::list(
                ctx.end_spaced_comments(self.span.start, condition_span.start)
                    .collect(),
            )
            .append(self.condition.doc(ctx)),
            condition_span.end,
            self.span.end,
            ctx,
        )
    }
}

impl<'s> DocGen<'s> for LessPercentKeyword {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("%")
    }
}

impl<'s> DocGen<'s> for LessPlugin<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let path = self.path.doc(ctx);
        if let Some(args) = &self.args {
            Doc::text("(")
                .append(
                    Doc::line_or_nil()
                        .append(args.doc(ctx))
                        .nest(ctx.indent_width),
                )
                .append(Doc::line_or_nil())
                .append(Doc::text(")"))
                .append(Doc::line_or_space())
                .append(path)
                .group()
                .nest(ctx.indent_width)
        } else {
            path
        }
    }
}

impl<'s> DocGen<'s> for LessPluginPath<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            LessPluginPath::Str(str) => str.doc(ctx),
            LessPluginPath::Url(url) => url.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for LessPropertyInterpolation<'s> {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text(format!("${}{}{}", '{', self.name.raw, '}'))
    }
}

impl<'s> DocGen<'s> for LessPropertyMerge {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        match self.kind {
            LessPropertyMergeKind::Comma => Doc::text("+"),
            LessPropertyMergeKind::Space => Doc::text("+_"),
        }
    }
}

impl<'s> DocGen<'s> for LessPropertyVariable<'s> {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text(format!("${}", self.name.raw))
    }
}

impl<'s> DocGen<'s> for LessVariable<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("@").append(self.name.doc(ctx))
    }
}

impl<'s> DocGen<'s> for LessVariableCall<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.variable.doc(ctx).append(Doc::text("()"))
    }
}

impl<'s> DocGen<'s> for LessVariableDeclaration<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = Vec::with_capacity(3);
        let value_span = self.value.span();

        docs.push(self.name.doc(ctx));

        docs.extend(ctx.start_spaced_comments(self.name.span.end, self.colon_span.start));
        docs.push(Doc::text(":"));

        let should_group = if let ComponentValue::LessList(LessList {
            elements,
            comma_spans: Some(comma_spans),
            span,
            ..
        }) = &self.value
        {
            docs.push(Doc::line_or_space());
            docs.extend(ctx.end_spaced_comments(self.colon_span.end, value_span.start));
            docs.push(helpers::format_comma_separated_list_with_trailing(
                elements,
                comma_spans,
                span.start,
                Doc::line_or_space(),
                ctx,
            ));
            if elements.len() == 1 {
                docs.push(Doc::text(","));
            }
            true
        } else {
            docs.push(Doc::space());
            docs.extend(ctx.end_spaced_comments(self.colon_span.end, value_span.start));
            docs.push(self.value.doc(ctx));
            false
        };

        let doc = Doc::list(docs);
        if should_group {
            doc.group().nest(ctx.indent_width)
        } else {
            doc
        }
    }
}

impl<'s> DocGen<'s> for LessVariableInterpolation<'s> {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text(format!("@{}{}{}", '{', self.name.raw, '}'))
    }
}

impl<'s> DocGen<'s> for LessVariableVariable<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("@@").append(self.variable.name.doc(ctx))
    }
}

fn is_preferred_quote_allowed(interpolated_str: &LessInterpolatedStr, ctx: &Ctx) -> bool {
    use crate::config::Quotes;

    match ctx.options.quotes {
        Quotes::AlwaysDouble | Quotes::AlwaysSingle => false,
        Quotes::PreferDouble => interpolated_str
            .elements
            .iter()
            .any(|element| match element {
                LessInterpolatedStrElement::Static(InterpolableStrStaticPart {
                    raw,
                    value,
                    ..
                }) => value.contains('"') && !raw.contains("\\\""),
                _ => false,
            }),
        Quotes::PreferSingle => interpolated_str
            .elements
            .iter()
            .any(|element| match element {
                LessInterpolatedStrElement::Static(InterpolableStrStaticPart {
                    raw,
                    value,
                    ..
                }) => value.contains('\'') && !raw.contains("\\'"),
                _ => false,
            }),
    }
}
