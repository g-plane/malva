use super::DocGen;
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl DocGen for Dimension<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let unit = match self.kind {
            DimensionKind::Length => {
                if self.unit.name.eq_ignore_ascii_case("Q") {
                    Doc::text("Q")
                } else {
                    Doc::text(self.unit.raw.to_ascii_lowercase())
                }
            }
            DimensionKind::Angle
            | DimensionKind::Duration
            | DimensionKind::Resolution
            | DimensionKind::Flex => Doc::text(self.unit.raw.to_ascii_lowercase()),
            DimensionKind::Frequency => {
                if self.unit.name.eq_ignore_ascii_case("Hz") {
                    Doc::text("Hz")
                } else if self.unit.name.eq_ignore_ascii_case("kHz") {
                    Doc::text("kHz")
                } else {
                    Doc::text(self.unit.raw.to_ascii_lowercase())
                }
            }
            DimensionKind::Unknown => self.unit.doc(ctx),
        };
        self.value.doc(ctx).append(unit)
    }
}

impl DocGen for Ident<'_> {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text(self.raw)
    }
}

impl DocGen for HexColor<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        use crate::options::HexCase;

        let hex = match ctx.options.hex_case {
            HexCase::Ignore => format!("#{}", self.raw),
            HexCase::Lower => format!("#{}", self.raw.to_ascii_lowercase()),
            HexCase::Upper => format!("#{}", self.raw.to_ascii_uppercase()),
        };
        Doc::text(hex)
    }
}

impl DocGen for InterpolableIdent<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            InterpolableIdent::Literal(literal) => literal.doc(ctx),
            _ => todo!(),
        }
    }
}

impl DocGen for InterpolableStr<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            InterpolableStr::Literal(literal) => literal.doc(ctx),
            _ => todo!(),
        }
    }
}

impl DocGen for Number<'_> {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text(self.raw)
    }
}

impl DocGen for Percentage<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        self.value.doc(ctx).append(Doc::text("%"))
    }
}

impl DocGen for Str<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        use crate::options::Quotes;

        let (left, right) = self.raw.split_at(1);
        let inner = &right[0..right.len() - 1];

        match ctx.options.quotes {
            Quotes::AlwaysDouble => {
                if left == "\"" {
                    Doc::text(self.raw)
                } else {
                    Doc::text(format!("\"{}\"", inner.replace('"', "\\\"")))
                }
            }
            Quotes::AlwaysSingle => {
                if left == "\'" {
                    Doc::text(self.raw)
                } else {
                    Doc::text(format!("'{}'", inner.replace('\'', "\\'")))
                }
            }
            Quotes::PreferDouble => {
                if left == "\"" || inner.contains("\\\"") {
                    Doc::text(self.raw)
                } else {
                    Doc::text(format!("\"{inner}\""))
                }
            }
            Quotes::PreferSingle => {
                if left == "\'" || inner.contains("\\\'") {
                    Doc::text(self.raw)
                } else {
                    Doc::text(format!("'{inner}'"))
                }
            }
        }
    }
}
