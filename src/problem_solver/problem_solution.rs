use std::fmt;

use super::Point;

/// A struct that represents a solution to the problem. It contains the points
/// that have been chosen for the solution
#[derive(Clone)]
pub struct ProblemSolution {
    pub(super) points: Vec<Point>,
}

impl ProblemSolution {
    /// Allows getting the z, which is the value we are trying to maximize.
    /// This value is defined as the sum of the distances between each point
    /// in the solution
    pub fn get_z(&self) -> f64 {
        let mut total = 0.0;
        for point in 0..self.points.len() {
            for other_point in point + 1..self.points.len() {
                total += self.points[point].distance_to(&self.points[other_point]);
            }
        }
        total
    }

    /// Allows getting the total completion time of each machine
    pub fn get_points(&self) -> &Vec<Point> {
        &self.points
    }
}

impl fmt::Display for ProblemSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self
            .points
            .iter()
            .map(|point| {
                point
                    .coordinates
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .collect::<Vec<String>>()
            .join("}, {");
        write!(f, "{{{}}}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_z() {
        let points = vec![
            Point {
                coordinates: vec![0.0, 0.0],
            },
            Point {
                coordinates: vec![0.0, 2.0],
            },
        ];
        let solution = ProblemSolution { points };
        assert_eq!(solution.get_z(), 2.0);
    }
}
