#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

mod elements;
mod game;
//mod synthesis;

fn main() {
    let elements = vec![
        elements::Element::Air,
        elements::Element::Water,
        elements::Element::Earth,
        elements::Element::Fire,
        elements::Element::Lead,
        elements::Element::Tin,
        elements::Element::Iron,
        elements::Element::Copper,
        elements::Element::Silver,
        elements::Element::Gold,
        elements::Element::Salt,
        elements::Element::Quicksilver,
        elements::Element::Vitae,
        elements::Element::Mors,
    ];

    println!("{:?}", game::Grid::new(4));
    println!("{:?}", game::Grid::new(4).iter());
}

//
//     _/ \
//    / \_/
//    \_/ \
//      \_/
//
//       _____
//      /     \
//     /       \
//     \       /
//      \_____/
//
//
//
