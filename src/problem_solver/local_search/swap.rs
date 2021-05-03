use super::*;
use crate::Point;

/// A local search that consists on doing swaps between points that are in the solution
/// and points that aren't
pub struct Swap {}

impl LocalSearch for Swap {
    fn perform_search(
        &self,
        instance: &ProblemInstance,
        solution: ProblemSolution,
    ) -> ProblemSolution {
        let solution_ref = &solution;
        (0..solution.points.len())
            .flat_map(|point_index| {
                instance
                    .points
                    .iter()
                    .filter(|possible_point| !solution_ref.points.contains(possible_point))
                    .map(move |possible_point| {
                        Swap::get_solution_after_swap(
                            solution_ref.clone(),
                            point_index,
                            possible_point.clone(),
                        )
                    })
            })
            .reduce(|acc, value| {
                if value.get_z() > acc.get_z() {
                    return value;
                }
                return acc;
            })
            .unwrap()
    }
}

impl Swap {
    pub fn new() -> Self {
        Swap {}
    }

    fn get_solution_after_swap(
        mut solution: ProblemSolution,
        index: usize,
        point_to_swap: Point,
    ) -> ProblemSolution {
        solution.points.remove(index);
        solution.points.push(point_to_swap);
        solution
    }
}
