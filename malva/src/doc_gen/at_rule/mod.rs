use super::{super::DocGen, helpers};
use crate::ctx::Ctx;
use raffia::{ast::*, Spanned};
use tiny_pretty::Doc;

mod container;
mod import;
mod media;
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

impl<'s> DocGen<'s> for ColorProfilePrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            ColorProfilePrelude::DashedIdent(dashed_ident) => dashed_ident.doc(ctx),
            ColorProfilePrelude::DeviceCmyk(device_cmyk) => device_cmyk.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for CustomMedia<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.name
            .doc(ctx)
            .append(Doc::space())
            .concat(ctx.end_spaced_comments(self.name.span().end, self.value.span().start))
            .append(self.value.doc(ctx))
    }
}

impl<'s> DocGen<'s> for CustomMediaValue<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            CustomMediaValue::MediaQueryList(media_query_list) => media_query_list.doc(ctx),
            CustomMediaValue::True(..) => Doc::text("true"),
            CustomMediaValue::False(..) => Doc::text("false"),
        }
    }
}

impl<'s> DocGen<'s> for DocumentPrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        helpers::SeparatedListFormatter::new(",", Doc::line_or_space())
            .format(&self.matchers, &self.comma_spans, self.span.start, ctx)
            .group()
            .nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for DocumentPreludeMatcher<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            DocumentPreludeMatcher::Function(function) => function.doc(ctx),
            DocumentPreludeMatcher::Url(url) => url.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for FontFamilyName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            FontFamilyName::Str(str) => str.doc(ctx),
            FontFamilyName::Unquoted(unquoted) => unquoted.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for KeyframeBlock<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        helpers::format_selectors_before_block(
            &self.selectors,
            &self.comma_spans,
            self.span.start,
            ctx,
        )
        .append(helpers::format_space_before_block(ctx))
        .concat(
            ctx.end_spaced_comments(
                self.selectors
                    .last()
                    .map(|selector| selector.span().end)
                    .unwrap_or(self.span.start),
                self.block.span.start,
            ),
        )
        .append(self.block.doc(ctx))
    }
}

impl<'s> DocGen<'s> for KeyframesName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            KeyframesName::Ident(ident) => ident.doc(ctx),
            KeyframesName::Str(str) => str.doc(ctx),
            KeyframesName::LessVariable(less_variable) => less_variable.doc(ctx),
            KeyframesName::LessEscapedStr(less_escaped_str) => less_escaped_str.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for KeyframeSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            KeyframeSelector::Percentage(percentage) => percentage.doc(ctx),
            KeyframeSelector::Ident(InterpolableIdent::Literal(Ident { name, .. }))
                if name.eq_ignore_ascii_case("from") =>
            {
                Doc::text("from")
            }
            KeyframeSelector::Ident(InterpolableIdent::Literal(Ident { name, .. }))
                if name.eq_ignore_ascii_case("to") =>
            {
                Doc::text("to")
            }
            KeyframeSelector::Ident(ident) => ident.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for LayerName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            itertools::intersperse(
                self.idents.iter().map(|ident| ident.doc(ctx)),
                Doc::text("."),
            )
            .collect(),
        )
    }
}

impl<'s> DocGen<'s> for NamespacePrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        if let Some(prefix) = &self.prefix {
            prefix
                .doc(ctx)
                .append(Doc::line_or_space())
                .concat(ctx.end_spaced_comments(prefix.span().end, self.uri.span().start))
                .append(self.uri.doc(ctx))
                .group()
                .nest(ctx.indent_width)
        } else {
            self.uri.doc(ctx)
        }
    }
}

impl<'s> DocGen<'s> for NamespacePreludeUri<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            NamespacePreludeUri::Str(str) => str.doc(ctx),
            NamespacePreludeUri::Url(url) => url.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for PageSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let pseudo = Doc::list(self.pseudo.iter().map(|pseudo| pseudo.doc(ctx)).collect());
        if let Some(name) = &self.name {
            name.doc(ctx).append(pseudo)
        } else {
            pseudo
        }
    }
}

impl<'s> DocGen<'s> for PageSelectorList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        helpers::format_selectors_before_block(
            &self.selectors,
            &self.comma_spans,
            self.span.start,
            ctx,
        )
        .nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for PseudoPage<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text(":").append(self.name.doc(ctx))
    }
}

impl<'s> DocGen<'s> for ScopeEnd<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("to ")
            .concat(ctx.end_spaced_comments(self.to_span.end, self.lparen_span.start))
            .append(helpers::format_parenthesized(
                self.selector.doc(ctx),
                self.selector.span.end,
                self.span.end,
                ctx,
            ))
    }
}

impl<'s> DocGen<'s> for ScopePrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            ScopePrelude::StartOnly(start_only) => start_only.doc(ctx),
            ScopePrelude::EndOnly(end_only) => end_only.doc(ctx),
            ScopePrelude::Both(both) => both.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for ScopeStart<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        helpers::format_parenthesized(
            self.selector.doc(ctx),
            self.selector.span.end,
            self.span.end,
            ctx,
        )
    }
}

impl<'s> DocGen<'s> for ScopeStartWithEnd<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.start
            .doc(ctx)
            .append(Doc::line_or_space().nest(ctx.indent_width))
            .append(self.end.doc(ctx))
            .group()
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

impl<'s> DocGen<'s> for UnquotedFontFamilyName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            itertools::intersperse(self.idents.iter().map(|ident| ident.doc(ctx)), Doc::space())
                .collect(),
        )
    }
}
