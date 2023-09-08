use super::DocGen;
use crate::ctx::Ctx;
use raffia::{ast::*, Spanned, Syntax};
use tiny_pretty::Doc;

impl DocGen for Declaration<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let mut docs = Vec::with_capacity(3);
        docs.push(self.name.doc(ctx));
        if let Some(less_property_merge) = &self.less_property_merge {
            docs.push(less_property_merge.doc(ctx));
        }
        docs.push(Doc::text(": "));

        let mut values = Vec::with_capacity(self.value.len() * 2);
        let mut iter = self.value.iter();
        if let Some(first) = iter.next() {
            values.push(first.doc(ctx));
        }
        iter.for_each(|value| {
            if !matches!(
                value,
                ComponentValue::Delimiter(Delimiter {
                    kind: DelimiterKind::Comma | DelimiterKind::Semicolon,
                    ..
                })
            ) {
                values.push(Doc::softline());
            }
            values.push(value.doc(ctx));
        });

        if let Some(important) = &self.important {
            values.push(Doc::softline());
            values.push(important.doc(ctx));
        }

        docs.push(Doc::list(values).nest(ctx.indent_width));

        Doc::list(docs)
    }
}

impl DocGen for ImportantAnnotation<'_> {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text("!important")
    }
}

impl DocGen for QualifiedRule<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        use crate::config::QualifiedRuleSelectorLineBreak;

        // we don't use `SelectorList::doc` here
        // because it's a special case for qualified rule
        Doc::list(
            itertools::intersperse(
                self.selector
                    .selectors
                    .iter()
                    .map(|selector| selector.doc(ctx)),
                Doc::text(",").append(match ctx.options.qualified_rule_selector_linebreak {
                    QualifiedRuleSelectorLineBreak::Always => Doc::hardline(),
                    QualifiedRuleSelectorLineBreak::Consistent => Doc::line_or_space(),
                    QualifiedRuleSelectorLineBreak::Wrap => Doc::softline(),
                }),
            )
            .collect(),
        )
        .group()
        .append(Doc::space())
        .append(self.block.doc(ctx))
    }
}

impl DocGen for SimpleBlock<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let is_sass = ctx.syntax == Syntax::Sass;
        let mut docs = vec![];

        if !is_sass {
            docs.push(Doc::text("{"));
        }

        let mut stmts = Vec::with_capacity(self.statements.len() * 2);
        let mut iter = self.statements.iter().peekable();
        while let Some(stmt) = iter.next() {
            stmts.push(Doc::hardline());
            stmts.push(stmt.doc(ctx));
            if let Some(next) = iter.peek() {
                if ctx
                    .line_bounds
                    .is_away_more_than_one_line(stmt.span().end, next.span().start)
                {
                    stmts.push(Doc::hardline());
                }
            }
        }
        docs.push(Doc::list(stmts).nest(ctx.indent_width));
        docs.push(Doc::hardline());

        if !is_sass {
            docs.push(Doc::text("}"));
        }

        Doc::list(docs)
    }
}

impl DocGen for Statement<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let stmt = match self {
            Statement::AtRule(at_rule) => at_rule.doc(ctx),
            Statement::Declaration(declaration) => declaration.doc(ctx),
            Statement::QualifiedRule(qualified_rule) => qualified_rule.doc(ctx),
            _ => todo!(),
        };
        if ctx.syntax == Syntax::Sass {
            stmt
        } else {
            match self {
                Statement::AtRule(at_rule) if at_rule.block.is_none() => {
                    stmt.append(Doc::text(";"))
                }
                Statement::Declaration(decl)
                    if !matches!(
                        decl.value.last(),
                        Some(ComponentValue::SassNestingDeclaration(..))
                    ) =>
                {
                    stmt.append(Doc::text(";"))
                }
                _ => stmt,
            }
        }
    }
}

impl DocGen for Stylesheet<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let mut stmts = Vec::with_capacity(self.statements.len() * 2);
        let mut iter = self.statements.iter().peekable();
        while let Some(stmt) = iter.next() {
            stmts.push(stmt.doc(ctx));
            stmts.push(Doc::hardline());
            if let Some(next) = iter.peek() {
                if ctx
                    .line_bounds
                    .is_away_more_than_one_line(stmt.span().end, next.span().start)
                {
                    stmts.push(Doc::hardline());
                }
            }
        }
        Doc::list(stmts)
    }
}
