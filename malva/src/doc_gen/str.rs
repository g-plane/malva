use crate::ctx::Ctx;
use std::borrow::Cow;

pub(super) fn format_str<'s>(
    raw: &'s str,
    formatter: impl StrRawFormatter<'s>,
    allow_prefer: bool,
    ctx: &Ctx<'_, 's>,
) -> Cow<'s, str> {
    use crate::config::Quotes;

    let content = formatter.content();
    match ctx.options.quotes {
        Quotes::AlwaysDouble => {
            if formatter.bound_check("\"") {
                raw.into()
            } else {
                formatter
                    .format('"', content.replace('"', "\\\"").into())
                    .into()
            }
        }
        Quotes::AlwaysSingle => {
            if formatter.bound_check("\'") {
                raw.into()
            } else {
                formatter
                    .format('\'', content.replace('\'', "\\'").into())
                    .into()
            }
        }
        Quotes::PreferDouble => {
            if formatter.bound_check("\"") || allow_prefer {
                raw.into()
            } else {
                formatter.format('"', content.into()).into()
            }
        }
        Quotes::PreferSingle => {
            if formatter.bound_check("\'") || allow_prefer {
                raw.into()
            } else {
                formatter.format('\'', content.into()).into()
            }
        }
    }
}

pub(super) trait StrRawFormatter<'s> {
    fn bound_check(&self, expectation: &'s str) -> bool;
    fn format(&self, bound: char, content: Cow<'s, str>) -> String;
    fn content(&self) -> &'s str;
}

pub(super) struct CssStrRawFormatter<'s> {
    bound: &'s str,
    content: &'s str,
}
impl<'s> CssStrRawFormatter<'s> {
    pub(super) fn new(raw: &'s str) -> Self {
        let (bound, content) = raw.split_at(1);
        let content = &content[0..content.len() - 1];
        Self { bound, content }
    }
}
impl<'s> StrRawFormatter<'s> for CssStrRawFormatter<'s> {
    fn bound_check(&self, expectation: &'s str) -> bool {
        self.bound == expectation
    }

    fn format(&self, bound: char, content: Cow<'s, str>) -> String {
        format!("{bound}{content}{bound}")
    }

    fn content(&self) -> &'s str {
        self.content
    }
}

pub(super) struct InterpolatedFirstStrRawFormatter<'s> {
    bound: &'s str,
    content: &'s str,
}
impl<'s> InterpolatedFirstStrRawFormatter<'s> {
    pub(super) fn new(raw: &'s str) -> Self {
        let (bound, content) = raw.split_at(1);
        Self { bound, content }
    }
}
impl<'s> StrRawFormatter<'s> for InterpolatedFirstStrRawFormatter<'s> {
    fn bound_check(&self, expectation: &'s str) -> bool {
        self.bound == expectation
    }

    fn format(&self, bound: char, content: Cow<'s, str>) -> String {
        format!("{bound}{content}")
    }

    fn content(&self) -> &'s str {
        self.content
    }
}

pub(super) struct InterpolatedLastStrRawFormatter<'s> {
    bound: &'s str,
    content: &'s str,
}
impl<'s> InterpolatedLastStrRawFormatter<'s> {
    pub(super) fn new(raw: &'s str) -> Self {
        let (content, bound) = raw.split_at(raw.len() - 1);
        Self { bound, content }
    }
}
impl<'s> StrRawFormatter<'s> for InterpolatedLastStrRawFormatter<'s> {
    fn bound_check(&self, expectation: &'s str) -> bool {
        self.bound == expectation
    }

    fn format(&self, bound: char, content: Cow<'s, str>) -> String {
        format!("{content}{bound}")
    }

    fn content(&self) -> &'s str {
        self.content
    }
}

pub(super) struct InterpolatedMidStrRawFormatter<'s> {
    content: &'s str,
}
impl<'s> InterpolatedMidStrRawFormatter<'s> {
    pub(super) fn new(raw: &'s str) -> Self {
        Self { content: raw }
    }
}
impl<'s> StrRawFormatter<'s> for InterpolatedMidStrRawFormatter<'s> {
    fn bound_check(&self, _: &'s str) -> bool {
        false
    }

    fn format(&self, _: char, content: Cow<'s, str>) -> String {
        content.into()
    }

    fn content(&self) -> &'s str {
        self.content
    }
}
