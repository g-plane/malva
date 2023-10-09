use super::{super::DocGen, helpers};
use crate::ctx::Ctx;
use raffia::{ast::*, Spanned};
use tiny_pretty::Doc;

mod color_profile;
mod container;
mod custom_media;
mod custom_selector;
mod document;
mod font_feature_values;
mod import;
mod keyframes;
mod layer;
mod media;
mod namespace;
mod page;
mod scope;
mod supports;

impl<'s> DocGen<'s> for AtRule<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = Vec::with_capacity(5);
        let mut pos = self.name.span.end;

        docs.push(Doc::text(format!(
            "@{}",
            self.name.raw.to_ascii_lowercase()
        )));

        if let Some(prelude) = &self.prelude {
            docs.push(Doc::space());
            let span = prelude.span();
            docs.extend(ctx.end_spaced_comments(pos, span.start));
            docs.push(prelude.doc(ctx));
            pos = span.end;
        }

        if let Some(block) = &self.block {
            docs.push(helpers::format_space_before_block(ctx));
            docs.extend(ctx.end_spaced_comments(pos, block.span.start));
            docs.push(block.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for AtRulePrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            AtRulePrelude::Media(media) => media.doc(ctx),
            AtRulePrelude::Charset(charset) => charset.doc(ctx),
            AtRulePrelude::ColorProfile(color_profile) => color_profile.doc(ctx),
            AtRulePrelude::Container(container) => container.doc(ctx),
            AtRulePrelude::CounterStyle(counter_style) => counter_style.doc(ctx),
            AtRulePrelude::CustomMedia(custom_media) => custom_media.doc(ctx),
            AtRulePrelude::CustomSelector(custom_selector) => custom_selector.doc(ctx),
            AtRulePrelude::Document(document) => document.doc(ctx),
            AtRulePrelude::FontFeatureValues(font_feature_values) => font_feature_values.doc(ctx),
            AtRulePrelude::FontPaletteValues(font_palette_values) => font_palette_values.doc(ctx),
            AtRulePrelude::Import(import) => import.doc(ctx),
            AtRulePrelude::Keyframes(keyframes) => keyframes.doc(ctx),
            AtRulePrelude::Layer(layer) => layer.doc(ctx),
            AtRulePrelude::LessImport(less_import) => less_import.doc(ctx),
            AtRulePrelude::LessPlugin(less_plugin) => less_plugin.doc(ctx),
            AtRulePrelude::Namespace(namespace) => namespace.doc(ctx),
            AtRulePrelude::Nest(nest) => nest.doc(ctx).group().nest(ctx.indent_width),
            AtRulePrelude::Page(page) => page.doc(ctx),
            AtRulePrelude::PositionFallback(position_fallback) => position_fallback.doc(ctx),
            AtRulePrelude::Property(property) => property.doc(ctx),
            AtRulePrelude::SassAtRoot(sass_at_root) => sass_at_root.doc(ctx),
            AtRulePrelude::SassContent(sass_content) => sass_content.doc(ctx),
            AtRulePrelude::SassEach(sass_each) => sass_each.doc(ctx),
            AtRulePrelude::SassExpr(sass_expr) => sass_expr.doc(ctx),
            AtRulePrelude::SassExtend(sass_extend) => sass_extend.doc(ctx),
            AtRulePrelude::SassFor(sass_for) => sass_for.doc(ctx),
            AtRulePrelude::SassForward(sass_forward) => sass_forward.doc(ctx),
            AtRulePrelude::SassFunction(sass_function) => sass_function.doc(ctx),
            AtRulePrelude::SassImport(sass_import) => sass_import.doc(ctx),
            AtRulePrelude::SassInclude(sass_include) => sass_include.doc(ctx),
            AtRulePrelude::SassMixin(sass_mixin) => sass_mixin.doc(ctx),
            AtRulePrelude::SassUse(sass_use) => sass_use.doc(ctx),
            AtRulePrelude::Scope(scope) => scope.doc(ctx),
            AtRulePrelude::ScrollTimeline(scroll_timeline) => scroll_timeline.doc(ctx),
            AtRulePrelude::Supports(supports) => supports.doc(ctx),
            AtRulePrelude::Unknown(unknown) => unknown.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for UnknownAtRulePrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            UnknownAtRulePrelude::ComponentValue(component_value) => component_value.doc(ctx),
            UnknownAtRulePrelude::TokenSeq(token_seq) => token_seq.doc(ctx),
        }
    }
}
