use rand::Rng;

type Point = (f64, f64);
const BOUNDS: Point = (0.0, 0.1);

fn assert_within_bounds(p: Point) {
    assert!(p.0 >= BOUNDS.0 && p.0 <= BOUNDS.1 && p.1 >= BOUNDS.0 && p.1 <= BOUNDS.1);
}

struct Voronoi {
    grid: Vec<Point>,
    _by_x: Vec<usize>,
    _by_y: Vec<usize>,
    expected_perimeter_sum: f64,
}

impl Voronoi {
    fn new(starting_point: Point) -> Self {
        assert_within_bounds(starting_point);
        return Self {
            grid: vec![starting_point],
            _by_x: vec![0],
            _by_y: vec![0],
            expected_perimeter_sum: 0.0,
        };
    }

    fn add_point(&mut self, p: Point) {
        self.expected_perimeter_sum += self.calculate_perimeter(p);

        let grid_index = self.grid.len();
        self.grid.push(p);

        // TODO: Binary search to insert
        let mut axis_index = grid_index;
        for (i, other) in self._by_x.iter().enumerate() {
            let other = self.grid[*other];
            if p.0 > other.0 {
                axis_index = i;
                break;
            }
        }
        self._by_x.insert(axis_index, grid_index);

        let mut axis_index = grid_index;
        for (i, other) in self._by_y.iter().enumerate() {
            let other = self.grid[*other];
            if p.1 > other.1 {
                axis_index = i;
                break;
            }
        }
        self._by_y.insert(axis_index, grid_index);
    }

    fn calculate_perimeter(&self, p: Point) -> f64 {
        // TODO: psueod
        // Points going clockwise (remember, graphics coords) from 0 radians to 2pi radians
        let mut nearest_points = vec![(2.0 - p.0, p.1), (p.0, 2.0 - p.1), (-p.0, p.1), (p.0, -p.1)];

        for other in self.grid.iter() {
            // if other inside nearest_points
            // find points before and after that would be outside the new nearest_points
            // splice the new point into nearest_points
        }

        // Convert the nearest_points into the voronoi shape and return the perimeter.
        return 0.0;
    }
}

fn main() {
    // NOTE: All coordinate math is based on graphics, i.e. top-left is origin
    // and down is positive y-axis.

    let mut rng = rand::thread_rng();
    let mut voronoi = Voronoi::new(rng.gen());

    let f: f64 = rng.gen();
    println!("{f}");
}
