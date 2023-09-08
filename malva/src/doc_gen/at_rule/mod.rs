use super::super::DocGen;
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

mod media;

impl DocGen for AtRule<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let mut docs = Vec::with_capacity(5);
        docs.push(Doc::text(format!(
            "@{}",
            self.name.raw.to_ascii_lowercase()
        )));
        if let Some(prelude) = &self.prelude {
            docs.push(Doc::space());
            docs.push(prelude.doc(ctx));
        }
        if let Some(block) = &self.block {
            docs.push(Doc::space());
            docs.push(block.doc(ctx));
        }
        Doc::list(docs)
    }
}

impl DocGen for AtRulePrelude<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            AtRulePrelude::Media(media) => media.doc(ctx),
            _ => todo!(),
        }
    }
}
