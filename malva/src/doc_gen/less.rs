use super::{
    helpers,
    str::{
        format_str, InterpolatedFirstStrRawFormatter, InterpolatedLastStrRawFormatter,
        InterpolatedMidStrRawFormatter,
    },
    DocGen,
};
use crate::{ctx::Ctx, state};
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

impl<'s> DocGen<'s> for LessBinaryOperation<'s> {
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

impl<'s> DocGen<'s> for LessConditionalQualifiedRule<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        helpers::format_selectors_before_block(
            &self.selector.selectors,
            &self.selector.comma_spans,
            self.selector.span.start,
            ctx,
        )
        .append(Doc::soft_line())
        .append(self.guard.doc(ctx))
        .concat(ctx.end_spaced_comments(self.selector.span.end, self.guard.span.start))
        .append(helpers::format_space_before_block(
            self.guard.span.end,
            self.block.span.start,
            ctx,
        ))
        .append(self.block.doc(ctx))
    }
}

impl<'s> DocGen<'s> for LessConditions<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("when ")
            .append(
                helpers::SeparatedListFormatter::new(
                    ",",
                    Doc::line_or_space().nest(ctx.indent_width),
                )
                .format(&self.conditions, &self.comma_spans, self.span.start, ctx),
            )
            .group()
    }
}

impl<'s> DocGen<'s> for LessDetachedRuleset<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        ctx.with_state(state::STATE_IN_LESS_DETACHED_RULESET, || {
            self.block.doc(ctx)
        })
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
        helpers::SeparatedListFormatter::new(",", Doc::space()).format(
            &self.elements,
            &self.comma_spans,
            self.span.start,
            ctx,
        )
    }
}

impl<'s> DocGen<'s> for LessExtendRule<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.nesting_selector
            .doc(ctx)
            .concat(ctx.unspaced_comments(
                self.nesting_selector.span.end,
                self.name_of_extend.span.start,
            ))
            .append(Doc::text(":extend("))
            .append({
                let mut extend_doc = vec![];

                if ctx.options.linebreak_in_pseudo_parens {
                    extend_doc.push(Doc::line_or_nil());
                }

                extend_doc.extend(ctx.end_spaced_comments(self.span.start, self.extend.span.start));
                extend_doc.push(self.extend.doc(ctx));

                extend_doc.extend(ctx.start_spaced_comments(self.extend.span.end, self.span.end));
                if ctx.options.linebreak_in_pseudo_parens {
                    Doc::list(extend_doc)
                        .nest(ctx.indent_width)
                        .append(Doc::line_or_nil())
                        .group()
                } else {
                    Doc::list(extend_doc)
                }
            })
            .append(Doc::text(")"))
    }
}

impl<'s> DocGen<'s> for LessFormatFunction {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("%")
    }
}

impl<'s> DocGen<'s> for LessImportOptions<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        helpers::format_parenthesized(
            helpers::SeparatedListFormatter::new(
                ",",
                helpers::get_smart_linebreak(self.span.start, &self.names, ctx),
            )
            .with_trailing()
            .format(&self.names, &self.comma_spans, self.span.start, ctx),
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
        docs.push(self.options.doc(ctx));
        let mut pos = self.options.span.end;

        docs.push(Doc::line_or_space());
        let href_span = self.href.span();
        docs.extend(
            ctx.end_spaced_comments(mem::replace(&mut pos, href_span.end), href_span.start),
        );
        docs.push(self.href.doc(ctx));

        if let Some(media) = &self.media {
            docs.push(Doc::line_or_space());
            docs.extend(ctx.end_spaced_comments(pos, media.span.start));
            docs.push(media.doc(ctx));
        }

        Doc::list(docs).group().nest(ctx.indent_width)
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
        let code = Doc::list(
            itertools::intersperse(
                self.raw
                    .split('\n')
                    .map(|s| Doc::text(s.strip_suffix('\r').unwrap_or(s))),
                Doc::empty_line(),
            )
            .collect(),
        );
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

impl<'s> DocGen<'s> for LessMixinArguments<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut has_last_line_comment = false;

        let is_detached_rulset_only = matches!(
            &self.args[..],
            [LessMixinArgument::Value(
                ComponentValue::LessDetachedRuleset(..)
            )]
        );
        let doc_close_to_paren = if is_detached_rulset_only {
            Doc::nil()
        } else {
            Doc::line_or_nil()
        };

        Doc::text("(")
            .append(
                doc_close_to_paren
                    .clone()
                    .append(
                        helpers::SeparatedListFormatter::new(
                            if self.is_comma_separated { "," } else { ";" },
                            helpers::get_smart_linebreak(self.span.start, &self.args, ctx),
                        )
                        .with_trailing()
                        .format(
                            &self.args,
                            &self.separator_spans,
                            self.span.start,
                            ctx,
                        ),
                    )
                    .concat(
                        ctx.start_spaced_comments_without_last_hard_line(
                            self.args
                                .last()
                                .map(|arg| arg.span().end)
                                .unwrap_or(self.span.start),
                            self.span.end,
                            &mut has_last_line_comment,
                        ),
                    )
                    .nest(if is_detached_rulset_only {
                        0
                    } else {
                        ctx.indent_width
                    })
                    .append(if has_last_line_comment {
                        Doc::hard_line()
                    } else {
                        doc_close_to_paren
                    })
                    .group(),
            )
            .append(Doc::text(")"))
    }
}

impl<'s> DocGen<'s> for LessMixinCall<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = vec![self.callee.doc(ctx)];
        let mut pos = self.callee.span.end;

        if let Some(args) = &self.args {
            docs.push(args.doc(ctx));
            pos = args.span.end;
        }

        if let Some(important) = &self.important {
            docs.push(Doc::soft_line().nest(ctx.indent_width));
            docs.extend(ctx.end_spaced_comments(pos, important.span.start));
            docs.push(important.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for LessMixinCallee<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = Vec::with_capacity(1);
        let mut pos = self.span.start;

        let mut iter = self.children.iter();
        if let Some(first) = iter.next() {
            docs.push(first.doc(ctx));
            pos = first.span.end;
        }

        let (docs, _) = iter.fold((docs, pos), |(mut docs, pos), child| {
            if pos < child.span.start {
                docs.push(Doc::line_or_space().nest(ctx.indent_width));
                docs.extend(ctx.end_spaced_comments(pos, child.span.start));
            } else {
                docs.extend(ctx.unspaced_comments(pos, child.span.start));
            }
            docs.push(child.doc(ctx));
            (docs, child.span.end)
        });

        Doc::list(docs).group()
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

impl<'s> DocGen<'s> for LessMixinDefinition<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = vec![self.name.doc(ctx), self.params.doc(ctx)];
        let mut pos = self.params.span.end;

        if let Some(guard) = &self.guard {
            docs.push(Doc::soft_line());
            docs.extend(ctx.end_spaced_comments(self.params.span.end, guard.span.start));
            docs.push(guard.doc(ctx));
            pos = guard.span.end;
        }

        docs.push(helpers::format_space_before_block(
            pos,
            self.block.span.start,
            ctx,
        ));
        docs.push(self.block.doc(ctx));

        Doc::list(docs)
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

impl<'s> DocGen<'s> for LessMixinNamedParameterDefaultValue<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text(": ")
            .concat(ctx.end_spaced_comments(self.colon_span.end, self.value.span().start))
            .append(self.value.doc(ctx))
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

impl<'s> DocGen<'s> for LessMixinParameters<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        helpers::format_parenthesized(
            helpers::SeparatedListFormatter::new(
                if self.is_comma_separated { "," } else { ";" },
                helpers::get_smart_linebreak(self.span.start, &self.params, ctx),
            )
            .with_trailing()
            .format(&self.params, &self.separator_spans, self.span.start, ctx),
            self.params
                .last()
                .map(|param| param.span().end)
                .unwrap_or(self.span.start),
            self.span.end,
            ctx,
        )
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

impl<'s> DocGen<'s> for LessNamespaceValue<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.callee
            .doc(ctx)
            .concat(ctx.unspaced_comments(self.callee.span().end, self.lookups.span.start))
            .append(self.lookups.doc(ctx))
    }
}

impl<'s> DocGen<'s> for LessNamespaceValueCallee<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            LessNamespaceValueCallee::LessMixinCall(mixin_call) => mixin_call.doc(ctx),
            LessNamespaceValueCallee::LessVariable(variable) => variable.doc(ctx),
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

impl<'s> DocGen<'s> for LessNegativeValue<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("-").append(self.value.doc(ctx))
    }
}

impl<'s> DocGen<'s> for LessOperationOperator {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        match self.kind {
            LessOperationOperatorKind::Multiply => Doc::text("*"),
            LessOperationOperatorKind::Division => Doc::text("/"),
            LessOperationOperatorKind::Plus => Doc::text("+"),
            LessOperationOperatorKind::Minus => Doc::text("-"),
        }
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

impl<'s> DocGen<'s> for LessParenthesizedOperation<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let operation_span = self.operation.span();
        helpers::format_parenthesized(
            Doc::list(
                ctx.end_spaced_comments(self.span.start, operation_span.start)
                    .collect(),
            )
            .append(self.operation.doc(ctx)),
            operation_span.end,
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
            docs.push(
                helpers::SeparatedListFormatter::new(",", Doc::line_or_space()).format(
                    elements,
                    comma_spans,
                    span.start,
                    ctx,
                ),
            );
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
                LessInterpolatedStrElement::Static(InterpolableStrStaticPart { value, .. }) => {
                    value.contains('"')
                }
                _ => false,
            }),
        Quotes::PreferSingle => interpolated_str
            .elements
            .iter()
            .any(|element| match element {
                LessInterpolatedStrElement::Static(InterpolableStrStaticPart { value, .. }) => {
                    value.contains('\'')
                }
                _ => false,
            }),
    }
}
