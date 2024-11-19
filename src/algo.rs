use rand::{seq::SliceRandom, Rng};

#[derive(Debug, Default, Clone, Copy)]
pub struct Prim;

impl Prim {
    pub fn new(row: usize, col: usize) -> Vec<Vec<char>> {
        let mut maz = vec![vec!['#'; col]; row];

        // First, enforce walls on all edges
        Self::enforce_edge_walls(&mut maz);

        // Select a random point on the edge as the start node
        let (sr, sc) = Self::random_edge_point(row, col);
        let s = Point::new(sr, sc, None);
        maz[s.r][s.c] = 'S';

        // Initialize the frontier with neighbors of the start point
        let mut frontier = Vec::new();
        Self::add_neighbors(sr, sc, row, col, &mut frontier, &s);

        // Keep track of potential end points during generation
        let mut potential_ends = Vec::new();

        // Maze generation loop
        while !frontier.is_empty() {
            let random_index = rand::thread_rng().gen_range(0..frontier.len());
            let curr = frontier.remove(random_index);

            if let Some(opposite) = curr.opposite(row, col) {
                let curr_is_edge = Self::is_edge_point(curr.r, curr.c, row, col);
                let opposite_is_edge = Self::is_edge_point(opposite.r, opposite.c, row, col);

                // Allow path creation if:
                // 1. Neither point is on the edge, OR
                // 2. The edge point could be a valid end point
                if (!curr_is_edge && !opposite_is_edge) || 
                   (curr_is_edge && Self::is_valid_end_location(&curr, sr, sc)) ||
                   (opposite_is_edge && Self::is_valid_end_location(&opposite, sr, sc)) {
                    
                    if maz[curr.r][curr.c] == '#' && maz[opposite.r][opposite.c] == '#' {
                        // Open paths between nodes
                        maz[curr.r][curr.c] = '.';
                        maz[opposite.r][opposite.c] = '.';

                        // If we created a path to an edge, store it as a potential end point
                        if curr_is_edge && Self::is_valid_end_location(&curr, sr, sc) {
                            potential_ends.push((curr.r, curr.c));
                        }
                        if opposite_is_edge && Self::is_valid_end_location(&opposite, sr, sc) {
                            potential_ends.push((opposite.r, opposite.c));
                        }

                        // Add neighbors of the opposite point to the frontier
                        Self::add_neighbors(opposite.r, opposite.c, row, col, &mut frontier, &opposite);
                    }
                }
            }
        }

        // Choose a random potential end point
        if let Some(&mut (er, ec)) = potential_ends.choose_mut(&mut rand::thread_rng()) {
            maz[er][ec] = 'E';
        } else {
            // If no potential ends were found during generation,
            // find one using the flood fill method
            let (er, ec) = Self::find_connected_end_point(&maz, sr, sc);
            maz[er][ec] = 'E';
        }

        maz
    }


    // Helper function to check if a point is on the edge
    fn is_edge_point(r: usize, c: usize, row: usize, col: usize) -> bool {
        r == 0 || r == row - 1 || c == 0 || c == col - 1
    }

    // Helper function to check if a location could be a valid end point
    fn is_valid_end_location(point: &Point, start_r: usize, start_c: usize) -> bool {
        // Must not be the same as start point
        point.r != start_r || point.c != start_c
    }

    fn find_connected_end_point(maz: &Vec<Vec<char>>, start_r: usize, start_c: usize) -> (usize, usize) {
        let row = maz.len();
        let col = maz[0].len();
        
        let mut visited = vec![vec![false; col]; row];
        let mut reachable_edges = Vec::new();
        
        let mut stack = vec![(start_r, start_c)];
        visited[start_r][start_c] = true;
        
        while let Some((r, c)) = stack.pop() {
            if (r == 0 || r == row - 1 || c == 0 || c == col - 1) && 
               (r != start_r || c != start_c) && 
               (maz[r][c] == '.' || maz[r][c] == '#') {
                reachable_edges.push((r, c));
            }
            
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_r = (r as isize + dr) as usize;
                let new_c = (c as isize + dc) as usize;
                
                if new_r < row && new_c < col && 
                   !visited[new_r][new_c] && 
                   maz[new_r][new_c] == '.' {
                    stack.push((new_r, new_c));
                    visited[new_r][new_c] = true;
                }
            }
        }
        
        if !reachable_edges.is_empty() {
            let idx = rand::thread_rng().gen_range(0..reachable_edges.len());
            reachable_edges[idx]
        } else {
            // If we still can't find an end point, force create one
            // by finding the closest reachable point to an edge
            for r in 0..row {
                for c in 0..col {
                    if visited[r][c] && 
                       (r == 0 || r == row-1 || c == 0 || c == col-1) && 
                       (r != start_r || c != start_c) {
                        return (r, c);
                    }
                }
            }
            panic!("No valid end point found!");
        }
    }

    // Other helper functions remain the same...
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

    fn random_edge_point(row: usize, col: usize) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => (0, rng.gen_range(0..col)),             // Top edge
            1 => (row - 1, rng.gen_range(0..col)),       // Bottom edge
            2 => (rng.gen_range(0..row), 0),             // Left edge
            _ => (rng.gen_range(0..row), col - 1),       // Right edge
        }
    }

    fn enforce_edge_walls(maz: &mut Vec<Vec<char>>) {
        let row = maz.len();
        let col = maz[0].len();

        for r in 0..row {
            for c in 0..col {
                if r == 0 || r == row - 1 || c == 0 || c == col - 1 {
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