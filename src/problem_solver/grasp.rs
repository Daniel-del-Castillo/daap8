use super::{
    local_search::LocalSearch, ProblemInstance, ProblemSolution, ProblemSolver,
    RandomizedGreedySolver,
};

/// A implementation of a GRASP algorithm.  The local search to be used can
/// be chosen and passed to the constructor. For the constructive phase it will use the
/// [Randomized greedy solver algorithm](super::RandomizedGreedySolver).
/// The number of points in the solution and the size of the restricted candidate list
pub struct GRASP<L: LocalSearch> {
    number_of_points: usize,
    rcl_size: usize,
    local_search: L,
    iterations: usize,
}

impl<L: LocalSearch> ProblemSolver for GRASP<L> {
    fn solve(&mut self, instance: &ProblemInstance) -> ProblemSolution {
        (0..self.iterations)
            .map(|_| {
                let mut solver = RandomizedGreedySolver::new(self.number_of_points, self.rcl_size);
                self.local_search.improve(instance, solver.solve(instance))
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

impl<L: LocalSearch> GRASP<L> {
    /// Creates a new GRASP with the specified arguments
    pub fn new(
        number_of_points: usize,
        rcl_size: usize,
        local_search: L,
        iterations: usize,
    ) -> Self {
        assert!(number_of_points > 0 && rcl_size > 0);
        GRASP {
            number_of_points,
            rcl_size,
            local_search,
            iterations,
        }
    }
}
