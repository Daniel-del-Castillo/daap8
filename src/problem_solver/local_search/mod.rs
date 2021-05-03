use super::{ProblemInstance, ProblemSolution};

mod swap;
pub use swap::Swap;
/// A trait that specifies how a local search should behave. A local search
/// should search for better solutions inside an specific environment and
/// only stop searching when it can't find a better one.
pub trait LocalSearch {
    /// Performs a local search that stops when there isn't a better solution
    fn improve(
        &self,
        instance: &ProblemInstance,
        mut solution: ProblemSolution,
    ) -> ProblemSolution {
        loop {
            let another_solution = self.perform_search(instance, solution.clone());
            if another_solution.get_z() <= solution.get_z() {
                return solution;
            }
            solution = another_solution;
        }
    }

    /// Performs a local search **only** in the environment of the actual solution.
    /// It can return a solution that **might** be a better one than the actual
    fn perform_search(
        &self,
        instance: &ProblemInstance,
        solution: ProblemSolution,
    ) -> ProblemSolution;
}
