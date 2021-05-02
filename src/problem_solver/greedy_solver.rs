use super::{Point, ProblemInstance, ProblemSolution, ProblemSolver};

/// A greedy algorithm that creates a solution for the problem by adding
/// to the solution in each step the point which is further from the center
/// of the points already in the solution
pub struct GreedySolver {
    number_of_points: usize,
}

impl ProblemSolver for GreedySolver {
    fn solve(&mut self, instance: &ProblemInstance) -> ProblemSolution {
        assert!(instance.points.len() >= self.number_of_points);
        let mut points = instance.points.clone();
        let mut center = GreedySolver::calculate_center(&points);
        let mut solution_points = Vec::new();
        while solution_points.len() < self.number_of_points {
            let new_point = GreedySolver::get_furthest_point(&mut points, &center);
            solution_points.push(new_point);
            center = GreedySolver::calculate_center(&solution_points);
        }
        ProblemSolution {
            points: solution_points,
        }
    }
}

impl GreedySolver {
    /// Creates a new solver. The number of points the solution will have needs to
    /// be passed as argument
    pub fn new(number_of_points: usize) -> Self {
        GreedySolver { number_of_points }
    }

    fn calculate_center(points: &Vec<Point>) -> Point {
        let coordinates = (0..points[0].get_dimensionality())
            .map(|index| {
                points
                    .iter()
                    .map(|point| point.coordinates[index])
                    .sum::<f64>()
                    / points.len() as f64
            })
            .collect();
        Point::new(coordinates)
    }

    fn get_furthest_point(available_points: &mut Vec<Point>, center: &Point) -> Point {
        available_points.remove(
            available_points
                .iter()
                .map(|point| point.distance_to(center))
                .enumerate()
                .reduce(|acc, value| {
                    if value.1 > acc.1 {
                        return value;
                    }
                    return acc;
                })
                // There must be at least one point in the vector
                .unwrap()
                .0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn center() {
        let center = GreedySolver::calculate_center(&vec![
            Point {
                coordinates: vec![0.0, 0.0],
            },
            Point {
                coordinates: vec![2.0, 0.0],
            },
            Point {
                coordinates: vec![0.0, 2.0],
            },
            Point {
                coordinates: vec![2.0, 2.0],
            },
        ]);
        assert_eq!((center.coordinates[0], center.coordinates[1]), (1.0, 1.0));
    }
    #[test]
    fn furthest_point() {
        let point = GreedySolver::get_furthest_point(
            &mut vec![
                Point {
                    coordinates: vec![0.0, 0.0],
                },
                Point {
                    coordinates: vec![3.0, 0.0],
                },
                Point {
                    coordinates: vec![1.0, 2.0],
                },
                Point {
                    coordinates: vec![2.0, 2.0],
                },
            ],
            &Point {
                coordinates: vec![1.0, 1.0],
            },
        );
        assert_eq!((point.coordinates[0], point.coordinates[1]), (3.0, 0.0));
    }
}
