use std::ops::{Index, IndexMut};

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
