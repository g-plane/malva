pub(crate) const STATE_IN_LESS_DETACHED_RULESET: State = State(1);

#[derive(Clone, Copy, Default)]
pub(crate) struct State(u8);

impl State {
    pub(crate) fn has(self, other: State) -> bool {
        self.0 & other.0 != 0
    }

    pub(crate) fn union(self, other: State) -> State {
        State(self.0 | other.0)
    }
}
