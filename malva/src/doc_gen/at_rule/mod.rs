use super::{super::DocGen, helpers};
use crate::{ctx::Ctx, state::State};
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
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = Vec::with_capacity(5);
        let mut pos = self.name.span.end;

        docs.push(Doc::text(format!(
            "@{}",
            self.name.raw.to_ascii_lowercase()
        )));

        let mut in_unknown_at_rule = false;

        if let Some(prelude) = &self.prelude {
            in_unknown_at_rule = matches!(prelude, AtRulePrelude::Unknown(_));
            docs.push(Doc::space());
            let span = prelude.span();
            docs.extend(ctx.end_spaced_comments(ctx.get_comments_between(pos, span.start)));
            docs.push(prelude.doc(ctx, state));
            pos = span.end;
        }

        if let Some(block) = &self.block {
            docs.push(helpers::format_space_before_block(
                pos,
                block.span.start,
                ctx,
            ));
            docs.push(block.doc(
                ctx,
                &State {
                    in_unknown_at_rule,
                    ..state.clone()
                },
            ));
        }

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for AtRulePrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            AtRulePrelude::Media(media) => media.doc(ctx, state),
            AtRulePrelude::Charset(charset) => Doc::text(charset.raw),
            AtRulePrelude::ColorProfile(color_profile) => color_profile.doc(ctx, state),
            AtRulePrelude::Container(container) => container.doc(ctx, state),
            AtRulePrelude::CounterStyle(counter_style) => counter_style.doc(ctx, state),
            AtRulePrelude::CustomMedia(custom_media) => custom_media.doc(ctx, state),
            AtRulePrelude::CustomSelector(custom_selector) => custom_selector.doc(ctx, state),
            AtRulePrelude::Document(document) => document.doc(ctx, state),
            AtRulePrelude::FontFeatureValues(font_feature_values) => {
                font_feature_values.doc(ctx, state)
            }
            AtRulePrelude::FontPaletteValues(font_palette_values) => {
                font_palette_values.doc(ctx, state)
            }
            AtRulePrelude::Import(import) => import.doc(ctx, state),
            AtRulePrelude::Keyframes(keyframes) => keyframes.doc(ctx, state),
            AtRulePrelude::Layer(layer) => layer.doc(ctx, state),
            AtRulePrelude::LessImport(less_import) => less_import.doc(ctx, state),
            AtRulePrelude::LessPlugin(less_plugin) => less_plugin.doc(ctx, state),
            AtRulePrelude::Namespace(namespace) => namespace.doc(ctx, state),
            AtRulePrelude::Nest(nest) => nest.doc(ctx, state).group().nest(ctx.indent_width),
            AtRulePrelude::Page(page) => page.doc(ctx, state),
            AtRulePrelude::PositionTry(position_try) => position_try.doc(ctx, state),
            AtRulePrelude::Property(property) => property.doc(ctx, state),
            AtRulePrelude::SassAtRoot(sass_at_root) => sass_at_root.doc(ctx, state),
            AtRulePrelude::SassContent(sass_content) => sass_content.doc(ctx, state),
            AtRulePrelude::SassEach(sass_each) => sass_each.doc(ctx, state),
            AtRulePrelude::SassExpr(sass_expr) => sass_expr.doc(ctx, state),
            AtRulePrelude::SassExtend(sass_extend) => sass_extend.doc(ctx, state),
            AtRulePrelude::SassFor(sass_for) => sass_for.doc(ctx, state),
            AtRulePrelude::SassForward(sass_forward) => sass_forward.doc(ctx, state),
            AtRulePrelude::SassFunction(sass_function) => sass_function.doc(ctx, state),
            AtRulePrelude::SassImport(sass_import) => sass_import.doc(ctx, state),
            AtRulePrelude::SassInclude(sass_include) => sass_include.doc(ctx, state),
            AtRulePrelude::SassMixin(sass_mixin) => sass_mixin.doc(ctx, state),
            AtRulePrelude::SassUse(sass_use) => sass_use.doc(ctx, state),
            AtRulePrelude::Scope(scope) => scope.doc(ctx, state),
            AtRulePrelude::ScrollTimeline(scroll_timeline) => scroll_timeline.doc(ctx, state),
            AtRulePrelude::Supports(supports) => supports.doc(ctx, state),
            AtRulePrelude::Unknown(unknown) => unknown.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for UnknownAtRulePrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            UnknownAtRulePrelude::ComponentValue(component_value) => {
                component_value.doc(ctx, state)
            }
            UnknownAtRulePrelude::TokenSeq(token_seq) => {
                token_seq.doc(ctx, state).nest(ctx.indent_width)
            }
        }
    }
}
