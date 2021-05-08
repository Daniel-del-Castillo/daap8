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
mod point;
pub use point::Point;

const SEPARATOR: &'static str = "\t";

/// An instance of the problem. It is composed of a set of points of the same dimensionality
pub struct ProblemInstance {
    pub(super) points: Vec<Point>,
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
    /// If the coordinates aren't integers you must use . as separator<br/>
    /// Keep in mind that two equal points shouldn't exist
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ProblemInstanceError> {
        let mut file_reader = BufReader::new(File::open(path)?);
        let mut line = String::new();
        file_reader.read_line(&mut line)?;
        let number_of_points = line.trim().parse::<usize>().map_err(|_| SyntaxError(1))?;
        line.clear();
        file_reader.read_line(&mut line)?;
        let dimensionality = line.trim().parse::<usize>().map_err(|_| SyntaxError(2))?;
        let mut points = Vec::new();
        for i in 0..number_of_points {
            line.clear();
            file_reader.read_line(&mut line)?;
            points.push(match ProblemInstance::parse_point(&line, SEPARATOR) {
                Some(point) if point.get_dimensionality() == dimensionality => point,
                _ => return Err(SyntaxError(i + 3)),
            });
        }
        Ok(ProblemInstance { points })
    }

    fn parse_point(point_str: &str, separator: &str) -> Option<Point> {
        point_str
            .trim()
            .replace(",", ".")
            .split(separator)
            .map(|coordinate| coordinate.parse::<f64>())
            .collect::<Result<Vec<f64>, ParseFloatError>>()
            .map(|coordinates| Point::new(coordinates))
            .ok()
    }

    /// Allows to get the list of points
    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }
}
