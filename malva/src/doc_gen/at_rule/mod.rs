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
            AtRulePrelude::Charset(charset) => charset.doc(ctx),
            AtRulePrelude::ColorProfile(color_profile) => color_profile.doc(ctx),
            AtRulePrelude::Container(container) => todo!(),
            AtRulePrelude::CounterStyle(counter_style) => counter_style.doc(ctx),
            AtRulePrelude::CustomMedia(custom_media) => custom_media.doc(ctx),
            AtRulePrelude::Document(document) => document.doc(ctx),
            AtRulePrelude::FontFeatureValues(font_feature_values) => font_feature_values.doc(ctx),
            AtRulePrelude::FontPaletteValues(font_palette_values) => font_palette_values.doc(ctx),
            AtRulePrelude::Nest(nest) => nest.doc(ctx),
            AtRulePrelude::PositionFallback(position_fallback) => position_fallback.doc(ctx),
            AtRulePrelude::Property(property) => property.doc(ctx),
            AtRulePrelude::ScrollTimeline(scroll_timeline) => scroll_timeline.doc(ctx),
            _ => todo!(),
        }
    }
}

impl DocGen for ColorProfilePrelude<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            ColorProfilePrelude::DashedIdent(dashed_ident) => dashed_ident.doc(ctx),
            ColorProfilePrelude::DeviceCmyk(device_cmyk) => device_cmyk.doc(ctx),
        }
    }
}

impl DocGen for CustomMedia<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        self.name
            .doc(ctx)
            .append(Doc::space())
            .append(self.value.doc(ctx))
    }
}

impl DocGen for CustomMediaValue<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            CustomMediaValue::MediaQueryList(media_query_list) => media_query_list.doc(ctx),
            CustomMediaValue::True(..) => Doc::text("true"),
            CustomMediaValue::False(..) => Doc::text("false"),
        }
    }
}

impl DocGen for DocumentPrelude<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::list(
            itertools::intersperse(
                self.matchers.iter().map(|matcher| matcher.doc(ctx)),
                Doc::text(",").append(Doc::line_or_space()),
            )
            .collect(),
        )
        .group()
        .nest(ctx.indent_width)
    }
}

impl DocGen for DocumentPreludeMatcher<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            DocumentPreludeMatcher::Function(function) => function.doc(ctx),
            DocumentPreludeMatcher::Url(url) => url.doc(ctx),
        }
    }
}

impl DocGen for FontFamilyName<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            FontFamilyName::Str(str) => str.doc(ctx),
            FontFamilyName::Unquoted(unquoted) => unquoted.doc(ctx),
        }
    }
}

impl DocGen for UnquotedFontFamilyName<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::list(
            itertools::intersperse(self.idents.iter().map(|ident| ident.doc(ctx)), Doc::space())
                .collect(),
        )
    }
}
