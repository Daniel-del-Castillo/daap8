use super::{Point, ProblemInstance, ProblemSolution, ProblemSolver};
mod partial_solution;
use partial_solution::PartialSolution;
mod deep_branch_and_bound;
pub use deep_branch_and_bound::DeepBranchAndBound;

/// A implementation of a branch and bound algorithm.
pub struct BranchAndBound<'a, S: ProblemSolver> {
    solver: &'a mut S,
    generated_nodes: usize,
}

impl<'a, S: ProblemSolver> ProblemSolver for BranchAndBound<'a, S> {
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
            let partial_solution =
                get_partial_solution_with_smallest_upper_bound(&mut partial_solutions);
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

impl<'a, S: ProblemSolver> BranchAndBound<'a, S> {
    /// Creates a new instance with the specified arguments. A [solver](super::ProblemSolver)
    /// needs to be passed as argument. It will be used to get the initial lower bound. The
    /// number of points in the solution will be infered from the solution given from the
    /// solver
    pub fn new(solver: &'a mut S) -> Self {
        BranchAndBound {
            solver,
            generated_nodes: 0,
        }
    }

    pub fn get_generated_nodes(&self) -> usize {
        self.generated_nodes
    }
}

/// Searches in the partial solutions vector and extracts the one with the smallest
/// upper bound
fn get_partial_solution_with_smallest_upper_bound(
    partial_solutions: &mut Vec<PartialSolution>,
) -> PartialSolution {
    let index = partial_solutions
        .iter()
        .enumerate()
        .reduce(|min, actual| {
            if actual.1.upper_bound < min.1.upper_bound {
                return actual;
            }
            return min;
        })
        .map(|(index, _)| index)
        .unwrap();
    partial_solutions.remove(index)
}

/// Get the points a certain partial solution can add. This is constrained so that
/// the search tree we get doesn't get repeated paths, which can happen because in
/// this problem the order in which we add the points doesn't matter. To prevent
/// this a partial solution can only add n - m + k - i nodes, starting from the
/// index of last point that was added added. n is the number of total points, m is
/// the number of points a complete solution has, k is the number of points already
/// in the partial solution and i is the index of the last added point in the
/// vector that contains all the points
fn get_possible_points_to_add(
    instance: &ProblemInstance,
    partial_solution: &PartialSolution,
    m: usize,
) -> Vec<Point> {
    let i = if partial_solution.solution.points.len() == 0 {
        0
    } else {
        instance
            .points
            .iter()
            .enumerate()
            .find(|(_, point)| point == &partial_solution.solution.points.last().unwrap())
            .map(|(index, _)| index)
            .unwrap()
            + 1
    };
    let n = instance.points.len();
    let k = partial_solution.solution.points.len();
    instance
        .points
        .iter()
        .skip(i)
        .take(n - m + k - i + 1)
        .cloned()
        .collect::<Vec<Point>>()
}

/// Get a new partial solution after adding another point
fn get_new_partial_solution(
    partial_solution: &PartialSolution,
    point: Point,
    instance: &ProblemInstance,
    number_of_points: usize,
) -> PartialSolution {
    let mut points = partial_solution.solution.points.clone();
    points.push(point);
    let partial_solution =
        PartialSolution::new(ProblemSolution { points }, instance, number_of_points);
    partial_solution
}

/// Prunes the partial solutions that have an upper bound lower or equal than
/// the lower bound we already have
fn prune(partial_solutions: Vec<PartialSolution>, lower_bound: f64) -> Vec<PartialSolution> {
    partial_solutions
        .into_iter()
        .filter(|partial_solution| partial_solution.upper_bound > lower_bound)
        .collect()
}
