use super::super::DocGen;
use crate::ctx::Ctx;
use raffia::{ast::*, Spanned};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for ImportPrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = Vec::with_capacity(3);
        docs.push(self.href.doc(ctx));
        let mut pos = self.href.span().end;

        if let Some(layer) = &self.layer {
            let span = layer.span();
            docs.push(Doc::line_or_space());
            docs.extend(ctx.end_spaced_comments(pos, span.start));
            docs.push(layer.doc(ctx));
            pos = span.end;
        }

        if let Some(supports) = &self.supports {
            let span = supports.span();
            docs.push(Doc::line_or_space());
            docs.extend(ctx.end_spaced_comments(pos, span.start));
            docs.push(supports.doc(ctx));
            pos = span.end;
        }

        if let Some(media) = &self.media {
            docs.push(Doc::line_or_space());
            docs.extend(ctx.end_spaced_comments(pos, media.span.start));
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
        let kind_span = self.kind.span();
        Doc::text("supports(")
            .concat(ctx.end_spaced_comments(self.span.start, kind_span.start))
            .append(match &self.kind {
                ImportPreludeSupportsKind::SupportsCondition(supports_condition) => {
                    supports_condition.doc(ctx)
                }
                ImportPreludeSupportsKind::Declaration(declaration) => declaration.doc(ctx),
            })
            .concat(ctx.start_spaced_comments(kind_span.end, self.span.end))
            .append(Doc::text(")"))
    }
}
