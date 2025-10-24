mod decompress;
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
    // decompress_17_bit,
    // decompress_18_bit,
    // decompress_19_bit,
    // decompress_20_bit,
];
#[derive(Debug, Clone, Copy)]
enum ExceptionSize {
    Bits8 = 0,
    Bits16 = 1,
    Bits32 = 2,
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
        }
    }
}

pub fn compress_batch(values: &[u32]) -> Vec<u8> {
    assert!(
        values.len() == BATCH_SIZE,
        "Batch must contain exactly 128 values"
    );

    // Find optimal b such that at least 90% of values fit in b bits
    let b = find_optimal_b(values);
    let threshold = 1u32 << b;

    // Identify exceptions
    let mut exceptions = Vec::new();
    for (i, &val) in values.iter().enumerate() {
        if val >= threshold {
            exceptions.push((i, val));
        }
    }

    // Force additional exceptions if gaps are too large
    let exceptions = force_intermediate_exceptions(&exceptions, b, values);

    // Determine exception size based on max value in batch
    let max_val = values[values.len() - 1];
    let exc_size = ExceptionSize::from_max_value(max_val);

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
    let exc_set: std::collections::HashSet<usize> = exceptions.iter().map(|(i, _)| *i).collect();

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
        }
    }

    compressed
}

fn find_optimal_b(values: &[u32]) -> usize {
    // Since input is sorted, check the value at the 90% position (0.9 * 128 = 115.2)
    // We use index 115 (0-indexed), which means 116 values will fit
    let idx_90 = ((BATCH_SIZE as f64 * 0.9) as usize).saturating_sub(1);
    let val_90 = values[idx_90];

    // Find number of bits required to encode val_90
    if val_90 == 0 {
        return 1;
    }

    // Number of bits needed is floor(log2(val_90)) + 1
    let b = 32 - val_90.leading_zeros() as usize;
    b
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
    let mut bits_in_buffer = 0;
    let mut is_slot_underused = false;
    let mut remainder: u32 = 0;
    for &val in values {
        if bits_per_value >= 8 {
            let mut mut_value = val.clone();
            let mask = 2 << 8 - 1;
            let mut number_of_bytes = bits_per_value / 8;
            if remainder != 0 {
                let bits_to_shift = 32 - remainder.leading_zeros() as usize;
                remainder = remainder << (8 - bits_to_shift);
                remainder |= mut_value & (2 << (8 - bits_to_shift) - 1);
                output.push(remainder as u8);
                mut_value = mut_value >> (8 - bits_to_shift);
                number_of_bytes = (32 - mut_value.leading_zeros() as usize) / 8;
            }
            match number_of_bytes {
                1 => {
                    let first_byte = mut_value & mask;
                    output.push(first_byte as u8);
                    mut_value = mut_value >> 8;
                }
                2 => {
                    let first_byte = mut_value & mask;
                    output.push(first_byte as u8);
                    mut_value = mut_value >> 8;
                    let second_byte = (mut_value) & mask;
                    output.push(second_byte as u8);
                    mut_value = mut_value >> 8;
                }
                3 => {
                    let first_byte = mut_value & mask;
                    output.push(first_byte as u8);
                    mut_value = mut_value >> 8;

                    let second_byte = (mut_value) & mask;
                    output.push(second_byte as u8);
                    mut_value = mut_value >> 8;

                    let third_byte = mut_value & mask;
                    output.push(third_byte as u8);
                    mut_value = mut_value >> 8;
                }
                4 => {
                    let first_byte = mut_value & mask;
                    output.push(first_byte as u8);
                    mut_value = mut_value >> 8;

                    let second_byte = (mut_value) & mask;
                    output.push(second_byte as u8);
                    mut_value = mut_value >> 8;

                    let third_byte = mut_value & mask;
                    output.push(third_byte as u8);
                    mut_value = mut_value >> 8;

                    let fourth_byte = (mut_value) & mask;
                    output.push(fourth_byte as u8);
                    mut_value = mut_value >> 8;
                }
                _ => {
                    panic!("wtf")
                }
            }
            remainder = mut_value & mask;
        } else {
            if is_slot_underused {
                let mask = (2 << (8 - bits_per_value) - 1) as u32;
                bits_in_buffer = (bits_in_buffer << (8 - bits_per_value)) | (val & mask);
                output.push(bits_in_buffer as u8);
                bits_in_buffer = (val >> (8 - bits_per_value)) & (2 << bits_per_value - 1)
            }
            let mask = 2 << bits_per_value - 1;
            bits_in_buffer |= val & mask;
            is_slot_underused = true;
        }
    }
}

fn u8_chunks_to_u32_vec(bytes: &[u8],chunk_size:usize) -> Vec<u32> {
    bytes
        .chunks_exact(chunk_size)
        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
        .collect()
}

pub fn decompress_batch(compressed: &[u8]) -> Vec<u32> {
    let mut pos = 0;

    let b = compressed[pos];
    pos = pos + 1;
    let exc_size_code = compressed[pos];
    pos += 1;
    let first_exc_idx = compressed[pos];
    pos += 1;

    let exc_size = match exc_size_code {
        0 => ExceptionSize::Bits8,
        1 => ExceptionSize::Bits16,
        _ => ExceptionSize::Bits32,
    };
    let pos_end = (BATCH_SIZE * b as usize + 7) / 8;

    // Read b-bit slots
    let mut result = read_packed_bits(
        &u8_chunks_to_u32_vec(&compressed[pos..pos_end],4),
        BATCH_SIZE,
        b.into(),
    );

    // Read exception values
    let mut exception_values: Vec<u32> = Vec::new();
    match exc_size {
        ExceptionSize::Bits8 => exception_values = u8_chunks_to_u32_vec(&compressed[pos_end..],1),
        ExceptionSize::Bits16 => exception_values = u8_chunks_to_u32_vec(&compressed[pos_end..],2),
        ExceptionSize::Bits32 => exception_values = u8_chunks_to_u32_vec(&compressed[pos_end..],4),
    }

    let mut curr_exc_idx=first_exc_idx as usize;
    // Follow linked list to find exception positions
    for i in 0..exception_values.len() {
        let offset_to_next_exception=result[curr_exc_idx];
        result[curr_exc_idx]=exception_values[i];
        curr_exc_idx=curr_exc_idx+(1+offset_to_next_exception) as usize;
    }

    result
}

fn read_packed_bits(input: &[u32], count: usize, bits_per_value: usize) -> Vec<u32> {
    let mut result = Vec::with_capacity(count);
    for mut i in 0..count {
        DECOMPRESSORS[bits_per_value - 1](&input[i..i + bits_per_value], &mut result[i..i + 32]);
        i += 32;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress() {
        let mut values = vec![0u32; BATCH_SIZE];
        for i in 0..BATCH_SIZE {
            values[i] = (i * 100) as u32;
        }
        values[10] = 1_000_000;
        values[50] = 2_000_000;
        values[120] = 3_000_000;

        let compressed = compress_batch(&values);
        let decompressed = decompress_batch(&compressed);

        assert_eq!(values, decompressed);
    }
}
