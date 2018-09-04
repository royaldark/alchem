use super::elements::Atom;
use std::collections::HashMap;

/*
     -z
  +y  |  +x
    \   /
    /   \
  -x  |  -y
     +z
*/

#[derive(Hash, PartialEq, Eq)]
struct GridPosition {
    x: i8,
    y: i8,
    z: i8,
}

struct GridDirection {
    x: i8,
    y: i8,
    z: i8,
}

struct GridSpace {
    atom: Option<Atom>,
    selected: bool,
}

struct Grid {
    radius: u8,
    spaces: HashMap<GridPosition, GridSpace>,
}

static DIRECTIONS: [&GridDirection; 6] = [
    &GridDirection { x: 1, y: -1, z: 0 },
    &GridDirection { x: 1, y: 0, z: -1 },
    &GridDirection { x: 0, y: 1, z: -1 },
    &GridDirection { x: -1, y: 1, z: 0 },
    &GridDirection { x: -1, y: 0, z: 1 },
    &GridDirection { x: 0, y: -1, z: 1 },
];

/*lazy_static! {

}*/

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

fn grid_ring(center: GridPosition, radius: i8) -> Vec<GridPosition> {
    let mut tiles: Vec<GridPosition> = Vec::new();

    let mut current = grid_add(&center, &grid_scale(DIRECTIONS[4], radius));

    for i in 0..6 {
        for j in 0..radius {
            tiles.push(current);
            current = grid_add(&current, DIRECTIONS[i]);
        }
    }

    tiles
}

impl Grid {
    fn new(radius: u8) -> Grid {
        let mut spaces: HashMap<GridPosition, GridSpace> = HashMap::new();

        let mut x = 1;
        let mut y = -1;
        let mut z = 0;

        // 1. (1, 0, -1)
        // 2. (1, -1, 0)
        // 3. (0, -1, 1)
        // 4. (-1, 0, 1)
        // 5. (-1, 1, 0)
        // 6. (0, 1, -1)

        // 0 -1 -1 0 1 1
        // -1 0 1 1 0 -1

        // key:
        // 1 1 0 -1 -1 0

        let deltas = [1, 1, 0, -1, -1, 0].iter().cycle();
        let mut i = 0;

        loop {
            assert!(x + y + z == 0);

            spaces.insert(
                GridPosition { x, y, z },
                GridSpace {
                    atom: None,
                    selected: false,
                },
            );

            // x += deltas[i]
            // y += deltas[i + 2]
            // z += deltas[i + 4]
        }

        Grid { radius, spaces }
    }
}
