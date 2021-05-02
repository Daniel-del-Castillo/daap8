//! This project was made by Daniel del Castillo de la Rosa for the Algorithm Design
//! and Analysis class (DAA in spanish) at the La Laguna University.
//!
//! This code can be used to solve a maximum diversity problem. For that it uses
//! different algorithms and Metaheuristics like GRASP or a Tabu search

mod problem_instance;
pub use problem_instance::{Point, ProblemInstance, ProblemInstanceError};
pub mod problem_solver;
