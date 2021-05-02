//! This module defines the class [ProblemInstance](ProblemInstance) which represents
//! an instance of this problem.
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{fs::File, num::ParseFloatError};

mod problem_instance_error;
pub use problem_instance_error::{
    ProblemInstanceError,
    ProblemInstanceError::{IOError, SyntaxError},
};

const SEPARATOR: &'static str = "\t";

/// An instance of the problem. It is composed of a set of points of the same dimensionality
pub struct ProblemInstance {
    points: Vec<Vec<f64>>,
}

impl ProblemInstance {
    /// This function allows to read a Problem instance from a file. The file must have
    /// have the following format.
    /// (You should substitute the {} with the correct values and use a tab as
    /// separator):<br/><br/>
    /// {number of points}<br/>
    /// {dimensionality of the points}<br/>
    /// {A point with a coordinate for each dimension separated by tabs}<br/>
    /// Continues until all the points have been described<br/><br/>
    /// If the coordinates aren't integers you must use . as separator
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ProblemInstanceError> {
        let mut file_reader = BufReader::new(File::open(path)?);
        let mut line = String::new();
        file_reader.read_line(&mut line)?;
        let number_of_points = line.parse::<usize>().map_err(|_| SyntaxError(1))?;
        line.clear();
        file_reader.read_line(&mut line)?;
        let dimensionality = line.parse::<usize>().map_err(|_| SyntaxError(2))?;
        let mut points = Vec::new();
        for i in 0..=number_of_points {
            line.clear();
            file_reader.read_line(&mut line)?;
            points.push(match ProblemInstance::parse_point(&line, SEPARATOR) {
                Some(point) if point.len() == dimensionality => point,
                _ => return Err(SyntaxError(i + 3)),
            });
        }
        Ok(ProblemInstance { points })
    }

    fn parse_point(point_str: &str, separator: &str) -> Option<Vec<f64>> {
        point_str
            .split(separator)
            .map(|coordinate| coordinate.parse::<f64>())
            .collect::<Result<Vec<f64>, ParseFloatError>>()
            .ok()
    }

    /// Allows to get the list of points
    pub fn points(&self) -> &Vec<Vec<f64>> {
        &self.points
    }
}
