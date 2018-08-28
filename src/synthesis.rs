use super::elements::{Atom, Element};

crate enum Spin {
    Up,
    Down,
    Neutral,
}

crate trait Quark {
    fn spin(&self) -> Spin;
}

crate struct VoidQuark {}

impl Quark for VoidQuark {
    fn spin(&self) -> Spin {
        Spin::Neutral
    }
}

crate struct GodQuark {}

impl Quark for GodQuark {
    fn spin(&self) -> Spin {
        Spin::Up
    }
}

trait SynthesisCost {
    fn base_synthesis_cost(&self) -> usize;
}

impl SynthesisCost for Element {
    fn base_synthesis_cost(&self) -> usize {
        match self {
            Element::Gold => 50,
            _ => 0,
        }
    }
}

crate enum SynthesisError {
    InsufficientEnergy,
}

type SynthesisState = (usize, Box<Atom>);

type SynthesisResult = Result<SynthesisState, SynthesisError>;

fn synthesize<Q, R>(energy: usize, q: Q, r: R) -> SynthesisResult
where
    Q: Quark,
    R: Quark,
{
    Ok((energy, Box::new(Element::Copper)))
}
