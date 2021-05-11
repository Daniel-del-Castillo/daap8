use super::{Point, ProblemInstance, ProblemSolution, ProblemSolver};
mod partial_solution;
use partial_solution::PartialSolution;

// NOT FINISHED

/// A implementation of a branch and bound algorithm.
pub struct BranchAndBound<'a, S: ProblemSolver> {
    solver: &'a mut S,
}

impl<'a, S: ProblemSolver> ProblemSolver for BranchAndBound<'a, S> {
    fn solve(&mut self, instance: &ProblemInstance) -> ProblemSolution {
        let initial_solution = self.solver.solve(instance);
        let number_of_points = initial_solution.points.len();
        let lower_bound = initial_solution.get_z();
        // let mut partial_solutions = Vec::new();
        initial_solution
    }
}

impl<'a, S: ProblemSolver> BranchAndBound<'a, S> {
    /// Creates a new instance with the specified arguments. A [solver](super::ProblemSolver)
    /// needs to be passed as argument. It will be used to get the initial lower bound. The
    /// number of points in the solution will be infered from the solution given from the
    /// solver
    pub fn new(solver: &'a mut S) -> Self {
        BranchAndBound { solver }
    }
}
