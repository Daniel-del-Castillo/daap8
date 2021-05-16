use super::{Point, ProblemInstance, ProblemSolution};

/// A helper struct to represent a partial solution. It stores the incomplete
/// solution and the upper bound that it has. The upper bound gets calculated
/// on the constructor
pub struct PartialSolution {
    pub solution: ProblemSolution,
    pub upper_bound: f64,
}

impl PartialSolution {
    /// The constructor for a partial solution. Stores the solution and calculates
    /// the upper bound, taking into account how many points are left to add.
    pub fn new(solution: ProblemSolution, instance: &ProblemInstance, size: usize) -> Self {
        let points_to_add = size - solution.points.len();
        let upper_bound = PartialSolution::get_uppper_bound(&solution, instance, points_to_add);
        PartialSolution {
            solution,
            upper_bound,
        }
    }

    /// Calculates the upper bound. It calculates the upper bounds of adding each point
    /// and then selects the ones that give the greater upper bound
    fn get_uppper_bound(
        solution: &ProblemSolution,
        instance: &ProblemInstance,
        points_to_add: usize,
    ) -> f64 {
        let mut upper_bound = solution.get_z();
        if points_to_add == 0 {
            return upper_bound;
        }
        let mut point_upper_bounds = instance
            .points
            .iter()
            .filter(|point| !solution.points.contains(point))
            .map(|point| {
                PartialSolution::get_point_upper_bound(
                    &solution,
                    instance,
                    point,
                    points_to_add - 1,
                )
            })
            .collect::<Vec<f64>>();
        for _ in 0..points_to_add {
            let index = point_upper_bounds
                .iter()
                .enumerate()
                .reduce(|max, actual| {
                    if actual.1 > max.1 {
                        return actual;
                    }
                    return max;
                })
                .map(|(index, _)| index)
                .unwrap();
            upper_bound += point_upper_bounds.remove(index);
        }
        upper_bound
    }

    /// Gets the upper bound produced by adding a certain point to the solution.
    fn get_point_upper_bound(
        solution: &ProblemSolution,
        instance: &ProblemInstance,
        point: &Point,
        points_to_add: usize,
    ) -> f64 {
        let mut result = solution
            .points
            .iter()
            .map(|solution_point| solution_point.distance_to(point))
            .sum();
        let mut distances = get_distances_to_points_outside_solution(instance, solution, point);
        for _ in 0..points_to_add {
            let index = distances
                .iter()
                .enumerate()
                .reduce(|max, actual| {
                    if actual.1 > max.1 {
                        return actual;
                    }
                    return max;
                })
                .map(|(index, _)| index)
                .unwrap();
            result += distances.remove(index) / 2.0;
        }
        result
    }
}

/// Gets all the distance from one point outside the solution
/// to other points outside the solution
fn get_distances_to_points_outside_solution(
    instance: &ProblemInstance,
    solution: &ProblemSolution,
    point: &Point,
) -> Vec<f64> {
    instance
        .points
        .iter()
        .filter(|other_point| !solution.points.contains(point) && point != *other_point)
        .map(|other_point| other_point.distance_to(point))
        .collect()
}
