use super::{super::DocGen, helpers};
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for PageSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let pseudo = Doc::list(self.pseudo.iter().map(|pseudo| pseudo.doc(ctx)).collect());
        if let Some(name) = &self.name {
            name.doc(ctx).append(pseudo)
        } else {
            pseudo
        }
    }
}

impl<'s> DocGen<'s> for PageSelectorList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        helpers::format_selectors_before_block(
            &self.selectors,
            &self.comma_spans,
            self.span.start,
            ctx,
        )
        .nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for PseudoPage<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text(":").append(self.name.doc(ctx))
    }
}
