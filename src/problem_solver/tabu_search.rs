use super::{local_search::Swap, Point, ProblemInstance, ProblemSolution, ProblemSolver, GRASP};
use std::collections::VecDeque;

/// A implementation of a tabu search. It uses [GRASP](super::GRASP) to get an initial
/// solution and then performs the search.
pub struct TabuSearch {
    number_of_points: usize,
    tenure: usize,
    iterations: usize,
    inner_iterations: usize,
}

impl ProblemSolver for TabuSearch {
    fn solve(&mut self, instance: &ProblemInstance) -> ProblemSolution {
        assert!(self.number_of_points + self.tenure < instance.points.len());
        (0..self.iterations)
            .map(|_| {
                let mut solver = GRASP::new(self.number_of_points, 2, Swap::new(), 1);
                self.perform_search(instance, solver.solve(instance))
            })
            .reduce(|best_solution, solution| {
                if solution.get_z() > best_solution.get_z() {
                    return solution;
                }
                return best_solution;
            })
            .unwrap()
    }
}

impl TabuSearch {
    /// Creates a new tabu search with the specified arguments. The number of points
    /// the solution will have, the tabu tenure, the number of iterations of the
    /// multiboot and the number of iterations without improvement for the actual
    /// search
    pub fn new(
        number_of_points: usize,
        tenure: usize,
        iterations: usize,
        inner_iterations: usize,
    ) -> Self {
        assert!(number_of_points > 0 && tenure > 0);
        TabuSearch {
            number_of_points,
            tenure,
            iterations,
            inner_iterations,
        }
    }

    /// Performs the tabu search with the specified number of iterations(interpreted
    /// as iterations without improvement) and tenure value
    fn perform_search(
        &self,
        instance: &ProblemInstance,
        solution: ProblemSolution,
    ) -> ProblemSolution {
        let mut best_solution = solution.clone();
        let mut actual_solution = solution.clone();
        let mut tabu = VecDeque::with_capacity(self.tenure);
        let mut iters_without_change = 0;
        while iters_without_change < self.inner_iterations {
            if tabu.len() > self.tenure {
                tabu.pop_front();
            }
            let (new_solution, possible_tabu_point2) =
                TabuSearch::get_best_solution_and_exiting_point(&actual_solution, instance, &tabu);
            if let Some((new_tabu_solution, possible_tabu_point1)) =
                TabuSearch::get_best_tabu_solution_and_exiting_point(
                    &actual_solution,
                    instance,
                    &tabu,
                )
            {
                if new_tabu_solution.get_z() > best_solution.get_z()
                    && new_tabu_solution.get_z() > new_solution.get_z()
                {
                    best_solution = new_tabu_solution;
                    actual_solution = best_solution.clone();
                    tabu.push_back(possible_tabu_point1);
                    iters_without_change = 0;
                    continue;
                }
            }
            if new_solution.get_z() > best_solution.get_z() {
                best_solution = new_solution.clone();
                iters_without_change = 0;
            } else {
                iters_without_change += 1;
            }
            actual_solution = new_solution;
            tabu.push_back(possible_tabu_point2);
        }
        best_solution
    }

    fn get_best_tabu_solution_and_exiting_point(
        solution: &ProblemSolution,
        instance: &ProblemInstance,
        tabu: &VecDeque<Point>,
    ) -> Option<(ProblemSolution, Point)> {
        (0..solution.points.len())
            .flat_map(|point_index| {
                instance
                    .points
                    .iter()
                    .filter(|possible_point| {
                        !solution.points.contains(possible_point) && tabu.contains(possible_point)
                    })
                    .map(move |possible_point| {
                        (
                            TabuSearch::get_solution_after_swap(
                                solution.clone(),
                                point_index,
                                possible_point.clone(),
                            ),
                            possible_point.clone(),
                        )
                    })
            })
            .reduce(|acc, value| {
                if value.0.get_z() > acc.0.get_z() {
                    return value;
                }
                return acc;
            })
    }

    fn get_best_solution_and_exiting_point(
        solution: &ProblemSolution,
        instance: &ProblemInstance,
        tabu: &VecDeque<Point>,
    ) -> (ProblemSolution, Point) {
        (0..solution.points.len())
            .flat_map(|point_index| {
                instance
                    .points
                    .iter()
                    .filter(|possible_point| {
                        !solution.points.contains(possible_point) && !tabu.contains(possible_point)
                    })
                    .map(move |possible_point| {
                        (
                            TabuSearch::get_solution_after_swap(
                                solution.clone(),
                                point_index,
                                possible_point.clone(),
                            ),
                            possible_point.clone(),
                        )
                    })
            })
            .reduce(|acc, value| {
                if value.0.get_z() > acc.0.get_z() {
                    return value;
                }
                return acc;
            })
            .unwrap()
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
