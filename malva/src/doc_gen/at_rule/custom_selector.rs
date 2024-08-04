use super::{super::DocGen, helpers};
use crate::{ctx::Ctx, state::State};
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for CustomSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = Vec::with_capacity(1);

        if let Some(prefix_arg) = &self.prefix_arg {
            docs.push(prefix_arg.doc(ctx, state));
        }

        docs.push(Doc::text(":"));
        docs.push(self.name.doc(ctx, state));

        if let Some(args) = &self.args {
            docs.push(args.doc(ctx, state));
        }

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for CustomSelectorArg<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        Doc::text("$").append(self.name.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for CustomSelectorArgs<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        helpers::format_parenthesized(
            helpers::SeparatedListFormatter::new(
                ",",
                helpers::get_smart_linebreak(
                    self.span.start,
                    &self.args,
                    ctx.options.selectors_prefer_single_line,
                    ctx,
                ),
            )
            .format(&self.args, &self.comma_spans, self.span.start, ctx, state),
            self.args
                .last()
                .map(|arg| arg.span.end)
                .unwrap_or(self.span.start),
            self.span.end,
            ctx,
        )
    }
}

impl<'s> DocGen<'s> for CustomSelectorPrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.custom_selector
            .doc(ctx, state)
            .append(Doc::line_or_space().nest(ctx.indent_width))
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.custom_selector.span.end, self.selector.span.start),
            ))
            .append(self.selector.doc(ctx, state))
            .group()
    }
}
