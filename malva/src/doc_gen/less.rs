use super::DocGen;
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl DocGen for LessEscapedStr<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::text("~").append(self.str.doc(ctx))
    }
}

impl DocGen for LessFormatFunction {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text("%")
    }
}

impl DocGen for LessListFunction {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text("~")
    }
}

impl DocGen for LessPropertyMerge {
    fn doc(&self, _: &Ctx) -> Doc {
        match self.kind {
            LessPropertyMergeKind::Comma => Doc::text("+"),
            LessPropertyMergeKind::Space => Doc::text("+_"),
        }
    }
}

impl DocGen for LessVariable<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::text("@").append(self.name.doc(ctx))
    }
}

impl DocGen for LessVariableVariable<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::text("@@").append(self.variable.name.doc(ctx))
    }
}
