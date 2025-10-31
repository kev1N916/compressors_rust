#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Simple8bSelector {
    item_width: u32,
    group_size: u32,
    wasted_bits: u32,
}

const SELECTOR_MASK: u64 = 0x0000000F;
const MAX_NUMBER_POSSIBLE: u64 = (1 << 60) - 1;
const SELECTOR_BITS: u64 = 4;

const SELECTORS: [Simple8bSelector; 16] = [
    Simple8bSelector {
        item_width: 0,
        group_size: 240,
        wasted_bits: 60,
    },
    Simple8bSelector {
        item_width: 0,
        group_size: 120,
        wasted_bits: 60,
    },
    Simple8bSelector {
        item_width: 1,
        group_size: 60,
        wasted_bits: 0,
    },
    Simple8bSelector {
        item_width: 2,
        group_size: 30,
        wasted_bits: 0,
    },
    Simple8bSelector {
        item_width: 3,
        group_size: 20,
        wasted_bits: 0,
    },
    Simple8bSelector {
        item_width: 4,
        group_size: 15,
        wasted_bits: 0,
    },
    Simple8bSelector {
        item_width: 5,
        group_size: 12,
        wasted_bits: 0,
    },
    Simple8bSelector {
        item_width: 6,
        group_size: 10,
        wasted_bits: 0,
    },
    Simple8bSelector {
        item_width: 7,
        group_size: 8,
        wasted_bits: 4,
    },
    Simple8bSelector {
        item_width: 8,
        group_size: 7,
        wasted_bits: 4,
    },
    Simple8bSelector {
        item_width: 10,
        group_size: 6,
        wasted_bits: 0,
    },
    Simple8bSelector {
        item_width: 12,
        group_size: 5,
        wasted_bits: 0,
    },
    Simple8bSelector {
        item_width: 15,
        group_size: 4,
        wasted_bits: 0,
    },
    Simple8bSelector {
        item_width: 20,
        group_size: 3,
        wasted_bits: 0,
    },
    Simple8bSelector {
        item_width: 30,
        group_size: 2,
        wasted_bits: 0,
    },
    Simple8bSelector {
        item_width: 60,
        group_size: 1,
        wasted_bits: 0,
    },
];

pub fn compress(list: &Vec<u64>) -> Vec<u8> {
    let mut encoded_result: Vec<u8> = vec![];

    let n = list.len();
    let mut i = 0;
    while i < n {
        for (selector_idx, selector) in SELECTORS.iter().enumerate() {
            match selector_idx {
                0 | 1 => {
                    // Special cases: run-length encoding of 1s
                    let run_length = if selector_idx == 0 { 240 } else { 120 };
                    let ones_count = list[i..n.min(i + run_length)]
                        .iter()
                        .take_while(|&&x| x == 1)
                        .count();

                    if ones_count == run_length {
                        encoded_result.extend_from_slice(&(selector_idx as u64).to_le_bytes());
                        i += run_length;
                        break;
                    }
                }
                _ => {
                    // Pack multiple items with specific bit width
                    let mut data = selector_idx as u64;
                    let mut shift = 0;
                    let mut no_of_items = 0;
                    let mut idx = i;

                    while idx < n {
                        if list[idx] > MAX_NUMBER_POSSIBLE {
                            panic!();
                        }

                        if no_of_items == selector.group_size
                            || list[idx] > ((1u64 << selector.item_width) - 1) as u64
                        {
                            break;
                        }
                        data |= list[idx] << (SELECTOR_BITS + shift);
                        shift += selector.item_width as u64;
                        no_of_items += 1;
                        idx += 1;
                    }

                    if no_of_items == selector.group_size || idx == n {
                        encoded_result.extend_from_slice(&data.to_le_bytes());
                        i = idx;
                        break;
                    }
                }
            }
        }
    }

    encoded_result
}

fn decompress_u64(data: u64, result: &mut Vec<u64>) {
    let mut mut_data = data.clone();
    let selector_idx = data & SELECTOR_MASK;
    mut_data = mut_data >> SELECTOR_BITS;
    if selector_idx == 0 {
        for _ in 0..240 {
            result.push(1);
        }
        return;
    }
    if selector_idx == 1 {
        for _ in 0..120 {
            result.push(1);
        }
        return;
    }
    let selector = &SELECTORS[selector_idx as usize];
    let mask = ((1u64 << selector.item_width) - 1) as u64;
    for _ in 0..selector.group_size {
        result.push(mut_data & mask);
        mut_data = mut_data >> selector.item_width;
    }
}

pub fn decompress_from_bytes(bytes: &[u8]) -> Vec<u64> {
    let mut decoded_result: Vec<u64> = vec![];

    for chunk in bytes.chunks(8) {
        let mut arr = [0u8; 8];
        arr.copy_from_slice(chunk);
        decompress_u64(u64::from_le_bytes(arr), &mut decoded_result);
    }
    decoded_result
}
pub fn decompress(list: Vec<u64>) -> Vec<u64> {
    let mut decoded_result: Vec<u64> = vec![];
    for data in list {
        decompress_u64(data, &mut decoded_result);
    }

    decoded_result
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to convert encoded bytes to u64 for decoding
    fn bytes_to_u64_vec(bytes: &[u8]) -> Vec<u64> {
        let mut result = vec![];
        for chunk in bytes.chunks(8) {
            let mut arr = [0u8; 8];
            arr.copy_from_slice(chunk);
            result.push(u64::from_le_bytes(arr));
        }
        result
    }

    // Basic Roundtrip Tests
    #[test]
    fn test_encode_decode_roundtrip_small() {
        let original = vec![1, 2, 3, 4, 5];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_roundtrip_two_elements() {
        let original = vec![7, 13];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_roundtrip_empty() {
        let original: Vec<u64> = vec![];
        let encoded = decompress(original.clone());

        assert_eq!(encoded.len(), 0);
    }

    // Test Different Bit Widths (Selectors)
    #[test]
    fn test_encode_decode_1_bit_values() {
        // Values 0-1 fit in 1 bit (selector 2: 60 items)
        let original = vec![0, 1, 1, 0, 1, 0, 1, 1, 0, 1];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_2_bit_values() {
        // Values 0-3 fit in 2 bits (selector 3: 30 items)
        let original = vec![0, 1, 2, 3, 2, 1, 0, 3];
        let encoded = compress(&original);
        let decoded = decompress_from_bytes(&encoded);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_3_bit_values() {
        // Values 0-7 fit in 3 bits (selector 4: 20 items)
        let original = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let encoded = compress(&original);
        let decoded = decompress_from_bytes(&encoded);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_4_bit_values() {
        // Values 0-15 fit in 4 bits (selector 5: 15 items)
        let original = vec![0, 5, 10, 15, 8, 12, 3, 7];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_5_bit_values() {
        // Values 0-31 fit in 5 bits (selector 6: 12 items)
        let original = vec![0, 10, 20, 31, 15, 25];
        let encoded = compress(&original);
        let decoded = decompress_from_bytes(&encoded);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_6_bit_values() {
        // Values 0-63 fit in 6 bits (selector 7: 10 items)
        let original = vec![0, 15, 32, 63, 45, 20];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_7_bit_values() {
        // Values 0-127 fit in 7 bits (selector 8: 8 items)
        let original = vec![0, 50, 100, 127, 64, 32];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_8_bit_values() {
        // Values 0-255 fit in 8 bits (selector 9: 7 items)
        let original = vec![2, 100, 200, 255, 128, 64];
        let encoded = compress(&original);
        let decoded = decompress_from_bytes(&encoded);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_10_bit_values() {
        // Values 0-1023 fit in 10 bits (selector 10: 6 items)
        let original = vec![0, 500, 1000, 1023, 512];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_12_bit_values() {
        // Values 0-4095 fit in 12 bits (selector 11: 5 items)
        let original = vec![0, 2000, 4095, 1024];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_15_bit_values() {
        // Values 0-32767 fit in 15 bits (selector 12: 4 items)
        let original = vec![2, 10000, 32767, 16384];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_20_bit_values() {
        // Values 0-1048575 fit in 20 bits (selector 13: 3 items)
        let original = vec![145, 500000, 1048575];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_30_bit_values() {
        // Values 0-1073741823 fit in 30 bits (selector 14: 2 items)
        let original = vec![69, 1073741823];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_60_bit_values() {
        // Large values fit in 60 bits (selector 15: 1 item)
        let original = vec![MAX_NUMBER_POSSIBLE];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    // Run-Length Encoding Tests
    #[test]
    fn test_encode_decode_240_ones() {
        // Selector 0: run of 240 ones
        let original = vec![1; 240];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_120_ones() {
        // Selector 1: run of 120 ones
        let original = vec![1; 120];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    // Mixed Selector Tests
    #[test]
    fn test_encode_decode_mixed_selectors() {
        // Mix of small and large values requiring different selectors
        let original = vec![1, 2, 3, 500, 1000, 100000];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_alternating_small_large() {
        let original = vec![1, 1000, 2, 2000, 3, 3000];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_sequential_small() {
        let original: Vec<u64> = (0..20).collect();
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_sequential_large() {
        let original: Vec<u64> = (0..100).collect();
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_powers_of_two() {
        let original = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_repeated_pattern() {
        let original = vec![5, 5, 5, 5, 5, 5, 5, 5];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    // Panic Test
    #[test]
    #[should_panic]
    fn test_encode_value_too_large() {
        // Value exceeds MAX_NUMBER_POSSIBLE (2^60 - 1)
        let original = vec![1 << 60]; // 2^60
        compress(&original);
    }

    // Large Dataset Tests
    #[test]
    fn test_encode_decode_large_uniform() {
        let original = vec![42; 1000];
        let encoded = compress(&original);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_large_random_pattern() {
        let original = vec![
            1, 5, 3, 8, 2, 9, 4, 7, 6, 10, 15, 12, 20, 18, 25, 30, 35, 40, 45, 50,
        ];
        let mut extended = vec![];
        for _ in 0..50 {
            extended.extend_from_slice(&original);
        }

        let encoded = compress(&extended);
        let encoded_u64 = bytes_to_u64_vec(&encoded);
        let decoded = decompress(encoded_u64);

        assert_eq!(&decoded[..extended.len()], &extended[..]);
    }
}
