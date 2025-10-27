
pub fn compress(numbers: &[u32]) -> Vec<u8> {
    let mut result = Vec::new();
    
    for &num in numbers {
        let mut n = num;
        
        // Extract 7-bit chunks and write in little-endian order
        loop {
            let mut byte = (n & 0x7F) as u8;
            n >>= 7;
            
            if n != 0 {
                // More bytes follow, set MSB = 1
                byte |= 0x80;
            }
            // else MSB = 0 (last byte)
            
            result.push(byte);
            
            if n == 0 {
                break;
            }
        }
    }
    
    result
}


pub fn decompress(encoded: &[u8]) -> Vec<u32> {
    let mut result = Vec::new();
    let mut current_num: u32 = 0;
    let mut shift=0;
    for &byte in encoded {
        // Extract the lower 7 bits
        let value = (byte & 0x7F) as u32;
        
        // Add to current number
        current_num = (current_num) | value<< (7*shift);
        shift+=1;
        
        // Check if this is the last byte (MSB = 0)
        if (byte & 0x80) == 0 {
            result.push(current_num);
            current_num = 0;
            shift=0;
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_267() {
        // 267 = 2 * 128 + 11
        let numbers = vec![267];
        let encoded = compress(&numbers);        
        let decoded = decompress(&encoded);
        assert_eq!(decoded, numbers);
    }

    #[test]
    fn test_small_numbers() {
        let numbers = vec![0, 1, 5, 127];
        let encoded = compress(&numbers);
        let decoded = decompress(&encoded);
        assert_eq!(decoded, numbers);
    }

    #[test]
    fn test_128() {
        // 128 = 1 * 128 + 0
        let numbers = vec![128];
        let encoded = compress(&numbers);        
        let decoded = decompress(&encoded);
        assert_eq!(decoded, numbers);
    }

    #[test]
    fn test_large_numbers() {
        let numbers = vec![16384, 2097151, u32::MAX];
        let encoded = compress(&numbers);
        let decoded = decompress(&encoded);
        assert_eq!(decoded, numbers);
    }

    #[test]
    fn test_mixed() {
        let numbers = vec![5, 267, 128, 1000000, 42];
        let encoded = compress(&numbers);
        let decoded = decompress(&encoded);
        assert_eq!(decoded, numbers);
    }
}