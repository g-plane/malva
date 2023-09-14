use super::DocGen;
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for LessEscapedStr<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("~").append(self.str.doc(ctx))
    }
}

impl<'s> DocGen<'s> for LessExtend<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let selector = self.selector.doc(ctx);
        if self.all.is_some() {
            selector.append(Doc::text(" all"))
        } else {
            selector
        }
    }
}

impl<'s> DocGen<'s> for LessExtendList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            itertools::intersperse(
                self.elements.iter().map(|extend| extend.doc(ctx)),
                Doc::text(", "),
            )
            .collect(),
        )
    }
}

impl<'s> DocGen<'s> for LessFormatFunction {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("%")
    }
}

impl<'s> DocGen<'s> for LessInterpolatedIdent<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
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

impl<'s> DocGen<'s> for LessListFunction {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("~")
    }
}

impl<'s> DocGen<'s> for LessPropertyInterpolation<'s> {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text(format!("${}{}{}", '{', self.name.raw, '}'))
    }
}

impl<'s> DocGen<'s> for LessPropertyMerge {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        match self.kind {
            LessPropertyMergeKind::Comma => Doc::text("+"),
            LessPropertyMergeKind::Space => Doc::text("+_"),
        }
    }
}

impl<'s> DocGen<'s> for LessPropertyVariable<'s> {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text(format!("${}", self.name.raw))
    }
}

impl<'s> DocGen<'s> for LessVariable<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("@").append(self.name.doc(ctx))
    }
}

impl<'s> DocGen<'s> for LessVariableInterpolation<'s> {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text(format!("@{}{}{}", '{', self.name.raw, '}'))
    }
}

impl<'s> DocGen<'s> for LessVariableVariable<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("@@").append(self.variable.name.doc(ctx))
    }
}
