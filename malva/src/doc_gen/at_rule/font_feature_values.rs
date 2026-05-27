use super::super::DocGen;
use crate::{ctx::Ctx, state::State};
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'a, 's: 'a> DocGen<'a, 's> for FontFamilyName<'s> {
    fn doc(&self, ctx: &Ctx<'a, 's>, state: &State) -> Doc<'s> {
        match self {
            FontFamilyName::Str(str) => str.doc(ctx, state),
            FontFamilyName::Unquoted(unquoted) => unquoted.doc(ctx, state),
        }
    }
}

impl<'a, 's: 'a> DocGen<'a, 's> for UnquotedFontFamilyName<'s> {
    fn doc(&self, ctx: &Ctx<'a, 's>, state: &State) -> Doc<'s> {
        Doc::list(
            itertools::intersperse(
                self.idents.iter().map(|ident| ident.doc(ctx, state)),
                Doc::space(),
            )
            .collect(),
        )
    }
}
