use super::super::DocGen;
use crate::ctx::Ctx;
use raffia::{ast::*, Spanned};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for CustomMedia<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.name
            .doc(ctx)
            .append(Doc::space())
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.name.span().end, self.value.span().start),
            ))
            .append(self.value.doc(ctx))
    }
}

impl<'s> DocGen<'s> for CustomMediaValue<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            CustomMediaValue::MediaQueryList(media_query_list) => media_query_list.doc(ctx),
            CustomMediaValue::True(..) => Doc::text("true"),
            CustomMediaValue::False(..) => Doc::text("false"),
        }
    }
}
