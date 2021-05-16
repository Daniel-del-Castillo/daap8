use super::{Point, ProblemInstance};
mod problem_solution;
pub use problem_solution::ProblemSolution;
mod greedy_solver;
pub use greedy_solver::GreedySolver;
mod randomized_greedy_solver;
pub use randomized_greedy_solver::RandomizedGreedySolver;
mod tabu_search;
pub use tabu_search::TabuSearch;
mod branch_and_bound;
pub use branch_and_bound::{BranchAndBound, DeepBranchAndBound};
mod grasp;
pub use grasp::GRASP;
pub mod local_search;

/// A trait for an algorithm that is able to solve an instance of the problem
pub trait ProblemSolver {
    /// Solves an instance of the problem
    fn solve(&mut self, instance: &ProblemInstance) -> ProblemSolution;
}
