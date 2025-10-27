mod decompress;
use rand::Rng;
use std::u32::MAX;

use decompress::*;
const BATCH_SIZE: usize = 128;
static DECOMPRESSORS: &[DecompressorFn] = &[
    decompress_1_bit,
    decompress_2_bit,
    decompress_3_bit,
    decompress_4_bit,
    decompress_5_bit,
    decompress_6_bit,
    decompress_7_bit,
    decompress_8_bit,
    decompress_9_bit,
    decompress_10_bit,
    decompress_11_bit,
    decompress_12_bit,
    decompress_13_bit,
    decompress_14_bit,
    decompress_15_bit,
    decompress_16_bit,
    decompress_17_bit,
    decompress_18_bit,
    decompress_19_bit,
    decompress_20_bit,
    decompress_21_bit,
    decompress_22_bit,
    decompress_23_bit,
    decompress_24_bit,
    decompress_25_bit,
    decompress_26_bit,
    decompress_27_bit,
    decompress_28_bit,
    decompress_29_bit,
    decompress_30_bit,
    decompress_31_bit,
    decompress_32_bit,
];
static COMPRESSORS: &[DecompressorFn] = &[
    compress_1_bit,
    compress_2_bit,
    compress_3_bit,
    compress_4_bit,
    compress_5_bit,
    compress_6_bit,
    compress_7_bit,
    compress_8_bit,
    compress_9_bit,
    compress_10_bit,
    compress_11_bit,
    compress_12_bit,
    compress_13_bit,
    compress_14_bit,
    compress_15_bit,
    compress_16_bit,
    compress_17_bit,
    compress_18_bit,
    compress_19_bit,
    compress_20_bit,
    compress_21_bit,
    compress_22_bit,
    compress_23_bit,
    compress_24_bit,
    compress_25_bit,
    compress_26_bit,
    compress_27_bit,
    compress_28_bit,
    compress_29_bit,
    compress_30_bit,
    compress_31_bit,
    compress_32_bit,
];
#[derive(Debug, Clone, Copy)]
enum ExceptionSize {
    Bits8 = 0,
    Bits16 = 1,
    Bits32 = 2,
    BitsNotNeeded = 3,
}

impl ExceptionSize {
    fn from_max_value(max_val: u32) -> Self {
        if max_val <= u8::MAX as u32 {
            ExceptionSize::Bits8
        } else if max_val <= u16::MAX as u32 {
            ExceptionSize::Bits16
        } else {
            ExceptionSize::Bits32
        }
    }

    fn bits(&self) -> usize {
        match self {
            ExceptionSize::Bits8 => 8,
            ExceptionSize::Bits16 => 16,
            ExceptionSize::Bits32 => 32,
            ExceptionSize::BitsNotNeeded => 0,
        }
    }
}

pub struct PForDelta {}

impl PForDelta {
    pub fn encode_batch(values: &[u32]) -> Vec<u8> {
        assert!(
            values.len() == BATCH_SIZE,
            "Batch must contain exactly 128 values"
        );

        // Find optimal b such that at least 90% of values fit in b bits
        let b = find_optimal_b(values);

        let threshold;
        if b == 32 {
            threshold = u32::MAX;
        } else {
            threshold = 1u32 << b;
        }
        // Identify exceptions
        let mut exceptions = Vec::new();
        for (i, &val) in values.iter().enumerate() {
            if val >= threshold {
                exceptions.push((i, val));
            }
        }

        // Force additional exceptions if gaps are too large
        let exceptions: Vec<(usize, u32)> = force_intermediate_exceptions(&exceptions, b, values);
        let mut exc_size = ExceptionSize::BitsNotNeeded;
        if exceptions.len() != 0 {
            let max_val = exceptions.iter().map(|(_, value)| *value).max().unwrap();
            exc_size = ExceptionSize::from_max_value(max_val);
        }

        // Build the compressed representation
        let mut compressed: Vec<u8> = Vec::new();

        // Write header: b (5 bits), exception_size (2 bits), first_exception_idx (7 bits)
        let first_exc_idx = if exceptions.is_empty() {
            127 // Invalid marker
        } else {
            exceptions[0].0 as u8
        };

        compressed.push(b as u8);
        compressed.push(exc_size.bits() as u8);
        compressed.push(first_exc_idx as u8);

        // Create b-bit slots
        let mut slots = vec![0u32; BATCH_SIZE];
        let exc_set: std::collections::HashSet<usize> =
            exceptions.iter().map(|(i, _)| *i).collect();

        // Fill slots with values or offsets
        for i in 0..BATCH_SIZE {
            if exc_set.contains(&i) {
                // Find offset to next exception
                let curr_pos = exceptions.iter().position(|(idx, _)| *idx == i).unwrap();
                if curr_pos + 1 < exceptions.len() {
                    let next_idx = exceptions[curr_pos + 1].0;
                    slots[i] = (next_idx - i - 1) as u32;
                } else {
                    slots[i] = 0; // Last exception
                }
            } else {
                slots[i] = values[i];
            }
        }

        // Write b-bit slots
        write_packed_bits(&mut compressed, &slots, b);

        // Write exception values
        for (_, val) in &exceptions {
            match exc_size {
                ExceptionSize::Bits8 => compressed.push(*val as u8),
                ExceptionSize::Bits16 => compressed.extend_from_slice(&(*val as u16).to_le_bytes()),
                ExceptionSize::Bits32 => compressed.extend_from_slice(&val.to_le_bytes()),
                ExceptionSize::BitsNotNeeded => break,
            }
        }

        compressed
    }

    pub fn decode_batch(compressed: &[u8]) -> Vec<u32> {
        let mut pos = 0;

        let b = compressed[pos];
        pos = pos + 1;
        let exc_size_code = compressed[pos];
        pos += 1;
        let first_exc_idx = compressed[pos];
        pos += 1;

        let exc_size = match exc_size_code {
            0 => ExceptionSize::BitsNotNeeded,
            8 => ExceptionSize::Bits8,
            16 => ExceptionSize::Bits16,
            32 => ExceptionSize::Bits32,
            _ => {
                panic!();
            }
        };
        let pos_end = (pos + 16usize * b as usize) as usize;
        // Read b-bit slots
        let mut result = read_packed_bits(
            &u8_chunks_to_u32_vec(&compressed[pos..pos_end]),
            BATCH_SIZE,
            b.into(),
        );
        // Read exception values
        let mut exception_values: Vec<u32> = Vec::new();

        match exc_size {
            ExceptionSize::Bits8 => {
                exception_values = u8_chunks_to_u8_vec(&compressed[pos_end..]);
            }
            ExceptionSize::Bits16 => {
                exception_values = u8_chunks_to_u16_vec(&compressed[pos_end..])
            }
            ExceptionSize::Bits32 => {
                exception_values = u8_chunks_to_u32_vec(&compressed[pos_end..])
            }
            ExceptionSize::BitsNotNeeded => {}
        }

        let mut curr_exc_idx = first_exc_idx as usize;

        // Follow linked list to find exception positions
        for i in 0..exception_values.len() {
            let offset_to_next_exception = result[curr_exc_idx];
            result[curr_exc_idx] = exception_values[i];
            curr_exc_idx = curr_exc_idx + (1 + offset_to_next_exception) as usize;
        }

        result
    }
}

fn find_optimal_b(values: &[u32]) -> usize {
    // Binary search on bit width: find the smallest b where at least 90% of values fit
    let target_count = (BATCH_SIZE as f64 * 0.9).ceil() as usize;

    let mut left = 1_usize;
    let mut right = 32_usize;
    let mut result = 32;

    while left <= right {
        let mid = (left + right) / 2;

        // Count how many values can be represented with 'mid' bits
        let max_value = if mid == 32 {
            u32::MAX
        } else {
            (1u32 << mid) - 1
        };

        let count = values.iter().filter(|&&v| v <= max_value).count();

        if count >= target_count {
            // This bit width works, try smaller
            result = mid;
            right = mid.saturating_sub(1);
        } else {
            // Need more bits
            left = mid + 1;
        }
    }
    result
}

fn force_intermediate_exceptions(
    exceptions: &[(usize, u32)],
    b: usize,
    values: &[u32],
) -> Vec<(usize, u32)> {
    if exceptions.is_empty() {
        return Vec::new();
    }

    let max_offset = (1usize << b) - 1;
    let mut result = Vec::new();

    for i in 0..exceptions.len() {
        result.push(exceptions[i]);

        if i + 1 < exceptions.len() {
            let curr_idx = exceptions[i].0;
            let next_idx = exceptions[i + 1].0;
            let gap = next_idx - curr_idx - 1;

            if gap > max_offset {
                // Force intermediate exceptions
                let mut pos = curr_idx + max_offset + 1;
                while pos < next_idx {
                    result.push((pos, values[pos]));
                    pos += max_offset + 1;
                }
            }
        }
    }

    result
}

fn write_packed_bits(output: &mut Vec<u8>, values: &[u32], bits_per_value: usize) {
    let mut compressed: Vec<u32> = vec![0; 4 * bits_per_value]; // Creates actual elements
    let mut i = 0;
    let mut j = 0;
    while i < 128 {
        COMPRESSORS[bits_per_value - 1](&values[i..i + 32], &mut compressed[j..j + bits_per_value]);
        i += 32;
        j += bits_per_value;
    }
    for j in compressed {
        output.extend_from_slice(&j.to_le_bytes());
    }
}

fn read_packed_bits(input: &[u32], count: usize, bits_per_value: usize) -> Vec<u32> {
    let mut result: Vec<u32> = vec![0; count];
    let mut j = 0;
    let mut i = 0;
    while i < 128 {
        DECOMPRESSORS[bits_per_value - 1](&input[j..j + bits_per_value], &mut result[i..i + 32]);
        i += 32;
        j += bits_per_value;
    }

    result
}

fn u8_chunks_to_u32_vec(bytes: &[u8]) -> Vec<u32> {
    let u32_vec = bytes
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
        .collect();
    u32_vec
}

fn u8_chunks_to_u16_vec(bytes: &[u8]) -> Vec<u32> {
    let u32_vec = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()) as u32)
        .collect();
    u32_vec
}
fn u8_chunks_to_u8_vec(bytes: &[u8]) -> Vec<u32> {
    let u32_vec = bytes
        .chunks_exact(1)
        .map(|chunk| u8::from_le_bytes(chunk.try_into().unwrap()) as u32)
        .collect();
    u32_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
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
                        1 % (1u32 << 1)
                    } else {
                        (val) % (1u32 << 1)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_2_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
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
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_3_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
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
                        1 % (1u32 << 3)
                    } else {
                        (val) % (1u32 << 3)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_4_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
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
                        1 % (1u32 << 4)
                    } else {
                        (val) % (1u32 << 4)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_5_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 14)
                    } else {
                        (1 + val) % (1u32 << 14)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 5)
                    } else {
                        (val) % (1u32 << 5)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_6_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 21)
                    } else {
                        (1 + val) % (1u32 << 21)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 6)
                    } else {
                        (val) % (1u32 << 6)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_7_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 21)
                    } else {
                        (1 + val) % (1u32 << 21)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 7)
                    } else {
                        (val) % (1u32 << 7)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_8_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 21)
                    } else {
                        (1 + val) % (1u32 << 21)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 8)
                    } else {
                        (val) % (1u32 << 8)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_9_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 21)
                    } else {
                        (1 + val) % (1u32 << 21)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 9)
                    } else {
                        (val) % (1u32 << 9)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_10_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 21)
                    } else {
                        (1 + val) % (1u32 << 21)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 10)
                    } else {
                        (val) % (1u32 << 10)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_11_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 21)
                    } else {
                        (1 + val) % (1u32 << 21)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 11)
                    } else {
                        (val) % (1u32 << 11)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_12_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 21)
                    } else {
                        (1 + val) % (1u32 << 21)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 12)
                    } else {
                        (val) % (1u32 << 12)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_13_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 21)
                    } else {
                        (1 + val) % (1u32 << 21)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 13)
                    } else {
                        (val) % (1u32 << 13)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_14_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 21)
                    } else {
                        (1 + val) % (1u32 << 21)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 14)
                    } else {
                        (val) % (1u32 << 14)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_15_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 21)
                    } else {
                        (1 + val) % (1u32 << 21)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 15)
                    } else {
                        (val) % (1u32 << 15)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_16_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 21)
                    } else {
                        (1 + val) % (1u32 << 21)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 16)
                    } else {
                        (val) % (1u32 << 16)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_17_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 21)
                    } else {
                        (1 + val) % (1u32 << 21)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 17)
                    } else {
                        (val) % (1u32 << 17)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_18_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 21)
                    } else {
                        (1 + val) % (1u32 << 21)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 18)
                    } else {
                        (val) % (1u32 << 18)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_19_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 21)
                    } else {
                        (1 + val) % (1u32 << 21)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 19)
                    } else {
                        (val) % (1u32 << 19)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_20_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 21)
                    } else {
                        (1 + val) % (1u32 << 21)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 20)
                    } else {
                        (val) % (1u32 << 20)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_21_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 31)
                    } else {
                        (1 + val) % (1u32 << 31)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 21)
                    } else {
                        (val) % (1u32 << 21)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_22_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 31)
                    } else {
                        (1 + val) % (1u32 << 31)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 22)
                    } else {
                        (val) % (1u32 << 22)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_23_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 31)
                    } else {
                        (1 + val) % (1u32 << 31)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 23)
                    } else {
                        (val) % (1u32 << 23)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_24_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 31)
                    } else {
                        (1 + val) % (1u32 << 31)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 24)
                    } else {
                        (val) % (1u32 << 24)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_25_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 31)
                    } else {
                        (1 + val) % (1u32 << 31)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 25)
                    } else {
                        (val) % (1u32 << 25)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_26_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 31)
                    } else {
                        (1 + val) % (1u32 << 31)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 26)
                    } else {
                        (val) % (1u32 << 26)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_27_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 31)
                    } else {
                        (1 + val) % (1u32 << 31)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 27)
                    } else {
                        (val) % (1u32 << 27)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_28_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 31)
                    } else {
                        (1 + val) % (1u32 << 31)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 28)
                    } else {
                        (val) % (1u32 << 28)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_29_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % (1u32 << 31)
                    } else {
                        (1 + val) % (1u32 << 31)
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 29)
                    } else {
                        (val) % (1u32 << 29)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_30_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % u32::MAX
                    } else {
                        (1 + val) % u32::MAX
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 30)
                    } else {
                        (val) % (1u32 << 30)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_31_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % u32::MAX
                    } else {
                        (1 + val) % u32::MAX
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % (1u32 << 31)
                    } else {
                        (val) % (1u32 << 31)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }

    #[test]
    fn test_32_bit() {
        let mut rng = rand::thread_rng();
        let original: Vec<u32> = (0..128)
            .map(|i| {
                let val: u32 = rng.r#gen();
                if i % 16 == 0 {
                    if val == 0 {
                        1 % u32::MAX
                    } else {
                        (1 + val) % u32::MAX
                    } // Ensure it's at least 1
                } else {
                    if val == 0 {
                        1 % u32::MAX
                    } else {
                        (val) % (u32::MAX)
                    } // Ensure it's at least 1
                }
            })
            .collect();
        let encoded = PForDelta::encode_batch(&original);

        let decoded = PForDelta::decode_batch(&encoded);

        // Decoded may have padding zeros, so check prefix matches
        assert!(decoded.len() == original.len());
        assert_eq!(&decoded[..original.len()], &original[..]);
    }
}
