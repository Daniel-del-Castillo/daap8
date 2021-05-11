use super::{Point, ProblemInstance, ProblemSolution};

pub struct PartialSolution {
    solution: ProblemSolution,
    upper_bound: f64,
}

impl PartialSolution {
    fn new(solution: ProblemSolution, instance: &ProblemInstance, size: usize) -> Self {
        let mut upper_bound = solution.get_z();
        let mut point_upper_bounds = instance
            .points
            .iter()
            .filter(|point| !solution.points.contains(point))
            .map(|point| PartialSolution::get_point_upper_bound(&solution, instance, point, size))
            .collect::<Vec<f64>>();
        for _ in 0..size - solution.points.len() {
            let (index, value) = point_upper_bounds
                .iter()
                .enumerate()
                .reduce(|max, actual| {
                    if actual.1 > max.1 {
                        return actual;
                    }
                    return max;
                })
                .unwrap();
            upper_bound += value;
            point_upper_bounds.remove(index);
        }
        PartialSolution {
            solution,
            upper_bound,
        }
    }

    fn get_point_upper_bound(
        solution: &ProblemSolution,
        instance: &ProblemInstance,
        point: &Point,
        size: usize,
    ) -> f64 {
        let mut result = solution
            .points
            .iter()
            .map(|solution_point| solution_point.distance_to(point))
            .sum();
        let mut distances = instance
            .points
            .iter()
            .filter(|other_point| !solution.points.contains(point) && point != *other_point)
            .map(|other_point| other_point.distance_to(point))
            .collect::<Vec<f64>>();
        for _ in 0..size - solution.points.len() - 1 {
            let (index, value) = distances
                .iter()
                .enumerate()
                .reduce(|max, actual| {
                    if actual.1 > max.1 {
                        return actual;
                    }
                    return max;
                })
                .unwrap();
            result += value / 2.0;
            distances.remove(index);
        }
        result
    }
}
