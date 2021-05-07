use daap8::{
    problem_solver::{local_search::Swap, ProblemSolver, GRASP},
    ProblemInstance,
};
use std::fs::File;
use std::io::{Result, Write};
use std::time::Instant;

fn main() -> Result<()> {
    let mut output = File::create("result/grasp3.md")?;
    print_headers(&mut output)?;
    for (file, n, k) in vec![
        ("problem_instances/max_div_15_3.txt", 15, 2),
        ("problem_instances/max_div_20_3.txt", 20, 2),
        ("problem_instances/max_div_30_3.txt", 30, 2),
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
                for rcl_size in vec![2, 3] {
                    write!(output, "|{}|{}|{}|{}|{}|{}|", file, n, k, m, iter, rcl_size)?;
                    print_results(&mut output, &instance, m, iter, rcl_size)?;
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
    rcl_size: usize,
) -> Result<()> {
    let mut solver = GRASP::new(m, rcl_size, Swap::new(), iter);
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
    write!(output, "|Problem|n|k|m|Iter|\\|LRC\\||z|S|CPU|\n")?;
    write!(output, "|---|---|---|---|---|---|---|---|---|\n")
}
