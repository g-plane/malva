#[derive(Clone)]
pub(crate) struct State {
    pub(crate) in_less_detached_ruleset: bool,
    pub(crate) in_unknown_at_rule: bool,
    pub(crate) selector_override: SelectorOverride,
}

#[derive(Clone)]
pub(crate) enum SelectorOverride {
    Unset,
    Ignore,
    Always,
    Consistent,
    Wrap,
}
