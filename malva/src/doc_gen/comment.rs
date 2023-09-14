use super::DocGen;
use crate::ctx::Ctx;
use raffia::token::{Comment, CommentKind};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for Comment<'s> {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        match self.kind {
            CommentKind::Block => {
                let mut docs = vec![Doc::text("/*")];
                docs.extend(itertools::intersperse(
                    self.content.split('\n').map(Doc::text),
                    Doc::empty_line(),
                ));
                docs.push(Doc::text("*/"));
                Doc::list(docs)
            }
            CommentKind::Line => Doc::text(format!("//{}", self.content.trim_end())),
        }
    }
}
