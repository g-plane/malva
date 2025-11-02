use super::{
    DocGen, helpers,
    str::{CssStrRawFormatter, format_str, is_preferred_quote_allowed},
};
use crate::{ctx::Ctx, state::State};
use raffia::{Spanned, ast::*};
use std::mem;
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for AnPlusB {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
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
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = Vec::with_capacity(5);
        docs.push(Doc::text("["));
        docs.extend(
            ctx.end_spaced_comments(
                ctx.get_comments_between(self.span.start, self.name.span.start),
            ),
        );
        docs.push(self.name.doc(ctx, state));

        let mut pos = self.name.span.end;
        if let Some((matcher, value)) = self.matcher.as_ref().zip(self.value.as_ref()) {
            docs.extend(ctx.end_spaced_comments(ctx.get_comments_between(
                mem::replace(&mut pos, matcher.span.end),
                matcher.span.start,
            )));
            docs.push(matcher.doc(ctx, state));

            let value_span = value.span();
            docs.extend(ctx.end_spaced_comments(
                ctx.get_comments_between(mem::replace(&mut pos, value_span.end), value_span.start),
            ));
            docs.push(value.doc(ctx, state));
            if let Some(modifier) = &self.modifier {
                docs.reserve(2);
                docs.push(Doc::space());
                docs.extend(ctx.end_spaced_comments(ctx.get_comments_between(
                    mem::replace(&mut pos, modifier.span.end),
                    modifier.span.start,
                )));
                docs.push(modifier.doc(ctx, state));
            }
        }

        docs.extend(ctx.start_spaced_comments(ctx.get_comments_between(pos, self.span.end)));
        docs.push(Doc::text("]"));
        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for AttributeSelectorMatcher {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
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
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match &self.ident {
            InterpolableIdent::Literal(ident) if matches!(&*ident.name, "I" | "S") => {
                Doc::text(ident.name.to_ascii_lowercase())
            }
            _ => self.ident.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for AttributeSelectorValue<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        use crate::config::{AttrValueQuotes, Quotes};

        let quotes = ctx
            .options
            .attr_selector_quotes
            .unwrap_or(ctx.options.quotes);

        match self {
            AttributeSelectorValue::Ident(ident) => match ctx.options.attr_value_quotes {
                AttrValueQuotes::Always => match quotes {
                    Quotes::AlwaysDouble | Quotes::PreferDouble => Doc::text("\"")
                        .append(ident.doc(ctx, state))
                        .append(Doc::text("\"")),
                    Quotes::AlwaysSingle | Quotes::PreferSingle => Doc::text("'")
                        .append(ident.doc(ctx, state))
                        .append(Doc::text("'")),
                },
                AttrValueQuotes::Ignore => ident.doc(ctx, state),
            },
            AttributeSelectorValue::Str(InterpolableStr::Literal(str)) => Doc::text(format_str(
                str.raw,
                CssStrRawFormatter::new(str.raw),
                is_preferred_quote_allowed(str.raw, quotes),
                quotes,
            )),
            AttributeSelectorValue::Str(str) => str.doc(ctx, state),
            AttributeSelectorValue::Percentage(percentage) => percentage.doc(ctx, state),
            AttributeSelectorValue::LessEscapedStr(less_escaped_str) => {
                less_escaped_str.doc(ctx, state)
            }
        }
    }
}

impl<'s> DocGen<'s> for ClassSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        Doc::text(".").append(self.name.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for Combinator {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
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
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = Vec::with_capacity(self.children.len() * 2);
        self.children.iter().fold(
            (None, self.span.start),
            |(prev_compound, pos), child| match child {
                ComplexSelectorChild::CompoundSelector(selector) => {
                    docs.extend(
                        ctx.end_spaced_comments(ctx.get_comments_between(pos, selector.span.start)),
                    );
                    if prev_compound.is_some() {
                        docs.push(selector.doc(ctx, state).nest(ctx.indent_width));
                    } else {
                        docs.push(selector.doc(ctx, state));
                    }
                    (Some(selector), selector.span.end)
                }
                ComplexSelectorChild::Combinator(Combinator {
                    kind: CombinatorKind::Descendant,
                    span,
                }) => {
                    let mut has_last_line_comment = false;
                    let mut pos = pos;
                    docs.extend(
                        ctx.start_spaced_comments_without_last_hard_line(
                            ctx.get_comments_between(pos, span.end)
                                .take_while(|comment| {
                                    ctx.line_bounds
                                        .line_distance(span.start, comment.span.start)
                                        == 0
                                })
                                .inspect(|comment| pos = comment.span.end),
                            &mut has_last_line_comment,
                        ),
                    );
                    if has_last_line_comment {
                        docs.push(Doc::hard_line().nest(ctx.indent_width));
                    } else if let Some(CompoundSelector { children, .. }) = prev_compound {
                        if let [SimpleSelector::Type(..) | SimpleSelector::Nesting(..)] =
                            &children[..]
                        {
                            docs.push(Doc::space());
                        } else {
                            docs.push(Doc::line_or_space().nest(ctx.indent_width));
                        }
                    }
                    (prev_compound, pos)
                }
                ComplexSelectorChild::Combinator(combinator) => {
                    if let Some(CompoundSelector { children, .. }) = prev_compound {
                        if let [SimpleSelector::Type(..) | SimpleSelector::Nesting(..)] =
                            &children[..]
                        {
                            docs.push(Doc::space());
                        } else {
                            docs.push(Doc::line_or_space().nest(ctx.indent_width));
                        }
                    }
                    docs.extend(
                        ctx.end_spaced_comments(
                            ctx.get_comments_between(pos, combinator.span.start),
                        ),
                    );
                    docs.push(combinator.doc(ctx, state));
                    docs.push(Doc::space());
                    (prev_compound, combinator.span.end)
                }
            },
        );

        Doc::list(docs).group()
    }
}

impl<'s> DocGen<'s> for CompoundSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        Doc::list(
            self.children
                .iter()
                .scan(self.span.start, |pos, selector| {
                    let selector_span = selector.span();
                    Some(
                        Doc::list(
                            ctx.unspaced_comments(ctx.get_comments_between(
                                mem::replace(pos, selector_span.end),
                                selector_span.start,
                            ))
                            .collect(),
                        )
                        .append(selector.doc(ctx, state)),
                    )
                })
                .collect(),
        )
    }
}

impl<'s> DocGen<'s> for CompoundSelectorList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        helpers::SeparatedListFormatter::new(",", Doc::space()).format(
            &self.selectors,
            &self.comma_spans,
            self.span.start,
            ctx,
            state,
        )
    }
}

impl<'s> DocGen<'s> for IdSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        Doc::text("#").append(self.name.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for LanguageRange<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            LanguageRange::Ident(ident) => ident.doc(ctx, state),
            LanguageRange::Str(str) => str.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for LanguageRangeList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        Doc::list(
            itertools::intersperse(
                self.ranges.iter().map(|selector| selector.doc(ctx, state)),
                Doc::text(", "),
            )
            .collect(),
        )
    }
}

impl<'s> DocGen<'s> for NestingSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let ampersand = Doc::text("&");
        if let Some(suffix) = &self.suffix {
            ampersand.append(suffix.doc(ctx, state))
        } else {
            ampersand
        }
    }
}

impl<'s> DocGen<'s> for NsPrefix<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let bar = Doc::text("|");
        if let Some(kind) = &self.kind {
            kind.doc(ctx, state)
                .concat(
                    ctx.unspaced_comments(ctx.get_comments_between(kind.span().end, self.span.end)),
                )
                .append(bar)
        } else {
            bar
        }
    }
}

impl<'s> DocGen<'s> for NsPrefixKind<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            NsPrefixKind::Ident(ident) => ident.doc(ctx, state),
            NsPrefixKind::Universal(..) => Doc::text("*"),
        }
    }
}

impl<'s> DocGen<'s> for Nth<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let index = self.index.doc(ctx, state);
        if let Some(matcher) = &self.matcher {
            index.append(Doc::space()).append(matcher.doc(ctx, state))
        } else {
            index
        }
    }
}

impl<'s> DocGen<'s> for NthIndex<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            Self::AnPlusB(an_plus_b) => an_plus_b.doc(ctx, state),
            Self::Odd(..) => Doc::text("odd"),
            Self::Even(..) => Doc::text("even"),
            Self::Integer(integer) => Doc::text((integer.value as i32).to_string()),
        }
    }
}

impl<'s> DocGen<'s> for NthMatcher<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let matcher = Doc::text("of");
        if let Some(selector) = &self.selector {
            matcher
                .append(Doc::space())
                .append(selector.doc(ctx, state))
        } else {
            matcher
        }
    }
}

impl<'s> DocGen<'s> for PseudoClassSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = vec![Doc::text(":")];
        docs.extend(
            ctx.unspaced_comments(
                ctx.get_comments_between(self.span.start, self.name.span().start),
            ),
        );
        docs.push(helpers::ident_to_lowercase(&self.name, ctx, state));

        if let Some(arg) = &self.arg {
            docs.push(Doc::text("("));

            let arg_span = arg.kind.span();
            let force_break = ctx
                .line_bounds
                .line_distance(arg.l_paren.end, arg_span.start)
                > 0;
            let mut arg_doc = vec![];

            if ctx.options.linebreak_in_pseudo_parens {
                if force_break {
                    arg_doc.push(Doc::hard_line());
                } else {
                    arg_doc.push(Doc::line_or_nil());
                }
                let mut comment_end = None;
                arg_doc.extend(ctx.end_spaced_comments_without_last_space(
                    ctx.get_comments_between(arg.l_paren.end, arg_span.start),
                    &mut comment_end,
                ));
                if let Some(comment_end) = comment_end {
                    if ctx.line_bounds.line_distance(comment_end, arg_span.start) > 0 {
                        arg_doc.push(Doc::hard_line());
                    } else {
                        arg_doc.push(Doc::soft_line());
                    }
                }
            } else {
                arg_doc.extend(ctx.end_spaced_comments(
                    ctx.get_comments_between(arg.l_paren.end, arg_span.start),
                ));
            }

            arg_doc.push(match &arg.kind {
                PseudoClassSelectorArgKind::CompoundSelector(compound_selector) => {
                    compound_selector.doc(ctx, state)
                }
                PseudoClassSelectorArgKind::CompoundSelectorList(compound_selector_list) => {
                    compound_selector_list.doc(ctx, state)
                }
                PseudoClassSelectorArgKind::Ident(ident) => ident.doc(ctx, state),
                PseudoClassSelectorArgKind::LanguageRangeList(language_range_list) => {
                    language_range_list.doc(ctx, state)
                }
                PseudoClassSelectorArgKind::Nth(nth) => nth.doc(ctx, state),
                PseudoClassSelectorArgKind::Number(number) => number.doc(ctx, state),
                PseudoClassSelectorArgKind::RelativeSelectorList(relative_selector_list) => {
                    relative_selector_list.doc(ctx, state)
                }
                PseudoClassSelectorArgKind::SelectorList(selector_list) => {
                    if ctx.options.linebreak_in_pseudo_parens {
                        let doc = selector_list.doc(ctx, state);
                        if force_break { doc } else { doc.group() }
                    } else {
                        helpers::SeparatedListFormatter::new(",", Doc::space()).format(
                            &selector_list.selectors,
                            &selector_list.comma_spans,
                            selector_list.span.start,
                            ctx,
                            state,
                        )
                    }
                }
                PseudoClassSelectorArgKind::LessExtendList(less_extend_list) => {
                    less_extend_list.doc(ctx, state)
                }
                PseudoClassSelectorArgKind::TokenSeq(token_seq) => {
                    format_pseudo_selector_arg_tokens(
                        token_seq,
                        ctx,
                        state,
                        token_seq.span.start,
                        token_seq.span.end,
                    )
                }
            });

            arg_doc.extend(
                ctx.start_spaced_comments(
                    ctx.get_comments_between(arg_span.end, arg.r_paren.start),
                ),
            );
            if ctx.options.linebreak_in_pseudo_parens {
                docs.push(
                    Doc::list(arg_doc)
                        .nest(ctx.indent_width)
                        .append(Doc::line_or_nil())
                        .group(),
                );
            } else {
                docs.append(&mut arg_doc);
            }
            docs.push(Doc::text(")"));
        }

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for PseudoElementSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = vec![Doc::text("::")];
        docs.extend(
            ctx.unspaced_comments(
                ctx.get_comments_between(self.span.start, self.name.span().start),
            ),
        );
        docs.push(helpers::ident_to_lowercase(&self.name, ctx, state));

        if let Some(arg) = &self.arg {
            docs.push(Doc::text("("));

            let arg_span = arg.kind.span();
            let mut arg_doc = vec![];

            if ctx.options.linebreak_in_pseudo_parens {
                arg_doc.push(Doc::line_or_nil());
                let mut comment_end = None;
                arg_doc.extend(ctx.end_spaced_comments_without_last_space(
                    ctx.get_comments_between(arg.l_paren.end, arg_span.start),
                    &mut comment_end,
                ));
                if let Some(comment_end) = comment_end {
                    if ctx.line_bounds.line_distance(comment_end, arg_span.start) > 0 {
                        arg_doc.push(Doc::hard_line());
                    } else {
                        arg_doc.push(Doc::soft_line());
                    }
                }
            } else {
                arg_doc.extend(ctx.end_spaced_comments(
                    ctx.get_comments_between(arg.l_paren.end, arg_span.start),
                ));
            }

            arg_doc.push(match &arg.kind {
                PseudoElementSelectorArgKind::CompoundSelector(compound_selector) => {
                    compound_selector.doc(ctx, state)
                }
                PseudoElementSelectorArgKind::Ident(ident) => ident.doc(ctx, state),
                PseudoElementSelectorArgKind::TokenSeq(token_seq) => {
                    format_pseudo_selector_arg_tokens(
                        token_seq,
                        ctx,
                        state,
                        token_seq.span.start,
                        token_seq.span.end,
                    )
                }
            });

            arg_doc.extend(
                ctx.start_spaced_comments(
                    ctx.get_comments_between(arg_span.end, arg.r_paren.start),
                ),
            );
            if ctx.options.linebreak_in_pseudo_parens {
                docs.push(
                    Doc::list(arg_doc)
                        .nest(ctx.indent_width)
                        .append(Doc::line_or_nil())
                        .group(),
                );
            } else {
                docs.append(&mut arg_doc);
            }
            docs.push(Doc::text(")"));
        }

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for RelativeSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        if let Some(combinator) = &self.combinator {
            combinator
                .doc(ctx, state)
                .append(Doc::space())
                .append(self.complex_selector.doc(ctx, state))
        } else {
            self.complex_selector.doc(ctx, state)
        }
    }
}

impl<'s> DocGen<'s> for RelativeSelectorList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        helpers::SeparatedListFormatter::new(",", Doc::space()).format(
            &self.selectors,
            &self.comma_spans,
            self.span.start,
            ctx,
            state,
        )
    }
}

impl<'s> DocGen<'s> for SimpleSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            SimpleSelector::Class(selector) => selector.doc(ctx, state),
            SimpleSelector::Id(selector) => selector.doc(ctx, state),
            SimpleSelector::Type(selector) => selector.doc(ctx, state),
            SimpleSelector::Attribute(selector) => selector.doc(ctx, state),
            SimpleSelector::PseudoClass(selector) => selector.doc(ctx, state),
            SimpleSelector::PseudoElement(selector) => selector.doc(ctx, state),
            SimpleSelector::Nesting(selector) => selector.doc(ctx, state),
            SimpleSelector::SassPlaceholder(selector) => selector.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for SelectorList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let space_after_separator = if ctx
            .options
            .selectors_prefer_single_line
            .unwrap_or(ctx.options.prefer_single_line)
            || self
                .selectors
                .first()
                .zip(self.selectors.get(1))
                .is_some_and(|(first, second)| {
                    ctx.line_bounds
                        .line_distance(first.span.end, second.span.start)
                        == 0
                }) {
            Doc::line_or_space()
        } else {
            Doc::hard_line()
        };
        helpers::SeparatedListFormatter::new(",", space_after_separator).format(
            &self.selectors,
            &self.comma_spans,
            self.span.start,
            ctx,
            state,
        )
    }
}

impl<'s> DocGen<'s> for TagNameSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let name = if let InterpolableIdent::Literal(ident) = &self.name.name {
            Doc::text(ident.raw.to_ascii_lowercase())
        } else {
            self.name.doc(ctx, state)
        };
        if let Some(prefix) = &self.name.prefix {
            prefix
                .doc(ctx, state)
                .concat(ctx.unspaced_comments(
                    ctx.get_comments_between(prefix.span.end, self.name.name.span().start),
                ))
                .append(name)
        } else {
            name
        }
    }
}

impl<'s> DocGen<'s> for TypeSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            TypeSelector::TagName(selector) => selector.doc(ctx, state),
            TypeSelector::Universal(selector) => selector.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for UniversalSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let asterisk = Doc::text("*");
        if let Some(prefix) = &self.prefix {
            prefix
                .doc(ctx, state)
                .concat(
                    ctx.unspaced_comments(ctx.get_comments_between(prefix.span.end, self.span.end)),
                )
                .append(asterisk)
        } else {
            asterisk
        }
    }
}

impl<'s> DocGen<'s> for WqName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let name = self.name.doc(ctx, state);
        if let Some(prefix) = &self.prefix {
            prefix
                .doc(ctx, state)
                .concat(ctx.unspaced_comments(
                    ctx.get_comments_between(prefix.span.end, self.name.span().start),
                ))
                .append(name)
        } else {
            name
        }
    }
}

fn format_pseudo_selector_arg_tokens<'a, 's: 'a>(
    token_seq: &TokenSeq<'s>,
    ctx: &Ctx<'a, 's>,
    state: &State,
    from: usize,
    to: usize,
) -> Doc<'s> {
    use raffia::token::{Token, TokenWithSpan};

    let mut pos = from;
    let mut docs = Vec::with_capacity(token_seq.tokens.len() * 2);
    let mut iter = token_seq.tokens.iter().peekable();
    while let Some(token) = iter.next() {
        docs.extend(ctx.end_spaced_comments(ctx.get_comments_between(pos, token.span.start)));

        docs.push(token.doc(ctx, state));
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

    docs.extend(ctx.start_spaced_comments(ctx.get_comments_between(pos, to)));

    Doc::list(docs)
}
