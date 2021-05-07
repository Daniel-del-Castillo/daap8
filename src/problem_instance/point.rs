use std::cmp::{Eq, PartialEq};

/// Represents a point in a n dimensional space
#[derive(Debug, Clone)]
pub struct Point {
    pub(crate) coordinates: Vec<f64>,
}

impl Point {
    /// Creates a new point from its coordinates
    pub fn new(coordinates: Vec<f64>) -> Point {
        Point { coordinates }
    }

    /// Gets the dimensionality of the point
    pub fn get_dimensionality(&self) -> usize {
        self.coordinates.len()
    }

    /// The euclidean distance to another point
    pub fn distance_to(&self, other: &Point) -> f64 {
        assert_eq!(self.get_dimensionality(), other.get_dimensionality());
        self.coordinates
            .iter()
            .zip(other.coordinates.iter())
            .map(|(c1, c2)| (c1 - c2).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates
            .iter()
            .zip(other.coordinates.iter())
            .all(|(c1, c2)| c1 == c2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn euclidean_distance() {
        let point1 = Point {
            coordinates: vec![2.0, 3.0, 4.0],
        };
        let point2 = Point {
            coordinates: vec![3.0, 1.0, 2.0],
        };
        assert_eq!(point1.distance_to(&point2), 3.0);
    }
    #[test]
    fn equality() {
        let point1 = Point {
            coordinates: vec![2.0, 3.0, 4.0],
        };
        let point2 = Point {
            coordinates: vec![2.0, 3.0, 4.0],
        };
        assert!(point1 == point2);
    }
    #[test]
    fn false_equality() {
        let point1 = Point {
            coordinates: vec![2.0, 3.0, 4.0],
        };
        let point2 = Point {
            coordinates: vec![3.0, 1.0, 2.0],
        };
        assert!(point1 != point2);
    }
}
