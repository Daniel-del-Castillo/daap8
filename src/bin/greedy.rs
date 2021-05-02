use daap8::{
    problem_solver::{GreedySolver, ProblemSolver},
    ProblemInstance,
};
use std::fs::File;
use std::io::{Result, Write};
use std::time::Instant;

fn main() -> Result<()> {
    let mut output = File::create("result/greedy.md")?;
    print_headers(&mut output)?;
    for (file, n, k) in vec![
        ("problem_instances/max_div_15_2.txt", 15, 2),
        ("problem_instances/max_div_20_2.txt", 20, 2),
        ("problem_instances/max_div_30_2.txt", 30, 2),
        ("problem_instances/max_div_15_3.txt", 15, 3),
        ("problem_instances/max_div_20_3.txt", 20, 3),
        ("problem_instances/max_div_30_3.txt", 30, 3),
    ] {
        let instance = match ProblemInstance::from_file(file) {
            Ok(instance) => instance,
            Err(err) => {
                println!("{}: {}", file, err);
                return Ok(());
            }
        };
        for m in 2..=5 {
            write!(output, "|{}|{}|{}|{}|", file, n, k, m)?;
            print_results(&mut output, &instance, m)?;
        }
    }
    Ok(())
}

fn print_results(output: &mut File, instance: &ProblemInstance, m: usize) -> Result<()> {
    let mut solver = GreedySolver::new(m);
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
    write!(output, "|Problem|n|k|m|z|S|CPU|\n")?;
    write!(output, "|---|---|---|---|---|---|---|\n")
}
