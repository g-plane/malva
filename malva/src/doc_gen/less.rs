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

impl DocGen for LessInterpolatedIdent<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
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

impl DocGen for LessListFunction {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text("~")
    }
}

impl DocGen for LessPropertyInterpolation<'_> {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text(format!("${}{}{}", '{', self.name.raw, '}'))
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

impl DocGen for LessPropertyVariable<'_> {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text(format!("${}", self.name.raw))
    }
}

impl DocGen for LessVariable<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::text("@").append(self.name.doc(ctx))
    }
}

impl DocGen for LessVariableInterpolation<'_> {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text(format!("@{}{}{}", '{', self.name.raw, '}'))
    }
}

impl DocGen for LessVariableVariable<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::text("@@").append(self.variable.name.doc(ctx))
    }
}
