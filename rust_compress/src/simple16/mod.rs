#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Simple16Selector {
    no_of_items: u32,
    no_of_used_bits: u32,
    no_of_wasted_bits: u32,
    layout: Option<&'static [u8]>,
}

const SELECTOR_MASK: u32 = 0x000F;
const MAX_NUMBER_POSSIBLE: u32 = (1 << 28) - 1;
const SELECTOR_BITS: u32 = 4;

const SELECTORS: [Simple16Selector; 16] = [
    Simple16Selector {
        no_of_items: 28,
        no_of_used_bits: 1,
        no_of_wasted_bits: 0,
        layout: None,
    },
    Simple16Selector {
        no_of_items: 14,
        no_of_used_bits: 2,
        no_of_wasted_bits: 0,
        layout: None,
    },
    Simple16Selector {
        no_of_items: 9,
        no_of_used_bits: 3,
        no_of_wasted_bits: 1,
        layout: None,
    },
    Simple16Selector {
        no_of_items: 7,
        no_of_used_bits: 4,
        no_of_wasted_bits: 0,
        layout: None,
    },
    Simple16Selector {
        no_of_items: 6,
        no_of_used_bits: 28,
        no_of_wasted_bits: 0,
        layout: Some(&[3, 5, 5, 5, 5, 5]),
    },
    Simple16Selector {
        no_of_items: 5,
        no_of_used_bits: 28,
        no_of_wasted_bits: 0,
        layout: Some(&[5, 5, 6, 6, 6]),
    },
    Simple16Selector {
        no_of_items: 5,
        no_of_used_bits: 28,
        no_of_wasted_bits: 0,
        layout: Some(&[6, 6, 6, 5, 5]),
    },
    Simple16Selector {
        no_of_items: 5,
        no_of_used_bits: 28,
        no_of_wasted_bits: 0,
        layout: Some(&[4, 6, 6, 6, 6]),
    },
    Simple16Selector {
        no_of_items: 5,
        no_of_used_bits: 28,
        no_of_wasted_bits: 0,
        layout: Some(&[6, 6, 6, 6, 4]),
    },
    Simple16Selector {
        no_of_items: 4,
        no_of_used_bits: 7,
        no_of_wasted_bits: 0,
        layout: None,
    },
    Simple16Selector {
        no_of_items: 4,
        no_of_used_bits: 28,
        no_of_wasted_bits: 0,
        layout: Some(&[10, 6, 6, 6]),
    },
    Simple16Selector {
        no_of_items: 3,
        no_of_used_bits: 9,
        no_of_wasted_bits: 1,
        layout: None,
    },
    Simple16Selector {
        no_of_items: 3,
        no_of_used_bits: 28,
        no_of_wasted_bits: 0,
        layout: Some(&[8, 10, 10]),
    },
    Simple16Selector {
        no_of_items: 3,
        no_of_used_bits: 28,
        no_of_wasted_bits: 0,
        layout: Some(&[10, 10, 8]),
    },
    Simple16Selector {
        no_of_items: 2,
        no_of_used_bits: 14,
        no_of_wasted_bits: 0,
        layout: None,
    },
    Simple16Selector {
        no_of_items: 1,
        no_of_used_bits: 28,
        no_of_wasted_bits: 0,
        layout: None,
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

            match selector.layout {
                Some(layout) => {
                    while idx < n {
                        if list[idx] > MAX_NUMBER_POSSIBLE {
                            panic!();
                        }
                        if no_of_items == selector.no_of_items {
                            break;
                        }
                        if (list[idx] as u32) > (((1 << (layout[no_of_items as usize])) - 1) as u32)
                        {
                            break;
                        }
                        data |= list[idx] << (SELECTOR_BITS + shift);
                        shift += layout[no_of_items as usize] as u32;
                        no_of_items += 1;
                        idx += 1;
                    }

                    if no_of_items == selector.no_of_items || idx == n {
                        encoded_result.extend_from_slice(&data.to_le_bytes());
                        i = idx;
                        break;
                    }
                }
                None => {
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
                        no_of_items = no_of_items + 1;
                        idx = idx + 1;
                    }

                    if no_of_items == selector.no_of_items || idx == n {
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

fn decompress_u32(data: u32, decoded_result: &mut Vec<u32>) {
    let mut mut_data = data.clone();
    let selector_idx = data & SELECTOR_MASK;
    mut_data = mut_data >> SELECTOR_BITS;
    let selector = &SELECTORS[selector_idx as usize];
    match selector.layout {
        Some(layout) => {
            for shift in layout {
                let mask = ((1u32 << shift) - 1) as u32;
                decoded_result.push(mut_data & mask);
                mut_data = mut_data >> shift;
            }
        }
        None => {
            let mask = ((1u32 << selector.no_of_used_bits) - 1) as u32;
            for _ in 0..selector.no_of_items {
                decoded_result.push(mut_data & mask);
                mut_data = mut_data >> selector.no_of_used_bits;
            }
        }
    }
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

        // Decoded may have padding zeros, so check prefix matches
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

        // Decoded may have padding zeros, so check prefix matches
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

        // Decoded may have padding zeros, so check prefix matches
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

        // Decoded may have padding zeros, so check prefix matches
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

        // Decoded may have padding zeros, so check prefix matches
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

        // Decoded may have padding zeros, so check prefix matches
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

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
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

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() >= original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_selector_4_layout_3_5_5_5_5_5() {
        // Selector 4: layout [3, 5, 5, 5, 5, 5] - 6 items
        // First item: up to 7 (2^3-1), rest: up to 31 (2^5-1)
        let original = vec![7, 31, 30, 29, 28, 27];
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
    fn test_selector_5_layout_5_5_6_6_6() {
        // Selector 5: layout [5, 5, 6, 6, 6] - 5 items
        // First two: up to 31 (2^5-1), last three: up to 63 (2^6-1)
        let original = vec![31, 30, 63, 62, 61];
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
    fn test_selector_6_layout_6_6_6_5_5() {
        // Selector 6: layout [6, 6, 6, 5, 5] - 5 items
        // First three: up to 63 (2^6-1), last two: up to 31 (2^5-1)
        let original = vec![63, 62, 61, 31, 30];
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
    fn test_selector_7_layout_4_6_6_6_6() {
        // Selector 7: layout [4, 6, 6, 6, 6] - 5 items
        // First: up to 15 (2^4-1), rest: up to 63 (2^6-1)
        let original = vec![15, 63, 62, 61, 60];
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
    fn test_selector_8_layout_6_6_6_6_4() {
        // Selector 8: layout [6, 6, 6, 6, 4] - 5 items
        // First four: up to 63 (2^6-1), last: up to 15 (2^4-1)
        let original = vec![63, 62, 61, 60, 15];
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
    fn test_selector_10_layout_10_6_6_6() {
        // Selector 10: layout [10, 6, 6, 6] - 4 items
        // First: up to 1023 (2^10-1), rest: up to 63 (2^6-1)
        let original = vec![1023, 63, 62, 61];
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
    fn test_selector_12_layout_8_10_10() {
        // Selector 12: layout [8, 10, 10] - 3 items
        // First: up to 255 (2^8-1), rest: up to 1023 (2^10-1)
        let original = vec![255, 1023, 1022];
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
    fn test_selector_13_layout_10_10_8() {
        // Selector 13: layout [10, 10, 8] - 3 items
        // First two: up to 1023 (2^10-1), last: up to 255 (2^8-1)
        let original = vec![1023, 1022, 255];
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
    fn test_mixed_selectors_with_layouts() {
        // Create a sequence that will use multiple different layout selectors
        let original = vec![
            7, 31, 31, 31, 31, 31, // Should use selector 4: [3,5,5,5,5,5]
            1023, 63, 63, 63, // Should use selector 10: [10,6,6,6]
            255, 1023, 1023, // Should use selector 12: [8,10,10]
        ];
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
    fn test_boundary_values_with_layouts() {
        // Test boundary values for various layouts
        // Using values at the exact maximum for each bit width
        let original = vec![
            7,    // 3 bits max
            15,   // 4 bits max
            31,   // 5 bits max
            63,   // 6 bits max
            255,  // 8 bits max
            1023, // 10 bits max
        ];
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
}
