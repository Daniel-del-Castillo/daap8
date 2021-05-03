use super::{Point, ProblemInstance};
mod problem_solution;
pub use problem_solution::ProblemSolution;
mod greedy_solver;
pub use greedy_solver::GreedySolver;
pub mod local_search;

/// A trait for an algorithm that is able to solve an instance of the problem
pub trait ProblemSolver {
    /// Solves an instance of the problem
    fn solve(&mut self, instance: &ProblemInstance) -> ProblemSolution;
}
