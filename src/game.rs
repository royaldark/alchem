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

struct GridSpace {
    atom: Option<Atom>,
    selected: bool,
}

struct Grid {
    radius: u8,
    spaces: HashMap<GridPosition, GridSpace>,
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

            x += deltas[i];

            // x += deltas[i]
            // y += deltas[i + 2]
            // z += deltas[i + 4]
        }

        Grid { radius, spaces }
    }
}
