use super::super::DocGen;
use crate::{ctx::Ctx, state::State};
use raffia::{Spanned, ast::*};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for CustomMedia<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.name
            .doc(ctx, state)
            .append(Doc::space())
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.name.span().end, self.value.span().start),
            ))
            .append(self.value.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for CustomMediaValue<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            CustomMediaValue::MediaQueryList(media_query_list) => media_query_list.doc(ctx, state),
            CustomMediaValue::True(..) => Doc::text("true"),
            CustomMediaValue::False(..) => Doc::text("false"),
        }
    }
}
