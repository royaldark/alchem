use super::elements::Atom;
use std::clone::Clone;
use std::collections::HashMap;
use std::iter::FromIterator;

use wasm_bindgen::prelude::*;

/*
     -z
  +y  |  +x
    \   /
    /   \
  -x  |  -y
     +z
*/

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct GridPosition {
    x: i8,
    y: i8,
    z: i8,
}

struct GridDirection {
    x: i8,
    y: i8,
    z: i8,
}

#[derive(Clone, Debug)]
pub struct GridSpace {
    atom: Option<Atom>,
    selected: bool,
}

#[derive(Debug)]
pub struct GridIterator<'a> {
    spaces: Vec<(&'a GridPosition, &'a GridSpace)>,
}

impl<'a> GridIterator<'a> {
    fn from_grid(grid: &'a Grid) -> Self {
        let as_vec: Vec<(&GridPosition, &GridSpace)> = grid.spaces.iter().collect();
        let mut spaces = as_vec.clone();
        spaces.sort_unstable_by_key(|(p, _)| (p.z, p.x));

        GridIterator { spaces }
    }
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = (GridPosition, GridSpace);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Grid {
    radius: u8,
    spaces: HashMap<GridPosition, GridSpace>,
}

impl Grid {
    pub fn iter(&self) -> GridIterator<'_> {
        GridIterator::from_grid(self)
    }
}

static DIRECTIONS: [&GridDirection; 6] = [
    &GridDirection { x: 1, y: -1, z: 0 },
    &GridDirection { x: 1, y: 0, z: -1 },
    &GridDirection { x: 0, y: 1, z: -1 },
    &GridDirection { x: -1, y: 1, z: 0 },
    &GridDirection { x: -1, y: 0, z: 1 },
    &GridDirection { x: 0, y: -1, z: 1 },
];

fn grid_add(pos: &GridPosition, dir: &GridDirection) -> GridPosition {
    GridPosition {
        x: pos.x + dir.x,
        y: pos.y + dir.y,
        z: pos.z + dir.z,
    }
}

fn grid_scale(dir: &GridDirection, radius: i8) -> GridDirection {
    GridDirection {
        x: dir.x * radius,
        y: dir.y * radius,
        z: dir.z * radius,
    }
}

fn grid_ring(center: &GridPosition, radius: i8) -> Vec<GridPosition> {
    let mut tiles: Vec<GridPosition> = Vec::new();

    let mut current = grid_add(&center, &grid_scale(DIRECTIONS[4], radius));

    for i in 0..6 {
        for _ in 0..radius {
            let new_current = grid_add(&current, DIRECTIONS[i]);
            tiles.push(current);
            current = new_current;
        }
    }

    tiles
}

fn grid_spiral(center: &GridPosition, radius: i8) -> Vec<GridPosition> {
    let mut tiles: Vec<GridPosition> = vec![center.clone()];

    for i in 1..=radius {
        let mut ring = grid_ring(&center, i);
        tiles.append(&mut ring);
    }

    tiles
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new(radius: u8) -> Grid {
        let home = GridPosition { x: 0, y: 0, z: 0 };

        let first_ring = grid_ring(&home, 1);
        let second_ring = grid_ring(&home, 2);

        println!("Ring 1: {:?}", first_ring);
        println!("Ring 2: {:?}", second_ring);

        let spiral = grid_spiral(&home, 3);

        println!("Circle: {:?}", spiral);

        let spaces: HashMap<GridPosition, GridSpace> =
            HashMap::from_iter(spiral.into_iter().map(|x| {
                (
                    x,
                    GridSpace {
                        atom: None,
                        selected: false,
                    },
                )
            }));

        Grid { radius, spaces }
    }
}

//#[wasm_bindgen]
//pub fn 
