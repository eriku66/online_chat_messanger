pub struct SliceUtil;

impl SliceUtil {
    pub fn split_slice(slice: &[u8], lengths: &[usize]) -> Vec<Vec<u8>> {
        let mut result = Vec::new();
        let mut index_to_index = 0;

        for length in lengths {
            result.push(slice[index_to_index..index_to_index + length].to_vec());

            index_to_index += length;
        }

        if index_to_index < slice.len() {
            result.push(slice[index_to_index..].to_vec());
        }

        result
    }
}
