use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
crate enum Element {
    Fire,
    Water,
    Earth,
    Air,
    Lead,
    Tin,
    Iron,
    Copper,
    Silver,
    Gold,
    Salt,
    Quicksilver,
    Vitae,
    Mors,
}

#[derive(Clone, Debug)]
crate struct Atom {
    element: Element,
}

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Atom<{}/{}>", self.symbol(), self.abbreviation())
    }
}

impl Atom {
    fn symbol(&self) -> String {
        match self.element {
            Element::Fire => "🔥".to_owned(),
            Element::Water => "🌊".to_owned(),
            Element::Earth => "⛰️".to_owned(),
            Element::Air => "💨".to_owned(),
            Element::Lead => "✏️".to_owned(),
            Element::Tin => "🥫".to_owned(),
            Element::Iron => "🏗️".to_owned(),
            Element::Copper => "🔌".to_owned(),
            Element::Silver => "🥈".to_owned(),
            Element::Gold => "🥇".to_owned(),
            Element::Salt => "🥨".to_owned(),
            Element::Quicksilver => "".to_owned(),
            Element::Vitae => "".to_owned(),
            Element::Mors => "".to_owned(),
        }
    }

    fn abbreviation(&self) -> String {
        match self.element {
            Element::Fire => "FIR".to_owned(),
            Element::Water => "WTR".to_owned(),
            Element::Earth => "️ERT".to_owned(),
            Element::Air => "AIR".to_owned(),
            Element::Lead => "LED".to_owned(),
            Element::Tin => "TIN".to_owned(),
            Element::Iron => "IRN".to_owned(),
            Element::Copper => "CPR".to_owned(),
            Element::Silver => "SLV".to_owned(),
            Element::Gold => "GLD".to_owned(),
            Element::Salt => "SLT".to_owned(),
            Element::Quicksilver => "QCK".to_owned(),
            Element::Vitae => "VIT".to_owned(),
            Element::Mors => "MOR".to_owned(),
        }
    }
}

/*crate struct SimpleBond<T, U>
where
    T: Atom,
    U: Atom,
{
    crate left: T,
    crate right: U,
}

impl<T, U> Display for SimpleBond<T, U>
where
    T: Atom + Display,
    U: Atom + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bond[{} <=> {}]", self.left, self.right)
    }
}*/
