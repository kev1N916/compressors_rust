pub fn decode_simple9_1bit(packed: u32, output: &mut Vec<u32>) {
    // Each number is 1 bit, 28 numbers total
    // Extract bits 0-27 (last 28 bits)
    output.push((packed >> 0) & 0b1);
    output.push((packed >> 1) & 0b1);
    output.push((packed >> 2) & 0b1);
    output.push((packed >> 3) & 0b1);
    output.push((packed >> 4) & 0b1);
    output.push((packed >> 5) & 0b1);
    output.push((packed >> 6) & 0b1);
    output.push((packed >> 7) & 0b1);
    output.push((packed >> 8) & 0b1);
    output.push((packed >> 9) & 0b1);
    output.push((packed >> 10) & 0b1);
    output.push((packed >> 11) & 0b1);
    output.push((packed >> 12) & 0b1);
    output.push((packed >> 13) & 0b1);
    output.push((packed >> 14) & 0b1);
    output.push((packed >> 15) & 0b1);
    output.push((packed >> 16) & 0b1);
    output.push((packed >> 17) & 0b1);
    output.push((packed >> 18) & 0b1);
    output.push((packed >> 19) & 0b1);
    output.push((packed >> 20) & 0b1);
    output.push((packed >> 21) & 0b1);
    output.push((packed >> 22) & 0b1);
    output.push((packed >> 23) & 0b1);
    output.push((packed >> 24) & 0b1);
    output.push((packed >> 25) & 0b1);
    output.push((packed >> 26) & 0b1);
    output.push((packed >> 27) & 0b1);
}

pub fn decode_simple9_2bit(packed: u32, output: &mut Vec<u32>) {
    // Each number is 2 bits, 14 numbers total
    // Extract bits 0-27 (last 28 bits)
    output.push((packed >> 0) & 0b11);
    output.push((packed >> 2) & 0b11);
    output.push((packed >> 4) & 0b11);
    output.push((packed >> 6) & 0b11);
    output.push((packed >> 8) & 0b11);
    output.push((packed >> 10) & 0b11);
    output.push((packed >> 12) & 0b11);
    output.push((packed >> 14) & 0b11);
    output.push((packed >> 16) & 0b11);
    output.push((packed >> 18) & 0b11);
    output.push((packed >> 20) & 0b11);
    output.push((packed >> 22) & 0b11);
    output.push((packed >> 24) & 0b11);
    output.push((packed >> 26) & 0b11);
}

pub fn decode_simple9_3bit(packed: u32, output: &mut Vec<u32>) {
    // Each number is 3 bits, 9 numbers total
    // 9 * 3 = 27 bits used, 1 bit wasted
    output.push((packed >> 0) & 0b111);
    output.push((packed >> 3) & 0b111);
    output.push((packed >> 6) & 0b111);
    output.push((packed >> 9) & 0b111);
    output.push((packed >> 12) & 0b111);
    output.push((packed >> 15) & 0b111);
    output.push((packed >> 18) & 0b111);
    output.push((packed >> 21) & 0b111);
    output.push((packed >> 24) & 0b111);
}

pub fn decode_simple9_4bit(packed: u32, output: &mut Vec<u32>) {
    // Each number is 4 bits, 7 numbers total
    // 7 * 4 = 28 bits used, 0 bits wasted
    output.push((packed >> 0) & 0b1111);
    output.push((packed >> 4) & 0b1111);
    output.push((packed >> 8) & 0b1111);
    output.push((packed >> 12) & 0b1111);
    output.push((packed >> 16) & 0b1111);
    output.push((packed >> 20) & 0b1111);
    output.push((packed >> 24) & 0b1111);
}

pub fn decode_simple9_5bit(packed: u32, output: &mut Vec<u32>) {
    // Each number is 5 bits, 5 numbers total
    // 5 * 5 = 25 bits used, 3 bits wasted
    output.push((packed >> 0) & 0b11111);
    output.push((packed >> 5) & 0b11111);
    output.push((packed >> 10) & 0b11111);
    output.push((packed >> 15) & 0b11111);
    output.push((packed >> 20) & 0b11111);
}

pub fn decode_simple9_7bit(packed: u32, output: &mut Vec<u32>) {
    // Each number is 7 bits, 4 numbers total
    output.push((packed >> 0) & 0b1111111);
    output.push((packed >> 7) & 0b1111111);
    output.push((packed >> 14) & 0b1111111);
    output.push((packed >> 21) & 0b1111111);
}

pub fn decode_simple9_9bit(packed: u32, output: &mut Vec<u32>) {
    // Each number is 9 bits, 3 numbers total
    output.push((packed >> 0) & 0b111111111);
    output.push((packed >> 9) & 0b111111111);
    output.push((packed >> 18) & 0b111111111);
}

pub fn decode_simple9_14bit(packed: u32, output: &mut Vec<u32>) {
    // Each number is 14 bits, 2 numbers total
    output.push((packed >> 0) & 0b11111111111111);
    output.push((packed >> 14) & 0b11111111111111);
}

pub fn decode_simple9_28bit(packed: u32, output: &mut Vec<u32>) {
    // Each number is 14 bits, 2 numbers total
    output.push(packed);
}