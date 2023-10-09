use super::{super::DocGen, helpers};
use crate::ctx::Ctx;
use raffia::{ast::*, Spanned};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for KeyframeBlock<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        helpers::format_selectors_before_block(
            &self.selectors,
            &self.comma_spans,
            self.span.start,
            ctx,
        )
        .append(helpers::format_space_before_block(ctx))
        .concat(
            ctx.end_spaced_comments(
                self.selectors
                    .last()
                    .map(|selector| selector.span().end)
                    .unwrap_or(self.span.start),
                self.block.span.start,
            ),
        )
        .append(self.block.doc(ctx))
    }
}

impl<'s> DocGen<'s> for KeyframesName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            KeyframesName::Ident(ident) => ident.doc(ctx),
            KeyframesName::Str(str) => str.doc(ctx),
            KeyframesName::LessVariable(less_variable) => less_variable.doc(ctx),
            KeyframesName::LessEscapedStr(less_escaped_str) => less_escaped_str.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for KeyframeSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            KeyframeSelector::Percentage(percentage) => percentage.doc(ctx),
            KeyframeSelector::Ident(InterpolableIdent::Literal(Ident { name, .. }))
                if name.eq_ignore_ascii_case("from") =>
            {
                Doc::text("from")
            }
            KeyframeSelector::Ident(InterpolableIdent::Literal(Ident { name, .. }))
                if name.eq_ignore_ascii_case("to") =>
            {
                Doc::text("to")
            }
            KeyframeSelector::Ident(ident) => ident.doc(ctx),
        }
    }
}
