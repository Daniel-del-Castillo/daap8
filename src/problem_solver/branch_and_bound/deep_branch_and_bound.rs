use super::{
    get_new_partial_solution, get_possible_points_to_add, prune, PartialSolution, ProblemInstance,
    ProblemSolution, ProblemSolver,
};

/// A implementation of a branch and bound algorithm. It chooses the deepest
/// node in each iteration unlike [BranchAndBound](super::BranchAndBound) which
/// chooses the node with the lowest upper bound
pub struct DeepBranchAndBound<'a, S: ProblemSolver> {
    solver: &'a mut S,
    generated_nodes: usize,
}

impl<'a, S: ProblemSolver> ProblemSolver for DeepBranchAndBound<'a, S> {
    fn solve(&mut self, instance: &ProblemInstance) -> ProblemSolution {
        self.generated_nodes = 0;
        let mut best_solution = self.solver.solve(instance);
        let number_of_points = best_solution.points.len();
        let mut lower_bound = best_solution.get_z();
        let mut partial_solutions = vec![PartialSolution::new(
            ProblemSolution { points: Vec::new() },
            instance,
            number_of_points,
        )];
        while !partial_solutions.is_empty() {
            let partial_solution = get_deepest_partial_solution(&mut partial_solutions);
            let possible_points =
                get_possible_points_to_add(instance, &partial_solution, number_of_points);
            self.generated_nodes += possible_points.len();
            for point in possible_points {
                let partial_solution =
                    get_new_partial_solution(&partial_solution, point, instance, number_of_points);
                if partial_solution.solution.points.len() == number_of_points {
                    if partial_solution.upper_bound > lower_bound {
                        lower_bound = partial_solution.upper_bound;
                        best_solution = partial_solution.solution;
                    }
                } else if partial_solution.upper_bound > lower_bound {
                    partial_solutions.push(partial_solution);
                }
            }
            partial_solutions = prune(partial_solutions, lower_bound);
        }
        best_solution
    }
}

impl<'a, S: ProblemSolver> DeepBranchAndBound<'a, S> {
    /// Creates a new instance with the specified arguments. A [solver](super::ProblemSolver)
    /// needs to be passed as argument. It will be used to get the initial lower bound. The
    /// number of points in the solution will be infered from the solution given from the
    /// solver
    pub fn new(solver: &'a mut S) -> Self {
        DeepBranchAndBound {
            solver,
            generated_nodes: 0,
        }
    }

    pub fn get_generated_nodes(&self) -> usize {
        self.generated_nodes
    }
}

/// Searches in the partial solutions vector and extracts the deepest one
fn get_deepest_partial_solution(partial_solutions: &mut Vec<PartialSolution>) -> PartialSolution {
    let index = partial_solutions
        .iter()
        .enumerate()
        .reduce(|deepest, actual| {
            if actual.1.solution.points.len() > deepest.1.solution.points.len() {
                return actual;
            }
            return deepest;
        })
        .map(|(index, _)| index)
        .unwrap();
    partial_solutions.remove(index)
}
