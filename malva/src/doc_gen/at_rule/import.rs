use super::super::DocGen;
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for ImportPrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = Vec::with_capacity(3);
        docs.push(self.href.doc(ctx));
        if let Some(layer) = &self.layer {
            docs.push(Doc::line_or_space());
            docs.push(layer.doc(ctx));
        }
        if let Some(supports) = &self.supports {
            docs.push(Doc::line_or_space());
            docs.push(supports.doc(ctx));
        }
        if let Some(media) = &self.media {
            docs.push(Doc::line_or_space());
            docs.push(media.doc(ctx));
        }

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for ImportPreludeHref<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            ImportPreludeHref::Str(str) => str.doc(ctx),
            ImportPreludeHref::Url(url) => url.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for ImportPreludeLayer<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            ImportPreludeLayer::Empty(empty) => empty.doc(ctx),
            ImportPreludeLayer::WithName(with_name) => with_name.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for ImportPreludeSupports<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("supports(")
            .append(match self {
                ImportPreludeSupports::SupportsCondition(supports_condition) => {
                    supports_condition.doc(ctx)
                }
                ImportPreludeSupports::Declaration(declaration) => declaration.doc(ctx),
            })
            .append(Doc::text(")"))
    }
}
