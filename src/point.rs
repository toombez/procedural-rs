use std::ops::{Index, IndexMut};
use std::hash::Hash;

pub trait PointCoordinate: Clone {}
impl<T: Clone> PointCoordinate for T {}

pub trait PointImplementation<const DIMENSION: usize, T: PointCoordinate>:
    Clone
    + From<Vec<T>>
    + for<'a> From<&'a Vec<T>>
    + From<[T; DIMENSION]>
    + for <'a> From<&'a [T; DIMENSION]>
    + Into<Vec<T>>
    + Into<[T; DIMENSION]>
    + IntoIterator
    + Index<usize>
    + IndexMut<usize>
{
    fn coordinates(&self) -> Vec<T>;

    fn dimension(&self) -> usize {
        DIMENSION
    }
}

// Base struct
#[derive(Debug, Clone)]
pub struct Point<const DIMENSION: usize, T: PointCoordinate> {
    coordinates: Vec<T>,
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

// From vector trait
impl <const DIMENSION: usize, T: PointCoordinate> From<Vec<T>> for Point<DIMENSION, T> {
    fn from(value: Vec<T>) -> Self {
        Self { coordinates: value[..DIMENSION].to_vec() }
    }
}
impl <'a, const DIMENSION: usize, T: PointCoordinate> From<&'a Vec<T>> for Point<DIMENSION, T> {
    fn from(value: &'a Vec<T>) -> Self {
        Self::from(value.clone())
    }
}

// From array trait
impl <const DIMENSION: usize, T: PointCoordinate> From<[T; DIMENSION]> for Point<DIMENSION, T> {
    fn from(value: [T; DIMENSION]) -> Self {
        Self::from(value.to_vec())
    }
}
impl <'a, const DIMENSION: usize, T: PointCoordinate> From<&'a [T; DIMENSION]> for Point<DIMENSION, T> {
    fn from(value: &'a [T; DIMENSION]) -> Self {
        Self::from(value.clone())
    }
}

// Into vector and array trait
impl <const DIMENSION: usize, T: PointCoordinate> Into<Vec<T>> for Point<DIMENSION, T> {
    fn into(self) -> Vec<T> {
        self.coordinates.clone()
    }
}
impl <const DIMENSION: usize, T: PointCoordinate> Into<[T; DIMENSION]> for Point<DIMENSION, T> {
    fn into(self) -> [T; DIMENSION] {
        self.coordinates
            .try_into()
            .unwrap_or_else(|_: Vec<T>| panic!("TODO: MESSAGE"))
    }
}

// IntoIterator, Index and IndexMut trait
impl <const DIMENSION: usize, T: PointCoordinate> IntoIterator for Point<DIMENSION, T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.coordinates.into_iter()
    }
}
impl <const DIMENSION: usize, T: PointCoordinate> Index<usize> for Point<DIMENSION, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coordinates[index]
    }
}
impl <const DIMENSION: usize, T: PointCoordinate> IndexMut<usize> for Point<DIMENSION, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.coordinates.index_mut(index)
    }
}

// Default trait
impl <const DIMENSION: usize, T: PointCoordinate + Default> Default for Point<DIMENSION, T> {
    fn default() -> Self {
        Self::from(vec![T::default(); DIMENSION])
    }
}

// Fill constructor
impl <const DIMENSION: usize, T: PointCoordinate> Point<DIMENSION, T> {
    pub fn fill(value: T) -> Self {
        Self::from(vec![value; DIMENSION])
    }
}

// Basic point dimensions
impl <T: PointCoordinate> Point<0, T> {
    pub fn point_1d(x: T) -> Point<1, T> {
        Point::<1, T>::from(vec![x])
    }

    pub fn point_2d(x: T, y: T) -> Point<2, T> {
        Point::<2, T>::from(vec![x, y])
    }

    pub fn point_3d(x: T, y: T, z: T) -> Point<3, T> {
        Point::<3, T>::from(vec![x, y, z])
    }
}

// Getters for 1, 2, 3 dimension points
impl <T: PointCoordinate> Point<1, T> {
    pub fn x(&self) -> T {
        self[0].clone()
    }
}
impl <T: PointCoordinate> Point<2, T> {
    pub fn x(&self) -> T {
        self[0].clone()
    }

    pub fn y(&self) -> T {
        self[1].clone()
    }
}
impl <T: PointCoordinate> Point<3, T> {
    pub fn x(&self) -> T {
        self[0].clone()
    }

    pub fn y(&self) -> T {
        self[1].clone()
    }

    pub fn z(&self) -> T {
        self[2].clone()
    }
}
