use raffia::Span;

#[derive(Clone)]
pub(crate) struct State {
    pub(crate) keep_decl_name_case: bool,
    pub(crate) selector_override: SelectorOverride,
    pub(crate) prev_decl_value_span: Option<Span>,
}

#[derive(Clone)]
pub(crate) enum SelectorOverride {
    Unset,
    Ignore,
    Always,
    Consistent,
    Wrap,
}
