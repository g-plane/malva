use super::super::DocGen;
use crate::{ctx::Ctx, state::State};
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'a, 's: 'a> DocGen<'a, 's> for ColorProfilePrelude<'s> {
    fn doc(&self, ctx: &Ctx<'a, 's>, state: &State) -> Doc<'s> {
        match self {
            ColorProfilePrelude::DashedIdent(dashed_ident) => dashed_ident.doc(ctx, state),
            ColorProfilePrelude::DeviceCmyk(device_cmyk) => device_cmyk.doc(ctx, state),
        }
    }
}
