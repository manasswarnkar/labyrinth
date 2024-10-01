use rand::Rng;

#[derive(Debug, Default, Clone, Copy)]
pub struct Prim;

impl Prim {
    pub fn new(row: usize, col: usize) -> Vec<Vec<char>> {
        let mut maz = vec![vec!['#'; col]; row];

        // Select a random point on the edge as the start node
        let (sr, sc) = Self::random_edge_point(row, col);
        let s = Point::new(sr, sc, None);
        maz[s.r][s.c] = 'S';

        // Initialize the frontier with neighbors of the start point
        let mut frontier = Vec::new();
        Self::add_neighbors(sr, sc, row, col, &mut frontier, &s);

        // let mut last: Option<Point> = None;

        // Maze generation loop
        while !frontier.is_empty() {
            let random_index = rand::thread_rng().gen_range(0..frontier.len());
            let curr = frontier.remove(random_index);

            if let Some(opposite) = curr.opposite(row, col) {
                if maz[curr.r][curr.c] == '#' && maz[opposite.r][opposite.c] == '#' {
                    // Open paths between nodes
                    maz[curr.r][curr.c] = '.';
                    maz[opposite.r][opposite.c] = '.';
                    // last = Some(opposite.clone());

                    // Add neighbors of the opposite point to the frontier
                    Self::add_neighbors(opposite.r, opposite.c, row, col, &mut frontier, &opposite);
                }
            }
        }

        // Find a suitable end point on the edge
        let (er, ec) = Self::find_end_point(&maz, row, col);
        maz[er][ec] = 'E';

        // Ensure all edges (except start and end) are walls
        Self::enforce_edge_walls(&mut maz);

        maz
    }

    // Helper to add neighbors to the frontier
    fn add_neighbors(
        r: usize,
        c: usize,
        max_r: usize,
        max_c: usize,
        frontier: &mut Vec<Point>,
        parent: &Point,
    ) {
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_r = r as isize + dr;
            let new_c = c as isize + dc;

            if new_r >= 0 && new_r < max_r as isize && new_c >= 0 && new_c < max_c as isize {
                let new_r = new_r as usize;
                let new_c = new_c as usize;
                frontier.push(Point::new(new_r, new_c, Some(Box::new(parent.clone()))));
            }
        }
    }

    // Helper to select a random edge point
    fn random_edge_point(row: usize, col: usize) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => (0, rng.gen_range(0..col)),             // Top edge
            1 => (row - 1, rng.gen_range(0..col)),       // Bottom edge
            2 => (rng.gen_range(0..row), 0),             // Left edge
            _ => (rng.gen_range(0..row), col - 1),       // Right edge
        }
    }

    // Helper to find a suitable end point on the edge
    fn find_end_point(maz: &Vec<Vec<char>>, row: usize, col: usize) -> (usize, usize) {
        loop {
            let (er, ec) = Self::random_edge_point(row, col);
            if maz[er][ec] == '.' && (er != 0 || ec != 0) {  // Ensure it's not the start point
                return (er, ec);
            }
        }
    }

    // Helper to enforce walls on all edges except start and end
    fn enforce_edge_walls(maz: &mut Vec<Vec<char>>) {
        let row = maz.len();
        let col = maz[0].len();

        for r in 0..row {
            for c in 0..col {
                if (r == 0 || r == row - 1 || c == 0 || c == col - 1) && maz[r][c] != 'S' && maz[r][c] != 'E' {
                    maz[r][c] = '#';
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Point {
    r: usize,
    c: usize,
    parent: Option<Box<Point>>,
}

impl Point {
    fn new(x: usize, y: usize, parent: Option<Box<Point>>) -> Point {
        Point { r: x, c: y, parent }
    }

    fn opposite(&self, max_r: usize, max_c: usize) -> Option<Point> {
        if let Some(ref parent) = self.parent {
            if self.r != parent.r {
                let new_r = if self.r > parent.r {
                    self.r + 1
                } else {
                    self.r.saturating_sub(1)
                };

                if new_r < max_r {
                    return Some(Point::new(new_r, self.c, Some(Box::new(self.clone()))));
                }
            }
            if self.c != parent.c {
                let new_c = if self.c > parent.c {
                    self.c + 1
                } else {
                    self.c.saturating_sub(1)
                };

                if new_c < max_c {
                    return Some(Point::new(self.r, new_c, Some(Box::new(self.clone()))));
                }
            }
        }
        None
    }
}