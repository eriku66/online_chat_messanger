use anyhow::{anyhow, Result};

pub struct SliceUtil;

impl SliceUtil {
    pub fn split_slice(slice: &[u8], lengths: &[usize]) -> Result<Vec<Vec<u8>>> {
        if slice.len() < lengths.iter().sum::<usize>() {
            return Err(anyhow!("Lengths are greater than slice length"));
        }

        let mut result = Vec::new();
        let mut index_to_index = 0;

        for length in lengths {
            result.push(slice[index_to_index..index_to_index + length].to_vec());

            index_to_index += length;
        }

        if index_to_index < slice.len() {
            result.push(slice[index_to_index..].to_vec());
        }

        Ok(result)
    }

    pub fn try_get_at_index<T>(slice: &[T], index: usize) -> Result<T>
    where
        T: Copy,
    {
        slice
            .get(index)
            .ok_or_else(|| anyhow!("Index out of bounds"))
            .copied()
    }

    pub fn try_get_range<T, R>(slice: &[T], range: R) -> Result<&[T]>
    where
        R: std::ops::RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&start) => start,
            std::ops::Bound::Excluded(&start) => start + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(&end) => end + 1,
            std::ops::Bound::Excluded(&end) => end,
            std::ops::Bound::Unbounded => slice.len(),
        };

        slice.get(start..end).ok_or_else(|| {
            anyhow!(
                "Range out of bounds. slice length: {} start: {} end: {}",
                slice.len(),
                start,
                end
            )
        })
    }
}
