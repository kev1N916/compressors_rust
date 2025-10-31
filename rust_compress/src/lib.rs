pub mod p_for_delta;
pub mod rice;
pub mod simple16;
pub mod simple8b;
pub mod simple9;
pub mod var_byte;

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn example_simple8b_usage() {
        // Shows how to use simple8b module
        let data = vec![1, 2, 3, 4, 5];
        let encoded = simple8b::compress(&data);
        let decoded = simple8b::decompress_from_bytes(&encoded);
        assert_eq!(&data, &decoded[..data.len()]);
    }

    #[test]
    fn example_simple9_usage() {
        // Shows how to use simple8b module
        let data = vec![1, 2, 3, 4, 5];
        let encoded = simple9::compress(&data);
        let decoded = simple9::decompress_from_bytes(&encoded);
        assert_eq!(&data, &decoded[..data.len()]);
    }

    #[test]
    fn example_simple16_usage() {
        // Shows how to use simple8b module
        let data = vec![1, 2, 3, 4, 5];
        let encoded = simple16::compress(&data);
        let decoded = simple16::decompress_from_bytes(&encoded);
        assert_eq!(&data, &decoded[..data.len()]);
    }

    #[test]
    fn example_p_for_delta_usage() {
        // Shows how to use simple8b module
        let mut rng = rand::thread_rng();
        let data: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 7)
                    } else {
                        (1 + val) % (1u32 << 7)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 2)
                    } else {
                        (val) % (1u32 << 2)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = p_for_delta::compress(&data);
        let decoded = p_for_delta::decompress(&encoded);
        assert_eq!(&data, &decoded[..data.len()]);
    }

    // #[test]
    // fn example_rice_encoding() {
    //     // Shows rice encoding
    //     let value = 42;
    //     let encoded = rice::compress(value, 4);
    //     assert!(!encoded.is_empty());
    // }

    // #[test]
    // fn example_var_byte() {
    //     let nums = vec![100, 200, 300];
    //     let encoded = var_byte::encode(&nums);
    //     let decoded = var_byte::decode(&encoded);
    //     assert_eq!(nums, decoded);
    // }
}
