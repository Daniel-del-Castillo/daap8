use daap8::{
    problem_solver::{ProblemSolver, TabuSearch},
    ProblemInstance,
};
use std::fs::File;
use std::io::{Result, Write};
use std::time::Instant;

fn main() -> Result<()> {
    let mut output = File::create("result/tabu_search2.md")?;
    print_headers(&mut output)?;
    for (file, n, k) in vec![
        ("problem_instances/max_div_15_2.txt", 15, 2),
        ("problem_instances/max_div_20_2.txt", 20, 2),
        ("problem_instances/max_div_30_2.txt", 30, 2),
    ] {
        let instance = match ProblemInstance::from_file(file) {
            Ok(instance) => instance,
            Err(err) => {
                println!("{}: {}", file, err);
                return Ok(());
            }
        };
        for m in 2..=5 {
            for iter in vec![10, 20] {
                for tabu_tenure in vec![2, 3] {
                    write!(
                        output,
                        "|{}|{}|{}|{}|{}|{}|",
                        file, n, k, m, iter, tabu_tenure
                    )?;
                    print_results(&mut output, &instance, m, iter, tabu_tenure)?;
                }
            }
        }
    }
    Ok(())
}

fn print_results(
    output: &mut File,
    instance: &ProblemInstance,
    m: usize,
    iter: usize,
    tabu_tenure: usize,
) -> Result<()> {
    let mut solver = TabuSearch::new(m, tabu_tenure, iter, 10);
    let instant = Instant::now();
    let solution = solver.solve(instance);
    let duration = instant.elapsed();
    write!(
        output,
        "{:.3}|{}|{}|\n",
        solution.get_z(),
        solution,
        duration.as_micros()
    )?;
    Ok(())
}

fn print_headers(output: &mut File) -> Result<()> {
    write!(output, "|Problem|n|k|m|Iter|Tabu tenure|z|S|CPU|\n")?;
    write!(output, "|---|---|---|---|---|---|---|---|---|\n")
}
