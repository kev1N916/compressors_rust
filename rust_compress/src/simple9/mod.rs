mod utils;
use utils::*;
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Simple9Selector {
    no_of_items: u32,
    no_of_used_bits: u32,
    no_of_wasted_bits: u32,
}

const SELECTOR_MASK: u32 = 0x000F;
const MAX_NUMBER_POSSIBLE: u32 = (1 << 28) - 1;
const SELECTOR_BITS: u32 = 4;
const SELECTORS: [Simple9Selector; 9] = [
    Simple9Selector {
        no_of_items: 28,
        no_of_used_bits: 1,
        no_of_wasted_bits: 0,
    },
    Simple9Selector {
        no_of_items: 14,
        no_of_used_bits: 2,
        no_of_wasted_bits: 0,
    },
    Simple9Selector {
        no_of_items: 9,
        no_of_used_bits: 3,
        no_of_wasted_bits: 1,
    },
    Simple9Selector {
        no_of_items: 7,
        no_of_used_bits: 4,
        no_of_wasted_bits: 0,
    },
    Simple9Selector {
        no_of_items: 5,
        no_of_used_bits: 5,
        no_of_wasted_bits: 3,
    },
    Simple9Selector {
        no_of_items: 4,
        no_of_used_bits: 7,
        no_of_wasted_bits: 0,
    },
    Simple9Selector {
        no_of_items: 3,
        no_of_used_bits: 9,
        no_of_wasted_bits: 1,
    },
    Simple9Selector {
        no_of_items: 2,
        no_of_used_bits: 14,
        no_of_wasted_bits: 0,
    },
    Simple9Selector {
        no_of_items: 1,
        no_of_used_bits: 28,
        no_of_wasted_bits: 0,
    },
];

pub fn compress(list: &Vec<u32>) -> Vec<u8> {
    let mut encoded_result: Vec<u8> = vec![];

    let n = list.len();
    let mut i = 0;
    while i < n {
        for (selector_idx, selector) in SELECTORS.iter().enumerate() {
            let mut data = selector_idx as u32; // last 4 bits are selector bits
            let mut shift = 0;
            let mut no_of_items = 0;
            let mut idx = i;

            while idx < n {
                if list[idx] > MAX_NUMBER_POSSIBLE {
                    panic!();
                }
                if no_of_items == selector.no_of_items {
                    break;
                }
                if (list[idx] as u32) > (((1u32 << selector.no_of_used_bits) - 1) as u32) {
                    break;
                }
                data |= list[idx] << (SELECTOR_BITS + shift);
                shift += selector.no_of_used_bits;
                no_of_items += 1;
                idx += 1;
            }

            if no_of_items == selector.no_of_items || idx == n {
                encoded_result.extend_from_slice(&data.to_le_bytes());
                i = idx;
                break;
            }
        }
    }

    encoded_result
}

fn decompress_u32(data: u32, decoded_result: &mut Vec<u32>) {
    let mut mut_data = data.clone();
    let selector_idx = data & SELECTOR_MASK;
    mut_data = mut_data >> SELECTOR_BITS;
    match selector_idx {
        0 => {
            decode_simple9_1bit(mut_data, decoded_result);
        }
        1 => {
            decode_simple9_2bit(mut_data, decoded_result);
        }
        2 => {
            decode_simple9_3bit(mut_data, decoded_result);
        }
        3 => {
            decode_simple9_4bit(mut_data, decoded_result);
        }
        4 => {
            decode_simple9_5bit(mut_data, decoded_result);
        }
        5 => {
            decode_simple9_7bit(mut_data, decoded_result);
        }
        6 => {
            decode_simple9_9bit(mut_data, decoded_result);
        }
        7 => {
            decode_simple9_14bit(mut_data, decoded_result);
        }
        8 => {
            decode_simple9_28bit(mut_data, decoded_result);
        }
        _ => {
            panic!();
        }
    }
    // let mask = ((1u32 << selector.no_of_used_bits) - 1) as u32;
    // for _ in 0..selector.no_of_items {
    //     decoded_result.push(mut_data & mask);
    //     decoded_result.ex
    //     mut_data = mut_data >> selector.no_of_used_bits;
    // }
}

pub fn decompress_from_bytes(bytes: &[u8]) -> Vec<u32> {
    let mut decoded_result: Vec<u32> = vec![];

    for chunk in bytes.chunks(4) {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(chunk);
        decompress_u32(u32::from_le_bytes(bytes), &mut decoded_result);
    }

    decoded_result
}
pub fn decompress(list: Vec<u32>) -> Vec<u32> {
    let mut decoded_result: Vec<u32> = vec![];
    for data in list {
        decompress_u32(data, &mut decoded_result);
    }
    decoded_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_single_element() {
        let list = vec![1];
        let result = compress(&list);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_encode_small_values() {
        let list = vec![1, 2, 3, 4, 5];
        let result = compress(&list);
        assert_eq!(result.len(), 4); // Should fit in one u32
    }

    #[test]
    fn test_encode_28_ones() {
        // Best case: 28 values that fit in 1 bit each (selector 0)
        let list = vec![1; 28];
        let result = compress(&list);
        assert_eq!(result.len(), 4); // One u32
    }

    #[test]
    fn test_encode_14_small_values() {
        // Values that fit in 2 bits (0-3) - selector 1
        let list = vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1];
        let result = compress(&list);
        assert_eq!(result.len(), 4); // One u32
    }

    #[test]
    fn test_encode_large_value() {
        // Value that requires 28 bits (selector 8)
        let list = vec![MAX_NUMBER_POSSIBLE];
        let result = compress(&list);
        assert_eq!(result.len(), 4);
    }

    #[test]
    #[should_panic]
    fn test_encode_value_too_large() {
        // Value exceeds MAX_NUMBER_POSSIBLE
        let list = vec![MAX_NUMBER_POSSIBLE + 1];
        compress(&list);
    }

    #[test]
    fn test_encode_decode_roundtrip_small() {
        let original = vec![1, 2, 3, 4, 5];
        let encoded = compress(&original);

        // Convert encoded bytes back to u32 for decoding
        let mut encoded_u32 = vec![];
        for chunk in encoded.chunks(4) {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(chunk);
            encoded_u32.push(u32::from_le_bytes(bytes));
        }

        let decoded = decompress(encoded_u32);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_roundtrip_mixed_sizes() {
        let original = vec![1, 10, 100, 1000, 10000];
        let encoded = compress(&original);

        // Convert encoded bytes back to u32 for decoding
        let mut encoded_u32 = vec![];
        for chunk in encoded.chunks(4) {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(chunk);
            encoded_u32.push(u32::from_le_bytes(bytes));
        }

        let decoded = decompress(encoded_u32);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_roundtrip_sequential() {
        let original = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let encoded = compress(&original);

        // Convert encoded bytes back to u32 for decoding
        let mut encoded_u32 = vec![];
        for chunk in encoded.chunks(4) {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(chunk);
            encoded_u32.push(u32::from_le_bytes(bytes));
        }

        let decoded = decompress(encoded_u32);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_roundtrip_repeated_values() {
        let original = vec![7, 7, 7, 7, 7, 7];
        let encoded = compress(&original);

        // Convert encoded bytes back to u32 for decoding
        let mut encoded_u32 = vec![];
        for chunk in encoded.chunks(4) {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(chunk);
            encoded_u32.push(u32::from_le_bytes(bytes));
        }

        let decoded = decompress(encoded_u32);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_roundtrip_max_single_bit() {
        let original = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let encoded = compress(&original);

        // Convert encoded bytes back to u32 for decoding
        let mut encoded_u32 = vec![];
        for chunk in encoded.chunks(4) {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(chunk);
            encoded_u32.push(u32::from_le_bytes(bytes));
        }

        let decoded = decompress(encoded_u32);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_roundtrip_large_dataset() {
        let original: Vec<u32> = (1..=100).collect();
        let encoded = compress(&original);

        // Convert encoded bytes back to u32 for decoding
        let mut encoded_u32 = vec![];
        for chunk in encoded.chunks(4) {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(chunk);
            encoded_u32.push(u32::from_le_bytes(bytes));
        }

        let decoded = decompress(encoded_u32);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_roundtrip_powers_of_two() {
        let original = vec![1, 2, 4, 8, 16, 32, 64, 128];
        let encoded = compress(&original);

        // Convert encoded bytes back to u32 for decoding
        let mut encoded_u32 = vec![];
        for chunk in encoded.chunks(4) {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(chunk);
            encoded_u32.push(u32::from_le_bytes(bytes));
        }

        let decoded = decompress(encoded_u32);

        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_encode_decode_roundtrip_ones() {
        let original = vec![1; 28];
        let encoded = compress(&original);

        // Convert encoded bytes back to u32 for decoding
        let mut encoded_u32 = vec![];
        for chunk in encoded.chunks(4) {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(chunk);
            encoded_u32.push(u32::from_le_bytes(bytes));
        }

        let decoded = decompress(encoded_u32);
        assert_eq!(&decoded[..28], &original[..]);
    }

    #[test]
    fn test_encode_decode_roundtrip_large_values() {
        let original = vec![1000, 2000, 3000, 4000];
        let encoded = compress(&original);

        // Convert encoded bytes back to u32 for decoding
        let mut encoded_u32 = vec![];
        for chunk in encoded.chunks(4) {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(chunk);
            encoded_u32.push(u32::from_le_bytes(bytes));
        }

        let decoded = decompress(encoded_u32);
        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }
}
