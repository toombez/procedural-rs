use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UniversalPoint<C, const D: usize>([C; D]);

impl<const D: usize, C> UniversalPoint<C, D> {
    pub fn new(values: [C; D]) -> Self {
        Self(values)
    }

    pub fn get(&self, index: usize) -> Option<&C> {
        self.0.get(index)
    }
}

impl<const D: usize, C> UniversalPoint<C, D>
where
    C: Copy,
{
    pub fn values(&self) -> [C; D] {
        self.0.clone()
    }
}

impl<const D: usize, C> UniversalPoint<C, D>
where
    C: Default + Copy,
{
    pub fn convert<C1>(&self) -> UniversalPoint<C1, D> where C1: Default + Copy + TryFrom<C> {
        let values = self.values()
            .map(|value| C1::try_from(value).unwrap_or_default())
            .into_iter();

        UniversalPoint::<C1, D>::from_iter(values)
    }
}

impl<const D: usize, C> FromIterator<C> for UniversalPoint<C, D>
where
    C: Default + Copy,
{
    fn from_iter<T: IntoIterator<Item = C>>(iter: T) -> Self {
        let mut values = [C::default(); D];

        iter.into_iter()
            .enumerate()
            .for_each(|(index, value)| values[index] = value);

        Self(values)
    }
}

impl<const D: usize, C> IntoIterator for UniversalPoint<C, D> {
    type Item = C;
    type IntoIter = std::array::IntoIter<C, D>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const D: usize, C> Index<usize> for UniversalPoint<C, D> {
    type Output = C;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0.index(index)
    }
}

impl<const D: usize, C> IndexMut<usize> for UniversalPoint<C, D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}
