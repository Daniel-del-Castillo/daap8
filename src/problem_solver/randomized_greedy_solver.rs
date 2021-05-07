use super::{Point, ProblemInstance, ProblemSolution, ProblemSolver};
use std::cmp::min;

/// A greedy algorithm that creates a solution for the problem by adding
/// to the solution in each step a random point from the k pointswhich
/// are further from the center of the points already in the solution
pub struct RandomizedGreedySolver {
    number_of_points: usize,
    rcl_size: usize,
}

impl ProblemSolver for RandomizedGreedySolver {
    fn solve(&mut self, instance: &ProblemInstance) -> ProblemSolution {
        assert!(instance.points.len() >= self.number_of_points);
        let mut points = instance.points.clone();
        let mut center = RandomizedGreedySolver::calculate_center(&points);
        let mut solution_points = Vec::new();
        while solution_points.len() < self.number_of_points {
            let possible_points_indexes = RandomizedGreedySolver::get_farther_points_indexes(
                &mut points,
                &center,
                self.rcl_size,
            );
            let new_point = points.remove(
                possible_points_indexes[rand::random::<usize>() % possible_points_indexes.len()],
            );
            solution_points.push(new_point);
            center = RandomizedGreedySolver::calculate_center(&solution_points);
        }
        ProblemSolution {
            points: solution_points,
        }
    }
}

impl RandomizedGreedySolver {
    /// Creates a new solver. The number of points the solution will have and
    /// the size of the restricted candidate list need to be passed as arguments
    pub fn new(number_of_points: usize, rcl_size: usize) -> Self {
        assert!(number_of_points > 0 && rcl_size > 0);
        RandomizedGreedySolver {
            number_of_points,
            rcl_size,
        }
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

    fn get_farther_points_indexes(
        available_points: &Vec<Point>,
        center: &Point,
        number_of_points: usize,
    ) -> Vec<usize> {
        let mut result = Vec::with_capacity(number_of_points);
        for _ in 0..min(number_of_points, available_points.len()) {
            result.push(
                available_points
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| !result.contains(index))
                    .map(|(index, point)| (index, point.distance_to(center)))
                    .reduce(|acc, value| {
                        if value.1 > acc.1 {
                            return value;
                        }
                        return acc;
                    })
                    // There must be at least one point in the vector
                    .unwrap()
                    .0,
            );
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn center() {
        let center = RandomizedGreedySolver::calculate_center(&vec![
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
        let indexes = RandomizedGreedySolver::get_farther_points_indexes(
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
                    coordinates: vec![2.0, 3.0],
                },
            ],
            &Point {
                coordinates: vec![1.0, 1.0],
            },
            2,
        );
        assert_eq!(indexes, vec![1, 3]);
    }
}
