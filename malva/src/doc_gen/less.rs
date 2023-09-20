use super::{
    helpers,
    str::{
        format_str, InterpolatedFirstStrRawFormatter, InterpolatedLastStrRawFormatter,
        InterpolatedMidStrRawFormatter,
    },
    DocGen,
};
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

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
        Doc::list(
            itertools::intersperse(
                self.elements.iter().map(|extend| extend.doc(ctx)),
                Doc::text(", "),
            )
            .collect(),
        )
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

impl<'s> DocGen<'s> for LessPercentKeyword {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("%")
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
