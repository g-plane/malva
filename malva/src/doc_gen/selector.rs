use super::{helpers, DocGen};
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for AnPlusB {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
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

impl<'s> DocGen<'s> for AttributeSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
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

impl<'s> DocGen<'s> for AttributeSelectorMatcher {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
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

impl<'s> DocGen<'s> for AttributeSelectorModifier<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match &self.ident {
            InterpolableIdent::Literal(ident) if matches!(&*ident.name, "I" | "S") => {
                Doc::text(ident.name.to_ascii_lowercase())
            }
            _ => self.ident.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for AttributeSelectorValue<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            AttributeSelectorValue::Ident(ident) => ident.doc(ctx),
            AttributeSelectorValue::Str(str) => str.doc(ctx),
            AttributeSelectorValue::Percentage(percentage) => percentage.doc(ctx),
            AttributeSelectorValue::LessEscapedStr(less_escaped_str) => less_escaped_str.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for ClassSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text(".").append(self.name.doc(ctx))
    }
}

impl<'s> DocGen<'s> for Combinator {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text(match self.kind {
            CombinatorKind::Descendant => " ",
            CombinatorKind::Child => ">",
            CombinatorKind::LaterSibling => "~",
            CombinatorKind::NextSibling => "+",
            CombinatorKind::Column => "||",
        })
    }
}

impl<'s> DocGen<'s> for ComplexSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
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

impl<'s> DocGen<'s> for CompoundSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            self.children
                .iter()
                .map(|selector| selector.doc(ctx))
                .collect(),
        )
    }
}

impl<'s> DocGen<'s> for CompoundSelectorList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            itertools::intersperse(
                self.selectors.iter().map(|selector| selector.doc(ctx)),
                Doc::text(", "),
            )
            .collect(),
        )
    }
}

impl<'s> DocGen<'s> for IdSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("#").append(self.name.doc(ctx))
    }
}

impl<'s> DocGen<'s> for LanguageRange<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            LanguageRange::Ident(ident) => ident.doc(ctx),
            LanguageRange::Str(str) => str.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for LanguageRangeList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            itertools::intersperse(
                self.ranges.iter().map(|selector| selector.doc(ctx)),
                Doc::text(", "),
            )
            .collect(),
        )
    }
}

impl<'s> DocGen<'s> for NestingSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let ampersand = Doc::text("&");
        if let Some(suffix) = &self.suffix {
            ampersand.append(suffix.doc(ctx))
        } else {
            ampersand
        }
    }
}

impl<'s> DocGen<'s> for NsPrefix<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let bar = Doc::text("|");
        if let Some(kind) = &self.kind {
            kind.doc(ctx).append(bar)
        } else {
            bar
        }
    }
}

impl<'s> DocGen<'s> for NsPrefixKind<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            NsPrefixKind::Ident(ident) => ident.doc(ctx),
            NsPrefixKind::Universal(..) => Doc::text("*"),
        }
    }
}

impl<'s> DocGen<'s> for Nth<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let index = self.index.doc(ctx);
        if let Some(matcher) = &self.matcher {
            index.append(Doc::space()).append(matcher.doc(ctx))
        } else {
            index
        }
    }
}

impl<'s> DocGen<'s> for NthIndex<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            Self::AnPlusB(an_plus_b) => an_plus_b.doc(ctx),
            Self::Odd(..) => Doc::text("odd"),
            Self::Even(..) => Doc::text("even"),
            Self::Integer(integer) => Doc::text((integer.value as i32).to_string()),
        }
    }
}

impl<'s> DocGen<'s> for NthMatcher<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let matcher = Doc::text("of");
        if let Some(selector) = &self.selector {
            matcher.append(Doc::space()).append(selector.doc(ctx))
        } else {
            matcher
        }
    }
}

impl<'s> DocGen<'s> for PseudoClassSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = vec![Doc::text(":"), helpers::ident_to_lowercase(&self.name, ctx)];

        if let Some(arg) = &self.arg {
            docs.reserve(3);
            docs.push(Doc::text("("));
            docs.push(match arg {
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
                PseudoClassSelectorArg::RelativeSelectorList(relative_selector_list) => {
                    relative_selector_list.doc(ctx)
                }
                PseudoClassSelectorArg::SelectorList(selector_list) => selector_list.doc(ctx),
                PseudoClassSelectorArg::LessExtendList(less_extend_list) => {
                    less_extend_list.doc(ctx)
                }
                PseudoClassSelectorArg::TokenSeq(token_seq) => format_pseudo_selector_arg_tokens(
                    token_seq,
                    ctx,
                    self.span.start,
                    self.span.end,
                ),
            });
            docs.push(Doc::text(")"));
        }
        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for PseudoElementSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = vec![
            Doc::text("::"),
            helpers::ident_to_lowercase(&self.name, ctx),
        ];

        if let Some(arg) = &self.arg {
            docs.reserve(3);
            docs.push(Doc::text("("));
            docs.push(match arg {
                PseudoElementSelectorArg::CompoundSelector(compound_selector) => {
                    compound_selector.doc(ctx)
                }
                PseudoElementSelectorArg::Ident(ident) => ident.doc(ctx),
                PseudoElementSelectorArg::TokenSeq(token_seq) => format_pseudo_selector_arg_tokens(
                    token_seq,
                    ctx,
                    self.span.start,
                    self.span.end,
                ),
            });
            docs.push(Doc::text(")"));
        }
        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for RelativeSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        if let Some(combinator) = &self.combinator {
            combinator
                .doc(ctx)
                .append(Doc::space())
                .append(self.complex_selector.doc(ctx))
        } else {
            self.complex_selector.doc(ctx)
        }
    }
}

impl<'s> DocGen<'s> for RelativeSelectorList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            itertools::intersperse(
                self.selectors.iter().map(|selector| selector.doc(ctx)),
                Doc::text(", "),
            )
            .collect(),
        )
    }
}

impl<'s> DocGen<'s> for SimpleSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
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

impl<'s> DocGen<'s> for SelectorList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            itertools::intersperse(
                self.selectors.iter().map(|selector| selector.doc(ctx)),
                Doc::text(", "),
            )
            .collect(),
        )
    }
}

impl<'s> DocGen<'s> for TagNameSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.name.doc(ctx)
    }
}

impl<'s> DocGen<'s> for TypeSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            TypeSelector::TagName(selector) => selector.doc(ctx),
            TypeSelector::Universal(selector) => selector.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for UniversalSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let asterisk = Doc::text("*");
        if let Some(prefix) = &self.prefix {
            prefix.doc(ctx).append(asterisk)
        } else {
            asterisk
        }
    }
}

impl<'s> DocGen<'s> for WqName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let name = if let InterpolableIdent::Literal(ident) = &self.name {
            Doc::text(ident.raw.to_ascii_lowercase())
        } else {
            self.name.doc(ctx)
        };
        if let Some(prefix) = &self.prefix {
            prefix.doc(ctx).append(name)
        } else {
            name
        }
    }
}

fn format_pseudo_selector_arg_tokens<'a, 's: 'a>(
    token_seq: &TokenSeq<'s>,
    ctx: &Ctx<'a, 's>,
    from: usize,
    to: usize,
) -> Doc<'s> {
    use raffia::token::{Token, TokenWithSpan};

    let mut pos = from;
    let mut docs = Vec::with_capacity(token_seq.tokens.len() * 2);
    let mut iter = token_seq.tokens.iter().peekable();
    while let Some(token) = iter.next() {
        docs.extend(ctx.end_padded_comments(pos, token.span.start));

        docs.push(token.doc(ctx));
        if let TokenWithSpan {
            token: Token::Comma(..) | Token::Semicolon(..),
            ..
        } = token
        {
            docs.push(Doc::space());
        } else {
            match iter.peek() {
                Some(TokenWithSpan {
                    token: Token::Comma(..) | Token::Semicolon(..),
                    ..
                }) => {}
                Some(next) if token.span.end < next.span.start => docs.push(Doc::space()),
                _ => {}
            }
        }

        pos = token.span.end;
    }

    docs.extend(ctx.start_padded_comments(pos, to));

    Doc::list(docs)
}
