#![allow(unused)]

use std::ops;
use std::fmt;

/// 3 Dimensional volume of data
pub struct Volume<T: Sized, const SIZE: usize>([[[T; SIZE]; SIZE]; SIZE]);

/// Iterator over a 3D volume
pub struct VolumeIterator<'volume, T: Sized, const SIZE: usize> {
    vol: &'volume Volume<T, SIZE>,
    idx: VolumeIdx
}

/// Iterator over the indices in a volume. Can be used instead of mutable iterators.
pub struct VolumeIndexIterator<const SIZE: usize>(VolumeIdx);

/// This type may be used to index a Volume
pub type VolumeIdx = (usize, usize, usize);

impl<T, const SIZE: usize> ops::Index<VolumeIdx> for Volume<T, SIZE> {
    type Output = T;

    #[inline]
    fn index(&self, index: VolumeIdx) -> &Self::Output {
        &self.0[index.0][index.1][index.2]
    }
}

impl<T, const SIZE: usize> ops::IndexMut<VolumeIdx> for Volume<T, SIZE> {
    #[inline]
    fn index_mut(&mut self, index: VolumeIdx) -> &mut Self::Output {
        &mut self.0[index.0][index.1][index.2]
    }
}

impl<T, const SIZE: usize> fmt::Debug for Volume<T, SIZE> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Volume<{0}x{0}x{0}>", SIZE)
    }
}

impl<T, const SIZE: usize> From<[[[T; SIZE]; SIZE]; SIZE]> for Volume<T, SIZE> {
    fn from(arr: [[[T; SIZE]; SIZE]; SIZE]) -> Self {
        Self { 0: arr }
    }
}

impl<T, const SIZE: usize> From<Volume<T, SIZE>> for [[[T; SIZE]; SIZE]; SIZE] {
    fn from(vol: Volume<T, SIZE>) -> Self {
        vol.0
    }
}

impl<T, const SIZE: usize> Volume<T, SIZE> {
    pub fn filled(item: T) -> Self where T: Copy {
        Self { 0: [[[item; SIZE]; SIZE]; SIZE] }
    }

    pub fn iter(&self) -> VolumeIterator<T, SIZE> {
        VolumeIterator {
            vol: self,
            idx: (0, 0, 0)
        }
    }

    pub fn iter_indices(&self) -> VolumeIndexIterator<SIZE> {
        VolumeIndexIterator { 0: (0, 0, 0) }
    }
    
    pub fn get(&self, idx: VolumeIdx) -> Option<&T> {
        if idx.0 >= SIZE || idx.1 >= SIZE || idx.2 >= SIZE {
            None
        } else {
            Some(&self[idx])
        }
    }
}

impl<'volume, T, const SIZE: usize> Iterator for VolumeIterator<'volume, T, SIZE> {
    type Item = (VolumeIdx, &'volume T);
    fn next(&mut self) -> Option<Self::Item> {

        if self.idx.2 >= SIZE {
            return None
        }

        let item = &self.vol[self.idx];
        let item_idx = self.idx;

        self.idx.0 += 1;
        if self.idx.0 >= SIZE {
            self.idx.0 = 0;
            self.idx.1 += 1;
        }

        if self.idx.1 >= SIZE {
            self.idx.1 = 0;
            self.idx.2 += 1;
        }

        Some((item_idx, item))
    }
}

impl<const SIZE: usize> Iterator for VolumeIndexIterator<SIZE> {
    type Item = VolumeIdx;
    fn next(&mut self) ->  Option<Self::Item> {
        let index_before = self.0;

        if self.0.2 >= SIZE {
            return None
        }

        self.0.0 += 1;
        if self.0.0 >= SIZE {
            self.0.0 = 0;
            self.0.1 += 1;
        }

        if self.0.1 >= SIZE {
            self.0.1 = 0;
            self.0.2 += 1;
        }

        Some(index_before)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill() {
        // Fill a volume up with zeroes
        let mut volume: Volume<u64, 32> = Volume::filled(0u64);

        // Is this spot here 0?
        assert_eq!(volume[(3, 5, 9)], 0u64);

        // Try changing that spot to something else...
        volume[(3, 5, 9)] = 128;

        // Did it change?
        assert_eq!(volume[(3, 5, 9)], 128u64);
    }

    #[test]
    fn iter() {
        // Fill a volume up with zeroes
        let mut volume: Volume<u64, 8> = Volume::filled(0u64);

        // Values correspond with sum of index
        volume[(0, 4, 4)] = 8;
        volume[(1, 4, 4)] = 9;
        volume[(2, 4, 4)] = 10;
        volume[(3, 4, 4)] = 11;
        volume[(4, 4, 4)] = 12;
        volume[(5, 4, 4)] = 13;
        volume[(6, 4, 4)] = 14;
        volume[(7, 4, 4)] = 15;

        // Check if that change actually happened
        for (idx, &value) in volume.iter() {
            if !(idx.1 == 4 && idx.2 == 4) { continue; }

            let sum = (idx.0 + idx.1 + idx.2) as u64;
            assert_eq!(sum, value)
        }
    }

    #[test]
    fn mut_in_iter() {
        // Fill a volume up with zeroes
        let mut volume: Volume<u64, 8> = Volume::filled(0u64);

        // Values correspond with sum of index
        volume[(0, 4, 4)] = 8;
        volume[(1, 4, 4)] = 9;
        volume[(2, 4, 4)] = 10;
        volume[(3, 4, 4)] = 11;
        volume[(4, 4, 4)] = 12;
        volume[(5, 4, 4)] = 13;
        volume[(6, 4, 4)] = 14;
        volume[(7, 4, 4)] = 15;

        // Just to break the pattern a little
        volume[(7, 3, 6)] = 12;

        // Double all 12s
        for idx in volume.iter_indices() {
            if volume[idx] == 12 {
                volume[idx] *= 2;
            }
        }

        // Did our changes happen?
        assert_eq!(volume[(4, 4, 4)], 12*2);
        assert_eq!(volume[(7, 3, 6)], 12*2);
    }

    #[test]
    fn iter_idx() {
        let mut volume: Volume<u64, 2> = Volume::filled(0u64);

        let mut it = volume.iter_indices();

        assert_eq!(it.next(), Some((0, 0, 0)));
        assert_eq!(it.next(), Some((1, 0, 0)));

        assert_eq!(it.next(), Some((0, 1, 0)));
        assert_eq!(it.next(), Some((1, 1, 0)));

        assert_eq!(it.next(), Some((0, 0, 1)));
        assert_eq!(it.next(), Some((1, 0, 1)));

        assert_eq!(it.next(), Some((0, 1, 1)));
        assert_eq!(it.next(), Some((1, 1, 1)));

        assert_eq!(it.next(), None);
    }
}
