use super::DocGen;
use crate::ctx::Ctx;
use raffia::{ast::*, Syntax};
use tiny_pretty::Doc;

impl DocGen for Declaration<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let mut docs = Vec::with_capacity(3);
        docs.push(self.name.doc(ctx));
        if let Some(less_property_merge) = &self.less_property_merge {
            docs.push(less_property_merge.doc(ctx));
        }
        docs.push(Doc::text(":"));

        Doc::list(docs)
    }
}

impl DocGen for QualifiedRule<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        self.selector
            .doc(ctx)
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
        self.statements.iter().for_each(|stmt| {
            stmts.push(Doc::hardline());
            stmts.push(stmt.doc(ctx));
        });
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
            Statement::Declaration(declaration) => declaration.doc(ctx),
            Statement::QualifiedRule(qualified_rule) => qualified_rule.doc(ctx),
            _ => todo!(),
        };
        if ctx.syntax == Syntax::Sass {
            stmt
        } else {
            match self {
                Statement::Declaration(..) => stmt.append(Doc::text(";")),
                Statement::QualifiedRule(..) => stmt,
                _ => stmt,
            }
        }
    }
}

impl DocGen for Stylesheet<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let mut stmts = Vec::with_capacity(self.statements.len() * 2);
        let mut iter = self.statements.iter();
        if let Some(first) = iter.next() {
            stmts.push(first.doc(ctx));
        }
        iter.for_each(|stmt| {
            stmts.push(Doc::hardline());
            stmts.push(stmt.doc(ctx));
        });
        Doc::list(stmts)
    }
}
