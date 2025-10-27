/// A simple bit writer to pack bits into bytes
struct BitWriter {
    bytes: Vec<u8>,
    current_byte: u8,
    bit_pos: u8,
}

impl BitWriter {
    fn new() -> Self {
        BitWriter {
            bytes: Vec::new(),
            current_byte: 0,
            bit_pos: 0,
        }
    }

    fn write_bit(&mut self, bit: bool) {
        if bit {
            self.current_byte |= 1 << (7 - self.bit_pos);
        }
        self.bit_pos += 1;
        if self.bit_pos == 8 {
            self.bytes.push(self.current_byte);
            self.current_byte = 0;
            self.bit_pos = 0;
        }
    }

    fn write_bits(&mut self, value: u32, num_bits: u8) {
        for i in (0..num_bits).rev() {
            self.write_bit((value >> i) & 1 == 1);
        }
    }

    fn finish(mut self) -> Vec<u8> {
        if self.bit_pos > 0 {
            self.bytes.push(self.current_byte);
        }
        self.bytes
    }
}

/// A simple bit reader to unpack bits from bytes
struct BitReader {
    bytes: Vec<u8>,
    byte_pos: usize,
    bit_pos: u8,
}

impl BitReader {
    fn new(bytes: Vec<u8>) -> Self {
        BitReader {
            bytes,
            byte_pos: 0,
            bit_pos: 0,
        }
    }

    fn read_bit(&mut self) -> Option<bool> {
        if self.byte_pos >= self.bytes.len() {
            return None;
        }
        let bit = (self.bytes[self.byte_pos] >> (7 - self.bit_pos)) & 1 == 1;
        self.bit_pos += 1;
        if self.bit_pos == 8 {
            self.byte_pos += 1;
            self.bit_pos = 0;
        }
        Some(bit)
    }

    fn read_bits(&mut self, num_bits: u8) -> Option<u32> {
        let mut value = 0u32;
        for _ in 0..num_bits {
            let bit = self.read_bit()?;
            value = (value << 1) | (bit as u32);
        }
        Some(value)
    }
}

/// Compresses u32 numbers using Rice coding
/// 
/// # Arguments
/// * `numbers` - Input array of u32 values to compress
/// * `k` - Rice parameter (b = 2^k). If None, automatically calculated as 0.69 * average
/// 
/// # Returns
/// Encoded byte array
pub fn compress(numbers: &[u32], k: Option<u8>) -> Vec<u8> {
    if numbers.is_empty() {
        return vec![0]; // Store k=0 for empty input
    }

    // Calculate k if not provided (b = 2^k, where b ≈ 0.69 * average)
    let k = k.unwrap_or_else(|| {
        let sum: u64 = numbers.iter().map(|&n| n as u64).sum();
        let avg = sum as f64 / numbers.len() as f64;
        let b = (0.69 * avg).max(1.0);
        let k = (b.log2().ceil() as u8).max(0);
        k.min(31) // Cap at 31 to prevent overflow
    });

    let mut writer = BitWriter::new();
    
    // Write the parameter k (5 bits, supports k from 0 to 31)
    writer.write_bits(k as u32, 5);
    
    // Write the number of values (32 bits)
    writer.write_bits(numbers.len() as u32, 32);

    // Encode each number
    for &n in numbers {
        // q = n / 2^k (quotient)
        let q = n >> k;
        // r = n % 2^k (remainder)
        let r = n & ((1 << k) - 1);

        // Write quotient in unary: q zeros followed by a one
        for _ in 0..q {
            writer.write_bit(false);
        }
        writer.write_bit(true);

        // Write remainder in binary (k bits)
        writer.write_bits(r, k);
    }

    writer.finish()
}

/// Decompresses Rice-coded data back to u32 numbers
/// 
/// # Arguments
/// * `encoded` - Encoded byte array from compress()
/// 
/// # Returns
/// Original array of u32 values, or None if decoding fails
pub fn decompress(encoded: &[u8]) -> Option<Vec<u32>> {
    if encoded.is_empty() {
        return None;
    }

    let mut reader = BitReader::new(encoded.to_vec());

    // Read the parameter k
    let k = reader.read_bits(5)? as u8;
    
    // Read the number of values
    let count = reader.read_bits(32)?;

    let mut numbers = Vec::with_capacity(count as usize);

    for _ in 0..count {
        // Read quotient (unary coded: count zeros until we hit a one)
        let mut q = 0u32;
        loop {
            let bit = reader.read_bit()?;
            if bit {
                break;
            }
            q += 1;
        }

        // Read remainder (k bits in binary)
        let r = reader.read_bits(k)?;

        // Reconstruct the number: n = q * 2^k + r
        let n = (q << k) | r;
        numbers.push(n);
    }

    Some(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_compression() {
        let numbers = vec![0, 1, 2, 3, 4, 5];
        let encoded = compress(&numbers, Some(2));
        let decoded = decompress(&encoded).unwrap();
        assert_eq!(numbers, decoded);
    }

    #[test]
    fn test_auto_parameter() {
        let numbers = vec![10, 12, 15, 11, 13, 14, 16];
        let encoded = compress(&numbers, None);
        let decoded = decompress(&encoded).unwrap();
        assert_eq!(numbers, decoded);
    }

    #[test]
    fn test_large_numbers() {
        let numbers = vec![1000, 1024, 2048, 4096];
        let encoded = compress(&numbers, Some(10));
        let decoded = decompress(&encoded).unwrap();
        assert_eq!(numbers, decoded);
    }

    #[test]
    fn test_single_value() {
        let numbers = vec![42];
        let encoded = compress(&numbers, Some(3));
        let decoded = decompress(&encoded).unwrap();
        assert_eq!(numbers, decoded);
    }

    #[test]
    fn test_zeros() {
        let numbers = vec![0, 0, 0, 0];
        let encoded = compress(&numbers, Some(1));
        let decoded = decompress(&encoded).unwrap();
        assert_eq!(numbers, decoded);
    }
}