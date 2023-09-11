use super::DocGen;
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl DocGen for AnPlusB {
    fn doc(&self, _: &Ctx) -> Doc {
        let a = match self.a {
            0 => Doc::nil(),
            1 => Doc::text("n"),
            -1 => Doc::text("-n"),
            a => Doc::text(format!("{a}n")),
        };
        let b = match self.b {
            0 => Doc::nil(),
            b if b > 0 => Doc::text(format!("+{b}")),
            b => Doc::text(b.to_string()),
        };
        a.append(b)
    }
}

impl DocGen for AttributeSelector<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let mut docs = Vec::with_capacity(5);
        docs.push(Doc::text("["));
        docs.push(self.name.doc(ctx));
        if let Some((matcher, value)) = self.matcher.as_ref().zip(self.value.as_ref()) {
            docs.push(matcher.doc(ctx));
            docs.push(value.doc(ctx));
            if let Some(modifier) = &self.modifier {
                docs.reserve(2);
                docs.push(Doc::space());
                docs.push(modifier.doc(ctx));
            }
        }
        docs.push(Doc::text("]"));
        Doc::list(docs)
    }
}

impl DocGen for AttributeSelectorMatcher {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text(match self.kind {
            AttributeSelectorMatcherKind::Exact => "=",
            AttributeSelectorMatcherKind::MatchWord => "~=",
            AttributeSelectorMatcherKind::ExactOrPrefixThenHyphen => "|=",
            AttributeSelectorMatcherKind::Prefix => "^=",
            AttributeSelectorMatcherKind::Suffix => "$=",
            AttributeSelectorMatcherKind::Substring => "*=",
        })
    }
}

impl DocGen for AttributeSelectorModifier<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match &self.ident {
            InterpolableIdent::Literal(ident) if matches!(&*ident.name, "I" | "S") => {
                Doc::text(ident.name.to_ascii_lowercase())
            }
            _ => self.ident.doc(ctx),
        }
    }
}

impl DocGen for AttributeSelectorValue<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            AttributeSelectorValue::Ident(ident) => ident.doc(ctx),
            AttributeSelectorValue::Str(str) => str.doc(ctx),
            AttributeSelectorValue::Percentage(percentage) => percentage.doc(ctx),
            AttributeSelectorValue::LessEscapedStr(less_escaped_str) => less_escaped_str.doc(ctx),
        }
    }
}

impl DocGen for ClassSelector<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::text(".").append(self.name.doc(ctx))
    }
}

impl DocGen for Combinator {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text(match self.kind {
            CombinatorKind::Descendant => " ",
            CombinatorKind::Child => ">",
            CombinatorKind::LaterSibling => "~",
            CombinatorKind::NextSibling => "+",
            CombinatorKind::Column => "||",
        })
    }
}

impl DocGen for ComplexSelector<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let mut docs = Vec::with_capacity(self.children.len() * 2);

        let mut children = self.children.iter();
        if let Some(first) = children.next() {
            docs.push(match first {
                ComplexSelectorChild::CompoundSelector(selector) => selector.doc(ctx),
                ComplexSelectorChild::Combinator(combinator) => combinator.doc(ctx),
            });
        }

        Doc::list(children.fold(docs, |mut docs, child| match child {
            ComplexSelectorChild::CompoundSelector(selector) => {
                docs.push(Doc::space());
                docs.push(selector.doc(ctx));
                docs
            }
            ComplexSelectorChild::Combinator(Combinator {
                kind: CombinatorKind::Descendant,
                ..
            }) => docs,
            ComplexSelectorChild::Combinator(combinator) => {
                docs.push(Doc::space());
                docs.push(combinator.doc(ctx));
                docs
            }
        }))
    }
}

impl DocGen for CompoundSelector<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::list(
            self.children
                .iter()
                .map(|selector| selector.doc(ctx))
                .collect(),
        )
    }
}

impl DocGen for CompoundSelectorList<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::list(
            itertools::intersperse(
                self.selectors.iter().map(|selector| selector.doc(ctx)),
                Doc::text(", "),
            )
            .collect(),
        )
    }
}

impl DocGen for IdSelector<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::text("#").append(self.name.doc(ctx))
    }
}

impl DocGen for LanguageRange<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            LanguageRange::Ident(ident) => ident.doc(ctx),
            LanguageRange::Str(str) => str.doc(ctx),
        }
    }
}

impl DocGen for LanguageRangeList<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::list(
            itertools::intersperse(
                self.ranges.iter().map(|selector| selector.doc(ctx)),
                Doc::text(", "),
            )
            .collect(),
        )
    }
}

impl DocGen for NestingSelector<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let ampersand = Doc::text("&");
        if let Some(suffix) = &self.suffix {
            ampersand.append(suffix.doc(ctx))
        } else {
            ampersand
        }
    }
}

impl DocGen for NsPrefix<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let bar = Doc::text("|");
        if let Some(kind) = &self.kind {
            kind.doc(ctx).append(bar)
        } else {
            bar
        }
    }
}

impl DocGen for NsPrefixKind<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            NsPrefixKind::Ident(ident) => ident.doc(ctx),
            NsPrefixKind::Universal(..) => Doc::text("*"),
        }
    }
}

impl DocGen for Nth<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let index = self.index.doc(ctx);
        if let Some(matcher) = &self.matcher {
            let doc = index.append(Doc::text(" of"));
            if let Some(selector) = &matcher.selector {
                doc.append(Doc::space()).append(selector.doc(ctx))
            } else {
                doc
            }
        } else {
            index
        }
    }
}

impl DocGen for NthIndex<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            Self::AnPlusB(an_plus_b) => an_plus_b.doc(ctx),
            Self::Odd(..) => Doc::text("odd"),
            Self::Even(..) => Doc::text("even"),
            Self::Integer(integer) => Doc::text((integer.value as i32).to_string()),
        }
    }
}

impl DocGen for PseudoClassSelector<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let name = match &self.name {
            InterpolableIdent::Literal(literal) => Doc::text(literal.raw.to_ascii_lowercase()),
            _ => self.name.doc(ctx),
        };
        let mut docs = vec![Doc::text(":"), name];

        if let Some(arg) = &self.arg {
            docs.reserve(3);
            docs.push(Doc::text("("));
            docs.push(arg.doc(ctx));
            docs.push(Doc::text(")"));
        }
        Doc::list(docs)
    }
}

impl DocGen for PseudoClassSelectorArg<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            PseudoClassSelectorArg::CompoundSelector(compound_selector) => {
                compound_selector.doc(ctx)
            }
            PseudoClassSelectorArg::CompoundSelectorList(compound_selector_list) => {
                compound_selector_list.doc(ctx)
            }
            PseudoClassSelectorArg::Ident(ident) => ident.doc(ctx),
            PseudoClassSelectorArg::LanguageRangeList(language_range_list) => {
                language_range_list.doc(ctx)
            }
            PseudoClassSelectorArg::Nth(nth) => nth.doc(ctx),
            PseudoClassSelectorArg::Number(number) => number.doc(ctx),
            PseudoClassSelectorArg::RelativeSelectorList(relative_selector_list) => todo!(),
            PseudoClassSelectorArg::SelectorList(selector_list) => selector_list.doc(ctx),
            PseudoClassSelectorArg::LessExtendList(less_extend_list) => todo!(),
            PseudoClassSelectorArg::TokenSeq(token_seq) => todo!(),
        }
    }
}

impl DocGen for PseudoElementSelector<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let name = match &self.name {
            InterpolableIdent::Literal(literal) => Doc::text(literal.raw.to_ascii_lowercase()),
            _ => self.name.doc(ctx),
        };
        let mut docs = vec![Doc::text("::"), name];

        if let Some(arg) = &self.arg {
            docs.reserve(3);
            docs.push(Doc::text("("));
            docs.push(arg.doc(ctx));
            docs.push(Doc::text(")"));
        }
        Doc::list(docs)
    }
}

impl DocGen for PseudoElementSelectorArg<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            PseudoElementSelectorArg::CompoundSelector(compound_selector) => {
                compound_selector.doc(ctx)
            }
            PseudoElementSelectorArg::Ident(ident) => ident.doc(ctx),
            PseudoElementSelectorArg::TokenSeq(token_seq) => todo!(),
        }
    }
}

impl DocGen for SimpleSelector<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            SimpleSelector::Class(selector) => selector.doc(ctx),
            SimpleSelector::Id(selector) => selector.doc(ctx),
            SimpleSelector::Type(selector) => selector.doc(ctx),
            SimpleSelector::Attribute(selector) => selector.doc(ctx),
            SimpleSelector::PseudoClass(selector) => selector.doc(ctx),
            SimpleSelector::PseudoElement(selector) => selector.doc(ctx),
            SimpleSelector::Nesting(selector) => selector.doc(ctx),
            SimpleSelector::SassPlaceholder(selector) => selector.doc(ctx),
        }
    }
}

impl DocGen for SelectorList<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::list(
            itertools::intersperse(
                self.selectors.iter().map(|selector| selector.doc(ctx)),
                Doc::text(", "),
            )
            .collect(),
        )
    }
}

impl DocGen for TagNameSelector<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        self.name.doc(ctx)
    }
}

impl DocGen for TypeSelector<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            TypeSelector::TagName(selector) => selector.doc(ctx),
            TypeSelector::Universal(selector) => selector.doc(ctx),
        }
    }
}

impl DocGen for UniversalSelector<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let asterisk = Doc::text("*");
        if let Some(prefix) = &self.prefix {
            prefix.doc(ctx).append(asterisk)
        } else {
            asterisk
        }
    }
}

impl DocGen for WqName<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        if let Some(prefix) = &self.prefix {
            prefix.doc(ctx).append(self.name.doc(ctx))
        } else {
            self.name.doc(ctx)
        }
    }
}
