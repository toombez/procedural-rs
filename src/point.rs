use std::ops::{Index, IndexMut};
use std::hash::Hash;

pub trait PointCoordinate: Clone {}
impl<T: Clone> PointCoordinate for T {}

pub trait PointImplementation<const DIMENSION: usize, T: PointCoordinate>:
    Clone
{
    fn coordinates(&self) -> Vec<T>;

    fn dimension(&self) -> usize {
        DIMENSION
    }
}

// Base struct
#[derive(Debug, Clone)]
pub struct Point<const DIMENSION: usize, T: PointCoordinate> {
    pub coordinates: Vec<T>,
}

// Base implementation
impl <const DIMENSION: usize, T: PointCoordinate> PointImplementation<DIMENSION, T> for Point<DIMENSION, T> {
    fn coordinates(&self) -> Vec<T> {
        self.coordinates.clone()
    }
}

// Eq trait
impl <const DIMENSION: usize, T: PointCoordinate + PartialEq> PartialEq for Point<DIMENSION, T> {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates
    }
}
impl <const DIMENSION: usize, T: PointCoordinate + Eq> Eq for Point<DIMENSION, T> {}

// Ord trait
impl <const DIMENSION: usize, T: PointCoordinate + PartialOrd> PartialOrd for Point<DIMENSION, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.coordinates.partial_cmp(&other.coordinates)
    }
}
impl <const DIMENSION: usize, T: PointCoordinate + Ord> Ord for Point<DIMENSION, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.coordinates.cmp(&other.coordinates)
    }
}

// Hash trait
impl <const DIMENSION: usize, T: PointCoordinate + Hash> Hash for Point<DIMENSION, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coordinates.hash(state);
    }
}
