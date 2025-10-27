pub type DecompressorFn = fn(&[u32], &mut [u32]);

// 32 values * 1 bit = 32 bits = 1 word
pub fn decompress_1_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] >> 0) & 0x1;
    dst[1] = (src[0] >> 1) & 0x1;
    dst[2] = (src[0] >> 2) & 0x1;
    dst[3] = (src[0] >> 3) & 0x1;
    dst[4] = (src[0] >> 4) & 0x1;
    dst[5] = (src[0] >> 5) & 0x1;
    dst[6] = (src[0] >> 6) & 0x1;
    dst[7] = (src[0] >> 7) & 0x1;
    dst[8] = (src[0] >> 8) & 0x1;
    dst[9] = (src[0] >> 9) & 0x1;
    dst[10] = (src[0] >> 10) & 0x1;
    dst[11] = (src[0] >> 11) & 0x1;
    dst[12] = (src[0] >> 12) & 0x1;
    dst[13] = (src[0] >> 13) & 0x1;
    dst[14] = (src[0] >> 14) & 0x1;
    dst[15] = (src[0] >> 15) & 0x1;
    dst[16] = (src[0] >> 16) & 0x1;
    dst[17] = (src[0] >> 17) & 0x1;
    dst[18] = (src[0] >> 18) & 0x1;
    dst[19] = (src[0] >> 19) & 0x1;
    dst[20] = (src[0] >> 20) & 0x1;
    dst[21] = (src[0] >> 21) & 0x1;
    dst[22] = (src[0] >> 22) & 0x1;
    dst[23] = (src[0] >> 23) & 0x1;
    dst[24] = (src[0] >> 24) & 0x1;
    dst[25] = (src[0] >> 25) & 0x1;
    dst[26] = (src[0] >> 26) & 0x1;
    dst[27] = (src[0] >> 27) & 0x1;
    dst[28] = (src[0] >> 28) & 0x1;
    dst[29] = (src[0] >> 29) & 0x1;
    dst[30] = (src[0] >> 30) & 0x1;
    dst[31] = (src[0] >> 31) & 0x1;
}

// 32 values * 1 bit = 32 bits = 1 word
pub fn compress_1_bit(src: &[u32], dst: &mut [u32]) {
    dst[0] = (src[0] & 0x1) << 0
        | (src[1] & 0x1) << 1
        | (src[2] & 0x1) << 2
        | (src[3] & 0x1) << 3
        | (src[4] & 0x1) << 4
        | (src[5] & 0x1) << 5
        | (src[6] & 0x1) << 6
        | (src[7] & 0x1) << 7
        | (src[8] & 0x1) << 8
        | (src[9] & 0x1) << 9
        | (src[10] & 0x1) << 10
        | (src[11] & 0x1) << 11
        | (src[12] & 0x1) << 12
        | (src[13] & 0x1) << 13
        | (src[14] & 0x1) << 14
        | (src[15] & 0x1) << 15
        | (src[16] & 0x1) << 16
        | (src[17] & 0x1) << 17
        | (src[18] & 0x1) << 18
        | (src[19] & 0x1) << 19
        | (src[20] & 0x1) << 20
        | (src[21] & 0x1) << 21
        | (src[22] & 0x1) << 22
        | (src[23] & 0x1) << 23
        | (src[24] & 0x1) << 24
        | (src[25] & 0x1) << 25
        | (src[26] & 0x1) << 26
        | (src[27] & 0x1) << 27
        | (src[28] & 0x1) << 28
        | (src[29] & 0x1) << 29
        | (src[30] & 0x1) << 30
        | (src[31] & 0x1) << 31;
}

// 32 values * 2 bits = 64 bits = 2 words
pub fn decompress_2_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] >> 0) & 0x3;
    dst[1] = (src[0] >> 2) & 0x3;
    dst[2] = (src[0] >> 4) & 0x3;
    dst[3] = (src[0] >> 6) & 0x3;
    dst[4] = (src[0] >> 8) & 0x3;
    dst[5] = (src[0] >> 10) & 0x3;
    dst[6] = (src[0] >> 12) & 0x3;
    dst[7] = (src[0] >> 14) & 0x3;
    dst[8] = (src[0] >> 16) & 0x3;
    dst[9] = (src[0] >> 18) & 0x3;
    dst[10] = (src[0] >> 20) & 0x3;
    dst[11] = (src[0] >> 22) & 0x3;
    dst[12] = (src[0] >> 24) & 0x3;
    dst[13] = (src[0] >> 26) & 0x3;
    dst[14] = (src[0] >> 28) & 0x3;
    dst[15] = (src[0] >> 30) & 0x3;

    // Word 1: bits 32-63
    dst[16] = (src[1] >> 0) & 0x3;
    dst[17] = (src[1] >> 2) & 0x3;
    dst[18] = (src[1] >> 4) & 0x3;
    dst[19] = (src[1] >> 6) & 0x3;
    dst[20] = (src[1] >> 8) & 0x3;
    dst[21] = (src[1] >> 10) & 0x3;
    dst[22] = (src[1] >> 12) & 0x3;
    dst[23] = (src[1] >> 14) & 0x3;
    dst[24] = (src[1] >> 16) & 0x3;
    dst[25] = (src[1] >> 18) & 0x3;
    dst[26] = (src[1] >> 20) & 0x3;
    dst[27] = (src[1] >> 22) & 0x3;
    dst[28] = (src[1] >> 24) & 0x3;
    dst[29] = (src[1] >> 26) & 0x3;
    dst[30] = (src[1] >> 28) & 0x3;
    dst[31] = (src[1] >> 30) & 0x3;
}

// 32 values * 2 bits = 64 bits = 2 words
pub fn compress_2_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x3) << 0
        | (src[1] & 0x3) << 2
        | (src[2] & 0x3) << 4
        | (src[3] & 0x3) << 6
        | (src[4] & 0x3) << 8
        | (src[5] & 0x3) << 10
        | (src[6] & 0x3) << 12
        | (src[7] & 0x3) << 14
        | (src[8] & 0x3) << 16
        | (src[9] & 0x3) << 18
        | (src[10] & 0x3) << 20
        | (src[11] & 0x3) << 22
        | (src[12] & 0x3) << 24
        | (src[13] & 0x3) << 26
        | (src[14] & 0x3) << 28
        | (src[15] & 0x3) << 30;

    // Word 1: bits 32-63
    dst[1] = (src[16] & 0x3) << 0
        | (src[17] & 0x3) << 2
        | (src[18] & 0x3) << 4
        | (src[19] & 0x3) << 6
        | (src[20] & 0x3) << 8
        | (src[21] & 0x3) << 10
        | (src[22] & 0x3) << 12
        | (src[23] & 0x3) << 14
        | (src[24] & 0x3) << 16
        | (src[25] & 0x3) << 18
        | (src[26] & 0x3) << 20
        | (src[27] & 0x3) << 22
        | (src[28] & 0x3) << 24
        | (src[29] & 0x3) << 26
        | (src[30] & 0x3) << 28
        | (src[31] & 0x3) << 30;
}

// 32 values * 3 bits = 96 bits = 3 words
pub fn decompress_3_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31 contain values 0-9 (partial)
    dst[0] = (src[0] >> 0) & 0x7; // bits 0-2
    dst[1] = (src[0] >> 3) & 0x7; // bits 3-5
    dst[2] = (src[0] >> 6) & 0x7; // bits 6-8
    dst[3] = (src[0] >> 9) & 0x7; // bits 9-11
    dst[4] = (src[0] >> 12) & 0x7; // bits 12-14
    dst[5] = (src[0] >> 15) & 0x7; // bits 15-17
    dst[6] = (src[0] >> 18) & 0x7; // bits 18-20
    dst[7] = (src[0] >> 21) & 0x7; // bits 21-23
    dst[8] = (src[0] >> 24) & 0x7; // bits 24-26
    dst[9] = (src[0] >> 27) & 0x7; // bits 27-29

    // Value 10 spans words 0-1
    dst[10] = ((src[0] >> 30) & 0x3) | ((src[1] & 0x1) << 2);

    // Word 1: bits 32-63
    dst[11] = (src[1] >> 1) & 0x7;
    dst[12] = (src[1] >> 4) & 0x7;
    dst[13] = (src[1] >> 7) & 0x7;
    dst[14] = (src[1] >> 10) & 0x7;
    dst[15] = (src[1] >> 13) & 0x7;
    dst[16] = (src[1] >> 16) & 0x7;
    dst[17] = (src[1] >> 19) & 0x7;
    dst[18] = (src[1] >> 22) & 0x7;
    dst[19] = (src[1] >> 25) & 0x7;
    dst[20] = (src[1] >> 28) & 0x7;

    // Value 21 spans words 1-2
    dst[21] = ((src[1] >> 31) & 0x1) | ((src[2] & 0x3) << 1);

    // Word 2: bits 64-95
    dst[22] = (src[2] >> 2) & 0x7;
    dst[23] = (src[2] >> 5) & 0x7;
    dst[24] = (src[2] >> 8) & 0x7;
    dst[25] = (src[2] >> 11) & 0x7;
    dst[26] = (src[2] >> 14) & 0x7;
    dst[27] = (src[2] >> 17) & 0x7;
    dst[28] = (src[2] >> 20) & 0x7;
    dst[29] = (src[2] >> 23) & 0x7;
    dst[30] = (src[2] >> 26) & 0x7;
    dst[31] = (src[2] >> 29) & 0x7;
}

// 32 values * 3 bits = 96 bits = 3 words
pub fn compress_3_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31 contain values 0-9 (partial)
    dst[0] = (src[0] & 0x7) << 0  // bits 0-2
        | (src[1] & 0x7) << 3      // bits 3-5
        | (src[2] & 0x7) << 6      // bits 6-8
        | (src[3] & 0x7) << 9      // bits 9-11
        | (src[4] & 0x7) << 12     // bits 12-14
        | (src[5] & 0x7) << 15     // bits 15-17
        | (src[6] & 0x7) << 18     // bits 18-20
        | (src[7] & 0x7) << 21     // bits 21-23
        | (src[8] & 0x7) << 24     // bits 24-26
        | (src[9] & 0x7) << 27     // bits 27-29
        | (src[10] & 0x3) << 30; // bits 30-31 (lower 2 bits of value 10)

    // Word 1: bits 32-63
    dst[1] = ((src[10] >> 2) & 0x1) << 0  // bit 0 (upper 1 bit of value 10)
        | (src[11] & 0x7) << 1             // bits 1-3
        | (src[12] & 0x7) << 4             // bits 4-6
        | (src[13] & 0x7) << 7             // bits 7-9
        | (src[14] & 0x7) << 10            // bits 10-12
        | (src[15] & 0x7) << 13            // bits 13-15
        | (src[16] & 0x7) << 16            // bits 16-18
        | (src[17] & 0x7) << 19            // bits 19-21
        | (src[18] & 0x7) << 22            // bits 22-24
        | (src[19] & 0x7) << 25            // bits 25-27
        | (src[20] & 0x7) << 28            // bits 28-30
        | (src[21] & 0x1) << 31; // bit 31 (lower 1 bit of value 21)

    // Word 2: bits 64-95
    dst[2] = ((src[21] >> 1) & 0x3) << 0  // bits 0-1 (upper 2 bits of value 21)
        | (src[22] & 0x7) << 2             // bits 2-4
        | (src[23] & 0x7) << 5             // bits 5-7
        | (src[24] & 0x7) << 8             // bits 8-10
        | (src[25] & 0x7) << 11            // bits 11-13
        | (src[26] & 0x7) << 14            // bits 14-16
        | (src[27] & 0x7) << 17            // bits 17-19
        | (src[28] & 0x7) << 20            // bits 20-22
        | (src[29] & 0x7) << 23            // bits 23-25
        | (src[30] & 0x7) << 26            // bits 26-28
        | (src[31] & 0x7) << 29; // bits 29-31
}

// 32 values * 4 bits = 128 bits = 4 words
pub fn decompress_4_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] >> 0) & 0xF;
    dst[1] = (src[0] >> 4) & 0xF;
    dst[2] = (src[0] >> 8) & 0xF;
    dst[3] = (src[0] >> 12) & 0xF;
    dst[4] = (src[0] >> 16) & 0xF;
    dst[5] = (src[0] >> 20) & 0xF;
    dst[6] = (src[0] >> 24) & 0xF;
    dst[7] = (src[0] >> 28) & 0xF;

    // Word 1: bits 32-63
    dst[8] = (src[1] >> 0) & 0xF;
    dst[9] = (src[1] >> 4) & 0xF;
    dst[10] = (src[1] >> 8) & 0xF;
    dst[11] = (src[1] >> 12) & 0xF;
    dst[12] = (src[1] >> 16) & 0xF;
    dst[13] = (src[1] >> 20) & 0xF;
    dst[14] = (src[1] >> 24) & 0xF;
    dst[15] = (src[1] >> 28) & 0xF;

    // Word 2: bits 64-95
    dst[16] = (src[2] >> 0) & 0xF;
    dst[17] = (src[2] >> 4) & 0xF;
    dst[18] = (src[2] >> 8) & 0xF;
    dst[19] = (src[2] >> 12) & 0xF;
    dst[20] = (src[2] >> 16) & 0xF;
    dst[21] = (src[2] >> 20) & 0xF;
    dst[22] = (src[2] >> 24) & 0xF;
    dst[23] = (src[2] >> 28) & 0xF;

    // Word 3: bits 96-127
    dst[24] = (src[3] >> 0) & 0xF;
    dst[25] = (src[3] >> 4) & 0xF;
    dst[26] = (src[3] >> 8) & 0xF;
    dst[27] = (src[3] >> 12) & 0xF;
    dst[28] = (src[3] >> 16) & 0xF;
    dst[29] = (src[3] >> 20) & 0xF;
    dst[30] = (src[3] >> 24) & 0xF;
    dst[31] = (src[3] >> 28) & 0xF;
}

// 32 values * 4 bits = 128 bits = 4 words
pub fn compress_4_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0xF) << 0
        | (src[1] & 0xF) << 4
        | (src[2] & 0xF) << 8
        | (src[3] & 0xF) << 12
        | (src[4] & 0xF) << 16
        | (src[5] & 0xF) << 20
        | (src[6] & 0xF) << 24
        | (src[7] & 0xF) << 28;

    // Word 1: bits 32-63
    dst[1] = (src[8] & 0xF) << 0
        | (src[9] & 0xF) << 4
        | (src[10] & 0xF) << 8
        | (src[11] & 0xF) << 12
        | (src[12] & 0xF) << 16
        | (src[13] & 0xF) << 20
        | (src[14] & 0xF) << 24
        | (src[15] & 0xF) << 28;

    // Word 2: bits 64-95
    dst[2] = (src[16] & 0xF) << 0
        | (src[17] & 0xF) << 4
        | (src[18] & 0xF) << 8
        | (src[19] & 0xF) << 12
        | (src[20] & 0xF) << 16
        | (src[21] & 0xF) << 20
        | (src[22] & 0xF) << 24
        | (src[23] & 0xF) << 28;

    // Word 3: bits 96-127
    dst[3] = (src[24] & 0xF) << 0
        | (src[25] & 0xF) << 4
        | (src[26] & 0xF) << 8
        | (src[27] & 0xF) << 12
        | (src[28] & 0xF) << 16
        | (src[29] & 0xF) << 20
        | (src[30] & 0xF) << 24
        | (src[31] & 0xF) << 28;
}

// 32 values * 5 bits = 160 bits = 5 words
pub fn decompress_5_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] >> 0) & 0x1F;
    dst[1] = (src[0] >> 5) & 0x1F;
    dst[2] = (src[0] >> 10) & 0x1F;
    dst[3] = (src[0] >> 15) & 0x1F;
    dst[4] = (src[0] >> 20) & 0x1F;
    dst[5] = (src[0] >> 25) & 0x1F;

    // Value 6 spans words 0-1
    dst[6] = ((src[0] >> 30) & 0x3) | ((src[1] & 0x7) << 2);

    // Word 1: bits 32-63
    dst[7] = (src[1] >> 3) & 0x1F;
    dst[8] = (src[1] >> 8) & 0x1F;
    dst[9] = (src[1] >> 13) & 0x1F;
    dst[10] = (src[1] >> 18) & 0x1F;
    dst[11] = (src[1] >> 23) & 0x1F;

    // Value 12 spans words 1-2
    dst[12] = ((src[1] >> 28) & 0xF) | ((src[2] & 0x1) << 4);

    // Word 2: bits 64-95
    dst[13] = (src[2] >> 1) & 0x1F;
    dst[14] = (src[2] >> 6) & 0x1F;
    dst[15] = (src[2] >> 11) & 0x1F;
    dst[16] = (src[2] >> 16) & 0x1F;
    dst[17] = (src[2] >> 21) & 0x1F;
    dst[18] = (src[2] >> 26) & 0x1F;

    // Value 19 spans words 2-3
    dst[19] = ((src[2] >> 31) & 0x1) | ((src[3] & 0xF) << 1);

    // Word 3: bits 96-127
    dst[20] = (src[3] >> 4) & 0x1F;
    dst[21] = (src[3] >> 9) & 0x1F;
    dst[22] = (src[3] >> 14) & 0x1F;
    dst[23] = (src[3] >> 19) & 0x1F;
    dst[24] = (src[3] >> 24) & 0x1F;

    // Value 25 spans words 3-4
    dst[25] = ((src[3] >> 29) & 0x7) | ((src[4] & 0x3) << 3);

    // Word 4: bits 128-159
    dst[26] = (src[4] >> 2) & 0x1F;
    dst[27] = (src[4] >> 7) & 0x1F;
    dst[28] = (src[4] >> 12) & 0x1F;
    dst[29] = (src[4] >> 17) & 0x1F;
    dst[30] = (src[4] >> 22) & 0x1F;
    dst[31] = (src[4] >> 27) & 0x1F;
}

// 32 values * 5 bits = 160 bits = 5 words
pub fn compress_5_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x1F) << 0
        | (src[1] & 0x1F) << 5
        | (src[2] & 0x1F) << 10
        | (src[3] & 0x1F) << 15
        | (src[4] & 0x1F) << 20
        | (src[5] & 0x1F) << 25
        | (src[6] & 0x3) << 30; // bits 30-31 (lower 2 bits of value 6)

    // Word 1: bits 32-63
    dst[1] = ((src[6] >> 2) & 0x7) << 0   // bits 0-2 (upper 3 bits of value 6)
        | (src[7] & 0x1F) << 3
        | (src[8] & 0x1F) << 8
        | (src[9] & 0x1F) << 13
        | (src[10] & 0x1F) << 18
        | (src[11] & 0x1F) << 23
        | (src[12] & 0xF) << 28; // bits 28-31 (lower 4 bits of value 12)

    // Word 2: bits 64-95
    dst[2] = ((src[12] >> 4) & 0x1) << 0  // bit 0 (upper 1 bit of value 12)
        | (src[13] & 0x1F) << 1
        | (src[14] & 0x1F) << 6
        | (src[15] & 0x1F) << 11
        | (src[16] & 0x1F) << 16
        | (src[17] & 0x1F) << 21
        | (src[18] & 0x1F) << 26
        | (src[19] & 0x1) << 31; // bit 31 (lower 1 bit of value 19)

    // Word 3: bits 96-127
    dst[3] = ((src[19] >> 1) & 0xF) << 0  // bits 0-3 (upper 4 bits of value 19)
        | (src[20] & 0x1F) << 4
        | (src[21] & 0x1F) << 9
        | (src[22] & 0x1F) << 14
        | (src[23] & 0x1F) << 19
        | (src[24] & 0x1F) << 24
        | (src[25] & 0x7) << 29; // bits 29-31 (lower 3 bits of value 25)

    // Word 4: bits 128-159
    dst[4] = ((src[25] >> 3) & 0x3) << 0  // bits 0-1 (upper 2 bits of value 25)
        | (src[26] & 0x1F) << 2
        | (src[27] & 0x1F) << 7
        | (src[28] & 0x1F) << 12
        | (src[29] & 0x1F) << 17
        | (src[30] & 0x1F) << 22
        | (src[31] & 0x1F) << 27;
}

// 32 values * 6 bits = 192 bits = 6 words
pub fn decompress_6_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] >> 0) & 0x3F;
    dst[1] = (src[0] >> 6) & 0x3F;
    dst[2] = (src[0] >> 12) & 0x3F;
    dst[3] = (src[0] >> 18) & 0x3F;
    dst[4] = (src[0] >> 24) & 0x3F;

    // Value 5 spans words 0-1
    dst[5] = ((src[0] >> 30) & 0x3) | ((src[1] & 0xF) << 2);

    // Word 1: bits 32-63
    dst[6] = (src[1] >> 4) & 0x3F;
    dst[7] = (src[1] >> 10) & 0x3F;
    dst[8] = (src[1] >> 16) & 0x3F;
    dst[9] = (src[1] >> 22) & 0x3F;

    // Value 10 spans words 1-2
    dst[10] = ((src[1] >> 28) & 0xF) | ((src[2] & 0x3) << 4);

    // Word 2: bits 64-95
    dst[11] = (src[2] >> 2) & 0x3F;
    dst[12] = (src[2] >> 8) & 0x3F;
    dst[13] = (src[2] >> 14) & 0x3F;
    dst[14] = (src[2] >> 20) & 0x3F;
    dst[15] = (src[2] >> 26) & 0x3F;

    // Word 3: bits 96-127
    dst[16] = (src[3] >> 0) & 0x3F;
    dst[17] = (src[3] >> 6) & 0x3F;
    dst[18] = (src[3] >> 12) & 0x3F;
    dst[19] = (src[3] >> 18) & 0x3F;
    dst[20] = (src[3] >> 24) & 0x3F;

    // Value 21 spans words 3-4
    dst[21] = ((src[3] >> 30) & 0x3) | ((src[4] & 0xF) << 2);

    // Word 4: bits 128-159
    dst[22] = (src[4] >> 4) & 0x3F;
    dst[23] = (src[4] >> 10) & 0x3F;
    dst[24] = (src[4] >> 16) & 0x3F;
    dst[25] = (src[4] >> 22) & 0x3F;

    // Value 26 spans words 4-5
    dst[26] = ((src[4] >> 28) & 0xF) | ((src[5] & 0x3) << 4);

    // Word 5: bits 160-191
    dst[27] = (src[5] >> 2) & 0x3F;
    dst[28] = (src[5] >> 8) & 0x3F;
    dst[29] = (src[5] >> 14) & 0x3F;
    dst[30] = (src[5] >> 20) & 0x3F;
    dst[31] = (src[5] >> 26) & 0x3F;
}

// 32 values * 6 bits = 192 bits = 6 words
pub fn compress_6_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x3F) << 0
        | (src[1] & 0x3F) << 6
        | (src[2] & 0x3F) << 12
        | (src[3] & 0x3F) << 18
        | (src[4] & 0x3F) << 24
        | (src[5] & 0x3) << 30; // bits 30-31 (lower 2 bits of value 5)

    // Word 1: bits 32-63
    dst[1] = ((src[5] >> 2) & 0xF) << 0   // bits 0-3 (upper 4 bits of value 5)
        | (src[6] & 0x3F) << 4
        | (src[7] & 0x3F) << 10
        | (src[8] & 0x3F) << 16
        | (src[9] & 0x3F) << 22
        | (src[10] & 0xF) << 28; // bits 28-31 (lower 4 bits of value 10)

    // Word 2: bits 64-95
    dst[2] = ((src[10] >> 4) & 0x3) << 0  // bits 0-1 (upper 2 bits of value 10)
        | (src[11] & 0x3F) << 2
        | (src[12] & 0x3F) << 8
        | (src[13] & 0x3F) << 14
        | (src[14] & 0x3F) << 20
        | (src[15] & 0x3F) << 26;

    // Word 3: bits 96-127
    dst[3] = (src[16] & 0x3F) << 0
        | (src[17] & 0x3F) << 6
        | (src[18] & 0x3F) << 12
        | (src[19] & 0x3F) << 18
        | (src[20] & 0x3F) << 24
        | (src[21] & 0x3) << 30; // bits 30-31 (lower 2 bits of value 21)

    // Word 4: bits 128-159
    dst[4] = ((src[21] >> 2) & 0xF) << 0  // bits 0-3 (upper 4 bits of value 21)
        | (src[22] & 0x3F) << 4
        | (src[23] & 0x3F) << 10
        | (src[24] & 0x3F) << 16
        | (src[25] & 0x3F) << 22
        | (src[26] & 0xF) << 28; // bits 28-31 (lower 4 bits of value 26)

    // Word 5: bits 160-191
    dst[5] = ((src[26] >> 4) & 0x3) << 0  // bits 0-1 (upper 2 bits of value 26)
        | (src[27] & 0x3F) << 2
        | (src[28] & 0x3F) << 8
        | (src[29] & 0x3F) << 14
        | (src[30] & 0x3F) << 20
        | (src[31] & 0x3F) << 26;
}

// 32 values * 7 bits = 224 bits = 7 words
pub fn decompress_7_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] >> 0) & 0x7F;
    dst[1] = (src[0] >> 7) & 0x7F;
    dst[2] = (src[0] >> 14) & 0x7F;
    dst[3] = (src[0] >> 21) & 0x7F;

    // Value 4 spans words 0-1
    dst[4] = ((src[0] >> 28) & 0xF) | ((src[1] & 0x7) << 4);

    // Word 1: bits 32-63
    dst[5] = (src[1] >> 3) & 0x7F;
    dst[6] = (src[1] >> 10) & 0x7F;
    dst[7] = (src[1] >> 17) & 0x7F;
    dst[8] = (src[1] >> 24) & 0x7F;

    // Value 9 spans words 1-2
    dst[9] = ((src[1] >> 31) & 0x1) | ((src[2] & 0x3F) << 1);

    // Word 2: bits 64-95
    dst[10] = (src[2] >> 6) & 0x7F;
    dst[11] = (src[2] >> 13) & 0x7F;
    dst[12] = (src[2] >> 20) & 0x7F;

    // Value 13 spans words 2-3
    dst[13] = ((src[2] >> 27) & 0x1F) | ((src[3] & 0x3) << 5);

    // Word 3: bits 96-127
    dst[14] = (src[3] >> 2) & 0x7F;
    dst[15] = (src[3] >> 9) & 0x7F;
    dst[16] = (src[3] >> 16) & 0x7F;
    dst[17] = (src[3] >> 23) & 0x7F;

    // Value 18 spans words 3-4
    dst[18] = ((src[3] >> 30) & 0x3) | ((src[4] & 0x1F) << 2);

    // Word 4: bits 128-159
    dst[19] = (src[4] >> 5) & 0x7F;
    dst[20] = (src[4] >> 12) & 0x7F;
    dst[21] = (src[4] >> 19) & 0x7F;

    // Value 22 spans words 4-5
    dst[22] = ((src[4] >> 26) & 0x3F) | ((src[5] & 0x1) << 6);

    // Word 5: bits 160-191
    dst[23] = (src[5] >> 1) & 0x7F;
    dst[24] = (src[5] >> 8) & 0x7F;
    dst[25] = (src[5] >> 15) & 0x7F;
    dst[26] = (src[5] >> 22) & 0x7F;

    // Value 27 spans words 5-6
    dst[27] = ((src[5] >> 29) & 0x7) | ((src[6] & 0xF) << 3);

    // Word 6: bits 192-223
    dst[28] = (src[6] >> 4) & 0x7F;
    dst[29] = (src[6] >> 11) & 0x7F;
    dst[30] = (src[6] >> 18) & 0x7F;
    dst[31] = (src[6] >> 25) & 0x7F;
}

// 32 values * 7 bits = 224 bits = 7 words
pub fn compress_7_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x7F) << 0
        | (src[1] & 0x7F) << 7
        | (src[2] & 0x7F) << 14
        | (src[3] & 0x7F) << 21
        | (src[4] & 0xF) << 28; // bits 28-31 (lower 4 bits of value 4)

    // Word 1: bits 32-63
    dst[1] = ((src[4] >> 4) & 0x7) << 0   // bits 0-2 (upper 3 bits of value 4)
        | (src[5] & 0x7F) << 3
        | (src[6] & 0x7F) << 10
        | (src[7] & 0x7F) << 17
        | (src[8] & 0x7F) << 24
        | (src[9] & 0x1) << 31; // bit 31 (lower 1 bit of value 9)

    // Word 2: bits 64-95
    dst[2] = ((src[9] >> 1) & 0x3F) << 0  // bits 0-5 (upper 6 bits of value 9)
        | (src[10] & 0x7F) << 6
        | (src[11] & 0x7F) << 13
        | (src[12] & 0x7F) << 20
        | (src[13] & 0x1F) << 27; // bits 27-31 (lower 5 bits of value 13)

    // Word 3: bits 96-127
    dst[3] = ((src[13] >> 5) & 0x3) << 0  // bits 0-1 (upper 2 bits of value 13)
        | (src[14] & 0x7F) << 2
        | (src[15] & 0x7F) << 9
        | (src[16] & 0x7F) << 16
        | (src[17] & 0x7F) << 23
        | (src[18] & 0x3) << 30; // bits 30-31 (lower 2 bits of value 18)

    // Word 4: bits 128-159
    dst[4] = ((src[18] >> 2) & 0x1F) << 0 // bits 0-4 (upper 5 bits of value 18)
        | (src[19] & 0x7F) << 5
        | (src[20] & 0x7F) << 12
        | (src[21] & 0x7F) << 19
        | (src[22] & 0x3F) << 26; // bits 26-31 (lower 6 bits of value 22)

    // Word 5: bits 160-191
    dst[5] = ((src[22] >> 6) & 0x1) << 0  // bit 0 (upper 1 bit of value 22)
        | (src[23] & 0x7F) << 1
        | (src[24] & 0x7F) << 8
        | (src[25] & 0x7F) << 15
        | (src[26] & 0x7F) << 22
        | (src[27] & 0x7) << 29; // bits 29-31 (lower 3 bits of value 27)

    // Word 6: bits 192-223
    dst[6] = ((src[27] >> 3) & 0xF) << 0  // bits 0-3 (upper 4 bits of value 27)
        | (src[28] & 0x7F) << 4
        | (src[29] & 0x7F) << 11
        | (src[30] & 0x7F) << 18
        | (src[31] & 0x7F) << 25;
}

// 32 values * 8 bits = 256 bits = 8 words
pub fn decompress_8_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] >> 0) & 0xFF;
    dst[1] = (src[0] >> 8) & 0xFF;
    dst[2] = (src[0] >> 16) & 0xFF;
    dst[3] = (src[0] >> 24) & 0xFF;

    // Word 1: bits 32-63
    dst[4] = (src[1] >> 0) & 0xFF;
    dst[5] = (src[1] >> 8) & 0xFF;
    dst[6] = (src[1] >> 16) & 0xFF;
    dst[7] = (src[1] >> 24) & 0xFF;

    // Word 2: bits 64-95
    dst[8] = (src[2] >> 0) & 0xFF;
    dst[9] = (src[2] >> 8) & 0xFF;
    dst[10] = (src[2] >> 16) & 0xFF;
    dst[11] = (src[2] >> 24) & 0xFF;

    // Word 3: bits 96-127
    dst[12] = (src[3] >> 0) & 0xFF;
    dst[13] = (src[3] >> 8) & 0xFF;
    dst[14] = (src[3] >> 16) & 0xFF;
    dst[15] = (src[3] >> 24) & 0xFF;

    // Word 4: bits 128-159
    dst[16] = (src[4] >> 0) & 0xFF;
    dst[17] = (src[4] >> 8) & 0xFF;
    dst[18] = (src[4] >> 16) & 0xFF;
    dst[19] = (src[4] >> 24) & 0xFF;

    // Word 5: bits 160-191
    dst[20] = (src[5] >> 0) & 0xFF;
    dst[21] = (src[5] >> 8) & 0xFF;
    dst[22] = (src[5] >> 16) & 0xFF;
    dst[23] = (src[5] >> 24) & 0xFF;

    // Word 6: bits 192-223
    dst[24] = (src[6] >> 0) & 0xFF;
    dst[25] = (src[6] >> 8) & 0xFF;
    dst[26] = (src[6] >> 16) & 0xFF;
    dst[27] = (src[6] >> 24) & 0xFF;

    // Word 7: bits 224-255
    dst[28] = (src[7] >> 0) & 0xFF;
    dst[29] = (src[7] >> 8) & 0xFF;
    dst[30] = (src[7] >> 16) & 0xFF;
    dst[31] = (src[7] >> 24) & 0xFF;
}

// 32 values * 8 bits = 256 bits = 8 words
pub fn compress_8_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] =
        (src[0] & 0xFF) << 0 | (src[1] & 0xFF) << 8 | (src[2] & 0xFF) << 16 | (src[3] & 0xFF) << 24;

    // Word 1: bits 32-63
    dst[1] =
        (src[4] & 0xFF) << 0 | (src[5] & 0xFF) << 8 | (src[6] & 0xFF) << 16 | (src[7] & 0xFF) << 24;

    // Word 2: bits 64-95
    dst[2] = (src[8] & 0xFF) << 0
        | (src[9] & 0xFF) << 8
        | (src[10] & 0xFF) << 16
        | (src[11] & 0xFF) << 24;

    // Word 3: bits 96-127
    dst[3] = (src[12] & 0xFF) << 0
        | (src[13] & 0xFF) << 8
        | (src[14] & 0xFF) << 16
        | (src[15] & 0xFF) << 24;

    // Word 4: bits 128-159
    dst[4] = (src[16] & 0xFF) << 0
        | (src[17] & 0xFF) << 8
        | (src[18] & 0xFF) << 16
        | (src[19] & 0xFF) << 24;

    // Word 5: bits 160-191
    dst[5] = (src[20] & 0xFF) << 0
        | (src[21] & 0xFF) << 8
        | (src[22] & 0xFF) << 16
        | (src[23] & 0xFF) << 24;

    // Word 6: bits 192-223
    dst[6] = (src[24] & 0xFF) << 0
        | (src[25] & 0xFF) << 8
        | (src[26] & 0xFF) << 16
        | (src[27] & 0xFF) << 24;

    // Word 7: bits 224-255
    dst[7] = (src[28] & 0xFF) << 0
        | (src[29] & 0xFF) << 8
        | (src[30] & 0xFF) << 16
        | (src[31] & 0xFF) << 24;
}

// 32 values * 9 bits = 288 bits = 9 words
pub fn decompress_9_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] >> 0) & 0x1FF;
    dst[1] = (src[0] >> 9) & 0x1FF;
    dst[2] = (src[0] >> 18) & 0x1FF;

    // Value 3 spans words 0-1
    dst[3] = ((src[0] >> 27) & 0x1F) | ((src[1] & 0xF) << 5);

    // Word 1: bits 32-63
    dst[4] = (src[1] >> 4) & 0x1FF;
    dst[5] = (src[1] >> 13) & 0x1FF;
    dst[6] = (src[1] >> 22) & 0x1FF;

    // Value 7 spans words 1-2
    dst[7] = ((src[1] >> 31) & 0x1) | ((src[2] & 0xFF) << 1);

    // Word 2: bits 64-95
    dst[8] = (src[2] >> 8) & 0x1FF;
    dst[9] = (src[2] >> 17) & 0x1FF;

    // Value 10 spans words 2-3
    dst[10] = ((src[2] >> 26) & 0x3F) | ((src[3] & 0x7) << 6);

    // Word 3: bits 96-127
    dst[11] = (src[3] >> 3) & 0x1FF;
    dst[12] = (src[3] >> 12) & 0x1FF;
    dst[13] = (src[3] >> 21) & 0x1FF;

    // Value 14 spans words 3-4
    dst[14] = ((src[3] >> 30) & 0x3) | ((src[4] & 0x7F) << 2);

    // Word 4: bits 128-159
    dst[15] = (src[4] >> 7) & 0x1FF;
    dst[16] = (src[4] >> 16) & 0x1FF;

    // Value 17 spans words 4-5
    dst[17] = ((src[4] >> 25) & 0x7F) | ((src[5] & 0x3) << 7);

    // Word 5: bits 160-191
    dst[18] = (src[5] >> 2) & 0x1FF;
    dst[19] = (src[5] >> 11) & 0x1FF;
    dst[20] = (src[5] >> 20) & 0x1FF;

    // Value 21 spans words 5-6
    dst[21] = ((src[5] >> 29) & 0x7) | ((src[6] & 0x3F) << 3);

    // Word 6: bits 192-223
    dst[22] = (src[6] >> 6) & 0x1FF;
    dst[23] = (src[6] >> 15) & 0x1FF;

    // Value 24 spans words 6-7
    dst[24] = ((src[6] >> 24) & 0xFF) | ((src[7] & 0x1) << 8);

    // Word 7: bits 224-255
    dst[25] = (src[7] >> 1) & 0x1FF;
    dst[26] = (src[7] >> 10) & 0x1FF;
    dst[27] = (src[7] >> 19) & 0x1FF;

    // Value 28 spans words 7-8
    dst[28] = ((src[7] >> 28) & 0xF) | ((src[8] & 0x1F) << 4);

    // Word 8: bits 256-287
    dst[29] = (src[8] >> 5) & 0x1FF;
    dst[30] = (src[8] >> 14) & 0x1FF;
    dst[31] = (src[8] >> 23) & 0x1FF;
}

// 32 values * 9 bits = 288 bits = 9 words
pub fn compress_9_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x1FF) << 0
        | (src[1] & 0x1FF) << 9
        | (src[2] & 0x1FF) << 18
        | (src[3] & 0x1F) << 27; // bits 27-31 (lower 5 bits of value 3)

    // Word 1: bits 32-63
    dst[1] = ((src[3] >> 5) & 0xF) << 0   // bits 0-3 (upper 4 bits of value 3)
        | (src[4] & 0x1FF) << 4
        | (src[5] & 0x1FF) << 13
        | (src[6] & 0x1FF) << 22
        | (src[7] & 0x1) << 31; // bit 31 (lower 1 bit of value 7)

    // Word 2: bits 64-95
    dst[2] = ((src[7] >> 1) & 0xFF) << 0  // bits 0-7 (upper 8 bits of value 7)
        | (src[8] & 0x1FF) << 8
        | (src[9] & 0x1FF) << 17
        | (src[10] & 0x3F) << 26; // bits 26-31 (lower 6 bits of value 10)

    // Word 3: bits 96-127
    dst[3] = ((src[10] >> 6) & 0x7) << 0  // bits 0-2 (upper 3 bits of value 10)
        | (src[11] & 0x1FF) << 3
        | (src[12] & 0x1FF) << 12
        | (src[13] & 0x1FF) << 21
        | (src[14] & 0x3) << 30; // bits 30-31 (lower 2 bits of value 14)

    // Word 4: bits 128-159
    dst[4] = ((src[14] >> 2) & 0x7F) << 0 // bits 0-6 (upper 7 bits of value 14)
        | (src[15] & 0x1FF) << 7
        | (src[16] & 0x1FF) << 16
        | (src[17] & 0x7F) << 25; // bits 25-31 (lower 7 bits of value 17)

    // Word 5: bits 160-191
    dst[5] = ((src[17] >> 7) & 0x3) << 0  // bits 0-1 (upper 2 bits of value 17)
        | (src[18] & 0x1FF) << 2
        | (src[19] & 0x1FF) << 11
        | (src[20] & 0x1FF) << 20
        | (src[21] & 0x7) << 29; // bits 29-31 (lower 3 bits of value 21)

    // Word 6: bits 192-223
    dst[6] = ((src[21] >> 3) & 0x3F) << 0 // bits 0-5 (upper 6 bits of value 21)
        | (src[22] & 0x1FF) << 6
        | (src[23] & 0x1FF) << 15
        | (src[24] & 0xFF) << 24; // bits 24-31 (lower 8 bits of value 24)

    // Word 7: bits 224-255
    dst[7] = ((src[24] >> 8) & 0x1) << 0  // bit 0 (upper 1 bit of value 24)
        | (src[25] & 0x1FF) << 1
        | (src[26] & 0x1FF) << 10
        | (src[27] & 0x1FF) << 19
        | (src[28] & 0xF) << 28; // bits 28-31 (lower 4 bits of value 28)

    // Word 8: bits 256-287
    dst[8] = ((src[28] >> 4) & 0x1F) << 0 // bits 0-4 (upper 5 bits of value 28)
        | (src[29] & 0x1FF) << 5
        | (src[30] & 0x1FF) << 14
        | (src[31] & 0x1FF) << 23;
}

// 32 values * 10 bits = 320 bits = 10 words
pub fn decompress_10_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] >> 0) & 0x3FF;
    dst[1] = (src[0] >> 10) & 0x3FF;
    dst[2] = (src[0] >> 20) & 0x3FF;

    // Value 3 spans words 0-1
    dst[3] = ((src[0] >> 30) & 0x3) | ((src[1] & 0xFF) << 2);

    // Word 1: bits 32-63
    dst[4] = (src[1] >> 8) & 0x3FF;
    dst[5] = (src[1] >> 18) & 0x3FF;

    // Value 6 spans words 1-2
    dst[6] = ((src[1] >> 28) & 0xF) | ((src[2] & 0x3F) << 4);

    // Word 2: bits 64-95
    dst[7] = (src[2] >> 6) & 0x3FF;
    dst[8] = (src[2] >> 16) & 0x3FF;

    // Value 9 spans words 2-3
    dst[9] = ((src[2] >> 26) & 0x3F) | ((src[3] & 0xF) << 6);

    // Word 3: bits 96-127
    dst[10] = (src[3] >> 4) & 0x3FF;
    dst[11] = (src[3] >> 14) & 0x3FF;

    // Value 12 spans words 3-4
    dst[12] = ((src[3] >> 24) & 0xFF) | ((src[4] & 0x3) << 8);

    // Word 4: bits 128-159
    dst[13] = (src[4] >> 2) & 0x3FF;
    dst[14] = (src[4] >> 12) & 0x3FF;
    dst[15] = (src[4] >> 22) & 0x3FF;

    // Word 5: bits 160-191
    dst[16] = (src[5] >> 0) & 0x3FF;
    dst[17] = (src[5] >> 10) & 0x3FF;
    dst[18] = (src[5] >> 20) & 0x3FF;

    // Value 19 spans words 5-6
    dst[19] = ((src[5] >> 30) & 0x3) | ((src[6] & 0xFF) << 2);

    // Word 6: bits 192-223
    dst[20] = (src[6] >> 8) & 0x3FF;
    dst[21] = (src[6] >> 18) & 0x3FF;

    // Value 22 spans words 6-7
    dst[22] = ((src[6] >> 28) & 0xF) | ((src[7] & 0x3F) << 4);

    // Word 7: bits 224-255
    dst[23] = (src[7] >> 6) & 0x3FF;
    dst[24] = (src[7] >> 16) & 0x3FF;

    // Value 25 spans words 7-8
    dst[25] = ((src[7] >> 26) & 0x3F) | ((src[8] & 0xF) << 6);

    // Word 8: bits 256-287
    dst[26] = (src[8] >> 4) & 0x3FF;
    dst[27] = (src[8] >> 14) & 0x3FF;

    // Value 28 spans words 8-9
    dst[28] = ((src[8] >> 24) & 0xFF) | ((src[9] & 0x3) << 8);

    // Word 9: bits 288-319
    dst[29] = (src[9] >> 2) & 0x3FF;
    dst[30] = (src[9] >> 12) & 0x3FF;
    dst[31] = (src[9] >> 22) & 0x3FF;
}

// 32 values * 10 bits = 320 bits = 10 words
pub fn compress_10_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x3FF) << 0
        | (src[1] & 0x3FF) << 10
        | (src[2] & 0x3FF) << 20
        | (src[3] & 0x3) << 30; // bits 30-31 (lower 2 bits of value 3)

    // Word 1: bits 32-63
    dst[1] = ((src[3] >> 2) & 0xFF) << 0  // bits 0-7 (upper 8 bits of value 3)
        | (src[4] & 0x3FF) << 8
        | (src[5] & 0x3FF) << 18
        | (src[6] & 0xF) << 28; // bits 28-31 (lower 4 bits of value 6)

    // Word 2: bits 64-95
    dst[2] = ((src[6] >> 4) & 0x3F) << 0  // bits 0-5 (upper 6 bits of value 6)
        | (src[7] & 0x3FF) << 6
        | (src[8] & 0x3FF) << 16
        | (src[9] & 0x3F) << 26; // bits 26-31 (lower 6 bits of value 9)

    // Word 3: bits 96-127
    dst[3] = ((src[9] >> 6) & 0xF) << 0   // bits 0-3 (upper 4 bits of value 9)
        | (src[10] & 0x3FF) << 4
        | (src[11] & 0x3FF) << 14
        | (src[12] & 0xFF) << 24; // bits 24-31 (lower 8 bits of value 12)

    // Word 4: bits 128-159
    dst[4] = ((src[12] >> 8) & 0x3) << 0  // bits 0-1 (upper 2 bits of value 12)
        | (src[13] & 0x3FF) << 2
        | (src[14] & 0x3FF) << 12
        | (src[15] & 0x3FF) << 22;

    // Word 5: bits 160-191
    dst[5] = (src[16] & 0x3FF) << 0
        | (src[17] & 0x3FF) << 10
        | (src[18] & 0x3FF) << 20
        | (src[19] & 0x3) << 30; // bits 30-31 (lower 2 bits of value 19)

    // Word 6: bits 192-223
    dst[6] = ((src[19] >> 2) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 19)
        | (src[20] & 0x3FF) << 8
        | (src[21] & 0x3FF) << 18
        | (src[22] & 0xF) << 28; // bits 28-31 (lower 4 bits of value 22)

    // Word 7: bits 224-255
    dst[7] = ((src[22] >> 4) & 0x3F) << 0 // bits 0-5 (upper 6 bits of value 22)
        | (src[23] & 0x3FF) << 6
        | (src[24] & 0x3FF) << 16
        | (src[25] & 0x3F) << 26; // bits 26-31 (lower 6 bits of value 25)

    // Word 8: bits 256-287
    dst[8] = ((src[25] >> 6) & 0xF) << 0  // bits 0-3 (upper 4 bits of value 25)
        | (src[26] & 0x3FF) << 4
        | (src[27] & 0x3FF) << 14
        | (src[28] & 0xFF) << 24; // bits 24-31 (lower 8 bits of value 28)

    // Word 9: bits 288-319
    dst[9] = ((src[28] >> 8) & 0x3) << 0  // bits 0-1 (upper 2 bits of value 28)
        | (src[29] & 0x3FF) << 2
        | (src[30] & 0x3FF) << 12
        | (src[31] & 0x3FF) << 22;
}

// 32 values * 11 bits = 352 bits = 11 words
pub fn decompress_11_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] >> 0) & 0x7FF;
    dst[1] = (src[0] >> 11) & 0x7FF;

    // Value 2 spans words 0-1
    dst[2] = ((src[0] >> 22) & 0x3FF) | ((src[1] & 0x1) << 10);

    // Word 1: bits 32-63
    dst[3] = (src[1] >> 1) & 0x7FF;
    dst[4] = (src[1] >> 12) & 0x7FF;

    // Value 5 spans words 1-2
    dst[5] = ((src[1] >> 23) & 0x1FF) | ((src[2] & 0x3) << 9);

    // Word 2: bits 64-95
    dst[6] = (src[2] >> 2) & 0x7FF;
    dst[7] = (src[2] >> 13) & 0x7FF;

    // Value 8 spans words 2-3
    dst[8] = ((src[2] >> 24) & 0xFF) | ((src[3] & 0x7) << 8);

    // Word 3: bits 96-127
    dst[9] = (src[3] >> 3) & 0x7FF;
    dst[10] = (src[3] >> 14) & 0x7FF;

    // Value 11 spans words 3-4
    dst[11] = ((src[3] >> 25) & 0x7F) | ((src[4] & 0xF) << 7);

    // Word 4: bits 128-159
    dst[12] = (src[4] >> 4) & 0x7FF;
    dst[13] = (src[4] >> 15) & 0x7FF;

    // Value 14 spans words 4-5
    dst[14] = ((src[4] >> 26) & 0x3F) | ((src[5] & 0x1F) << 6);

    // Word 5: bits 160-191
    dst[15] = (src[5] >> 5) & 0x7FF;
    dst[16] = (src[5] >> 16) & 0x7FF;

    // Value 17 spans words 5-6
    dst[17] = ((src[5] >> 27) & 0x1F) | ((src[6] & 0x3F) << 5);

    // Word 6: bits 192-223
    dst[18] = (src[6] >> 6) & 0x7FF;
    dst[19] = (src[6] >> 17) & 0x7FF;

    // Value 20 spans words 6-7
    dst[20] = ((src[6] >> 28) & 0xF) | ((src[7] & 0x7F) << 4);

    // Word 7: bits 224-255
    dst[21] = (src[7] >> 7) & 0x7FF;
    dst[22] = (src[7] >> 18) & 0x7FF;

    // Value 23 spans words 7-8
    dst[23] = ((src[7] >> 29) & 0x7) | ((src[8] & 0xFF) << 3);

    // Word 8: bits 256-287
    dst[24] = (src[8] >> 8) & 0x7FF;
    dst[25] = (src[8] >> 19) & 0x7FF;

    // Value 26 spans words 8-9
    dst[26] = ((src[8] >> 30) & 0x3) | ((src[9] & 0x1FF) << 2);

    // Word 9: bits 288-319
    dst[27] = (src[9] >> 9) & 0x7FF;
    dst[28] = (src[9] >> 20) & 0x7FF;

    // Value 29 spans words 9-10
    dst[29] = ((src[9] >> 31) & 0x1) | ((src[10] & 0x3FF) << 1);

    // Word 10: bits 320-351
    dst[30] = (src[10] >> 10) & 0x7FF;
    dst[31] = (src[10] >> 21) & 0x7FF;
}

// 32 values * 11 bits = 352 bits = 11 words
pub fn compress_11_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x7FF) << 0 | (src[1] & 0x7FF) << 11 | (src[2] & 0x3FF) << 22; // bits 22-31 (lower 10 bits of value 2)

    // Word 1: bits 32-63
    dst[1] = ((src[2] >> 10) & 0x1) << 0  // bit 0 (upper 1 bit of value 2)
        | (src[3] & 0x7FF) << 1
        | (src[4] & 0x7FF) << 12
        | (src[5] & 0x1FF) << 23; // bits 23-31 (lower 9 bits of value 5)

    // Word 2: bits 64-95
    dst[2] = ((src[5] >> 9) & 0x3) << 0   // bits 0-1 (upper 2 bits of value 5)
        | (src[6] & 0x7FF) << 2
        | (src[7] & 0x7FF) << 13
        | (src[8] & 0xFF) << 24; // bits 24-31 (lower 8 bits of value 8)

    // Word 3: bits 96-127
    dst[3] = ((src[8] >> 8) & 0x7) << 0   // bits 0-2 (upper 3 bits of value 8)
        | (src[9] & 0x7FF) << 3
        | (src[10] & 0x7FF) << 14
        | (src[11] & 0x7F) << 25; // bits 25-31 (lower 7 bits of value 11)

    // Word 4: bits 128-159
    dst[4] = ((src[11] >> 7) & 0xF) << 0  // bits 0-3 (upper 4 bits of value 11)
        | (src[12] & 0x7FF) << 4
        | (src[13] & 0x7FF) << 15
        | (src[14] & 0x3F) << 26; // bits 26-31 (lower 6 bits of value 14)

    // Word 5: bits 160-191
    dst[5] = ((src[14] >> 6) & 0x1F) << 0 // bits 0-4 (upper 5 bits of value 14)
        | (src[15] & 0x7FF) << 5
        | (src[16] & 0x7FF) << 16
        | (src[17] & 0x1F) << 27; // bits 27-31 (lower 5 bits of value 17)

    // Word 6: bits 192-223
    dst[6] = ((src[17] >> 5) & 0x3F) << 0 // bits 0-5 (upper 6 bits of value 17)
        | (src[18] & 0x7FF) << 6
        | (src[19] & 0x7FF) << 17
        | (src[20] & 0xF) << 28; // bits 28-31 (lower 4 bits of value 20)

    // Word 7: bits 224-255
    dst[7] = ((src[20] >> 4) & 0x7F) << 0 // bits 0-6 (upper 7 bits of value 20)
        | (src[21] & 0x7FF) << 7
        | (src[22] & 0x7FF) << 18
        | (src[23] & 0x7) << 29; // bits 29-31 (lower 3 bits of value 23)

    // Word 8: bits 256-287
    dst[8] = ((src[23] >> 3) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 23)
        | (src[24] & 0x7FF) << 8
        | (src[25] & 0x7FF) << 19
        | (src[26] & 0x3) << 30; // bits 30-31 (lower 2 bits of value 26)

    // Word 9: bits 288-319
    dst[9] = ((src[26] >> 2) & 0x1FF) << 0 // bits 0-8 (upper 9 bits of value 26)
        | (src[27] & 0x7FF) << 9
        | (src[28] & 0x7FF) << 20
        | (src[29] & 0x1) << 31; // bit 31 (lower 1 bit of value 29)

    // Word 10: bits 320-351
    dst[10] = ((src[29] >> 1) & 0x3FF) << 0 // bits 0-9 (upper 10 bits of value 29)
        | (src[30] & 0x7FF) << 10
        | (src[31] & 0x7FF) << 21;
}

// 32 values * 12 bits = 384 bits = 12 words
pub fn decompress_12_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] >> 0) & 0xFFF;
    dst[1] = (src[0] >> 12) & 0xFFF;

    // Value 2 spans words 0-1
    dst[2] = ((src[0] >> 24) & 0xFF) | ((src[1] & 0xF) << 8);

    // Word 1: bits 32-63
    dst[3] = (src[1] >> 4) & 0xFFF;
    dst[4] = (src[1] >> 16) & 0xFFF;

    // Value 5 spans words 1-2
    dst[5] = ((src[1] >> 28) & 0xF) | ((src[2] & 0xFF) << 4);

    // Word 2: bits 64-95
    dst[6] = (src[2] >> 8) & 0xFFF;
    dst[7] = (src[2] >> 20) & 0xFFF;

    // Word 3: bits 96-127
    dst[8] = (src[3] >> 0) & 0xFFF;
    dst[9] = (src[3] >> 12) & 0xFFF;

    // Value 10 spans words 3-4
    dst[10] = ((src[3] >> 24) & 0xFF) | ((src[4] & 0xF) << 8);

    // Word 4: bits 128-159
    dst[11] = (src[4] >> 4) & 0xFFF;
    dst[12] = (src[4] >> 16) & 0xFFF;

    // Value 13 spans words 4-5
    dst[13] = ((src[4] >> 28) & 0xF) | ((src[5] & 0xFF) << 4);

    // Word 5: bits 160-191
    dst[14] = (src[5] >> 8) & 0xFFF;
    dst[15] = (src[5] >> 20) & 0xFFF;

    // Word 6: bits 192-223
    dst[16] = (src[6] >> 0) & 0xFFF;
    dst[17] = (src[6] >> 12) & 0xFFF;

    // Value 18 spans words 6-7
    dst[18] = ((src[6] >> 24) & 0xFF) | ((src[7] & 0xF) << 8);

    // Word 7: bits 224-255
    dst[19] = (src[7] >> 4) & 0xFFF;
    dst[20] = (src[7] >> 16) & 0xFFF;

    // Value 21 spans words 7-8
    dst[21] = ((src[7] >> 28) & 0xF) | ((src[8] & 0xFF) << 4);

    // Word 8: bits 256-287
    dst[22] = (src[8] >> 8) & 0xFFF;
    dst[23] = (src[8] >> 20) & 0xFFF;

    // Word 9: bits 288-319
    dst[24] = (src[9] >> 0) & 0xFFF;
    dst[25] = (src[9] >> 12) & 0xFFF;

    // Value 26 spans words 9-10
    dst[26] = ((src[9] >> 24) & 0xFF) | ((src[10] & 0xF) << 8);

    // Word 10: bits 320-351
    dst[27] = (src[10] >> 4) & 0xFFF;
    dst[28] = (src[10] >> 16) & 0xFFF;

    // Value 29 spans words 10-11
    dst[29] = ((src[10] >> 28) & 0xF) | ((src[11] & 0xFF) << 4);

    // Word 11: bits 352-383
    dst[30] = (src[11] >> 8) & 0xFFF;
    dst[31] = (src[11] >> 20) & 0xFFF;
}

// 32 values * 12 bits = 384 bits = 12 words
pub fn compress_12_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0xFFF) << 0
        | (src[1] & 0xFFF) << 12
        | (src[2] & 0xFF) << 24;   // bits 24-31 (lower 8 bits of value 2)

    // Word 1: bits 32-63
    dst[1] = ((src[2] >> 8) & 0xF) << 0   // bits 0-3 (upper 4 bits of value 2)
        | (src[3] & 0xFFF) << 4
        | (src[4] & 0xFFF) << 16
        | (src[5] & 0xF) << 28;    // bits 28-31 (lower 4 bits of value 5)

    // Word 2: bits 64-95
    dst[2] = ((src[5] >> 4) & 0xFF) << 0  // bits 0-7 (upper 8 bits of value 5)
        | (src[6] & 0xFFF) << 8
        | (src[7] & 0xFFF) << 20;

    // Word 3: bits 96-127
    dst[3] = (src[8] & 0xFFF) << 0
        | (src[9] & 0xFFF) << 12
        | (src[10] & 0xFF) << 24;  // bits 24-31 (lower 8 bits of value 10)

    // Word 4: bits 128-159
    dst[4] = ((src[10] >> 8) & 0xF) << 0  // bits 0-3 (upper 4 bits of value 10)
        | (src[11] & 0xFFF) << 4
        | (src[12] & 0xFFF) << 16
        | (src[13] & 0xF) << 28;   // bits 28-31 (lower 4 bits of value 13)

    // Word 5: bits 160-191
    dst[5] = ((src[13] >> 4) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 13)
        | (src[14] & 0xFFF) << 8
        | (src[15] & 0xFFF) << 20;

    // Word 6: bits 192-223
    dst[6] = (src[16] & 0xFFF) << 0
        | (src[17] & 0xFFF) << 12
        | (src[18] & 0xFF) << 24;  // bits 24-31 (lower 8 bits of value 18)

    // Word 7: bits 224-255
    dst[7] = ((src[18] >> 8) & 0xF) << 0  // bits 0-3 (upper 4 bits of value 18)
        | (src[19] & 0xFFF) << 4
        | (src[20] & 0xFFF) << 16
        | (src[21] & 0xF) << 28;   // bits 28-31 (lower 4 bits of value 21)

    // Word 8: bits 256-287
    dst[8] = ((src[21] >> 4) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 21)
        | (src[22] & 0xFFF) << 8
        | (src[23] & 0xFFF) << 20;

    // Word 9: bits 288-319
    dst[9] = (src[24] & 0xFFF) << 0
        | (src[25] & 0xFFF) << 12
        | (src[26] & 0xFF) << 24;  // bits 24-31 (lower 8 bits of value 26)

    // Word 10: bits 320-351
    dst[10] = ((src[26] >> 8) & 0xF) << 0 // bits 0-3 (upper 4 bits of value 26)
        | (src[27] & 0xFFF) << 4
        | (src[28] & 0xFFF) << 16
        | (src[29] & 0xF) << 28;   // bits 28-31 (lower 4 bits of value 29)

    // Word 11: bits 352-383
    dst[11] = ((src[29] >> 4) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 29)
        | (src[30] & 0xFFF) << 8
        | (src[31] & 0xFFF) << 20;
}

pub fn decompress_13_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0]  = (src[0] >> 0)  & 0x1FFF;  // bits 0-12
    dst[1]  = (src[0] >> 13) & 0x1FFF;  // bits 13-25
    dst[2]  = ((src[0] >> 26) | (src[1] << 6)) & 0x1FFF;  // bits 26-38

    // Word 1: bits 32-63
    dst[3]  = (src[1] >> 7)  & 0x1FFF;  // bits 39-51
    dst[4]  = ((src[1] >> 20) | (src[2] << 12)) & 0x1FFF;  // bits 52-64

    // Word 2: bits 64-95
    dst[5]  = (src[2] >> 1)  & 0x1FFF;  // bits 65-77
    dst[6]  = (src[2] >> 14) & 0x1FFF;  // bits 78-90
    dst[7]  = ((src[2] >> 27) | (src[3] << 5)) & 0x1FFF;  // bits 91-103

    // Word 3: bits 96-127
    dst[8]  = (src[3] >> 8)  & 0x1FFF;  // bits 104-116
    dst[9]  = ((src[3] >> 21) | (src[4] << 11)) & 0x1FFF;  // bits 117-129

    // Word 4: bits 128-159
    dst[10] = (src[4] >> 2)  & 0x1FFF;  // bits 130-142
    dst[11] = (src[4] >> 15) & 0x1FFF;  // bits 143-155
    dst[12] = ((src[4] >> 28) | (src[5] << 4)) & 0x1FFF;  // bits 156-168

    // Word 5: bits 160-191
    dst[13] = (src[5] >> 9)  & 0x1FFF;  // bits 169-181
    dst[14] = ((src[5] >> 22) | (src[6] << 10)) & 0x1FFF;  // bits 182-194

    // Word 6: bits 192-223
    dst[15] = (src[6] >> 3)  & 0x1FFF;  // bits 195-207
    dst[16] = (src[6] >> 16) & 0x1FFF;  // bits 208-220
    dst[17] = ((src[6] >> 29) | (src[7] << 3)) & 0x1FFF;  // bits 221-233

    // Word 7: bits 224-255
    dst[18] = (src[7] >> 10) & 0x1FFF;  // bits 234-246
    dst[19] = ((src[7] >> 23) | (src[8] << 9)) & 0x1FFF;  // bits 247-259

    // Word 8: bits 256-287
    dst[20] = (src[8] >> 4)  & 0x1FFF;  // bits 260-272
    dst[21] = (src[8] >> 17) & 0x1FFF;  // bits 273-285
    dst[22] = ((src[8] >> 30) | (src[9] << 2)) & 0x1FFF;  // bits 286-298

    // Word 9: bits 288-319
    dst[23] = (src[9] >> 11) & 0x1FFF;  // bits 299-311
    dst[24] = ((src[9] >> 24) | (src[10] << 8)) & 0x1FFF;  // bits 312-324

    // Word 10: bits 320-351
    dst[25] = (src[10] >> 5)  & 0x1FFF;  // bits 325-337
    dst[26] = (src[10] >> 18) & 0x1FFF;  // bits 338-350
    dst[27] = ((src[10] >> 31) | (src[11] << 1)) & 0x1FFF;  // bits 351-363

    // Word 11: bits 352-383
    dst[28] = (src[11] >> 12) & 0x1FFF;  // bits 364-376
    dst[29] = ((src[11] >> 25) | (src[12] << 7)) & 0x1FFF;  // bits 377-389

    // Word 12: bits 384-415
    dst[30] = (src[12] >> 6)  & 0x1FFF;  // bits 390-402
    dst[31] = (src[12] >> 19) & 0x1FFF;  // bits 403-415
}

// 32 values * 13 bits = 416 bits = 13 words
pub fn compress_13_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x1FFF) << 0   // bits 0-12
        | (src[1] & 0x1FFF) << 13     // bits 13-25
        | (src[2] & 0x3F) << 26;      // bits 26-31 (lower 6 bits of value 2)

    // Word 1: bits 32-63
    dst[1] = ((src[2] >> 6) & 0x7F) << 0   // bits 0-6 (upper 7 bits of value 2)
        | (src[3] & 0x1FFF) << 7           // bits 7-19
        | (src[4] & 0xFFF) << 20;          // bits 20-31 (lower 12 bits of value 4)

    // Word 2: bits 64-95
    dst[2] = ((src[4] >> 12) & 0x1) << 0   // bit 0 (upper 1 bit of value 4)
        | (src[5] & 0x1FFF) << 1           // bits 1-13
        | (src[6] & 0x1FFF) << 14          // bits 14-26
        | (src[7] & 0x1F) << 27;           // bits 27-31 (lower 5 bits of value 7)

    // Word 3: bits 96-127
    dst[3] = ((src[7] >> 5) & 0xFF) << 0   // bits 0-7 (upper 8 bits of value 7)
        | (src[8] & 0x1FFF) << 8           // bits 8-20
        | (src[9] & 0x7FF) << 21;          // bits 21-31 (lower 11 bits of value 9)

    // Word 4: bits 128-159
    dst[4] = ((src[9] >> 11) & 0x3) << 0   // bits 0-1 (upper 2 bits of value 9)
        | (src[10] & 0x1FFF) << 2          // bits 2-14
        | (src[11] & 0x1FFF) << 15         // bits 15-27
        | (src[12] & 0xF) << 28;           // bits 28-31 (lower 4 bits of value 12)

    // Word 5: bits 160-191
    dst[5] = ((src[12] >> 4) & 0x1FF) << 0  // bits 0-8 (upper 9 bits of value 12)
        | (src[13] & 0x1FFF) << 9           // bits 9-21
        | (src[14] & 0x3FF) << 22;          // bits 22-31 (lower 10 bits of value 14)

    // Word 6: bits 192-223
    dst[6] = ((src[14] >> 10) & 0x7) << 0   // bits 0-2 (upper 3 bits of value 14)
        | (src[15] & 0x1FFF) << 3           // bits 3-15
        | (src[16] & 0x1FFF) << 16          // bits 16-28
        | (src[17] & 0x7) << 29;            // bits 29-31 (lower 3 bits of value 17)

    // Word 7: bits 224-255
    dst[7] = ((src[17] >> 3) & 0x3FF) << 0  // bits 0-9 (upper 10 bits of value 17)
        | (src[18] & 0x1FFF) << 10          // bits 10-22
        | (src[19] & 0x1FF) << 23;          // bits 23-31 (lower 9 bits of value 19)

    // Word 8: bits 256-287
    dst[8] = ((src[19] >> 9) & 0xF) << 0    // bits 0-3 (upper 4 bits of value 19)
        | (src[20] & 0x1FFF) << 4           // bits 4-16
        | (src[21] & 0x1FFF) << 17          // bits 17-29
        | (src[22] & 0x3) << 30;            // bits 30-31 (lower 2 bits of value 22)

    // Word 9: bits 288-319
    dst[9] = ((src[22] >> 2) & 0x7FF) << 0  // bits 0-10 (upper 11 bits of value 22)
        | (src[23] & 0x1FFF) << 11          // bits 11-23
        | (src[24] & 0xFF) << 24;           // bits 24-31 (lower 8 bits of value 24)

    // Word 10: bits 320-351
    dst[10] = ((src[24] >> 8) & 0x1F) << 0  // bits 0-4 (upper 5 bits of value 24)
        | (src[25] & 0x1FFF) << 5           // bits 5-17
        | (src[26] & 0x1FFF) << 18          // bits 18-30
        | (src[27] & 0x1) << 31;            // bit 31 (lower 1 bit of value 27)

    // Word 11: bits 352-383
    dst[11] = ((src[27] >> 1) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 27)
        | (src[28] & 0x1FFF) << 12          // bits 12-24
        | (src[29] & 0x7F) << 25;           // bits 25-31 (lower 7 bits of value 29)

    // Word 12: bits 384-415
    dst[12] = ((src[29] >> 7) & 0x3F) << 0  // bits 0-5 (upper 6 bits of value 29)
        | (src[30] & 0x1FFF) << 6           // bits 6-18
        | (src[31] & 0x1FFF) << 19;         // bits 19-31
}

pub fn decompress_14_bit(src: &[u32], dst: &mut [u32]) {
    dst[0]  =  (src[0] >> 0)  & 0x3FFF;
    dst[1]  =  (src[0] >> 14) & 0x3FFF;
    dst[2]  = ((src[0] >> 28) | (src[1] << 4)) & 0x3FFF;

    dst[3]  =  (src[1] >> 10) & 0x3FFF;
    dst[4]  = ((src[1] >> 24) | (src[2] << 8)) & 0x3FFF;

    dst[5]  =  (src[2] >> 6)  & 0x3FFF;
    dst[6]  = ((src[2] >> 20) | (src[3] << 12)) & 0x3FFF;

    dst[7]  =  (src[3] >> 2)  & 0x3FFF;
    dst[8]  =  (src[3] >> 16) & 0x3FFF;
    dst[9]  = ((src[3] >> 30) | (src[4] << 2)) & 0x3FFF;

    dst[10] =  (src[4] >> 12) & 0x3FFF;
    dst[11] = ((src[4] >> 26) | (src[5] << 6)) & 0x3FFF;

    dst[12] =  (src[5] >> 8)  & 0x3FFF;
    dst[13] = ((src[5] >> 22) | (src[6] << 10)) & 0x3FFF;

    dst[14] =  (src[6] >> 4)  & 0x3FFF;
    dst[15] = (src[6] >> 18) & 0x3FFF;

    dst[16] =  (src[7] >> 0)  & 0x3FFF;
    dst[17] =  (src[7] >> 14) & 0x3FFF;
    dst[18] = ((src[7] >> 28) | (src[8] << 4)) & 0x3FFF;

    dst[19] =  (src[8] >> 10) & 0x3FFF;
    dst[20] = ((src[8] >> 24) | (src[9] << 8)) & 0x3FFF;

    dst[21] =  (src[9] >> 6)  & 0x3FFF;
    dst[22] = ((src[9] >> 20) | (src[10] << 12)) & 0x3FFF;

    dst[23] =  (src[10] >> 2) & 0x3FFF;
    dst[24] =  (src[10] >> 16) & 0x3FFF;
    dst[25] = ((src[10] >> 30) | (src[11] << 2)) & 0x3FFF;

    dst[26] =  (src[11] >> 12) & 0x3FFF;
    dst[27] = ((src[11] >> 26) | (src[12] << 6)) & 0x3FFF;

    dst[28] =  (src[12] >> 8)  & 0x3FFF;
    dst[29] = ((src[12] >> 22) | (src[13] << 10)) & 0x3FFF;

    dst[30] =  (src[13] >> 4)  & 0x3FFF;
    dst[31] =  (src[13] >> 18) & 0x3FFF;
}

pub fn compress_14_bit(src: &[u32], dst: &mut [u32]) {
    dst[0] = (src[0] & 0x3FFF)
        | ((src[1] & 0x3FFF) << 14)
        | ((src[2] & 0xF) << 28);

    dst[1] = ((src[2] >> 4) & 0x3FF)
        | ((src[3] & 0x3FFF) << 10)
        | ((src[4] & 0xFF) << 24);

    dst[2] = ((src[4] >> 8) & 0x3F)
        | ((src[5] & 0x3FFF) << 6)
        | ((src[6] & 0xFFF) << 20);

    dst[3] = ((src[6] >> 12) & 0x3)
        | ((src[7] & 0x3FFF) << 2)
        | ((src[8] & 0x3FFF) << 16)
        | ((src[9] & 0x3) << 30);

    dst[4] = ((src[9] >> 2) & 0xFFF)
        | ((src[10] & 0x3FFF) << 12)
        | ((src[11] & 0x3F) << 26);

    dst[5] = ((src[11] >> 6) & 0xFF)
        | ((src[12] & 0x3FFF) << 8)
        | ((src[13] & 0x3FF) << 22);

    dst[6] = ((src[13] >> 10) & 0xF)
        | ((src[14] & 0x3FFF) << 4)
        | ((src[15] & 0x3FFF) << 18);

    dst[7] = (src[16] & 0x3FFF)
        | ((src[17] & 0x3FFF) << 14)
        | ((src[18] & 0xF) << 28);

    dst[8] = ((src[18] >> 4) & 0x3FF)
        | ((src[19] & 0x3FFF) << 10)
        | ((src[20] & 0xFF) << 24);

    dst[9] = ((src[20] >> 8) & 0x3F)
        | ((src[21] & 0x3FFF) << 6)
        | ((src[22] & 0xFFF) << 20);

    dst[10] = ((src[22] >> 12) & 0x3)
        | ((src[23] & 0x3FFF) << 2)
        | ((src[24] & 0x3FFF) << 16)
        | ((src[25] & 0x3) << 30);

    dst[11] = ((src[25] >> 2) & 0xFFF)
        | ((src[26] & 0x3FFF) << 12)
        | ((src[27] & 0x3F) << 26);

    dst[12] = ((src[27] >> 6) & 0xFF)
        | ((src[28] & 0x3FFF) << 8)
        | ((src[29] & 0x3FF) << 22);

    dst[13] = ((src[29] >> 10) & 0xF)
        | ((src[30] & 0x3FFF) << 4)
        | ((src[31] & 0x3FFF) << 18);
}

// 32 values * 15 bits = 480 bits = 15 u32 words
pub fn decompress_15_bit(src: &[u32], dst: &mut [u32]) {
    dst[0]  = (src[0] >> 0)  & 0x7FFF;
    dst[1]  = (src[0] >> 15) & 0x7FFF;
    dst[2]  = ((src[0] >> 30) | (src[1] << 2)) & 0x7FFF;

    dst[3]  = (src[1] >> 13) & 0x7FFF;
    dst[4]  = ((src[1] >> 28) | (src[2] << 4)) & 0x7FFF;

    dst[5]  = (src[2] >> 11) & 0x7FFF;
    dst[6]  = ((src[2] >> 26) | (src[3] << 6)) & 0x7FFF;

    dst[7]  = (src[3] >> 9)  & 0x7FFF;
    dst[8]  = ((src[3] >> 24) | (src[4] << 8)) & 0x7FFF;

    dst[9]  = (src[4] >> 7)  & 0x7FFF;
    dst[10] = ((src[4] >> 22) | (src[5] << 10)) & 0x7FFF;

    dst[11] = (src[5] >> 5)  & 0x7FFF;
    dst[12] = ((src[5] >> 20) | (src[6] << 12)) & 0x7FFF;

    dst[13] = (src[6] >> 3)  & 0x7FFF;
    dst[14] = ((src[6] >> 18) | (src[7] << 14)) & 0x7FFF;

    dst[15] = (src[7] >> 1)  & 0x7FFF;
    dst[16] = (src[7] >> 16) & 0x7FFF; // note shifting into higher bits is fine
    dst[17] = ((src[7] >> 31) | (src[8] << 1)) & 0x7FFF;

    dst[18] = (src[8] >> 14) & 0x7FFF;
    dst[19] = ((src[8] >> 29) | (src[9] << 3)) & 0x7FFF;

    dst[20] = (src[9] >> 12) & 0x7FFF;
    dst[21] = ((src[9] >> 27) | (src[10] << 5)) & 0x7FFF;

    dst[22] = (src[10] >> 10) & 0x7FFF;
    dst[23] = ((src[10] >> 25) | (src[11] << 7)) & 0x7FFF;

    dst[24] = (src[11] >> 8)  & 0x7FFF;
    dst[25] = ((src[11] >> 23) | (src[12] << 9)) & 0x7FFF;

    dst[26] = (src[12] >> 6)  & 0x7FFF;
    dst[27] = ((src[12] >> 21) | (src[13] << 11)) & 0x7FFF;

    dst[28] = (src[13] >> 4)  & 0x7FFF;
    dst[29] = ((src[13] >> 19) | (src[14] << 13)) & 0x7FFF;

    dst[30] = (src[14] >> 2) & 0x7FFF;
    dst[31] = (src[14] >> 17) & 0x7FFF;
}

// 32 values * 15 bits = 480 bits = 15 u32 words
pub fn compress_15_bit(src: &[u32], dst: &mut [u32]) {
    dst[0] = (src[0] & 0x7FFF)
        | ((src[1] & 0x7FFF) << 15)
        | ((src[2] & 0x3) << 30);

    dst[1] = ((src[2] >> 2) & 0x1FFF)
        | ((src[3] & 0x7FFF) << 13)
        | ((src[4] & 0xF) << 28);

    dst[2] = ((src[4] >> 4) & 0x7FF)
        | ((src[5] & 0x7FFF) << 11)
        | ((src[6] & 0x3F) << 26);

    dst[3] = ((src[6] >> 6) & 0x1FF)
        | ((src[7] & 0x7FFF) << 9)
        | ((src[8] & 0xFF) << 24);

    dst[4] = ((src[8] >> 8) & 0x7F)
        | ((src[9] & 0x7FFF) << 7)
        | ((src[10] & 0x3FF) << 22);

    dst[5] = ((src[10] >> 10) & 0x1F)
        | ((src[11] & 0x7FFF) << 5)
        | ((src[12] & 0xFFF) << 20);

    dst[6] = ((src[12] >> 12) & 0x7)
        | ((src[13] & 0x7FFF) << 3)
        | ((src[14] & 0x3FFF) << 18);

    dst[7] = ((src[14] >> 14) & 0x1)
        | ((src[15] & 0x7FFF) << 1)
        | ((src[16] & 0x7FFF) << 16)
        | ((src[17] & 0x1) << 31);

    dst[8] = ((src[17] >> 1) & 0x3FFF)
        | ((src[18] & 0x7FFF) << 14)
        | ((src[19] & 0x7) << 29);

    dst[9] = ((src[19] >> 3) & 0xFFF)
        | ((src[20] & 0x7FFF) << 12)
        | ((src[21] & 0x1F) << 27);

    dst[10] = ((src[21] >> 5) & 0x3FF)
        | ((src[22] & 0x7FFF) << 10)
        | ((src[23] & 0x7F) << 25);

    dst[11] = ((src[23] >> 7) & 0xFF)
        | ((src[24] & 0x7FFF) << 8)
        | ((src[25] & 0x1FF) << 23);

    dst[12] = ((src[25] >> 9) & 0x3F)
        | ((src[26] & 0x7FFF) << 6)
        | ((src[27] & 0x7FF) << 21);

    dst[13] = ((src[27] >> 11) & 0xF)
        | ((src[28] & 0x7FFF) << 4)
        | ((src[29] & 0x1FFF) << 19);

    dst[14] = ((src[29] >> 13) & 0x3)
        | ((src[30] & 0x7FFF) << 2)
        | ((src[31] & 0x7FFF) << 17);
}

// 32 values * 16 bits = 512 bits = 16 words
pub fn decompress_16_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] >> 0) & 0xFFFF;
    dst[1] = (src[0] >> 16) & 0xFFFF;

    // Word 1: bits 32-63
    dst[2] = (src[1] >> 0) & 0xFFFF;
    dst[3] = (src[1] >> 16) & 0xFFFF;

    // Word 2: bits 64-95
    dst[4] = (src[2] >> 0) & 0xFFFF;
    dst[5] = (src[2] >> 16) & 0xFFFF;

    // Word 3: bits 96-127
    dst[6] = (src[3] >> 0) & 0xFFFF;
    dst[7] = (src[3] >> 16) & 0xFFFF;

    // Word 4: bits 128-159
    dst[8] = (src[4] >> 0) & 0xFFFF;
    dst[9] = (src[4] >> 16) & 0xFFFF;

    // Word 5: bits 160-191
    dst[10] = (src[5] >> 0) & 0xFFFF;
    dst[11] = (src[5] >> 16) & 0xFFFF;

    // Word 6: bits 192-223
    dst[12] = (src[6] >> 0) & 0xFFFF;
    dst[13] = (src[6] >> 16) & 0xFFFF;

    // Word 7: bits 224-255
    dst[14] = (src[7] >> 0) & 0xFFFF;
    dst[15] = (src[7] >> 16) & 0xFFFF;

    // Word 8: bits 256-287
    dst[16] = (src[8] >> 0) & 0xFFFF;
    dst[17] = (src[8] >> 16) & 0xFFFF;

    // Word 9: bits 288-319
    dst[18] = (src[9] >> 0) & 0xFFFF;
    dst[19] = (src[9] >> 16) & 0xFFFF;

    // Word 10: bits 320-351
    dst[20] = (src[10] >> 0) & 0xFFFF;
    dst[21] = (src[10] >> 16) & 0xFFFF;

    // Word 11: bits 352-383
    dst[22] = (src[11] >> 0) & 0xFFFF;
    dst[23] = (src[11] >> 16) & 0xFFFF;

    // Word 12: bits 384-415
    dst[24] = (src[12] >> 0) & 0xFFFF;
    dst[25] = (src[12] >> 16) & 0xFFFF;

    // Word 13: bits 416-447
    dst[26] = (src[13] >> 0) & 0xFFFF;
    dst[27] = (src[13] >> 16) & 0xFFFF;

    // Word 14: bits 448-479
    dst[28] = (src[14] >> 0) & 0xFFFF;
    dst[29] = (src[14] >> 16) & 0xFFFF;

    // Word 15: bits 480-511
    dst[30] = (src[15] >> 0) & 0xFFFF;
    dst[31] = (src[15] >> 16) & 0xFFFF;
}

// 32 values * 16 bits = 512 bits = 16 words
pub fn compress_16_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0xFFFF) | ((src[1] & 0xFFFF) << 16);

    // Word 1: bits 32-63
    dst[1] = (src[2] & 0xFFFF) | ((src[3] & 0xFFFF) << 16);

    // Word 2: bits 64-95
    dst[2] = (src[4] & 0xFFFF) | ((src[5] & 0xFFFF) << 16);

    // Word 3: bits 96-127
    dst[3] = (src[6] & 0xFFFF) | ((src[7] & 0xFFFF) << 16);

    // Word 4: bits 128-159
    dst[4] = (src[8] & 0xFFFF) | ((src[9] & 0xFFFF) << 16);

    // Word 5: bits 160-191
    dst[5] = (src[10] & 0xFFFF) | ((src[11] & 0xFFFF) << 16);

    // Word 6: bits 192-223
    dst[6] = (src[12] & 0xFFFF) | ((src[13] & 0xFFFF) << 16);

    // Word 7: bits 224-255
    dst[7] = (src[14] & 0xFFFF) | ((src[15] & 0xFFFF) << 16);

    // Word 8: bits 256-287
    dst[8] = (src[16] & 0xFFFF) | ((src[17] & 0xFFFF) << 16);

    // Word 9: bits 288-319
    dst[9] = (src[18] & 0xFFFF) | ((src[19] & 0xFFFF) << 16);

    // Word 10: bits 320-351
    dst[10] = (src[20] & 0xFFFF) | ((src[21] & 0xFFFF) << 16);

    // Word 11: bits 352-383
    dst[11] = (src[22] & 0xFFFF) | ((src[23] & 0xFFFF) << 16);

    // Word 12: bits 384-415
    dst[12] = (src[24] & 0xFFFF) | ((src[25] & 0xFFFF) << 16);

    // Word 13: bits 416-447
    dst[13] = (src[26] & 0xFFFF) | ((src[27] & 0xFFFF) << 16);

    // Word 14: bits 448-479
    dst[14] = (src[28] & 0xFFFF) | ((src[29] & 0xFFFF) << 16);

    // Word 15: bits 480-511
    dst[15] = (src[30] & 0xFFFF) | ((src[31] & 0xFFFF) << 16);
}


// 32 values * 17 bits = 544 bits = 17 words
pub fn decompress_17_bit(src: &[u32], dst: &mut [u32]) {
    dst[0]  = (src[0] >> 0)  & 0x1FFFF;  // bits 0-16
    dst[1]  = ((src[0] >> 17) | (src[1] << 15)) & 0x1FFFF;  // bits 17-33

    dst[2]  = (src[1] >> 2)  & 0x1FFFF;  // bits 34-50
    dst[3]  = ((src[1] >> 19) | (src[2] << 13)) & 0x1FFFF;  // bits 51-67

    dst[4]  = (src[2] >> 4)  & 0x1FFFF;  // bits 68-84
    dst[5]  = ((src[2] >> 21) | (src[3] << 11)) & 0x1FFFF;  // bits 85-101

    dst[6]  = (src[3] >> 6)  & 0x1FFFF;  // bits 102-118
    dst[7]  = ((src[3] >> 23) | (src[4] << 9)) & 0x1FFFF;  // bits 119-135

    dst[8]  = (src[4] >> 8)  & 0x1FFFF;  // bits 136-152
    dst[9]  = ((src[4] >> 25) | (src[5] << 7)) & 0x1FFFF;  // bits 153-169

    dst[10] = (src[5] >> 10) & 0x1FFFF;  // bits 170-186
    dst[11] = ((src[5] >> 27) | (src[6] << 5)) & 0x1FFFF;  // bits 187-203

    dst[12] = (src[6] >> 12) & 0x1FFFF;  // bits 204-220
    dst[13] = ((src[6] >> 29) | (src[7] << 3)) & 0x1FFFF;  // bits 221-237

    dst[14] = (src[7] >> 14) & 0x1FFFF;  // bits 238-254
    dst[15] = ((src[7] >> 31) | (src[8] << 1)) & 0x1FFFF;  // bits 255-271

    dst[16] = ((src[8] >> 16) | (src[9] << 16)) & 0x1FFFF;  // bits 272-288

    dst[17] = (src[9] >> 1)  & 0x1FFFF;  // bits 289-305
    dst[18] = ((src[9] >> 18) | (src[10] << 14)) & 0x1FFFF;  // bits 306-322

    dst[19] = (src[10] >> 3) & 0x1FFFF;  // bits 323-339
    dst[20] = ((src[10] >> 20) | (src[11] << 12)) & 0x1FFFF;  // bits 340-356

    dst[21] = (src[11] >> 5)  & 0x1FFFF;  // bits 357-373
    dst[22] = ((src[11] >> 22) | (src[12] << 10)) & 0x1FFFF;  // bits 374-390

    dst[23] = (src[12] >> 7)  & 0x1FFFF;  // bits 391-407
    dst[24] = ((src[12] >> 24) | (src[13] << 8)) & 0x1FFFF;  // bits 408-424

    dst[25] = (src[13] >> 9)  & 0x1FFFF;  // bits 425-441
    dst[26] = ((src[13] >> 26) | (src[14] << 6)) & 0x1FFFF;  // bits 442-458

    dst[27] = (src[14] >> 11) & 0x1FFFF;  // bits 459-475
    dst[28] = ((src[14] >> 28) | (src[15] << 4)) & 0x1FFFF;  // bits 476-492

    dst[29] = (src[15] >> 13) & 0x1FFFF;  // bits 493-509
    dst[30] = ((src[15] >> 30) | (src[16] << 2)) & 0x1FFFF;  // bits 510-526

    dst[31] = (src[16] >> 15) & 0x1FFFF;  // bits 527-543
}

// 32 values * 17 bits = 544 bits = 17 words
pub fn compress_17_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x1FFFF) << 0   // bits 0-16
        | (src[1] & 0x7FFF) << 17;     // bits 17-31 (lower 15 bits of value 1)

    // Word 1: bits 32-63
    dst[1] = ((src[1] >> 15) & 0x3) << 0   // bits 0-1 (upper 2 bits of value 1)
        | (src[2] & 0x1FFFF) << 2          // bits 2-18
        | (src[3] & 0x1FFF) << 19;         // bits 19-31 (lower 13 bits of value 3)

    // Word 2: bits 64-95
    dst[2] = ((src[3] >> 13) & 0xF) << 0   // bits 0-3 (upper 4 bits of value 3)
        | (src[4] & 0x1FFFF) << 4          // bits 4-20
        | (src[5] & 0x7FF) << 21;          // bits 21-31 (lower 11 bits of value 5)

    // Word 3: bits 96-127
    dst[3] = ((src[5] >> 11) & 0x3F) << 0  // bits 0-5 (upper 6 bits of value 5)
        | (src[6] & 0x1FFFF) << 6          // bits 6-22
        | (src[7] & 0x1FF) << 23;          // bits 23-31 (lower 9 bits of value 7)

    // Word 4: bits 128-159
    dst[4] = ((src[7] >> 9) & 0xFF) << 0   // bits 0-7 (upper 8 bits of value 7)
        | (src[8] & 0x1FFFF) << 8          // bits 8-24
        | (src[9] & 0x7F) << 25;           // bits 25-31 (lower 7 bits of value 9)

    // Word 5: bits 160-191
    dst[5] = ((src[9] >> 7) & 0x3FF) << 0  // bits 0-9 (upper 10 bits of value 9)
        | (src[10] & 0x1FFFF) << 10        // bits 10-26
        | (src[11] & 0x1F) << 27;          // bits 27-31 (lower 5 bits of value 11)

    // Word 6: bits 192-223
    dst[6] = ((src[11] >> 5) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 11)
        | (src[12] & 0x1FFFF) << 12        // bits 12-28
        | (src[13] & 0x7) << 29;           // bits 29-31 (lower 3 bits of value 13)

    // Word 7: bits 224-255
    dst[7] = ((src[13] >> 3) & 0x3FFF) << 0 // bits 0-13 (upper 14 bits of value 13)
        | (src[14] & 0x1FFFF) << 14        // bits 14-30
        | (src[15] & 0x1) << 31;           // bit 31 (lower 1 bit of value 15)

    // Word 8: bits 256-287
    dst[8] = ((src[15] >> 1) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 15)
        | (src[16] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 16)

    // Word 9: bits 288-319
    dst[9] = ((src[16] >> 16) & 0x1) << 0  // bit 0 (upper 1 bit of value 16)
        | (src[17] & 0x1FFFF) << 1         // bits 1-17
        | (src[18] & 0x3FFF) << 18;        // bits 18-31 (lower 14 bits of value 18)

    // Word 10: bits 320-351
    dst[10] = ((src[18] >> 14) & 0x7) << 0  // bits 0-2 (upper 3 bits of value 18)
        | (src[19] & 0x1FFFF) << 3          // bits 3-19
        | (src[20] & 0xFFF) << 20;          // bits 20-31 (lower 12 bits of value 20)

    // Word 11: bits 352-383
    dst[11] = ((src[20] >> 12) & 0x1F) << 0 // bits 0-4 (upper 5 bits of value 20)
        | (src[21] & 0x1FFFF) << 5          // bits 5-21
        | (src[22] & 0x3FF) << 22;          // bits 22-31 (lower 10 bits of value 22)

    // Word 12: bits 384-415
    dst[12] = ((src[22] >> 10) & 0x7F) << 0 // bits 0-6 (upper 7 bits of value 22)
        | (src[23] & 0x1FFFF) << 7          // bits 7-23
        | (src[24] & 0xFF) << 24;           // bits 24-31 (lower 8 bits of value 24)

    // Word 13: bits 416-447
    dst[13] = ((src[24] >> 8) & 0x1FF) << 0 // bits 0-8 (upper 9 bits of value 24)
        | (src[25] & 0x1FFFF) << 9          // bits 9-25
        | (src[26] & 0x3F) << 26;           // bits 26-31 (lower 6 bits of value 26)

    // Word 14: bits 448-479
    dst[14] = ((src[26] >> 6) & 0x7FF) << 0 // bits 0-10 (upper 11 bits of value 26)
        | (src[27] & 0x1FFFF) << 11         // bits 11-27
        | (src[28] & 0xF) << 28;            // bits 28-31 (lower 4 bits of value 28)

    // Word 15: bits 480-511
    dst[15] = ((src[28] >> 4) & 0x1FFF) << 0 // bits 0-12 (upper 13 bits of value 28)
        | (src[29] & 0x1FFFF) << 13         // bits 13-29
        | (src[30] & 0x3) << 30;            // bits 30-31 (lower 2 bits of value 30)

    // Word 16: bits 512-543
    dst[16] = ((src[30] >> 2) & 0x7FFF) << 0 // bits 0-14 (upper 15 bits of value 30)
        | (src[31] & 0x1FFFF) << 15;        // bits 15-31
}

// 32 values * 18 bits = 576 bits = 18 words
pub fn decompress_18_bit(src: &[u32], dst: &mut [u32]) {
    dst[0]  = (src[0] >> 0)  & 0x3FFFF;
    dst[1]  = ((src[0] >> 18) | (src[1] << 14)) & 0x3FFFF;

    dst[2]  = (src[1] >> 4)  & 0x3FFFF;
    dst[3]  = ((src[1] >> 22) | (src[2] << 10)) & 0x3FFFF;

    dst[4]  = (src[2] >> 8)  & 0x3FFFF;
    dst[5]  = ((src[2] >> 26) | (src[3] << 6)) & 0x3FFFF;

    dst[6]  = (src[3] >> 12) & 0x3FFFF;
    dst[7]  = ((src[3] >> 30) | (src[4] << 2)) & 0x3FFFF;

    dst[8]  = ((src[4] >> 16) | (src[5] << 16)) & 0x3FFFF;
    dst[9]  = (src[5] >> 2)  & 0x3FFFF;
    
    dst[10] = ((src[5] >> 20) | (src[6] << 12)) & 0x3FFFF;

    dst[11] = (src[6] >> 6)  & 0x3FFFF;
    dst[12] = ((src[6] >> 24) | (src[7] << 8)) & 0x3FFFF;

    dst[13] = (src[7] >> 10) & 0x3FFFF;
    dst[14] = ((src[7] >> 28) | (src[8] << 4)) & 0x3FFFF;

    dst[15] = (src[8] >> 14) & 0x3FFFF;

    dst[16] = (src[9] >> 0)  & 0x3FFFF;
    
    dst[17] = ((src[9] >> 18) | (src[10] << 14)) & 0x3FFFF;

    dst[18] = (src[10] >> 4)  & 0x3FFFF;
    dst[19] = ((src[10] >> 22) | (src[11] << 10)) & 0x3FFFF;

    dst[20] = (src[11] >> 8)  & 0x3FFFF;
    dst[21] = ((src[11] >> 26) | (src[12] << 6)) & 0x3FFFF;

    dst[22] = (src[12] >> 12) & 0x3FFFF;
    dst[23] = ((src[12] >> 30) | (src[13] << 2)) & 0x3FFFF;

    dst[24] = ((src[13] >> 16) | (src[14] << 16)) & 0x3FFFF;
    dst[25] = (src[14] >> 2)  & 0x3FFFF;
    
    dst[26] = ((src[14] >> 20) | (src[15] << 12)) & 0x3FFFF;

    dst[27] = (src[15] >> 6)  & 0x3FFFF;
    dst[28] = ((src[15] >> 24) | (src[16] << 8)) & 0x3FFFF;

    dst[29] = (src[16] >> 10) & 0x3FFFF;
    dst[30] = ((src[16] >> 28) | (src[17] << 4)) & 0x3FFFF;

    dst[31] = (src[17] >> 14) & 0x3FFFF;
}

// 32 values * 18 bits = 576 bits = 18 words
pub fn compress_18_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x3FFFF) << 0   // bits 0-17
        | (src[1] & 0x3FFF) << 18;     // bits 18-31 (lower 14 bits of value 1)

    // Word 1: bits 32-63
    dst[1] = ((src[1] >> 14) & 0xF) << 0   // bits 0-3 (upper 4 bits of value 1)
        | (src[2] & 0x3FFFF) << 4          // bits 4-21
        | (src[3] & 0x3FF) << 22;          // bits 22-31 (lower 10 bits of value 3)

    // Word 2: bits 64-95
    dst[2] = ((src[3] >> 10) & 0xFF) << 0  // bits 0-7 (upper 8 bits of value 3)
        | (src[4] & 0x3FFFF) << 8          // bits 8-25
        | (src[5] & 0x3F) << 26;           // bits 26-31 (lower 6 bits of value 5)

    // Word 3: bits 96-127
    dst[3] = ((src[5] >> 6) & 0xFFF) << 0  // bits 0-11 (upper 12 bits of value 5)
        | (src[6] & 0x3FFFF) << 12         // bits 12-29
        | (src[7] & 0x3) << 30;            // bits 30-31 (lower 2 bits of value 7)

    // Word 4: bits 128-159
    dst[4] = ((src[7] >> 2) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 7)
        | (src[8] & 0xFFFF) << 16;         // bits 16-31 (lower 16 bits of value 8)

    // Word 5: bits 160-191
    dst[5] = ((src[8] >> 16) & 0x3) << 0   // bits 0-1 (upper 2 bits of value 8)
        | (src[9] & 0x3FFFF) << 2          // bits 2-19
        | (src[10] & 0xFFF) << 20;         // bits 20-31 (lower 12 bits of value 10)

    // Word 6: bits 192-223
    dst[6] = ((src[10] >> 12) & 0x3F) << 0 // bits 0-5 (upper 6 bits of value 10)
        | (src[11] & 0x3FFFF) << 6         // bits 6-23
        | (src[12] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 12)

    // Word 7: bits 224-255
    dst[7] = ((src[12] >> 8) & 0x3FF) << 0 // bits 0-9 (upper 10 bits of value 12)
        | (src[13] & 0x3FFFF) << 10        // bits 10-27
        | (src[14] & 0xF) << 28;           // bits 28-31 (lower 4 bits of value 14)

    // Word 8: bits 256-287
    dst[8] = ((src[14] >> 4) & 0x3FFF) << 0 // bits 0-13 (upper 14 bits of value 14)
        | (src[15] & 0x3FFFF) << 14;       // bits 14-31

    // Word 9: bits 288-319
    dst[9] = (src[16] & 0x3FFFF) << 0      // bits 0-17
        | (src[17] & 0x3FFF) << 18;        // bits 18-31 (lower 14 bits of value 17)

    // Word 10: bits 320-351
    dst[10] = ((src[17] >> 14) & 0xF) << 0  // bits 0-3 (upper 4 bits of value 17)
        | (src[18] & 0x3FFFF) << 4          // bits 4-21
        | (src[19] & 0x3FF) << 22;          // bits 22-31 (lower 10 bits of value 19)

    // Word 11: bits 352-383
    dst[11] = ((src[19] >> 10) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 19)
        | (src[20] & 0x3FFFF) << 8          // bits 8-25
        | (src[21] & 0x3F) << 26;           // bits 26-31 (lower 6 bits of value 21)

    // Word 12: bits 384-415
    dst[12] = ((src[21] >> 6) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 21)
        | (src[22] & 0x3FFFF) << 12         // bits 12-29
        | (src[23] & 0x3) << 30;            // bits 30-31 (lower 2 bits of value 23)

    // Word 13: bits 416-447
    dst[13] = ((src[23] >> 2) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 23)
        | (src[24] & 0xFFFF) << 16;         // bits 16-31 (lower 16 bits of value 24)

    // Word 14: bits 448-479
    dst[14] = ((src[24] >> 16) & 0x3) << 0  // bits 0-1 (upper 2 bits of value 24)
        | (src[25] & 0x3FFFF) << 2          // bits 2-19
        | (src[26] & 0xFFF) << 20;          // bits 20-31 (lower 12 bits of value 26)

    // Word 15: bits 480-511
    dst[15] = ((src[26] >> 12) & 0x3F) << 0 // bits 0-5 (upper 6 bits of value 26)
        | (src[27] & 0x3FFFF) << 6          // bits 6-23
        | (src[28] & 0xFF) << 24;           // bits 24-31 (lower 8 bits of value 28)

    // Word 16: bits 512-543
    dst[16] = ((src[28] >> 8) & 0x3FF) << 0 // bits 0-9 (upper 10 bits of value 28)
        | (src[29] & 0x3FFFF) << 10         // bits 10-27
        | (src[30] & 0xF) << 28;            // bits 28-31 (lower 4 bits of value 30)

    // Word 17: bits 544-575
    dst[17] = ((src[30] >> 4) & 0x3FFF) << 0 // bits 0-13 (upper 14 bits of value 30)
        | (src[31] & 0x3FFFF) << 14;        // bits 14-31
}

// 32 values * 19 bits = 608 bits = 19 words
pub fn decompress_19_bit(src: &[u32], dst: &mut [u32]) {
    // dst[0]: bits 0-18 (word 0, pos 0-18)
    dst[0]  = (src[0] >> 0)  & 0x7FFFF;
    
    // dst[1]: bits 19-37 (word 0 pos 19-31, word 1 pos 0-5)
    dst[1]  = ((src[0] >> 19) | (src[1] << 13)) & 0x7FFFF;

    // dst[2]: bits 38-56 (word 1, pos 6-24)
    dst[2]  = (src[1] >> 6)  & 0x7FFFF;
    
    // dst[3]: bits 57-75 (word 1 pos 25-31, word 2 pos 0-11)
    dst[3]  = ((src[1] >> 25) | (src[2] << 7)) & 0x7FFFF;

    // dst[4]: bits 76-94 (word 2, pos 12-30)
    dst[4]  = (src[2] >> 12) & 0x7FFFF;
    
    // dst[5]: bits 95-113 (word 2 pos 31, word 3 pos 0-17)
    dst[5]  = ((src[2] >> 31) | (src[3] << 1)) & 0x7FFFF;

    // dst[6]: bits 114-132 (word 3 pos 18-31, word 4 pos 0-4)
    dst[6]  = ((src[3] >> 18) | (src[4] << 14)) & 0x7FFFF;

    // dst[7]: bits 133-151 (word 4, pos 5-23)
    dst[7]  = (src[4] >> 5)  & 0x7FFFF;
    
    // dst[8]: bits 152-170 (word 4 pos 24-31, word 5 pos 0-10)
    dst[8]  = ((src[4] >> 24) | (src[5] << 8)) & 0x7FFFF;

    // dst[9]: bits 171-189 (word 5, pos 11-29)
    dst[9]  = (src[5] >> 11) & 0x7FFFF;
    
    // dst[10]: bits 190-208 (word 5 pos 30-31, word 6 pos 0-16)
    dst[10] = ((src[5] >> 30) | (src[6] << 2)) & 0x7FFFF;

    // dst[11]: bits 209-227 (word 6, pos 17-31, word 7 pos 0-3)
    dst[11] = ((src[6] >> 17) | (src[7] << 15)) & 0x7FFFF;

    // dst[12]: bits 228-246 (word 7, pos 4-22)
    dst[12] = (src[7] >> 4)  & 0x7FFFF;

    // dst[13]: bits 247-265 (word 7 pos 23-31, word 8 pos 0-8)
    dst[13] = ((src[7] >> 23) | (src[8] << 9)) & 0x7FFFF;

    // dst[14]: bits 266-284 (word 8, pos 10-28)
    dst[14] = (src[8] >> 10) & 0x7FFFF;
    
    // dst[15]: bits 285-303 (word 8 pos 29-31, word 9 pos 0-15)
    dst[15] = ((src[8] >> 29) | (src[9] << 3)) & 0x7FFFF;

    // dst[16]: bits 304-322 (word 9, pos 16-31, word 10 pos 0-2)
    dst[16] = ((src[9] >> 16) | (src[10] << 16)) & 0x7FFFF;

    // dst[17]: bits 323-341 (word 10, pos 3-21)
    dst[17] = (src[10] >> 3)  & 0x7FFFF;

    // dst[18]: bits 342-360 (word 10 pos 22-31, word 11 pos 0-9)
    dst[18] = ((src[10] >> 22) | (src[11] << 10)) & 0x7FFFF;

    // dst[19]: bits 361-379 (word 11, pos 9-27)
    dst[19] = (src[11] >> 9)  & 0x7FFFF;
    
    // dst[20]: bits 380-398 (word 11 pos 28-31, word 12 pos 0-14)
    dst[20] = ((src[11] >> 28) | (src[12] << 4)) & 0x7FFFF;

    // dst[21]: bits 399-417 (word 12, pos 15-31, word 13 pos 0-1)
    dst[21] = ((src[12] >> 15) | (src[13] << 17)) & 0x7FFFF;

    // dst[22]: bits 418-436 (word 13, pos 2-20)
    dst[22] = (src[13] >> 2)  & 0x7FFFF;

    // dst[23]: bits 437-455 (word 13 pos 21-31, word 14 pos 0-7)
    dst[23] = ((src[13] >> 21) | (src[14] << 11)) & 0x7FFFF;

    // dst[24]: bits 456-474 (word 14, pos 8-26)
    dst[24] = (src[14] >> 8)  & 0x7FFFF;
    
    // dst[25]: bits 475-493 (word 14 pos 27-31, word 15 pos 0-13)
    dst[25] = ((src[14] >> 27) | (src[15] << 5)) & 0x7FFFF;

    // dst[26]: bits 494-512 (word 15, pos 14-31, word 16 pos 0)
    dst[26] = ((src[15] >> 14) | (src[16] << 18)) & 0x7FFFF;

    // dst[27]: bits 513-531 (word 16, pos 1-19)
    dst[27] = (src[16] >> 1)  & 0x7FFFF;

    // dst[28]: bits 532-550 (word 16 pos 20-31, word 17 pos 0-6)
    dst[28] = ((src[16] >> 20) | (src[17] << 12)) & 0x7FFFF;

    // dst[29]: bits 551-569 (word 17, pos 7-25)
    dst[29] = (src[17] >> 7)  & 0x7FFFF;
    
    // dst[30]: bits 570-588 (word 17 pos 26-31, word 18 pos 0-12)
    dst[30] = ((src[17] >> 26) | (src[18] << 6)) & 0x7FFFF;

    // dst[31]: bits 589-607 (word 18, pos 13-31)
    dst[31] = (src[18] >> 13) & 0x7FFFF;
}

// 32 values * 19 bits = 608 bits = 19 words
pub fn compress_19_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x7FFFF) << 0       // bits 0-18
        | (src[1] & 0x1FFF) << 19;         // bits 19-31 (lower 13 bits of value 1)

    // Word 1: bits 32-63
    dst[1] = ((src[1] >> 13) & 0x3F) << 0  // bits 0-5 (upper 6 bits of value 1)
        | (src[2] & 0x7FFFF) << 6          // bits 6-24
        | (src[3] & 0x7F) << 25;           // bits 25-31 (lower 7 bits of value 3)

    // Word 2: bits 64-95
    dst[2] = ((src[3] >> 7) & 0xFFF) << 0  // bits 0-11 (upper 12 bits of value 3)
        | (src[4] & 0x7FFFF) << 12         // bits 12-30
        | (src[5] & 0x1) << 31;            // bit 31 (lower 1 bit of value 5)

    // Word 3: bits 96-127
    dst[3] = ((src[5] >> 1) & 0x3FFFF) << 0 // bits 0-17 (upper 18 bits of value 5)
        | (src[6] & 0x3FFF) << 18;         // bits 18-31 (lower 14 bits of value 6)

    // Word 4: bits 128-159
    dst[4] = ((src[6] >> 14) & 0x1F) << 0  // bits 0-4 (upper 5 bits of value 6)
        | (src[7] & 0x7FFFF) << 5          // bits 5-23
        | (src[8] & 0xFF) << 24;           // bits 24-31 (lower 8 bits of value 8)

    // Word 5: bits 160-191
    dst[5] = ((src[8] >> 8) & 0x7FF) << 0  // bits 0-10 (upper 11 bits of value 8)
        | (src[9] & 0x7FFFF) << 11         // bits 11-29
        | (src[10] & 0x3) << 30;           // bits 30-31 (lower 2 bits of value 10)

    // Word 6: bits 192-223
    dst[6] = ((src[10] >> 2) & 0x1FFFF) << 0 // bits 0-16 (upper 17 bits of value 10)
        | (src[11] & 0x7FFF) << 17;        // bits 17-31 (lower 15 bits of value 11)

    // Word 7: bits 224-255
    dst[7] = ((src[11] >> 15) & 0xF) << 0  // bits 0-3 (upper 4 bits of value 11)
        | (src[12] & 0x7FFFF) << 4         // bits 4-22
        | (src[13] & 0x1FF) << 23;         // bits 23-31 (lower 9 bits of value 13)

    // Word 8: bits 256-287
    dst[8] = ((src[13] >> 9) & 0x3FF) << 0 // bits 0-9 (upper 10 bits of value 13)
        | (src[14] & 0x7FFFF) << 10        // bits 10-28
        | (src[15] & 0x7) << 29;           // bits 29-31 (lower 3 bits of value 15)

    // Word 9: bits 288-319
    dst[9] = ((src[15] >> 3) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 15)
        | (src[16] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 16)

    // Word 10: bits 320-351
    dst[10] = ((src[16] >> 16) & 0x7) << 0 // bits 0-2 (upper 3 bits of value 16)
        | (src[17] & 0x7FFFF) << 3         // bits 3-21
        | (src[18] & 0x3FF) << 22;         // bits 22-31 (lower 10 bits of value 18)

    // Word 11: bits 352-383
    dst[11] = ((src[18] >> 10) & 0x1FF) << 0 // bits 0-8 (upper 9 bits of value 18)
        | (src[19] & 0x7FFFF) << 9         // bits 9-27
        | (src[20] & 0xF) << 28;           // bits 28-31 (lower 4 bits of value 20)

    // Word 12: bits 384-415
    dst[12] = ((src[20] >> 4) & 0x7FFF) << 0 // bits 0-14 (upper 15 bits of value 20)
        | (src[21] & 0x1FFFF) << 15;       // bits 15-31 (lower 17 bits of value 21)

    // Word 13: bits 416-447
    dst[13] = ((src[21] >> 17) & 0x3) << 0 // bits 0-1 (upper 2 bits of value 21)
        | (src[22] & 0x7FFFF) << 2         // bits 2-20
        | (src[23] & 0x7FF) << 21;         // bits 21-31 (lower 11 bits of value 23)

    // Word 14: bits 448-479
    dst[14] = ((src[23] >> 11) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 23)
        | (src[24] & 0x7FFFF) << 8         // bits 8-26
        | (src[25] & 0x1F) << 27;          // bits 27-31 (lower 5 bits of value 25)

    // Word 15: bits 480-511
    dst[15] = ((src[25] >> 5) & 0x3FFF) << 0 // bits 0-13 (upper 14 bits of value 25)
        | (src[26] & 0x3FFFF) << 14;       // bits 14-31 (lower 18 bits of value 26)

    // Word 16: bits 512-543
    dst[16] = ((src[26] >> 18) & 0x1) << 0 // bit 0 (upper 1 bit of value 26)
        | (src[27] & 0x7FFFF) << 1         // bits 1-19
        | (src[28] & 0xFFF) << 20;         // bits 20-31 (lower 12 bits of value 28)

    // Word 17: bits 544-575
    dst[17] = ((src[28] >> 12) & 0x7F) << 0 // bits 0-6 (upper 7 bits of value 28)
        | (src[29] & 0x7FFFF) << 7         // bits 7-25
        | (src[30] & 0x3F) << 26;          // bits 26-31 (lower 6 bits of value 30)

    // Word 18: bits 576-607
    dst[18] = ((src[30] >> 6) & 0x1FFF) << 0 // bits 0-12 (upper 13 bits of value 30)
        | (src[31] & 0x7FFFF) << 13;       // bits 13-31
}

// 32 values * 20 bits = 640 bits = 20 words
pub fn decompress_20_bit(src: &[u32], dst: &mut [u32]) {
    // dst[0]: bits 0-19 (in src[0] bits 0-19)
    dst[0]  = (src[0] >> 0)  & 0xFFFFF;
    
    // dst[1]: bits 20-39 (spans src[0] bits 20-31 + src[1] bits 0-7)
    dst[1]  = ((src[0] >> 20) | (src[1] << 12)) & 0xFFFFF;

    // dst[2]: bits 40-59 (in src[1] bits 8-27)
    dst[2]  = (src[1] >> 8)  & 0xFFFFF;
    
    // dst[3]: bits 60-79 (spans src[1] bits 28-31 + src[2] bits 0-15)
    dst[3]  = ((src[1] >> 28) | (src[2] << 4)) & 0xFFFFF;

    // dst[4]: bits 80-99 (in src[2] bits 16-31 + src[3] bits 0-3)
    dst[4]  = ((src[2] >> 16) | (src[3] << 16)) & 0xFFFFF;
    
    // dst[5]: bits 100-119 (in src[3] bits 4-23)
    dst[5]  = (src[3] >> 4)  & 0xFFFFF;

    // dst[6]: bits 120-139 (spans src[3] bits 24-31 + src[4] bits 0-11)
    dst[6]  = ((src[3] >> 24) | (src[4] << 8)) & 0xFFFFF;
    
    // dst[7]: bits 140-159 (in src[4] bits 12-31)
    dst[7]  = (src[4] >> 12) & 0xFFFFF;

    // dst[8]: bits 160-179 (in src[5] bits 0-19)
    dst[8]  = (src[5] >> 0)  & 0xFFFFF;
    
    // dst[9]: bits 180-199 (spans src[5] bits 20-31 + src[6] bits 0-7)
    dst[9]  = ((src[5] >> 20) | (src[6] << 12)) & 0xFFFFF;

    // dst[10]: bits 200-219 (in src[6] bits 8-27)
    dst[10] = (src[6] >> 8)  & 0xFFFFF;
    
    // dst[11]: bits 220-239 (spans src[6] bits 28-31 + src[7] bits 0-15)
    dst[11] = ((src[6] >> 28) | (src[7] << 4)) & 0xFFFFF;

    // dst[12]: bits 240-259 (spans src[7] bits 16-31 + src[8] bits 0-3)
    dst[12] = ((src[7] >> 16) | (src[8] << 16)) & 0xFFFFF;
    
    // dst[13]: bits 260-279 (in src[8] bits 4-23)
    dst[13] = (src[8] >> 4)  & 0xFFFFF;

    // dst[14]: bits 280-299 (spans src[8] bits 24-31 + src[9] bits 0-11)
    dst[14] = ((src[8] >> 24) | (src[9] << 8)) & 0xFFFFF;
    
    // dst[15]: bits 300-319 (in src[9] bits 12-31)
    dst[15] = (src[9] >> 12) & 0xFFFFF;

    // dst[16]: bits 320-339 (in src[10] bits 0-19)
    dst[16] = (src[10] >> 0)  & 0xFFFFF;
    
    // dst[17]: bits 340-359 (spans src[10] bits 20-31 + src[11] bits 0-7)
    dst[17] = ((src[10] >> 20) | (src[11] << 12)) & 0xFFFFF;

    // dst[18]: bits 360-379 (in src[11] bits 8-27)
    dst[18] = (src[11] >> 8)  & 0xFFFFF;
    
    // dst[19]: bits 380-399 (spans src[11] bits 28-31 + src[12] bits 0-15)
    dst[19] = ((src[11] >> 28) | (src[12] << 4)) & 0xFFFFF;

    // dst[20]: bits 400-419 (spans src[12] bits 16-31 + src[13] bits 0-3)
    dst[20] = ((src[12] >> 16) | (src[13] << 16)) & 0xFFFFF;
    
    // dst[21]: bits 420-439 (in src[13] bits 4-23)
    dst[21] = (src[13] >> 4)  & 0xFFFFF;

    // dst[22]: bits 440-459 (spans src[13] bits 24-31 + src[14] bits 0-11)
    dst[22] = ((src[13] >> 24) | (src[14] << 8)) & 0xFFFFF;
    
    // dst[23]: bits 460-479 (in src[14] bits 12-31)
    dst[23] = (src[14] >> 12) & 0xFFFFF;

    // dst[24]: bits 480-499 (in src[15] bits 0-19)
    dst[24] = (src[15] >> 0)  & 0xFFFFF;
    
    // dst[25]: bits 500-519 (spans src[15] bits 20-31 + src[16] bits 0-7)
    dst[25] = ((src[15] >> 20) | (src[16] << 12)) & 0xFFFFF;

    // dst[26]: bits 520-539 (in src[16] bits 8-27)
    dst[26] = (src[16] >> 8)  & 0xFFFFF;
    
    // dst[27]: bits 540-559 (spans src[16] bits 28-31 + src[17] bits 0-15)
    dst[27] = ((src[16] >> 28) | (src[17] << 4)) & 0xFFFFF;

    // dst[28]: bits 560-579 (spans src[17] bits 16-31 + src[18] bits 0-3)
    dst[28] = ((src[17] >> 16) | (src[18] << 16)) & 0xFFFFF;
    
    // dst[29]: bits 580-599 (in src[18] bits 4-23)
    dst[29] = (src[18] >> 4)  & 0xFFFFF;

    // dst[30]: bits 600-619 (spans src[18] bits 24-31 + src[19] bits 0-11)
    dst[30] = ((src[18] >> 24) | (src[19] << 8)) & 0xFFFFF;
    
    // dst[31]: bits 620-639 (in src[19] bits 12-31)
    dst[31] = (src[19] >> 12) & 0xFFFFF;
}

// 32 values * 20 bits = 640 bits = 20 words
pub fn compress_20_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0xFFFFF) << 0       // bits 0-19
        | (src[1] & 0xFFF) << 20;          // bits 20-31 (lower 12 bits of value 1)

    // Word 1: bits 32-63
    dst[1] = ((src[1] >> 12) & 0xFF) << 0  // bits 0-7 (upper 8 bits of value 1)
        | (src[2] & 0xFFFFF) << 8          // bits 8-27
        | (src[3] & 0xF) << 28;            // bits 28-31 (lower 4 bits of value 3)

    // Word 2: bits 64-95
    dst[2] = ((src[3] >> 4) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 3)
        | (src[4] & 0xFFFF) << 16;         // bits 16-31 (lower 16 bits of value 4)

    // Word 3: bits 96-127
    dst[3] = ((src[4] >> 16) & 0xF) << 0   // bits 0-3 (upper 4 bits of value 4)
        | (src[5] & 0xFFFFF) << 4          // bits 4-23
        | (src[6] & 0xFF) << 24;           // bits 24-31 (lower 8 bits of value 6)

    // Word 4: bits 128-159
    dst[4] = ((src[6] >> 8) & 0xFFF) << 0  // bits 0-11 (upper 12 bits of value 6)
        | (src[7] & 0xFFFFF) << 12;        // bits 12-31

    // Word 5: bits 160-191
    dst[5] = (src[8] & 0xFFFFF) << 0       // bits 0-19
        | (src[9] & 0xFFF) << 20;          // bits 20-31 (lower 12 bits of value 9)

    // Word 6: bits 192-223
    dst[6] = ((src[9] >> 12) & 0xFF) << 0  // bits 0-7 (upper 8 bits of value 9)
        | (src[10] & 0xFFFFF) << 8         // bits 8-27
        | (src[11] & 0xF) << 28;           // bits 28-31 (lower 4 bits of value 11)

    // Word 7: bits 224-255
    dst[7] = ((src[11] >> 4) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 11)
        | (src[12] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 12)

    // Word 8: bits 256-287
    dst[8] = ((src[12] >> 16) & 0xF) << 0  // bits 0-3 (upper 4 bits of value 12)
        | (src[13] & 0xFFFFF) << 4         // bits 4-23
        | (src[14] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 14)

    // Word 9: bits 288-319
    dst[9] = ((src[14] >> 8) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 14)
        | (src[15] & 0xFFFFF) << 12;       // bits 12-31

    // Word 10: bits 320-351
    dst[10] = (src[16] & 0xFFFFF) << 0     // bits 0-19
        | (src[17] & 0xFFF) << 20;         // bits 20-31 (lower 12 bits of value 17)

    // Word 11: bits 352-383
    dst[11] = ((src[17] >> 12) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 17)
        | (src[18] & 0xFFFFF) << 8         // bits 8-27
        | (src[19] & 0xF) << 28;           // bits 28-31 (lower 4 bits of value 19)

    // Word 12: bits 384-415
    dst[12] = ((src[19] >> 4) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 19)
        | (src[20] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 20)

    // Word 13: bits 416-447
    dst[13] = ((src[20] >> 16) & 0xF) << 0 // bits 0-3 (upper 4 bits of value 20)
        | (src[21] & 0xFFFFF) << 4         // bits 4-23
        | (src[22] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 22)

    // Word 14: bits 448-479
    dst[14] = ((src[22] >> 8) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 22)
        | (src[23] & 0xFFFFF) << 12;       // bits 12-31

    // Word 15: bits 480-511
    dst[15] = (src[24] & 0xFFFFF) << 0     // bits 0-19
        | (src[25] & 0xFFF) << 20;         // bits 20-31 (lower 12 bits of value 25)

    // Word 16: bits 512-543
    dst[16] = ((src[25] >> 12) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 25)
        | (src[26] & 0xFFFFF) << 8         // bits 8-27
        | (src[27] & 0xF) << 28;           // bits 28-31 (lower 4 bits of value 27)

    // Word 17: bits 544-575
    dst[17] = ((src[27] >> 4) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 27)
        | (src[28] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 28)

    // Word 18: bits 576-607
    dst[18] = ((src[28] >> 16) & 0xF) << 0 // bits 0-3 (upper 4 bits of value 28)
        | (src[29] & 0xFFFFF) << 4         // bits 4-23
        | (src[30] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 30)

    // Word 19: bits 608-639
    dst[19] = ((src[30] >> 8) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 30)
        | (src[31] & 0xFFFFF) << 12;       // bits 12-31
}

// 32 values * 21 bits = 672 bits = 21 words
pub fn decompress_21_bit(src: &[u32], dst: &mut [u32]) {
    // dst[0]: bits 0-20 (word 0, pos 0-20)
    dst[0]  = (src[0] >> 0)  & 0x1FFFFF;
    
    // dst[1]: bits 21-41 (word 0 pos 21-31, word 1 pos 0-9)
    dst[1]  = ((src[0] >> 21) | (src[1] << 11)) & 0x1FFFFF;

    // dst[2]: bits 42-62 (word 1, pos 10-30)
    dst[2]  = (src[1] >> 10) & 0x1FFFFF;
    
    // dst[3]: bits 63-83 (word 1 pos 31, word 2 pos 0-19)
    dst[3]  = ((src[1] >> 31) | (src[2] << 1)) & 0x1FFFFF;

    // dst[4]: bits 84-104 (word 2, pos 20-31, word 3 pos 0-8)
    dst[4]  = ((src[2] >> 20) | (src[3] << 12)) & 0x1FFFFF;
    
    // dst[5]: bits 105-125 (word 3, pos 9-29)
    dst[5]  = (src[3] >> 9) & 0x1FFFFF;

    // dst[6]: bits 126-146 (word 3 pos 30-31, word 4 pos 0-18)
    dst[6]  = ((src[3] >> 30) | (src[4] << 2)) & 0x1FFFFF;

    // dst[7]: bits 147-167 (word 4, pos 19-31, word 5 pos 0-7)
    dst[7]  = ((src[4] >> 19) | (src[5] << 13)) & 0x1FFFFF;
    
    // dst[8]: bits 168-188 (word 5, pos 8-28)
    dst[8]  = (src[5] >> 8) & 0x1FFFFF;

    // dst[9]: bits 189-209 (word 5 pos 29-31, word 6 pos 0-17)
    dst[9]  = ((src[5] >> 29) | (src[6] << 3)) & 0x1FFFFF;
    
    // dst[10]: bits 210-230 (word 6 pos 18-31, word 7 pos 0-6)
    dst[10] = ((src[6] >> 18) | (src[7] << 14)) & 0x1FFFFF;

    // dst[11]: bits 231-251 (word 7, pos 7-27)
    dst[11] = (src[7] >> 7) & 0x1FFFFF;

    // dst[12]: bits 252-272 (word 7 pos 28-31, word 8 pos 0-16)
    dst[12] = ((src[7] >> 28) | (src[8] << 4)) & 0x1FFFFF;

    // dst[13]: bits 273-293 (word 8 pos 17-31, word 9 pos 0-5)
    dst[13] = ((src[8] >> 17) | (src[9] << 15)) & 0x1FFFFF;

    // dst[14]: bits 294-314 (word 9, pos 6-26)
    dst[14] = (src[9] >> 6) & 0x1FFFFF;
    
    // dst[15]: bits 315-335 (word 9 pos 27-31, word 10 pos 0-15)
    dst[15] = ((src[9] >> 27) | (src[10] << 5)) & 0x1FFFFF;

    // dst[16]: bits 336-356 (word 10 pos 16-31, word 11 pos 0-4)
    dst[16] = ((src[10] >> 16) | (src[11] << 16)) & 0x1FFFFF;

    // dst[17]: bits 357-377 (word 11, pos 5-25)
    dst[17] = (src[11] >> 5) & 0x1FFFFF;

    // dst[18]: bits 378-398 (word 11 pos 26-31, word 12 pos 0-14)
    dst[18] = ((src[11] >> 26) | (src[12] << 6)) & 0x1FFFFF;

    // dst[19]: bits 399-419 (word 12 pos 15-31, word 13 pos 0-3)
    dst[19] = ((src[12] >> 15) | (src[13] << 17)) & 0x1FFFFF;
    
    // dst[20]: bits 420-440 (word 13, pos 4-24)
    dst[20] = (src[13] >> 4) & 0x1FFFFF;

    // dst[21]: bits 441-461 (word 13 pos 25-31, word 14 pos 0-13)
    dst[21] = ((src[13] >> 25) | (src[14] << 7)) & 0x1FFFFF;

    // dst[22]: bits 462-482 (word 14 pos 14-31, word 15 pos 0-2)
    dst[22] = ((src[14] >> 14) | (src[15] << 18)) & 0x1FFFFF;

    // dst[23]: bits 483-503 (word 15, pos 3-23)
    dst[23] = (src[15] >> 3) & 0x1FFFFF;

    // dst[24]: bits 504-524 (word 15 pos 24-31, word 16 pos 0-12)
    dst[24] = ((src[15] >> 24) | (src[16] << 8)) & 0x1FFFFF;
    
    // dst[25]: bits 525-545 (word 16 pos 13-31, word 17 pos 0-1)
    dst[25] = ((src[16] >> 13) | (src[17] << 19)) & 0x1FFFFF;

    // dst[26]: bits 546-566 (word 17, pos 2-22)
    dst[26] = (src[17] >> 2) & 0x1FFFFF;

    // dst[27]: bits 567-587 (word 17 pos 23-31, word 18 pos 0-11)
    dst[27] = ((src[17] >> 23) | (src[18] << 9)) & 0x1FFFFF;

    // dst[28]: bits 588-608 (word 18 pos 12-31, word 19 pos 0)
    dst[28] = ((src[18] >> 12) | (src[19] << 20)) & 0x1FFFFF;

    // dst[29]: bits 609-629 (word 19, pos 1-21)
    dst[29] = (src[19] >> 1) & 0x1FFFFF;
    
    // dst[30]: bits 630-650 (word 19 pos 22-31, word 20 pos 0-10)
    dst[30] = ((src[19] >> 22) | (src[20] << 10)) & 0x1FFFFF;

    // dst[31]: bits 651-671 (word 20, pos 11-31)
    dst[31] = (src[20] >> 11) & 0x1FFFFF;
}

// 32 values * 21 bits = 672 bits = 21 words
pub fn compress_21_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x1FFFFF) << 0      // bits 0-20
        | (src[1] & 0x7FF) << 21;          // bits 21-31 (lower 11 bits of value 1)

    // Word 1: bits 32-63
    dst[1] = ((src[1] >> 11) & 0x3FF) << 0 // bits 0-9 (upper 10 bits of value 1)
        | (src[2] & 0x1FFFFF) << 10        // bits 10-30
        | (src[3] & 0x1) << 31;            // bit 31 (lower 1 bit of value 3)

    // Word 2: bits 64-95
    dst[2] = ((src[3] >> 1) & 0xFFFFF) << 0 // bits 0-19 (upper 20 bits of value 3)
        | (src[4] & 0xFFF) << 20;          // bits 20-31 (lower 12 bits of value 4)

    // Word 3: bits 96-127
    dst[3] = ((src[4] >> 12) & 0x1FF) << 0 // bits 0-8 (upper 9 bits of value 4)
        | (src[5] & 0x1FFFFF) << 9         // bits 9-29
        | (src[6] & 0x3) << 30;            // bits 30-31 (lower 2 bits of value 6)

    // Word 4: bits 128-159
    dst[4] = ((src[6] >> 2) & 0x7FFFF) << 0 // bits 0-18 (upper 19 bits of value 6)
        | (src[7] & 0x1FFF) << 19;         // bits 19-31 (lower 13 bits of value 7)

    // Word 5: bits 160-191
    dst[5] = ((src[7] >> 13) & 0xFF) << 0  // bits 0-7 (upper 8 bits of value 7)
        | (src[8] & 0x1FFFFF) << 8         // bits 8-28
        | (src[9] & 0x7) << 29;            // bits 29-31 (lower 3 bits of value 9)

    // Word 6: bits 192-223
    dst[6] = ((src[9] >> 3) & 0x3FFFF) << 0 // bits 0-17 (upper 18 bits of value 9)
        | (src[10] & 0x3FFF) << 18;        // bits 18-31 (lower 14 bits of value 10)

    // Word 7: bits 224-255
    dst[7] = ((src[10] >> 14) & 0x7F) << 0 // bits 0-6 (upper 7 bits of value 10)
        | (src[11] & 0x1FFFFF) << 7        // bits 7-27
        | (src[12] & 0xF) << 28;           // bits 28-31 (lower 4 bits of value 12)

    // Word 8: bits 256-287
    dst[8] = ((src[12] >> 4) & 0x1FFFF) << 0 // bits 0-16 (upper 17 bits of value 12)
        | (src[13] & 0x7FFF) << 17;        // bits 17-31 (lower 15 bits of value 13)

    // Word 9: bits 288-319
    dst[9] = ((src[13] >> 15) & 0x3F) << 0 // bits 0-5 (upper 6 bits of value 13)
        | (src[14] & 0x1FFFFF) << 6        // bits 6-26
        | (src[15] & 0x1F) << 27;          // bits 27-31 (lower 5 bits of value 15)

    // Word 10: bits 320-351
    dst[10] = ((src[15] >> 5) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 15)
        | (src[16] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 16)

    // Word 11: bits 352-383
    dst[11] = ((src[16] >> 16) & 0x1F) << 0 // bits 0-4 (upper 5 bits of value 16)
        | (src[17] & 0x1FFFFF) << 5        // bits 5-25
        | (src[18] & 0x3F) << 26;          // bits 26-31 (lower 6 bits of value 18)

    // Word 12: bits 384-415
    dst[12] = ((src[18] >> 6) & 0x7FFF) << 0 // bits 0-14 (upper 15 bits of value 18)
        | (src[19] & 0x1FFFF) << 15;       // bits 15-31 (lower 17 bits of value 19)

    // Word 13: bits 416-447
    dst[13] = ((src[19] >> 17) & 0xF) << 0 // bits 0-3 (upper 4 bits of value 19)
        | (src[20] & 0x1FFFFF) << 4        // bits 4-24
        | (src[21] & 0x7F) << 25;          // bits 25-31 (lower 7 bits of value 21)

    // Word 14: bits 448-479
    dst[14] = ((src[21] >> 7) & 0x3FFF) << 0 // bits 0-13 (upper 14 bits of value 21)
        | (src[22] & 0x3FFFF) << 14;       // bits 14-31 (lower 18 bits of value 22)

    // Word 15: bits 480-511
    dst[15] = ((src[22] >> 18) & 0x7) << 0 // bits 0-2 (upper 3 bits of value 22)
        | (src[23] & 0x1FFFFF) << 3        // bits 3-23
        | (src[24] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 24)

    // Word 16: bits 512-543
    dst[16] = ((src[24] >> 8) & 0x1FFF) << 0 // bits 0-12 (upper 13 bits of value 24)
        | (src[25] & 0x7FFFF) << 13;       // bits 13-31 (lower 19 bits of value 25)

    // Word 17: bits 544-575
    dst[17] = ((src[25] >> 19) & 0x3) << 0 // bits 0-1 (upper 2 bits of value 25)
        | (src[26] & 0x1FFFFF) << 2        // bits 2-22
        | (src[27] & 0x1FF) << 23;         // bits 23-31 (lower 9 bits of value 27)

    // Word 18: bits 576-607
    dst[18] = ((src[27] >> 9) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 27)
        | (src[28] & 0xFFFFF) << 12;       // bits 12-31 (lower 20 bits of value 28)

    // Word 19: bits 608-639
    dst[19] = ((src[28] >> 20) & 0x1) << 0 // bit 0 (upper 1 bit of value 28)
        | (src[29] & 0x1FFFFF) << 1        // bits 1-21
        | (src[30] & 0x3FF) << 22;         // bits 22-31 (lower 10 bits of value 30)

    // Word 20: bits 640-671
    dst[20] = ((src[30] >> 10) & 0x7FF) << 0 // bits 0-10 (upper 11 bits of value 30)
        | (src[31] & 0x1FFFFF) << 11;      // bits 11-31
}

// 32 values * 22 bits = 704 bits = 22 words
pub fn decompress_22_bit(src: &[u32], dst: &mut [u32]) {
    // dst[0]: bits 0-21 (in src[0] bits 0-21)
    dst[0]  = (src[0] >> 0)  & 0x3FFFFF;
    
    // dst[1]: bits 22-43 (src[0] bits 22-31 = 10 bits, src[1] bits 0-11 = 12 bits)
    dst[1]  = ((src[0] >> 22) | (src[1] << 10)) & 0x3FFFFF;

    // dst[2]: bits 44-65 (src[1] bits 12-31 = 20 bits, src[2] bits 0-1 = 2 bits)
    dst[2]  = ((src[1] >> 12) | (src[2] << 20)) & 0x3FFFFF;
    
    // dst[3]: bits 66-87 (in src[2] bits 2-23)
    dst[3]  = (src[2] >> 2)  & 0x3FFFFF;

    // dst[4]: bits 88-109 (src[2] bits 24-31 = 8 bits, src[3] bits 0-13 = 14 bits)
    dst[4]  = ((src[2] >> 24) | (src[3] << 8)) & 0x3FFFFF;
    
    // dst[5]: bits 110-131 (src[3] bits 14-31 = 18 bits, src[4] bits 0-3 = 4 bits)
    dst[5]  = ((src[3] >> 14) | (src[4] << 18)) & 0x3FFFFF;

    // dst[6]: bits 132-153 (in src[4] bits 4-25)
    dst[6]  = (src[4] >> 4)  & 0x3FFFFF;
    
    // dst[7]: bits 154-175 (src[4] bits 26-31 = 6 bits, src[5] bits 0-15 = 16 bits)
    dst[7]  = ((src[4] >> 26) | (src[5] << 6)) & 0x3FFFFF;

    // dst[8]: bits 176-197 (src[5] bits 16-31 = 16 bits, src[6] bits 0-5 = 6 bits)
    dst[8]  = ((src[5] >> 16) | (src[6] << 16)) & 0x3FFFFF;
    
    // dst[9]: bits 198-219 (in src[6] bits 6-27)
    dst[9]  = (src[6] >> 6)  & 0x3FFFFF;

    // dst[10]: bits 220-241 (src[6] bits 28-31 = 4 bits, src[7] bits 0-17 = 18 bits)
    dst[10] = ((src[6] >> 28) | (src[7] << 4)) & 0x3FFFFF;
    
    // dst[11]: bits 242-263 (src[7] bits 18-31 = 14 bits, src[8] bits 0-7 = 8 bits)
    dst[11] = ((src[7] >> 18) | (src[8] << 14)) & 0x3FFFFF;

    // dst[12]: bits 264-285 (src[8] bits 8-29 = 22 bits)
    dst[12] = (src[8] >> 8) & 0x3FFFFF;
    
    // dst[13]: bits 286-307 (src[8] bits 30-31 = 2 bits, src[9] bits 0-19 = 20 bits)
    dst[13] = ((src[8] >> 30) | (src[9] << 2)) & 0x3FFFFF;

    // dst[14]: bits 308-329 (src[9] bits 20-31 = 12 bits, src[10] bits 0-9 = 10 bits)
    dst[14] = ((src[9] >> 20) | (src[10] << 12)) & 0x3FFFFF;
    
    // dst[15]: bits 330-351 (in src[10] bits 10-31)
    dst[15] = (src[10] >> 10) & 0x3FFFFF;

    // dst[16]: bits 352-373 (in src[11] bits 0-21)
    dst[16] = (src[11] >> 0)  & 0x3FFFFF;
    
    // dst[17]: bits 374-395 (src[11] bits 22-31 = 10 bits, src[12] bits 0-11 = 12 bits)
    dst[17] = ((src[11] >> 22) | (src[12] << 10)) & 0x3FFFFF;

    // dst[18]: bits 396-417 (src[12] bits 12-31 = 20 bits, src[13] bits 0-1 = 2 bits)
    dst[18] = ((src[12] >> 12) | (src[13] << 20)) & 0x3FFFFF;
    
    // dst[19]: bits 418-439 (in src[13] bits 2-23)
    dst[19] = (src[13] >> 2)  & 0x3FFFFF;

    // dst[20]: bits 440-461 (src[13] bits 24-31 = 8 bits, src[14] bits 0-13 = 14 bits)
    dst[20] = ((src[13] >> 24) | (src[14] << 8)) & 0x3FFFFF;
    
    // dst[21]: bits 462-483 (src[14] bits 14-31 = 18 bits, src[15] bits 0-3 = 4 bits)
    dst[21] = ((src[14] >> 14) | (src[15] << 18)) & 0x3FFFFF;

    // dst[22]: bits 484-505 (in src[15] bits 4-25)
    dst[22] = (src[15] >> 4)  & 0x3FFFFF;
    
    // dst[23]: bits 506-527 (src[15] bits 26-31 = 6 bits, src[16] bits 0-15 = 16 bits)
    dst[23] = ((src[15] >> 26) | (src[16] << 6)) & 0x3FFFFF;

    // dst[24]: bits 528-549 (src[16] bits 16-31 = 16 bits, src[17] bits 0-5 = 6 bits)
    dst[24] = ((src[16] >> 16) | (src[17] << 16)) & 0x3FFFFF;
    
    // dst[25]: bits 550-571 (in src[17] bits 6-27)
    dst[25] = (src[17] >> 6) & 0x3FFFFF;

    // dst[26]: bits 572-593 (src[17] bits 28-31 = 4 bits, src[18] bits 0-17 = 18 bits)
    dst[26] = ((src[17] >> 28) | (src[18] << 4)) & 0x3FFFFF;
    
    // dst[27]: bits 594-615 (src[18] bits 18-31 = 14 bits, src[19] bits 0-7 = 8 bits)
    dst[27] = ((src[18] >> 18) | (src[19] << 14)) & 0x3FFFFF;

    // dst[28]: bits 616-637 (in src[19] bits 8-29)
    dst[28] = (src[19] >> 8) & 0x3FFFFF;
    
    // dst[29]: bits 638-659 (src[19] bits 30-31 = 2 bits, src[20] bits 0-19 = 20 bits)
    dst[29] = ((src[19] >> 30) | (src[20] << 2)) & 0x3FFFFF;

    // dst[30]: bits 660-681 (src[20] bits 20-31 = 12 bits, src[21] bits 0-9 = 10 bits)
    dst[30] = ((src[20] >> 20) | (src[21] << 12)) & 0x3FFFFF;
    
    // dst[31]: bits 682-703 (in src[21] bits 10-31)
    dst[31] = (src[21] >> 10) & 0x3FFFFF;
}

// Compress 32 values of 22 bits each into 22 words of 32 bits
pub fn compress_22_bit(src: &[u32], dst: &mut [u32]) {
 
    // dst[0]: src[0] bits 0-21 at positions 0-21, src[1] bits 0-9 at positions 22-31
    dst[0] = (src[0] & 0x3FFFFF) | ((src[1] & 0x3FF) << 22);
    
    // dst[1]: src[1] bits 10-21 at positions 0-11, src[2] bits 0-19 at positions 12-31
    dst[1] = ((src[1] >> 10) & 0xFFF) | ((src[2] & 0xFFFFF) << 12);
    
    // dst[2]: src[2] bits 20-21 at positions 0-1, src[3] bits 0-21 at positions 2-23, src[4] bits 0-7 at positions 24-31
    dst[2] = ((src[2] >> 20) & 0x3) | ((src[3] & 0x3FFFFF) << 2) | ((src[4] & 0xFF) << 24);
    
    // dst[3]: src[4] bits 8-21 at positions 0-13, src[5] bits 0-17 at positions 14-31
    dst[3] = ((src[4] >> 8) & 0x3FFF) | ((src[5] & 0x3FFFF) << 14);
    
    // dst[4]: src[5] bits 18-21 at positions 0-3, src[6] bits 0-21 at positions 4-25, src[7] bits 0-5 at positions 26-31
    dst[4] = ((src[5] >> 18) & 0xF) | ((src[6] & 0x3FFFFF) << 4) | ((src[7] & 0x3F) << 26);
    
    // dst[5]: src[7] bits 6-21 at positions 0-15, src[8] bits 0-15 at positions 16-31
    dst[5] = ((src[7] >> 6) & 0xFFFF) | ((src[8] & 0xFFFF) << 16);
    
    // dst[6]: src[8] bits 16-21 at positions 0-5, src[9] bits 0-21 at positions 6-27, src[10] bits 0-3 at positions 28-31
    dst[6] = ((src[8] >> 16) & 0x3F) | ((src[9] & 0x3FFFFF) << 6) | ((src[10] & 0xF) << 28);
    
    // dst[7]: src[10] bits 4-21 at positions 0-17, src[11] bits 0-13 at positions 18-31
    dst[7] = ((src[10] >> 4) & 0x3FFFF) | ((src[11] & 0x3FFF) << 18);
    
    // dst[8]: src[11] bits 14-21 at positions 0-7, src[12] bits 0-21 at positions 8-29, src[13] bits 0-1 at positions 30-31
    dst[8] = ((src[11] >> 14) & 0xFF) | ((src[12] & 0x3FFFFF) << 8) | ((src[13] & 0x3) << 30);
    
    // dst[9]: src[13] bits 2-21 at positions 0-19, src[14] bits 0-11 at positions 20-31
    dst[9] = ((src[13] >> 2) & 0xFFFFF) | ((src[14] & 0xFFF) << 20);
    
    // dst[10]: src[14] bits 12-21 at positions 0-9, src[15] bits 0-21 at positions 10-31
    dst[10] = ((src[14] >> 12) & 0x3FF) | ((src[15] & 0x3FFFFF) << 10);
    
    // dst[11]: src[16] bits 0-21 at positions 0-21, src[17] bits 0-9 at positions 22-31
    dst[11] = (src[16] & 0x3FFFFF) | ((src[17] & 0x3FF) << 22);
    
    // dst[12]: src[17] bits 10-21 at positions 0-11, src[18] bits 0-19 at positions 12-31
    dst[12] = ((src[17] >> 10) & 0xFFF) | ((src[18] & 0xFFFFF) << 12);
    
    // dst[13]: src[18] bits 20-21 at positions 0-1, src[19] bits 0-21 at positions 2-23, src[20] bits 0-7 at positions 24-31
    dst[13] = ((src[18] >> 20) & 0x3) | ((src[19] & 0x3FFFFF) << 2) | ((src[20] & 0xFF) << 24);
    
    // dst[14]: src[20] bits 8-21 at positions 0-13, src[21] bits 0-17 at positions 14-31
    dst[14] = ((src[20] >> 8) & 0x3FFF) | ((src[21] & 0x3FFFF) << 14);
    
    // dst[15]: src[21] bits 18-21 at positions 0-3, src[22] bits 0-21 at positions 4-25, src[23] bits 0-5 at positions 26-31
    dst[15] = ((src[21] >> 18) & 0xF) | ((src[22] & 0x3FFFFF) << 4) | ((src[23] & 0x3F) << 26);
    
    // dst[16]: src[23] bits 6-21 at positions 0-15, src[24] bits 0-15 at positions 16-31
    dst[16] = ((src[23] >> 6) & 0xFFFF) | ((src[24] & 0xFFFF) << 16);
    
    // dst[17]: src[24] bits 16-21 at positions 0-5, src[25] bits 0-21 at positions 6-27, src[26] bits 0-3 at positions 28-31
    dst[17] = ((src[24] >> 16) & 0x3F) | ((src[25] & 0x3FFFFF) << 6) | ((src[26] & 0xF) << 28);
    
    // dst[18]: src[26] bits 4-21 at positions 0-17, src[27] bits 0-13 at positions 18-31
    dst[18] = ((src[26] >> 4) & 0x3FFFF) | ((src[27] & 0x3FFF) << 18);
    
    // dst[19]: src[27] bits 14-21 at positions 0-7, src[28] bits 0-21 at positions 8-29, src[29] bits 0-1 at positions 30-31
    dst[19] = ((src[27] >> 14) & 0xFF) | ((src[28] & 0x3FFFFF) << 8) | ((src[29] & 0x3) << 30);
    
    // dst[20]: src[29] bits 2-21 at positions 0-19, src[30] bits 0-11 at positions 20-31
    dst[20] = ((src[29] >> 2) & 0xFFFFF) | ((src[30] & 0xFFF) << 20);
    
    // dst[21]: src[30] bits 12-21 at positions 0-9, src[31] bits 0-21 at positions 10-31
    dst[21] = ((src[30] >> 12) & 0x3FF) | ((src[31] & 0x3FFFFF) << 10);
}

// 32 values * 23 bits = 736 bits = 23 words
pub fn decompress_23_bit(src: &[u32], dst: &mut [u32]) {
    // dst[0]: bits 0-22 (word 0, pos 0-22)
    dst[0]  = (src[0] >> 0)  & 0x7FFFFF;
    
    // dst[1]: bits 23-45 (word 0 pos 23-31, word 1 pos 0-13)
    dst[1]  = ((src[0] >> 23) | (src[1] << 9)) & 0x7FFFFF;

    // dst[2]: bits 46-68 (word 1, pos 14-31, word 2 pos 0-4)
    dst[2]  = ((src[1] >> 14) | (src[2] << 18)) & 0x7FFFFF;
    
    // dst[3]: bits 69-91 (word 2, pos 5-27)
    dst[3]  = (src[2] >> 5) & 0x7FFFFF;

    // dst[4]: bits 92-114 (word 2 pos 28-31, word 3 pos 0-18)
    dst[4]  = ((src[2] >> 28) | (src[3] << 4)) & 0x7FFFFF;
    
    // dst[5]: bits 115-137 (word 3 pos 19-31, word 4 pos 0-9)
    dst[5]  = ((src[3] >> 19) | (src[4] << 13)) & 0x7FFFFF;

    // dst[6]: bits 138-160 (word 4, pos 10-31, word 5 pos 0)
    dst[6]  = ((src[4] >> 10) | (src[5] << 22)) & 0x7FFFFF;

    // dst[7]: bits 161-183 (word 5, pos 1-23)
    dst[7]  = (src[5] >> 1) & 0x7FFFFF;
    
    // dst[8]: bits 184-206 (word 5 pos 24-31, word 6 pos 0-14)
    dst[8]  = ((src[5] >> 24) | (src[6] << 8)) & 0x7FFFFF;

    // dst[9]: bits 207-229 (word 6 pos 15-31, word 7 pos 0-5)
    dst[9]  = ((src[6] >> 15) | (src[7] << 17)) & 0x7FFFFF;
    
    // dst[10]: bits 230-252 (word 7, pos 6-28)
    dst[10] = (src[7] >> 6) & 0x7FFFFF;

    // dst[11]: bits 253-275 (word 7 pos 29-31, word 8 pos 0-19)
    dst[11] = ((src[7] >> 29) | (src[8] << 3)) & 0x7FFFFF;

    // dst[12]: bits 276-298 (word 8 pos 20-31, word 9 pos 0-10)
    dst[12] = ((src[8] >> 20) | (src[9] << 12)) & 0x7FFFFF;

    // dst[13]: bits 299-321 (word 9 pos 11-31, word 10 pos 0-1)
    dst[13] = ((src[9] >> 11) | (src[10] << 21)) & 0x7FFFFF;

    // dst[14]: bits 322-344 (word 10, pos 2-24)
    dst[14] = (src[10] >> 2) & 0x7FFFFF;
    
    // dst[15]: bits 345-367 (word 10 pos 25-31, word 11 pos 0-15)
    dst[15] = ((src[10] >> 25) | (src[11] << 7)) & 0x7FFFFF;

    // dst[16]: bits 368-390 (word 11 pos 16-31, word 12 pos 0-6)
    dst[16] = ((src[11] >> 16) | (src[12] << 16)) & 0x7FFFFF;

    // dst[17]: bits 391-413 (word 12, pos 7-29)
    dst[17] = (src[12] >> 7) & 0x7FFFFF;

    // dst[18]: bits 414-436 (word 12 pos 30-31, word 13 pos 0-20)
    dst[18] = ((src[12] >> 30) | (src[13] << 2)) & 0x7FFFFF;

    // dst[19]: bits 437-459 (word 13 pos 21-31, word 14 pos 0-11)
    dst[19] = ((src[13] >> 21) | (src[14] << 11)) & 0x7FFFFF;
    
    // dst[20]: bits 460-482 (word 14 pos 12-31, word 15 pos 0-2)
    dst[20] = ((src[14] >> 12) | (src[15] << 20)) & 0x7FFFFF;

    // dst[21]: bits 483-505 (word 15, pos 3-25)
    dst[21] = (src[15] >> 3) & 0x7FFFFF;

    // dst[22]: bits 506-528 (word 15 pos 26-31, word 16 pos 0-16)
    dst[22] = ((src[15] >> 26) | (src[16] << 6)) & 0x7FFFFF;

    // dst[23]: bits 529-551 (word 16 pos 17-31, word 17 pos 0-7)
    dst[23] = ((src[16] >> 17) | (src[17] << 15)) & 0x7FFFFF;

    // dst[24]: bits 552-574 (word 17 pos 8-30)
    dst[24] = (src[17] >> 8) & 0x7FFFFF;
    
    // dst[25]: bits 575-597 (word 17 pos 31, word 18 pos 0-21)
    dst[25] = ((src[17] >> 31) | (src[18] << 1)) & 0x7FFFFF;

    // dst[26]: bits 598-620 (word 18 pos 22-31, word 19 pos 0-12)
    dst[26] = ((src[18] >> 22) | (src[19] << 10)) & 0x7FFFFF;

    // dst[27]: bits 621-643 (word 19 pos 13-31, word 20 pos 0-3)
    dst[27] = ((src[19] >> 13) | (src[20] << 19)) & 0x7FFFFF;

    // dst[28]: bits 644-666 (word 20, pos 4-26)
    dst[28] = (src[20] >> 4) & 0x7FFFFF;

    // dst[29]: bits 667-689 (word 20 pos 27-31, word 21 pos 0-17)
    dst[29] = ((src[20] >> 27) | (src[21] << 5)) & 0x7FFFFF;
    
    // dst[30]: bits 690-712 (word 21 pos 18-31, word 22 pos 0-8)
    dst[30] = ((src[21] >> 18) | (src[22] << 14)) & 0x7FFFFF;

    // dst[31]: bits 713-735 (word 22, pos 9-31)
    dst[31] = (src[22] >> 9) & 0x7FFFFF;
}

// 32 values * 23 bits = 736 bits = 23 words
pub fn compress_23_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x7FFFFF) << 0      // bits 0-22
        | (src[1] & 0x1FF) << 23;          // bits 23-31 (lower 9 bits of value 1)

    // Word 1: bits 32-63
    dst[1] = ((src[1] >> 9) & 0x3FFF) << 0 // bits 0-13 (upper 14 bits of value 1)
        | (src[2] & 0x3FFFF) << 14;        // bits 14-31 (lower 18 bits of value 2)

    // Word 2: bits 64-95
    dst[2] = ((src[2] >> 18) & 0x1F) << 0  // bits 0-4 (upper 5 bits of value 2)
        | (src[3] & 0x7FFFFF) << 5         // bits 5-27
        | (src[4] & 0xF) << 28;            // bits 28-31 (lower 4 bits of value 4)

    // Word 3: bits 96-127
    dst[3] = ((src[4] >> 4) & 0x7FFFF) << 0 // bits 0-18 (upper 19 bits of value 4)
        | (src[5] & 0x1FFF) << 19;         // bits 19-31 (lower 13 bits of value 5)

    // Word 4: bits 128-159
    dst[4] = ((src[5] >> 13) & 0x3FF) << 0 // bits 0-9 (upper 10 bits of value 5)
        | (src[6] & 0x3FFFFF) << 10;       // bits 10-31 (lower 22 bits of value 6)

    // Word 5: bits 160-191
    dst[5] = ((src[6] >> 22) & 0x1) << 0   // bit 0 (upper 1 bit of value 6)
        | (src[7] & 0x7FFFFF) << 1         // bits 1-23
        | (src[8] & 0xFF) << 24;           // bits 24-31 (lower 8 bits of value 8)

    // Word 6: bits 192-223
    dst[6] = ((src[8] >> 8) & 0x7FFF) << 0 // bits 0-14 (upper 15 bits of value 8)
        | (src[9] & 0x1FFFF) << 15;        // bits 15-31 (lower 17 bits of value 9)

    // Word 7: bits 224-255
    dst[7] = ((src[9] >> 17) & 0x3F) << 0  // bits 0-5 (upper 6 bits of value 9)
        | (src[10] & 0x7FFFFF) << 6        // bits 6-28
        | (src[11] & 0x7) << 29;           // bits 29-31 (lower 3 bits of value 11)

    // Word 8: bits 256-287
    dst[8] = ((src[11] >> 3) & 0xFFFFF) << 0 // bits 0-19 (upper 20 bits of value 11)
        | (src[12] & 0xFFF) << 20;         // bits 20-31 (lower 12 bits of value 12)

    // Word 9: bits 288-319
    dst[9] = ((src[12] >> 12) & 0x7FF) << 0 // bits 0-10 (upper 11 bits of value 12)
        | (src[13] & 0x1FFFFF) << 11;      // bits 11-31 (lower 21 bits of value 13)

    // Word 10: bits 320-351
    dst[10] = ((src[13] >> 21) & 0x3) << 0 // bits 0-1 (upper 2 bits of value 13)
        | (src[14] & 0x7FFFFF) << 2        // bits 2-24
        | (src[15] & 0x7F) << 25;          // bits 25-31 (lower 7 bits of value 15)

    // Word 11: bits 352-383
    dst[11] = ((src[15] >> 7) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 15)
        | (src[16] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 16)

    // Word 12: bits 384-415
    dst[12] = ((src[16] >> 16) & 0x7F) << 0 // bits 0-6 (upper 7 bits of value 16)
        | (src[17] & 0x7FFFFF) << 7        // bits 7-29
        | (src[18] & 0x3) << 30;           // bits 30-31 (lower 2 bits of value 18)

    // Word 13: bits 416-447
    dst[13] = ((src[18] >> 2) & 0x1FFFFF) << 0 // bits 0-20 (upper 21 bits of value 18)
        | (src[19] & 0x7FF) << 21;         // bits 21-31 (lower 11 bits of value 19)

    // Word 14: bits 448-479
    dst[14] = ((src[19] >> 11) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 19)
        | (src[20] & 0xFFFFF) << 12;       // bits 12-31 (lower 20 bits of value 20)

    // Word 15: bits 480-511
    dst[15] = ((src[20] >> 20) & 0x7) << 0 // bits 0-2 (upper 3 bits of value 20)
        | (src[21] & 0x7FFFFF) << 3        // bits 3-25
        | (src[22] & 0x3F) << 26;          // bits 26-31 (lower 6 bits of value 22)

    // Word 16: bits 512-543
    dst[16] = ((src[22] >> 6) & 0x1FFFF) << 0 // bits 0-16 (upper 17 bits of value 22)
        | (src[23] & 0x7FFF) << 17;        // bits 17-31 (lower 15 bits of value 23)

    // Word 17: bits 544-575
    dst[17] = ((src[23] >> 15) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 23)
        | (src[24] & 0x7FFFFF) << 8        // bits 8-30
        | (src[25] & 0x1) << 31;           // bit 31 (lower 1 bit of value 25)

    // Word 18: bits 576-607
    dst[18] = ((src[25] >> 1) & 0x3FFFFF) << 0 // bits 0-21 (upper 22 bits of value 25)
        | (src[26] & 0x3FF) << 22;         // bits 22-31 (lower 10 bits of value 26)

    // Word 19: bits 608-639
    dst[19] = ((src[26] >> 10) & 0x1FFF) << 0 // bits 0-12 (upper 13 bits of value 26)
        | (src[27] & 0x7FFFF) << 13;       // bits 13-31 (lower 19 bits of value 27)

    // Word 20: bits 640-671
    dst[20] = ((src[27] >> 19) & 0xF) << 0 // bits 0-3 (upper 4 bits of value 27)
        | (src[28] & 0x7FFFFF) << 4        // bits 4-26
        | (src[29] & 0x1F) << 27;          // bits 27-31 (lower 5 bits of value 29)

    // Word 21: bits 672-703
    dst[21] = ((src[29] >> 5) & 0x3FFFF) << 0 // bits 0-17 (upper 18 bits of value 29)
        | (src[30] & 0x3FFF) << 18;        // bits 18-31 (lower 14 bits of value 30)

    // Word 22: bits 704-735
    dst[22] = ((src[30] >> 14) & 0x1FF) << 0 // bits 0-8 (upper 9 bits of value 30)
        | (src[31] & 0x7FFFFF) << 9;       // bits 9-31
}

// 32 values * 24 bits = 768 bits = 24 words
pub fn decompress_24_bit(src: &[u32], dst: &mut [u32]) {
    // dst[0]: bits 0-23 (in src[0] bits 0-23)
    dst[0]  = (src[0] >> 0)  & 0xFFFFFF;
    
    // dst[1]: bits 24-47 (spans src[0] bits 24-31 + src[1] bits 0-15)
    dst[1]  = ((src[0] >> 24) | (src[1] << 8)) & 0xFFFFFF;

    // dst[2]: bits 48-71 (spans src[1] bits 16-31 + src[2] bits 0-7)
    dst[2]  = ((src[1] >> 16) | (src[2] << 16)) & 0xFFFFFF;
    
    // dst[3]: bits 72-95 (spans src[2] bits 8-31 + src[3] bits 0-0, but src[2] has exactly 24 bits)
    dst[3]  = (src[2] >> 8)  & 0xFFFFFF;

    // dst[4]: bits 96-119 (in src[3] bits 0-23)
    dst[4]  = (src[3] >> 0)  & 0xFFFFFF;
    
    // dst[5]: bits 120-143 (spans src[3] bits 24-31 + src[4] bits 0-15)
    dst[5]  = ((src[3] >> 24) | (src[4] << 8)) & 0xFFFFFF;

    // dst[6]: bits 144-167 (spans src[4] bits 16-31 + src[5] bits 0-7)
    dst[6]  = ((src[4] >> 16) | (src[5] << 16)) & 0xFFFFFF;
    
    // dst[7]: bits 168-191 (in src[5] bits 8-31)
    dst[7]  = (src[5] >> 8)  & 0xFFFFFF;

    // dst[8]: bits 192-215 (in src[6] bits 0-23)
    dst[8]  = (src[6] >> 0)  & 0xFFFFFF;
    
    // dst[9]: bits 216-239 (spans src[6] bits 24-31 + src[7] bits 0-15)
    dst[9]  = ((src[6] >> 24) | (src[7] << 8)) & 0xFFFFFF;

    // dst[10]: bits 240-263 (spans src[7] bits 16-31 + src[8] bits 0-7)
    dst[10] = ((src[7] >> 16) | (src[8] << 16)) & 0xFFFFFF;
    
    // dst[11]: bits 264-287 (in src[8] bits 8-31)
    dst[11] = (src[8] >> 8)  & 0xFFFFFF;

    // dst[12]: bits 288-311 (in src[9] bits 0-23)
    dst[12] = (src[9] >> 0)  & 0xFFFFFF;
    
    // dst[13]: bits 312-335 (spans src[9] bits 24-31 + src[10] bits 0-15)
    dst[13] = ((src[9] >> 24) | (src[10] << 8)) & 0xFFFFFF;

    // dst[14]: bits 336-359 (spans src[10] bits 16-31 + src[11] bits 0-7)
    dst[14] = ((src[10] >> 16) | (src[11] << 16)) & 0xFFFFFF;
    
    // dst[15]: bits 360-383 (in src[11] bits 8-31)
    dst[15] = (src[11] >> 8)  & 0xFFFFFF;

    // dst[16]: bits 384-407 (in src[12] bits 0-23)
    dst[16] = (src[12] >> 0)  & 0xFFFFFF;
    
    // dst[17]: bits 408-431 (spans src[12] bits 24-31 + src[13] bits 0-15)
    dst[17] = ((src[12] >> 24) | (src[13] << 8)) & 0xFFFFFF;

    // dst[18]: bits 432-455 (spans src[13] bits 16-31 + src[14] bits 0-7)
    dst[18] = ((src[13] >> 16) | (src[14] << 16)) & 0xFFFFFF;
    
    // dst[19]: bits 456-479 (in src[14] bits 8-31)
    dst[19] = (src[14] >> 8)  & 0xFFFFFF;

    // dst[20]: bits 480-503 (in src[15] bits 0-23)
    dst[20] = (src[15] >> 0)  & 0xFFFFFF;
    
    // dst[21]: bits 504-527 (spans src[15] bits 24-31 + src[16] bits 0-15)
    dst[21] = ((src[15] >> 24) | (src[16] << 8)) & 0xFFFFFF;

    // dst[22]: bits 528-551 (spans src[16] bits 16-31 + src[17] bits 0-7)
    dst[22] = ((src[16] >> 16) | (src[17] << 16)) & 0xFFFFFF;
    
    // dst[23]: bits 552-575 (in src[17] bits 8-31)
    dst[23] = (src[17] >> 8)  & 0xFFFFFF;

    // dst[24]: bits 576-599 (in src[18] bits 0-23)
    dst[24] = (src[18] >> 0)  & 0xFFFFFF;
    
    // dst[25]: bits 600-623 (spans src[18] bits 24-31 + src[19] bits 0-15)
    dst[25] = ((src[18] >> 24) | (src[19] << 8)) & 0xFFFFFF;

    // dst[26]: bits 624-647 (spans src[19] bits 16-31 + src[20] bits 0-7)
    dst[26] = ((src[19] >> 16) | (src[20] << 16)) & 0xFFFFFF;
    
    // dst[27]: bits 648-671 (in src[20] bits 8-31)
    dst[27] = (src[20] >> 8)  & 0xFFFFFF;

    // dst[28]: bits 672-695 (in src[21] bits 0-23)
    dst[28] = (src[21] >> 0)  & 0xFFFFFF;
    
    // dst[29]: bits 696-719 (spans src[21] bits 24-31 + src[22] bits 0-15)
    dst[29] = ((src[21] >> 24) | (src[22] << 8)) & 0xFFFFFF;

    // dst[30]: bits 720-743 (spans src[22] bits 16-31 + src[23] bits 0-7)
    dst[30] = ((src[22] >> 16) | (src[23] << 16)) & 0xFFFFFF;
    
    // dst[31]: bits 744-767 (in src[23] bits 8-31)
    dst[31] = (src[23] >> 8)  & 0xFFFFFF;
}
// 32 values * 24 bits = 768 bits = 24 words
pub fn compress_24_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0xFFFFFF) << 0      // bits 0-23
        | (src[1] & 0xFF) << 24;           // bits 24-31 (lower 8 bits of value 1)

    // Word 1: bits 32-63
    dst[1] = ((src[1] >> 8) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 1)
        | (src[2] & 0xFFFF) << 16;         // bits 16-31 (lower 16 bits of value 2)

    // Word 2: bits 64-95
    dst[2] = ((src[2] >> 16) & 0xFF) << 0  // bits 0-7 (upper 8 bits of value 2)
        | (src[3] & 0xFFFFFF) << 8;        // bits 8-31

    // Word 3: bits 96-127
    dst[3] = (src[4] & 0xFFFFFF) << 0      // bits 0-23
        | (src[5] & 0xFF) << 24;           // bits 24-31 (lower 8 bits of value 5)

    // Word 4: bits 128-159
    dst[4] = ((src[5] >> 8) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 5)
        | (src[6] & 0xFFFF) << 16;         // bits 16-31 (lower 16 bits of value 6)

    // Word 5: bits 160-191
    dst[5] = ((src[6] >> 16) & 0xFF) << 0  // bits 0-7 (upper 8 bits of value 6)
        | (src[7] & 0xFFFFFF) << 8;        // bits 8-31

    // Word 6: bits 192-223
    dst[6] = (src[8] & 0xFFFFFF) << 0      // bits 0-23
        | (src[9] & 0xFF) << 24;           // bits 24-31 (lower 8 bits of value 9)

    // Word 7: bits 224-255
    dst[7] = ((src[9] >> 8) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 9)
        | (src[10] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 10)

    // Word 8: bits 256-287
    dst[8] = ((src[10] >> 16) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 10)
        | (src[11] & 0xFFFFFF) << 8;       // bits 8-31

    // Word 9: bits 288-319
    dst[9] = (src[12] & 0xFFFFFF) << 0     // bits 0-23
        | (src[13] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 13)

    // Word 10: bits 320-351
    dst[10] = ((src[13] >> 8) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 13)
        | (src[14] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 14)

    // Word 11: bits 352-383
    dst[11] = ((src[14] >> 16) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 14)
        | (src[15] & 0xFFFFFF) << 8;       // bits 8-31

    // Word 12: bits 384-415
    dst[12] = (src[16] & 0xFFFFFF) << 0    // bits 0-23
        | (src[17] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 17)

    // Word 13: bits 416-447
    dst[13] = ((src[17] >> 8) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 17)
        | (src[18] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 18)

    // Word 14: bits 448-479
    dst[14] = ((src[18] >> 16) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 18)
        | (src[19] & 0xFFFFFF) << 8;       // bits 8-31

    // Word 15: bits 480-511
    dst[15] = (src[20] & 0xFFFFFF) << 0    // bits 0-23
        | (src[21] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 21)

    // Word 16: bits 512-543
    dst[16] = ((src[21] >> 8) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 21)
        | (src[22] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 22)

    // Word 17: bits 544-575
    dst[17] = ((src[22] >> 16) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 22)
        | (src[23] & 0xFFFFFF) << 8;       // bits 8-31

    // Word 18: bits 576-607
    dst[18] = (src[24] & 0xFFFFFF) << 0    // bits 0-23
        | (src[25] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 25)

    // Word 19: bits 608-639
    dst[19] = ((src[25] >> 8) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 25)
        | (src[26] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 26)

    // Word 20: bits 640-671
    dst[20] = ((src[26] >> 16) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 26)
        | (src[27] & 0xFFFFFF) << 8;       // bits 8-31

    // Word 21: bits 672-703
    dst[21] = (src[28] & 0xFFFFFF) << 0    // bits 0-23
        | (src[29] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 29)

    // Word 22: bits 704-735
    dst[22] = ((src[29] >> 8) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 29)
        | (src[30] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 30)

    // Word 23: bits 736-767
    dst[23] = ((src[30] >> 16) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 30)
        | (src[31] & 0xFFFFFF) << 8;       // bits 8-31
}

// 32 values * 25 bits = 800 bits = 25 words
pub fn decompress_25_bit(src: &[u32], dst: &mut [u32]) {
    // dst[0]: bits 0-24 (word 0, pos 0-24)
    dst[0]  = (src[0] >> 0)  & 0x1FFFFFF;
    
    // dst[1]: bits 25-49 (word 0 pos 25-31, word 1 pos 0-17)
    dst[1]  = ((src[0] >> 25) | (src[1] << 7)) & 0x1FFFFFF;

    // dst[2]: bits 50-74 (word 1 pos 18-31, word 2 pos 0-10)
    dst[2]  = ((src[1] >> 18) | (src[2] << 14)) & 0x1FFFFFF;
    
    // dst[3]: bits 75-99 (word 2 pos 11-31, word 3 pos 0-3)
    dst[3]  = ((src[2] >> 11) | (src[3] << 21)) & 0x1FFFFFF;

    // dst[4]: bits 100-124 (word 3, pos 4-28)
    dst[4]  = (src[3] >> 4) & 0x1FFFFFF;
    
    // dst[5]: bits 125-149 (word 3 pos 29-31, word 4 pos 0-21)
    dst[5]  = ((src[3] >> 29) | (src[4] << 3)) & 0x1FFFFFF;

    // dst[6]: bits 150-174 (word 4 pos 22-31, word 5 pos 0-14)
    dst[6]  = ((src[4] >> 22) | (src[5] << 10)) & 0x1FFFFFF;

    // dst[7]: bits 175-199 (word 5 pos 15-31, word 6 pos 0-7)
    dst[7]  = ((src[5] >> 15) | (src[6] << 17)) & 0x1FFFFFF;
    
    // dst[8]: bits 200-224 (word 6, pos 8-31, word 7 pos 0)
    dst[8]  = ((src[6] >> 8) | (src[7] << 24)) & 0x1FFFFFF;

    // dst[9]: bits 225-249 (word 7, pos 1-25)
    dst[9]  = (src[7] >> 1) & 0x1FFFFFF;
    
    // dst[10]: bits 250-274 (word 7 pos 26-31, word 8 pos 0-18)
    dst[10] = ((src[7] >> 26) | (src[8] << 6)) & 0x1FFFFFF;

    // dst[11]: bits 275-299 (word 8 pos 19-31, word 9 pos 0-11)
    dst[11] = ((src[8] >> 19) | (src[9] << 13)) & 0x1FFFFFF;

    // dst[12]: bits 300-324 (word 9 pos 12-31, word 10 pos 0-4)
    dst[12] = ((src[9] >> 12) | (src[10] << 20)) & 0x1FFFFFF;

    // dst[13]: bits 325-349 (word 10, pos 5-29)
    dst[13] = (src[10] >> 5) & 0x1FFFFFF;

    // dst[14]: bits 350-374 (word 10 pos 30-31, word 11 pos 0-22)
    dst[14] = ((src[10] >> 30) | (src[11] << 2)) & 0x1FFFFFF;
    
    // dst[15]: bits 375-399 (word 11 pos 23-31, word 12 pos 0-15)
    dst[15] = ((src[11] >> 23) | (src[12] << 9)) & 0x1FFFFFF;

    // dst[16]: bits 400-424 (word 12 pos 16-31, word 13 pos 0-8)
    dst[16] = ((src[12] >> 16) | (src[13] << 16)) & 0x1FFFFFF;

    // dst[17]: bits 425-449 (word 13 pos 9-31, word 14 pos 0-1)
    dst[17] = ((src[13] >> 9) | (src[14] << 23)) & 0x1FFFFFF;

    // dst[18]: bits 450-474 (word 14, pos 2-26)
    dst[18] = (src[14] >> 2) & 0x1FFFFFF;

    // dst[19]: bits 475-499 (word 14 pos 27-31, word 15 pos 0-19)
    dst[19] = ((src[14] >> 27) | (src[15] << 5)) & 0x1FFFFFF;
    
    // dst[20]: bits 500-524 (word 15 pos 20-31, word 16 pos 0-12)
    dst[20] = ((src[15] >> 20) | (src[16] << 12)) & 0x1FFFFFF;

    // dst[21]: bits 525-549 (word 16 pos 13-31, word 17 pos 0-5)
    dst[21] = ((src[16] >> 13) | (src[17] << 19)) & 0x1FFFFFF;

    // dst[22]: bits 550-574 (word 17, pos 6-30)
    dst[22] = (src[17] >> 6) & 0x1FFFFFF;

    // dst[23]: bits 575-599 (word 17 pos 31, word 18 pos 0-23)
    dst[23] = ((src[17] >> 31) | (src[18] << 1)) & 0x1FFFFFF;

    // dst[24]: bits 600-624 (word 18 pos 24-31, word 19 pos 0-16)
    dst[24] = ((src[18] >> 24) | (src[19] << 8)) & 0x1FFFFFF;
    
    // dst[25]: bits 625-649 (word 19 pos 17-31, word 20 pos 0-9)
    dst[25] = ((src[19] >> 17) | (src[20] << 15)) & 0x1FFFFFF;

    // dst[26]: bits 650-674 (word 20 pos 10-31, word 21 pos 0-2)
    dst[26] = ((src[20] >> 10) | (src[21] << 22)) & 0x1FFFFFF;

    // dst[27]: bits 675-699 (word 21, pos 3-27)
    dst[27] = (src[21] >> 3) & 0x1FFFFFF;

    // dst[28]: bits 700-724 (word 21 pos 28-31, word 22 pos 0-20)
    dst[28] = ((src[21] >> 28) | (src[22] << 4)) & 0x1FFFFFF;

    // dst[29]: bits 725-749 (word 22 pos 21-31, word 23 pos 0-13)
    dst[29] = ((src[22] >> 21) | (src[23] << 11)) & 0x1FFFFFF;
    
    // dst[30]: bits 750-774 (word 23 pos 14-31, word 24 pos 0-6)
    dst[30] = ((src[23] >> 14) | (src[24] << 18)) & 0x1FFFFFF;

    // dst[31]: bits 775-799 (word 24, pos 7-31)
    dst[31] = (src[24] >> 7) & 0x1FFFFFF;
}

// 32 values * 25 bits = 800 bits = 25 words
pub fn compress_25_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x1FFFFFF) << 0     // bits 0-24
        | (src[1] & 0x7F) << 25;           // bits 25-31 (lower 7 bits of value 1)

    // Word 1: bits 32-63
    dst[1] = ((src[1] >> 7) & 0x3FFFF) << 0 // bits 0-17 (upper 18 bits of value 1)
        | (src[2] & 0x3FFF) << 18;         // bits 18-31 (lower 14 bits of value 2)

    // Word 2: bits 64-95
    dst[2] = ((src[2] >> 14) & 0x7FF) << 0 // bits 0-10 (upper 11 bits of value 2)
        | (src[3] & 0x1FFFFF) << 11;       // bits 11-31 (lower 21 bits of value 3)

    // Word 3: bits 96-127
    dst[3] = ((src[3] >> 21) & 0xF) << 0   // bits 0-3 (upper 4 bits of value 3)
        | (src[4] & 0x1FFFFFF) << 4        // bits 4-28
        | (src[5] & 0x7) << 29;            // bits 29-31 (lower 3 bits of value 5)

    // Word 4: bits 128-159
    dst[4] = ((src[5] >> 3) & 0x3FFFFF) << 0 // bits 0-21 (upper 22 bits of value 5)
        | (src[6] & 0x3FF) << 22;          // bits 22-31 (lower 10 bits of value 6)

    // Word 5: bits 160-191
    dst[5] = ((src[6] >> 10) & 0x7FFF) << 0 // bits 0-14 (upper 15 bits of value 6)
        | (src[7] & 0x1FFFF) << 15;        // bits 15-31 (lower 17 bits of value 7)

    // Word 6: bits 192-223
    dst[6] = ((src[7] >> 17) & 0xFF) << 0  // bits 0-7 (upper 8 bits of value 7)
        | (src[8] & 0xFFFFFF) << 8;        // bits 8-31 (lower 24 bits of value 8)

    // Word 7: bits 224-255
    dst[7] = ((src[8] >> 24) & 0x1) << 0   // bit 0 (upper 1 bit of value 8)
        | (src[9] & 0x1FFFFFF) << 1        // bits 1-25
        | (src[10] & 0x3F) << 26;          // bits 26-31 (lower 6 bits of value 10)

    // Word 8: bits 256-287
    dst[8] = ((src[10] >> 6) & 0x7FFFF) << 0 // bits 0-18 (upper 19 bits of value 10)
        | (src[11] & 0x1FFF) << 19;        // bits 19-31 (lower 13 bits of value 11)

    // Word 9: bits 288-319
    dst[9] = ((src[11] >> 13) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 11)
        | (src[12] & 0xFFFFF) << 12;       // bits 12-31 (lower 20 bits of value 12)

    // Word 10: bits 320-351
    dst[10] = ((src[12] >> 20) & 0x1F) << 0 // bits 0-4 (upper 5 bits of value 12)
        | (src[13] & 0x1FFFFFF) << 5       // bits 5-29
        | (src[14] & 0x3) << 30;           // bits 30-31 (lower 2 bits of value 14)

    // Word 11: bits 352-383
    dst[11] = ((src[14] >> 2) & 0x7FFFFF) << 0 // bits 0-22 (upper 23 bits of value 14)
        | (src[15] & 0x1FF) << 23;         // bits 23-31 (lower 9 bits of value 15)

    // Word 12: bits 384-415
    dst[12] = ((src[15] >> 9) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 15)
        | (src[16] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 16)

    // Word 13: bits 416-447
    dst[13] = ((src[16] >> 16) & 0x1FF) << 0 // bits 0-8 (upper 9 bits of value 16)
        | (src[17] & 0x7FFFFF) << 9;       // bits 9-31 (lower 23 bits of value 17)

    // Word 14: bits 448-479
    dst[14] = ((src[17] >> 23) & 0x3) << 0 // bits 0-1 (upper 2 bits of value 17)
        | (src[18] & 0x1FFFFFF) << 2       // bits 2-26
        | (src[19] & 0x1F) << 27;          // bits 27-31 (lower 5 bits of value 19)

    // Word 15: bits 480-511
    dst[15] = ((src[19] >> 5) & 0xFFFFF) << 0 // bits 0-19 (upper 20 bits of value 19)
        | (src[20] & 0xFFF) << 20;         // bits 20-31 (lower 12 bits of value 20)

    // Word 16: bits 512-543
    dst[16] = ((src[20] >> 12) & 0x1FFF) << 0 // bits 0-12 (upper 13 bits of value 20)
        | (src[21] & 0x7FFFF) << 13;       // bits 13-31 (lower 19 bits of value 21)

    // Word 17: bits 544-575
    dst[17] = ((src[21] >> 19) & 0x3F) << 0 // bits 0-5 (upper 6 bits of value 21)
        | (src[22] & 0x1FFFFFF) << 6       // bits 6-30
        | (src[23] & 0x1) << 31;           // bit 31 (lower 1 bit of value 23)

    // Word 18: bits 576-607
    dst[18] = ((src[23] >> 1) & 0xFFFFFF) << 0 // bits 0-23 (upper 24 bits of value 23)
        | (src[24] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 24)

    // Word 19: bits 608-639
    dst[19] = ((src[24] >> 8) & 0x1FFFF) << 0 // bits 0-16 (upper 17 bits of value 24)
        | (src[25] & 0x7FFF) << 17;        // bits 17-31 (lower 15 bits of value 25)

    // Word 20: bits 640-671
    dst[20] = ((src[25] >> 15) & 0x3FF) << 0 // bits 0-9 (upper 10 bits of value 25)
        | (src[26] & 0x3FFFFF) << 10;      // bits 10-31 (lower 22 bits of value 26)

    // Word 21: bits 672-703
    dst[21] = ((src[26] >> 22) & 0x7) << 0 // bits 0-2 (upper 3 bits of value 26)
        | (src[27] & 0x1FFFFFF) << 3       // bits 3-27
        | (src[28] & 0xF) << 28;           // bits 28-31 (lower 4 bits of value 28)

    // Word 22: bits 704-735
    dst[22] = ((src[28] >> 4) & 0x1FFFFF) << 0 // bits 0-20 (upper 21 bits of value 28)
        | (src[29] & 0x7FF) << 21;         // bits 21-31 (lower 11 bits of value 29)

    // Word 23: bits 736-767
    dst[23] = ((src[29] >> 11) & 0x3FFF) << 0 // bits 0-13 (upper 14 bits of value 29)
        | (src[30] & 0x3FFFF) << 14;       // bits 14-31 (lower 18 bits of value 30)

    // Word 24: bits 768-799
    dst[24] = ((src[30] >> 18) & 0x7F) << 0 // bits 0-6 (upper 7 bits of value 30)
        | (src[31] & 0x1FFFFFF) << 7;      // bits 7-31
}

// 32 values * 26 bits = 832 bits = 26 words
pub fn decompress_26_bit(src: &[u32], dst: &mut [u32]) {
    // dst[0]: bits 0-25 (in src[0] bits 0-25)
    dst[0]  = (src[0] >> 0)  & 0x3FFFFFF;
    
    // dst[1]: bits 26-51 (spans src[0] bits 26-31 + src[1] bits 0-19)
    dst[1]  = ((src[0] >> 26) | (src[1] << 6)) & 0x3FFFFFF;

    // dst[2]: bits 52-77 (spans src[1] bits 20-31 + src[2] bits 0-13)
    dst[2]  = ((src[1] >> 20) | (src[2] << 12)) & 0x3FFFFFF;
    
    // dst[3]: bits 78-103 (spans src[2] bits 14-31 + src[3] bits 0-7)
    dst[3]  = ((src[2] >> 14) | (src[3] << 18)) & 0x3FFFFFF;

    // dst[4]: bits 104-129 (spans src[3] bits 8-31 + src[4] bits 0-1)
    dst[4]  = ((src[3] >> 8) | (src[4] << 24)) & 0x3FFFFFF;
    
    // dst[5]: bits 130-155 (in src[4] bits 2-27)
    dst[5]  = (src[4] >> 2)  & 0x3FFFFFF;

    // dst[6]: bits 156-181 (spans src[4] bits 28-31 + src[5] bits 0-21)
    dst[6]  = ((src[4] >> 28) | (src[5] << 4)) & 0x3FFFFFF;
    
    // dst[7]: bits 182-207 (spans src[5] bits 22-31 + src[6] bits 0-15)
    dst[7]  = ((src[5] >> 22) | (src[6] << 10)) & 0x3FFFFFF;

    // dst[8]: bits 208-233 (spans src[6] bits 16-31 + src[7] bits 0-9)
    dst[8]  = ((src[6] >> 16) | (src[7] << 16)) & 0x3FFFFFF;
    
    // dst[9]: bits 234-259 (spans src[7] bits 10-31 + src[8] bits 0-3)
    dst[9]  = ((src[7] >> 10) | (src[8] << 22)) & 0x3FFFFFF;

    // dst[10]: bits 260-285 (in src[8] bits 4-29)
    dst[10] = (src[8] >> 4)  & 0x3FFFFFF;
    
    // dst[11]: bits 286-311 (spans src[8] bits 30-31 + src[9] bits 0-23)
    dst[11] = ((src[8] >> 30) | (src[9] << 2)) & 0x3FFFFFF;

    // dst[12]: bits 312-337 (spans src[9] bits 24-31 + src[10] bits 0-17)
    dst[12] = ((src[9] >> 24) | (src[10] << 8)) & 0x3FFFFFF;
    
    // dst[13]: bits 338-363 (spans src[10] bits 18-31 + src[11] bits 0-11)
    dst[13] = ((src[10] >> 18) | (src[11] << 14)) & 0x3FFFFFF;

    // dst[14]: bits 364-389 (spans src[11] bits 12-31 + src[12] bits 0-5)
    dst[14] = ((src[11] >> 12) | (src[12] << 20)) & 0x3FFFFFF;
    
    // dst[15]: bits 390-415 (spans src[12] bits 6-31 + src[13] bits 0-0, but src[12] has exactly 26 bits from bit 6)
    dst[15] = (src[12] >> 6)  & 0x3FFFFFF;

    // dst[16]: bits 416-441 (in src[13] bits 0-25)
    dst[16] = (src[13] >> 0)  & 0x3FFFFFF;
    
    // dst[17]: bits 442-467 (spans src[13] bits 26-31 + src[14] bits 0-19)
    dst[17] = ((src[13] >> 26) | (src[14] << 6)) & 0x3FFFFFF;

    // dst[18]: bits 468-493 (spans src[14] bits 20-31 + src[15] bits 0-13)
    dst[18] = ((src[14] >> 20) | (src[15] << 12)) & 0x3FFFFFF;
    
    // dst[19]: bits 494-519 (spans src[15] bits 14-31 + src[16] bits 0-7)
    dst[19] = ((src[15] >> 14) | (src[16] << 18)) & 0x3FFFFFF;

    // dst[20]: bits 520-545 (spans src[16] bits 8-31 + src[17] bits 0-1)
    dst[20] = ((src[16] >> 8) | (src[17] << 24)) & 0x3FFFFFF;
    
    // dst[21]: bits 546-571 (in src[17] bits 2-27)
    dst[21] = (src[17] >> 2)  & 0x3FFFFFF;

    // dst[22]: bits 572-597 (spans src[17] bits 28-31 + src[18] bits 0-21)
    dst[22] = ((src[17] >> 28) | (src[18] << 4)) & 0x3FFFFFF;
    
    // dst[23]: bits 598-623 (spans src[18] bits 22-31 + src[19] bits 0-15)
    dst[23] = ((src[18] >> 22) | (src[19] << 10)) & 0x3FFFFFF;

    // dst[24]: bits 624-649 (spans src[19] bits 16-31 + src[20] bits 0-9)
    dst[24] = ((src[19] >> 16) | (src[20] << 16)) & 0x3FFFFFF;
    
    // dst[25]: bits 650-675 (spans src[20] bits 10-31 + src[21] bits 0-3)
    dst[25] = ((src[20] >> 10) | (src[21] << 22)) & 0x3FFFFFF;

    // dst[26]: bits 676-701 (in src[21] bits 4-29)
    dst[26] = (src[21] >> 4)  & 0x3FFFFFF;
    
    // dst[27]: bits 702-727 (spans src[21] bits 30-31 + src[22] bits 0-23)
    dst[27] = ((src[21] >> 30) | (src[22] << 2)) & 0x3FFFFFF;

    // dst[28]: bits 728-753 (spans src[22] bits 24-31 + src[23] bits 0-17)
    dst[28] = ((src[22] >> 24) | (src[23] << 8)) & 0x3FFFFFF;
    
    // dst[29]: bits 754-779 (spans src[23] bits 18-31 + src[24] bits 0-11)
    dst[29] = ((src[23] >> 18) | (src[24] << 14)) & 0x3FFFFFF;

    // dst[30]: bits 780-805 (spans src[24] bits 12-31 + src[25] bits 0-5)
    dst[30] = ((src[24] >> 12) | (src[25] << 20)) & 0x3FFFFFF;
    
    // dst[31]: bits 806-831 (in src[25] bits 6-31)
    dst[31] = (src[25] >> 6)  & 0x3FFFFFF;
}

// 32 values * 26 bits = 832 bits = 26 words
pub fn compress_26_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x3FFFFFF) << 0     // bits 0-25
        | (src[1] & 0x3F) << 26;           // bits 26-31 (lower 6 bits of value 1)

    // Word 1: bits 32-63
    dst[1] = ((src[1] >> 6) & 0xFFFFF) << 0 // bits 0-19 (upper 20 bits of value 1)
        | (src[2] & 0xFFF) << 20;          // bits 20-31 (lower 12 bits of value 2)

    // Word 2: bits 64-95
    dst[2] = ((src[2] >> 12) & 0x3FFF) << 0 // bits 0-13 (upper 14 bits of value 2)
        | (src[3] & 0x3FFFF) << 14;        // bits 14-31 (lower 18 bits of value 3)

    // Word 3: bits 96-127
    dst[3] = ((src[3] >> 18) & 0xFF) << 0  // bits 0-7 (upper 8 bits of value 3)
        | (src[4] & 0xFFFFFF) << 8;        // bits 8-31 (lower 24 bits of value 4)

    // Word 4: bits 128-159
    dst[4] = ((src[4] >> 24) & 0x3) << 0   // bits 0-1 (upper 2 bits of value 4)
        | (src[5] & 0x3FFFFFF) << 2        // bits 2-27
        | (src[6] & 0xF) << 28;            // bits 28-31 (lower 4 bits of value 6)

    // Word 5: bits 160-191
    dst[5] = ((src[6] >> 4) & 0x3FFFFF) << 0 // bits 0-21 (upper 22 bits of value 6)
        | (src[7] & 0x3FF) << 22;          // bits 22-31 (lower 10 bits of value 7)

    // Word 6: bits 192-223
    dst[6] = ((src[7] >> 10) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 7)
        | (src[8] & 0xFFFF) << 16;         // bits 16-31 (lower 16 bits of value 8)

    // Word 7: bits 224-255
    dst[7] = ((src[8] >> 16) & 0x3FF) << 0 // bits 0-9 (upper 10 bits of value 8)
        | (src[9] & 0x3FFFFF) << 10;       // bits 10-31 (lower 22 bits of value 9)

    // Word 8: bits 256-287
    dst[8] = ((src[9] >> 22) & 0xF) << 0   // bits 0-3 (upper 4 bits of value 9)
        | (src[10] & 0x3FFFFFF) << 4       // bits 4-29
        | (src[11] & 0x3) << 30;           // bits 30-31 (lower 2 bits of value 11)

    // Word 9: bits 288-319
    dst[9] = ((src[11] >> 2) & 0xFFFFFF) << 0 // bits 0-23 (upper 24 bits of value 11)
        | (src[12] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 12)

    // Word 10: bits 320-351
    dst[10] = ((src[12] >> 8) & 0x3FFFF) << 0 // bits 0-17 (upper 18 bits of value 12)
        | (src[13] & 0x3FFF) << 18;        // bits 18-31 (lower 14 bits of value 13)

    // Word 11: bits 352-383
    dst[11] = ((src[13] >> 14) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 13)
        | (src[14] & 0xFFFFF) << 12;       // bits 12-31 (lower 20 bits of value 14)

    // Word 12: bits 384-415
    dst[12] = ((src[14] >> 20) & 0x3F) << 0 // bits 0-5 (upper 6 bits of value 14)
        | (src[15] & 0x3FFFFFF) << 6;      // bits 6-31

    // Word 13: bits 416-447
    dst[13] = (src[16] & 0x3FFFFFF) << 0   // bits 0-25
        | (src[17] & 0x3F) << 26;          // bits 26-31 (lower 6 bits of value 17)

    // Word 14: bits 448-479
    dst[14] = ((src[17] >> 6) & 0xFFFFF) << 0 // bits 0-19 (upper 20 bits of value 17)
        | (src[18] & 0xFFF) << 20;         // bits 20-31 (lower 12 bits of value 18)

    // Word 15: bits 480-511
    dst[15] = ((src[18] >> 12) & 0x3FFF) << 0 // bits 0-13 (upper 14 bits of value 18)
        | (src[19] & 0x3FFFF) << 14;       // bits 14-31 (lower 18 bits of value 19)

    // Word 16: bits 512-543
    dst[16] = ((src[19] >> 18) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 19)
        | (src[20] & 0xFFFFFF) << 8;       // bits 8-31 (lower 24 bits of value 20)

    // Word 17: bits 544-575
    dst[17] = ((src[20] >> 24) & 0x3) << 0 // bits 0-1 (upper 2 bits of value 20)
        | (src[21] & 0x3FFFFFF) << 2       // bits 2-27
        | (src[22] & 0xF) << 28;           // bits 28-31 (lower 4 bits of value 22)

    // Word 18: bits 576-607
    dst[18] = ((src[22] >> 4) & 0x3FFFFF) << 0 // bits 0-21 (upper 22 bits of value 22)
        | (src[23] & 0x3FF) << 22;         // bits 22-31 (lower 10 bits of value 23)

    // Word 19: bits 608-639
    dst[19] = ((src[23] >> 10) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 23)
        | (src[24] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 24)

    // Word 20: bits 640-671
    dst[20] = ((src[24] >> 16) & 0x3FF) << 0 // bits 0-9 (upper 10 bits of value 24)
        | (src[25] & 0x3FFFFF) << 10;      // bits 10-31 (lower 22 bits of value 25)

    // Word 21: bits 672-703
    dst[21] = ((src[25] >> 22) & 0xF) << 0 // bits 0-3 (upper 4 bits of value 25)
        | (src[26] & 0x3FFFFFF) << 4       // bits 4-29
        | (src[27] & 0x3) << 30;           // bits 30-31 (lower 2 bits of value 27)

    // Word 22: bits 704-735
    dst[22] = ((src[27] >> 2) & 0xFFFFFF) << 0 // bits 0-23 (upper 24 bits of value 27)
        | (src[28] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 28)

    // Word 23: bits 736-767
    dst[23] = ((src[28] >> 8) & 0x3FFFF) << 0 // bits 0-17 (upper 18 bits of value 28)
        | (src[29] & 0x3FFF) << 18;        // bits 18-31 (lower 14 bits of value 29)

    // Word 24: bits 768-799
    dst[24] = ((src[29] >> 14) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 29)
        | (src[30] & 0xFFFFF) << 12;       // bits 12-31 (lower 20 bits of value 30)

    // Word 25: bits 800-831
    dst[25] = ((src[30] >> 20) & 0x3F) << 0 // bits 0-5 (upper 6 bits of value 30)
        | (src[31] & 0x3FFFFFF) << 6;      // bits 6-31
}

// 32 values * 27 bits = 864 bits = 27 words
pub fn decompress_27_bit(src: &[u32], dst: &mut [u32]) {
    // dst[0]: bits 0-26 (word 0, pos 0-26)
    dst[0]  = (src[0] >> 0)  & 0x7FFFFFF;
    
    // dst[1]: bits 27-53 (word 0 pos 27-31, word 1 pos 0-21)
    dst[1]  = ((src[0] >> 27) | (src[1] << 5)) & 0x7FFFFFF;

    // dst[2]: bits 54-80 (word 1 pos 22-31, word 2 pos 0-16)
    dst[2]  = ((src[1] >> 22) | (src[2] << 10)) & 0x7FFFFFF;
    
    // dst[3]: bits 81-107 (word 2 pos 17-31, word 3 pos 0-11)
    dst[3]  = ((src[2] >> 17) | (src[3] << 15)) & 0x7FFFFFF;

    // dst[4]: bits 108-134 (word 3 pos 12-31, word 4 pos 0-6)
    dst[4]  = ((src[3] >> 12) | (src[4] << 20)) & 0x7FFFFFF;
    
    // dst[5]: bits 135-161 (word 4 pos 7-31, word 5 pos 0-1)
    dst[5]  = ((src[4] >> 7) | (src[5] << 25)) & 0x7FFFFFF;

    // dst[6]: bits 162-188 (word 5, pos 2-28)
    dst[6]  = (src[5] >> 2) & 0x7FFFFFF;

    // dst[7]: bits 189-215 (word 5 pos 29-31, word 6 pos 0-23)
    dst[7]  = ((src[5] >> 29) | (src[6] << 3)) & 0x7FFFFFF;
    
    // dst[8]: bits 216-242 (word 6 pos 24-31, word 7 pos 0-18)
    dst[8]  = ((src[6] >> 24) | (src[7] << 8)) & 0x7FFFFFF;

    // dst[9]: bits 243-269 (word 7 pos 19-31, word 8 pos 0-13)
    dst[9]  = ((src[7] >> 19) | (src[8] << 13)) & 0x7FFFFFF;
    
    // dst[10]: bits 270-296 (word 8 pos 14-31, word 9 pos 0-8)
    dst[10] = ((src[8] >> 14) | (src[9] << 18)) & 0x7FFFFFF;

    // dst[11]: bits 297-323 (word 9 pos 9-31, word 10 pos 0-3)
    dst[11] = ((src[9] >> 9) | (src[10] << 23)) & 0x7FFFFFF;

    // dst[12]: bits 324-350 (word 10, pos 4-30)
    dst[12] = (src[10] >> 4) & 0x7FFFFFF;

    // dst[13]: bits 351-377 (word 10 pos 31, word 11 pos 0-25)
    dst[13] = ((src[10] >> 31) | (src[11] << 1)) & 0x7FFFFFF;

    // dst[14]: bits 378-404 (word 11 pos 26-31, word 12 pos 0-20)
    dst[14] = ((src[11] >> 26) | (src[12] << 6)) & 0x7FFFFFF;
    
    // dst[15]: bits 405-431 (word 12 pos 21-31, word 13 pos 0-15)
    dst[15] = ((src[12] >> 21) | (src[13] << 11)) & 0x7FFFFFF;

    // dst[16]: bits 432-458 (word 13 pos 16-31, word 14 pos 0-10)
    dst[16] = ((src[13] >> 16) | (src[14] << 16)) & 0x7FFFFFF;

    // dst[17]: bits 459-485 (word 14 pos 11-31, word 15 pos 0-5)
    dst[17] = ((src[14] >> 11) | (src[15] << 21)) & 0x7FFFFFF;

    // dst[18]: bits 486-512 (word 15 pos 6-31, word 16 pos 0)
    dst[18] = ((src[15] >> 6) | (src[16] << 26)) & 0x7FFFFFF;

    // dst[19]: bits 513-539 (word 16, pos 1-27)
    dst[19] = (src[16] >> 1) & 0x7FFFFFF;
    
    // dst[20]: bits 540-566 (word 16 pos 28-31, word 17 pos 0-22)
    dst[20] = ((src[16] >> 28) | (src[17] << 4)) & 0x7FFFFFF;

    // dst[21]: bits 567-593 (word 17 pos 23-31, word 18 pos 0-17)
    dst[21] = ((src[17] >> 23) | (src[18] << 9)) & 0x7FFFFFF;

    // dst[22]: bits 594-620 (word 18 pos 18-31, word 19 pos 0-12)
    dst[22] = ((src[18] >> 18) | (src[19] << 14)) & 0x7FFFFFF;

    // dst[23]: bits 621-647 (word 19 pos 13-31, word 20 pos 0-7)
    dst[23] = ((src[19] >> 13) | (src[20] << 19)) & 0x7FFFFFF;

    // dst[24]: bits 648-674 (word 20 pos 8-31, word 21 pos 0-2)
    dst[24] = ((src[20] >> 8) | (src[21] << 24)) & 0x7FFFFFF;
    
    // dst[25]: bits 675-701 (word 21, pos 3-29)
    dst[25] = (src[21] >> 3) & 0x7FFFFFF;

    // dst[26]: bits 702-728 (word 21 pos 30-31, word 22 pos 0-24)
    dst[26] = ((src[21] >> 30) | (src[22] << 2)) & 0x7FFFFFF;

    // dst[27]: bits 729-755 (word 22 pos 25-31, word 23 pos 0-19)
    dst[27] = ((src[22] >> 25) | (src[23] << 7)) & 0x7FFFFFF;

    // dst[28]: bits 756-782 (word 23 pos 20-31, word 24 pos 0-14)
    dst[28] = ((src[23] >> 20) | (src[24] << 12)) & 0x7FFFFFF;

    // dst[29]: bits 783-809 (word 24 pos 15-31, word 25 pos 0-9)
    dst[29] = ((src[24] >> 15) | (src[25] << 17)) & 0x7FFFFFF;
    
    // dst[30]: bits 810-836 (word 25 pos 10-31, word 26 pos 0-4)
    dst[30] = ((src[25] >> 10) | (src[26] << 22)) & 0x7FFFFFF;

    // dst[31]: bits 837-863 (word 26, pos 5-31)
    dst[31] = (src[26] >> 5) & 0x7FFFFFF;
}

// 32 values * 27 bits = 864 bits = 27 words
pub fn compress_27_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x7FFFFFF) << 0     // bits 0-26
        | (src[1] & 0x1F) << 27;           // bits 27-31 (lower 5 bits of value 1)

    // Word 1: bits 32-63
    dst[1] = ((src[1] >> 5) & 0x3FFFFF) << 0 // bits 0-21 (upper 22 bits of value 1)
        | (src[2] & 0x3FF) << 22;          // bits 22-31 (lower 10 bits of value 2)

    // Word 2: bits 64-95
    dst[2] = ((src[2] >> 10) & 0x1FFFF) << 0 // bits 0-16 (upper 17 bits of value 2)
        | (src[3] & 0x7FFF) << 17;         // bits 17-31 (lower 15 bits of value 3)

    // Word 3: bits 96-127
    dst[3] = ((src[3] >> 15) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 3)
        | (src[4] & 0xFFFFF) << 12;        // bits 12-31 (lower 20 bits of value 4)

    // Word 4: bits 128-159
    dst[4] = ((src[4] >> 20) & 0x7F) << 0  // bits 0-6 (upper 7 bits of value 4)
        | (src[5] & 0x1FFFFFF) << 7;       // bits 7-31 (lower 25 bits of value 5)

    // Word 5: bits 160-191
    dst[5] = ((src[5] >> 25) & 0x3) << 0   // bits 0-1 (upper 2 bits of value 5)
        | (src[6] & 0x7FFFFFF) << 2        // bits 2-28
        | (src[7] & 0x7) << 29;            // bits 29-31 (lower 3 bits of value 7)

    // Word 6: bits 192-223
    dst[6] = ((src[7] >> 3) & 0xFFFFFF) << 0 // bits 0-23 (upper 24 bits of value 7)
        | (src[8] & 0xFF) << 24;           // bits 24-31 (lower 8 bits of value 8)

    // Word 7: bits 224-255
    dst[7] = ((src[8] >> 8) & 0x7FFFF) << 0 // bits 0-18 (upper 19 bits of value 8)
        | (src[9] & 0x1FFF) << 19;         // bits 19-31 (lower 13 bits of value 9)

    // Word 8: bits 256-287
    dst[8] = ((src[9] >> 13) & 0x3FFF) << 0 // bits 0-13 (upper 14 bits of value 9)
        | (src[10] & 0x3FFFF) << 14;       // bits 14-31 (lower 18 bits of value 10)

    // Word 9: bits 288-319
    dst[9] = ((src[10] >> 18) & 0x1FF) << 0 // bits 0-8 (upper 9 bits of value 10)
        | (src[11] & 0x7FFFFF) << 9;       // bits 9-31 (lower 23 bits of value 11)

    // Word 10: bits 320-351
    dst[10] = ((src[11] >> 23) & 0xF) << 0 // bits 0-3 (upper 4 bits of value 11)
        | (src[12] & 0x7FFFFFF) << 4       // bits 4-30
        | (src[13] & 0x1) << 31;           // bit 31 (lower 1 bit of value 13)

    // Word 11: bits 352-383
    dst[11] = ((src[13] >> 1) & 0x3FFFFFF) << 0 // bits 0-25 (upper 26 bits of value 13)
        | (src[14] & 0x3F) << 26;          // bits 26-31 (lower 6 bits of value 14)

    // Word 12: bits 384-415
    dst[12] = ((src[14] >> 6) & 0x1FFFFF) << 0 // bits 0-20 (upper 21 bits of value 14)
        | (src[15] & 0x7FF) << 21;         // bits 21-31 (lower 11 bits of value 15)

    // Word 13: bits 416-447
    dst[13] = ((src[15] >> 11) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 15)
        | (src[16] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 16)

    // Word 14: bits 448-479
    dst[14] = ((src[16] >> 16) & 0x7FF) << 0 // bits 0-10 (upper 11 bits of value 16)
        | (src[17] & 0x1FFFFF) << 11;      // bits 11-31 (lower 21 bits of value 17)

    // Word 15: bits 480-511
    dst[15] = ((src[17] >> 21) & 0x3F) << 0 // bits 0-5 (upper 6 bits of value 17)
        | (src[18] & 0x3FFFFFF) << 6;      // bits 6-31 (lower 26 bits of value 18)

    // Word 16: bits 512-543
    dst[16] = ((src[18] >> 26) & 0x1) << 0 // bit 0 (upper 1 bit of value 18)
        | (src[19] & 0x7FFFFFF) << 1       // bits 1-27
        | (src[20] & 0xF) << 28;           // bits 28-31 (lower 4 bits of value 20)

    // Word 17: bits 544-575
    dst[17] = ((src[20] >> 4) & 0x7FFFFF) << 0 // bits 0-22 (upper 23 bits of value 20)
        | (src[21] & 0x1FF) << 23;         // bits 23-31 (lower 9 bits of value 21)

    // Word 18: bits 576-607
    dst[18] = ((src[21] >> 9) & 0x3FFFF) << 0 // bits 0-17 (upper 18 bits of value 21)
        | (src[22] & 0x3FFF) << 18;        // bits 18-31 (lower 14 bits of value 22)

    // Word 19: bits 608-639
    dst[19] = ((src[22] >> 14) & 0x1FFF) << 0 // bits 0-12 (upper 13 bits of value 22)
        | (src[23] & 0x7FFFF) << 13;       // bits 13-31 (lower 19 bits of value 23)

    // Word 20: bits 640-671
    dst[20] = ((src[23] >> 19) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 23)
        | (src[24] & 0xFFFFFF) << 8;       // bits 8-31 (lower 24 bits of value 24)

    // Word 21: bits 672-703
    dst[21] = ((src[24] >> 24) & 0x7) << 0 // bits 0-2 (upper 3 bits of value 24)
        | (src[25] & 0x7FFFFFF) << 3       // bits 3-29
        | (src[26] & 0x3) << 30;           // bits 30-31 (lower 2 bits of value 26)

    // Word 22: bits 704-735
    dst[22] = ((src[26] >> 2) & 0x1FFFFFF) << 0 // bits 0-24 (upper 25 bits of value 26)
        | (src[27] & 0x7F) << 25;          // bits 25-31 (lower 7 bits of value 27)

    // Word 23: bits 736-767
    dst[23] = ((src[27] >> 7) & 0xFFFFF) << 0 // bits 0-19 (upper 20 bits of value 27)
        | (src[28] & 0xFFF) << 20;         // bits 20-31 (lower 12 bits of value 28)

    // Word 24: bits 768-799
    dst[24] = ((src[28] >> 12) & 0x7FFF) << 0 // bits 0-14 (upper 15 bits of value 28)
        | (src[29] & 0x1FFFF) << 15;       // bits 15-31 (lower 17 bits of value 29)

    // Word 25: bits 800-831
    dst[25] = ((src[29] >> 17) & 0x3FF) << 0 // bits 0-9 (upper 10 bits of value 29)
        | (src[30] & 0x3FFFFF) << 10;      // bits 10-31 (lower 22 bits of value 30)

    // Word 26: bits 832-863
    dst[26] = ((src[30] >> 22) & 0x1F) << 0 // bits 0-4 (upper 5 bits of value 30)
        | (src[31] & 0x7FFFFFF) << 5;      // bits 5-31
}

// 32 values * 28 bits = 896 bits = 28 words
pub fn decompress_28_bit(src: &[u32], dst: &mut [u32]) {
    // dst[0]: bits 0-27 (in src[0] bits 0-27)
    dst[0]  = (src[0] >> 0)  & 0xFFFFFFF;
    
    // dst[1]: bits 28-55 (spans src[0] bits 28-31 + src[1] bits 0-23)
    dst[1]  = ((src[0] >> 28) | (src[1] << 4)) & 0xFFFFFFF;

    // dst[2]: bits 56-83 (spans src[1] bits 24-31 + src[2] bits 0-19)
    dst[2]  = ((src[1] >> 24) | (src[2] << 8)) & 0xFFFFFFF;
    
    // dst[3]: bits 84-111 (spans src[2] bits 20-31 + src[3] bits 0-15)
    dst[3]  = ((src[2] >> 20) | (src[3] << 12)) & 0xFFFFFFF;

    // dst[4]: bits 112-139 (spans src[3] bits 16-31 + src[4] bits 0-11)
    dst[4]  = ((src[3] >> 16) | (src[4] << 16)) & 0xFFFFFFF;
    
    // dst[5]: bits 140-167 (spans src[4] bits 12-31 + src[5] bits 0-7)
    dst[5]  = ((src[4] >> 12) | (src[5] << 20)) & 0xFFFFFFF;

    // dst[6]: bits 168-195 (spans src[5] bits 8-31 + src[6] bits 0-3)
    dst[6]  = ((src[5] >> 8) | (src[6] << 24)) & 0xFFFFFFF;
    
    // dst[7]: bits 196-223 (in src[6] bits 4-31)
    dst[7]  = (src[6] >> 4)  & 0xFFFFFFF;

    // dst[8]: bits 224-251 (in src[7] bits 0-27)
    dst[8]  = (src[7] >> 0)  & 0xFFFFFFF;
    
    // dst[9]: bits 252-279 (spans src[7] bits 28-31 + src[8] bits 0-23)
    dst[9]  = ((src[7] >> 28) | (src[8] << 4)) & 0xFFFFFFF;

    // dst[10]: bits 280-307 (spans src[8] bits 24-31 + src[9] bits 0-19)
    dst[10] = ((src[8] >> 24) | (src[9] << 8)) & 0xFFFFFFF;
    
    // dst[11]: bits 308-335 (spans src[9] bits 20-31 + src[10] bits 0-15)
    dst[11] = ((src[9] >> 20) | (src[10] << 12)) & 0xFFFFFFF;

    // dst[12]: bits 336-363 (spans src[10] bits 16-31 + src[11] bits 0-11)
    dst[12] = ((src[10] >> 16) | (src[11] << 16)) & 0xFFFFFFF;
    
    // dst[13]: bits 364-391 (spans src[11] bits 12-31 + src[12] bits 0-7)
    dst[13] = ((src[11] >> 12) | (src[12] << 20)) & 0xFFFFFFF;

    // dst[14]: bits 392-419 (spans src[12] bits 8-31 + src[13] bits 0-3)
    dst[14] = ((src[12] >> 8) | (src[13] << 24)) & 0xFFFFFFF;
    
    // dst[15]: bits 420-447 (in src[13] bits 4-31)
    dst[15] = (src[13] >> 4)  & 0xFFFFFFF;

    // dst[16]: bits 448-475 (in src[14] bits 0-27)
    dst[16] = (src[14] >> 0)  & 0xFFFFFFF;
    
    // dst[17]: bits 476-503 (spans src[14] bits 28-31 + src[15] bits 0-23)
    dst[17] = ((src[14] >> 28) | (src[15] << 4)) & 0xFFFFFFF;

    // dst[18]: bits 504-531 (spans src[15] bits 24-31 + src[16] bits 0-19)
    dst[18] = ((src[15] >> 24) | (src[16] << 8)) & 0xFFFFFFF;
    
    // dst[19]: bits 532-559 (spans src[16] bits 20-31 + src[17] bits 0-15)
    dst[19] = ((src[16] >> 20) | (src[17] << 12)) & 0xFFFFFFF;

    // dst[20]: bits 560-587 (spans src[17] bits 16-31 + src[18] bits 0-11)
    dst[20] = ((src[17] >> 16) | (src[18] << 16)) & 0xFFFFFFF;
    
    // dst[21]: bits 588-615 (spans src[18] bits 12-31 + src[19] bits 0-7)
    dst[21] = ((src[18] >> 12) | (src[19] << 20)) & 0xFFFFFFF;

    // dst[22]: bits 616-643 (spans src[19] bits 8-31 + src[20] bits 0-3)
    dst[22] = ((src[19] >> 8) | (src[20] << 24)) & 0xFFFFFFF;
    
    // dst[23]: bits 644-671 (in src[20] bits 4-31)
    dst[23] = (src[20] >> 4)  & 0xFFFFFFF;

    // dst[24]: bits 672-699 (in src[21] bits 0-27)
    dst[24] = (src[21] >> 0)  & 0xFFFFFFF;
    
    // dst[25]: bits 700-727 (spans src[21] bits 28-31 + src[22] bits 0-23)
    dst[25] = ((src[21] >> 28) | (src[22] << 4)) & 0xFFFFFFF;

    // dst[26]: bits 728-755 (spans src[22] bits 24-31 + src[23] bits 0-19)
    dst[26] = ((src[22] >> 24) | (src[23] << 8)) & 0xFFFFFFF;
    
    // dst[27]: bits 756-783 (spans src[23] bits 20-31 + src[24] bits 0-15)
    dst[27] = ((src[23] >> 20) | (src[24] << 12)) & 0xFFFFFFF;

    // dst[28]: bits 784-811 (spans src[24] bits 16-31 + src[25] bits 0-11)
    dst[28] = ((src[24] >> 16) | (src[25] << 16)) & 0xFFFFFFF;
    
    // dst[29]: bits 812-839 (spans src[25] bits 12-31 + src[26] bits 0-7)
    dst[29] = ((src[25] >> 12) | (src[26] << 20)) & 0xFFFFFFF;

    // dst[30]: bits 840-867 (spans src[26] bits 8-31 + src[27] bits 0-3)
    dst[30] = ((src[26] >> 8) | (src[27] << 24)) & 0xFFFFFFF;
    
    // dst[31]: bits 868-895 (in src[27] bits 4-31)
    dst[31] = (src[27] >> 4)  & 0xFFFFFFF;
}

// 32 values * 28 bits = 896 bits = 28 words
pub fn compress_28_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0xFFFFFFF) << 0     // bits 0-27
        | (src[1] & 0xF) << 28;            // bits 28-31 (lower 4 bits of value 1)

    // Word 1: bits 32-63
    dst[1] = ((src[1] >> 4) & 0xFFFFFF) << 0 // bits 0-23 (upper 24 bits of value 1)
        | (src[2] & 0xFF) << 24;           // bits 24-31 (lower 8 bits of value 2)

    // Word 2: bits 64-95
    dst[2] = ((src[2] >> 8) & 0xFFFFF) << 0 // bits 0-19 (upper 20 bits of value 2)
        | (src[3] & 0xFFF) << 20;          // bits 20-31 (lower 12 bits of value 3)

    // Word 3: bits 96-127
    dst[3] = ((src[3] >> 12) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 3)
        | (src[4] & 0xFFFF) << 16;         // bits 16-31 (lower 16 bits of value 4)

    // Word 4: bits 128-159
    dst[4] = ((src[4] >> 16) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 4)
        | (src[5] & 0xFFFFF) << 12;        // bits 12-31 (lower 20 bits of value 5)

    // Word 5: bits 160-191
    dst[5] = ((src[5] >> 20) & 0xFF) << 0  // bits 0-7 (upper 8 bits of value 5)
        | (src[6] & 0xFFFFFF) << 8;        // bits 8-31 (lower 24 bits of value 6)

    // Word 6: bits 192-223
    dst[6] = ((src[6] >> 24) & 0xF) << 0   // bits 0-3 (upper 4 bits of value 6)
        | (src[7] & 0xFFFFFFF) << 4;       // bits 4-31

    // Word 7: bits 224-255
    dst[7] = (src[8] & 0xFFFFFFF) << 0     // bits 0-27
        | (src[9] & 0xF) << 28;            // bits 28-31 (lower 4 bits of value 9)

    // Word 8: bits 256-287
    dst[8] = ((src[9] >> 4) & 0xFFFFFF) << 0 // bits 0-23 (upper 24 bits of value 9)
        | (src[10] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 10)

    // Word 9: bits 288-319
    dst[9] = ((src[10] >> 8) & 0xFFFFF) << 0 // bits 0-19 (upper 20 bits of value 10)
        | (src[11] & 0xFFF) << 20;         // bits 20-31 (lower 12 bits of value 11)

    // Word 10: bits 320-351
    dst[10] = ((src[11] >> 12) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 11)
        | (src[12] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 12)

    // Word 11: bits 352-383
    dst[11] = ((src[12] >> 16) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 12)
        | (src[13] & 0xFFFFF) << 12;       // bits 12-31 (lower 20 bits of value 13)

    // Word 12: bits 384-415
    dst[12] = ((src[13] >> 20) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 13)
        | (src[14] & 0xFFFFFF) << 8;       // bits 8-31 (lower 24 bits of value 14)

    // Word 13: bits 416-447
    dst[13] = ((src[14] >> 24) & 0xF) << 0 // bits 0-3 (upper 4 bits of value 14)
        | (src[15] & 0xFFFFFFF) << 4;      // bits 4-31

    // Word 14: bits 448-479
    dst[14] = (src[16] & 0xFFFFFFF) << 0   // bits 0-27
        | (src[17] & 0xF) << 28;           // bits 28-31 (lower 4 bits of value 17)

    // Word 15: bits 480-511
    dst[15] = ((src[17] >> 4) & 0xFFFFFF) << 0 // bits 0-23 (upper 24 bits of value 17)
        | (src[18] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 18)

    // Word 16: bits 512-543
    dst[16] = ((src[18] >> 8) & 0xFFFFF) << 0 // bits 0-19 (upper 20 bits of value 18)
        | (src[19] & 0xFFF) << 20;         // bits 20-31 (lower 12 bits of value 19)

    // Word 17: bits 544-575
    dst[17] = ((src[19] >> 12) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 19)
        | (src[20] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 20)

    // Word 18: bits 576-607
    dst[18] = ((src[20] >> 16) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 20)
        | (src[21] & 0xFFFFF) << 12;       // bits 12-31 (lower 20 bits of value 21)

    // Word 19: bits 608-639
    dst[19] = ((src[21] >> 20) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 21)
        | (src[22] & 0xFFFFFF) << 8;       // bits 8-31 (lower 24 bits of value 22)

    // Word 20: bits 640-671
    dst[20] = ((src[22] >> 24) & 0xF) << 0 // bits 0-3 (upper 4 bits of value 22)
        | (src[23] & 0xFFFFFFF) << 4;      // bits 4-31

    // Word 21: bits 672-703
    dst[21] = (src[24] & 0xFFFFFFF) << 0   // bits 0-27
        | (src[25] & 0xF) << 28;           // bits 28-31 (lower 4 bits of value 25)

    // Word 22: bits 704-735
    dst[22] = ((src[25] >> 4) & 0xFFFFFF) << 0 // bits 0-23 (upper 24 bits of value 25)
        | (src[26] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 26)

    // Word 23: bits 736-767
    dst[23] = ((src[26] >> 8) & 0xFFFFF) << 0 // bits 0-19 (upper 20 bits of value 26)
        | (src[27] & 0xFFF) << 20;         // bits 20-31 (lower 12 bits of value 27)

    // Word 24: bits 768-799
    dst[24] = ((src[27] >> 12) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 27)
        | (src[28] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 28)

    // Word 25: bits 800-831
    dst[25] = ((src[28] >> 16) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 28)
        | (src[29] & 0xFFFFF) << 12;       // bits 12-31 (lower 20 bits of value 29)

    // Word 26: bits 832-863
    dst[26] = ((src[29] >> 20) & 0xFF) << 0 // bits 0-7 (upper 8 bits of value 29)
        | (src[30] & 0xFFFFFF) << 8;       // bits 8-31 (lower 24 bits of value 30)

    // Word 27: bits 864-895
    dst[27] = ((src[30] >> 24) & 0xF) << 0 // bits 0-3 (upper 4 bits of value 30)
        | (src[31] & 0xFFFFFFF) << 4;      // bits 4-31
}

// 32 values * 29 bits = 928 bits = 29 words
pub fn decompress_29_bit(src: &[u32], dst: &mut [u32]) {
    // dst[0]: bits 0-28 (word 0, pos 0-28)
    dst[0]  = (src[0] >> 0)  & 0x1FFFFFFF;
    
    // dst[1]: bits 29-57 (word 0 pos 29-31, word 1 pos 0-25)
    dst[1]  = ((src[0] >> 29) | (src[1] << 3)) & 0x1FFFFFFF;

    // dst[2]: bits 58-86 (word 1 pos 26-31, word 2 pos 0-22)
    dst[2]  = ((src[1] >> 26) | (src[2] << 6)) & 0x1FFFFFFF;
    
    // dst[3]: bits 87-115 (word 2 pos 23-31, word 3 pos 0-19)
    dst[3]  = ((src[2] >> 23) | (src[3] << 9)) & 0x1FFFFFFF;

    // dst[4]: bits 116-144 (word 3 pos 20-31, word 4 pos 0-16)
    dst[4]  = ((src[3] >> 20) | (src[4] << 12)) & 0x1FFFFFFF;
    
    // dst[5]: bits 145-173 (word 4 pos 17-31, word 5 pos 0-13)
    dst[5]  = ((src[4] >> 17) | (src[5] << 15)) & 0x1FFFFFFF;

    // dst[6]: bits 174-202 (word 5 pos 14-31, word 6 pos 0-10)
    dst[6]  = ((src[5] >> 14) | (src[6] << 18)) & 0x1FFFFFFF;

    // dst[7]: bits 203-231 (word 6 pos 11-31, word 7 pos 0-7)
    dst[7]  = ((src[6] >> 11) | (src[7] << 21)) & 0x1FFFFFFF;
    
    // dst[8]: bits 232-260 (word 7 pos 8-31, word 8 pos 0-4)
    dst[8]  = ((src[7] >> 8) | (src[8] << 24)) & 0x1FFFFFFF;

    // dst[9]: bits 261-289 (word 8 pos 5-31, word 9 pos 0-1)
    dst[9]  = ((src[8] >> 5) | (src[9] << 27)) & 0x1FFFFFFF;
    
    // dst[10]: bits 290-318 (word 9, pos 2-30)
    dst[10] = (src[9] >> 2) & 0x1FFFFFFF;

    // dst[11]: bits 319-347 (word 9 pos 31, word 10 pos 0-27)
    dst[11] = ((src[9] >> 31) | (src[10] << 1)) & 0x1FFFFFFF;

    // dst[12]: bits 348-376 (word 10 pos 28-31, word 11 pos 0-24)
    dst[12] = ((src[10] >> 28) | (src[11] << 4)) & 0x1FFFFFFF;

    // dst[13]: bits 377-405 (word 11 pos 25-31, word 12 pos 0-21)
    dst[13] = ((src[11] >> 25) | (src[12] << 7)) & 0x1FFFFFFF;

    // dst[14]: bits 406-434 (word 12 pos 22-31, word 13 pos 0-18)
    dst[14] = ((src[12] >> 22) | (src[13] << 10)) & 0x1FFFFFFF;
    
    // dst[15]: bits 435-463 (word 13 pos 19-31, word 14 pos 0-15)
    dst[15] = ((src[13] >> 19) | (src[14] << 13)) & 0x1FFFFFFF;

    // dst[16]: bits 464-492 (word 14 pos 16-31, word 15 pos 0-12)
    dst[16] = ((src[14] >> 16) | (src[15] << 16)) & 0x1FFFFFFF;

    // dst[17]: bits 493-521 (word 15 pos 13-31, word 16 pos 0-9)
    dst[17] = ((src[15] >> 13) | (src[16] << 19)) & 0x1FFFFFFF;

    // dst[18]: bits 522-550 (word 16 pos 10-31, word 17 pos 0-6)
    dst[18] = ((src[16] >> 10) | (src[17] << 22)) & 0x1FFFFFFF;

    // dst[19]: bits 551-579 (word 17 pos 7-31, word 18 pos 0-3)
    dst[19] = ((src[17] >> 7) | (src[18] << 25)) & 0x1FFFFFFF;
    
    // dst[20]: bits 580-608 (word 18, pos 4-31, word 19 pos 0)
    dst[20] = ((src[18] >> 4) | (src[19] << 28)) & 0x1FFFFFFF;

    // dst[21]: bits 609-637 (word 19, pos 1-29)
    dst[21] = (src[19] >> 1) & 0x1FFFFFFF;

    // dst[22]: bits 638-666 (word 19 pos 30-31, word 20 pos 0-26)
    dst[22] = ((src[19] >> 30) | (src[20] << 2)) & 0x1FFFFFFF;

    // dst[23]: bits 667-695 (word 20 pos 27-31, word 21 pos 0-23)
    dst[23] = ((src[20] >> 27) | (src[21] << 5)) & 0x1FFFFFFF;

    // dst[24]: bits 696-724 (word 21 pos 24-31, word 22 pos 0-20)
    dst[24] = ((src[21] >> 24) | (src[22] << 8)) & 0x1FFFFFFF;
    
    // dst[25]: bits 725-753 (word 22 pos 21-31, word 23 pos 0-17)
    dst[25] = ((src[22] >> 21) | (src[23] << 11)) & 0x1FFFFFFF;

    // dst[26]: bits 754-782 (word 23 pos 18-31, word 24 pos 0-14)
    dst[26] = ((src[23] >> 18) | (src[24] << 14)) & 0x1FFFFFFF;

    // dst[27]: bits 783-811 (word 24 pos 15-31, word 25 pos 0-11)
    dst[27] = ((src[24] >> 15) | (src[25] << 17)) & 0x1FFFFFFF;

    // dst[28]: bits 812-840 (word 25 pos 12-31, word 26 pos 0-8)
    dst[28] = ((src[25] >> 12) | (src[26] << 20)) & 0x1FFFFFFF;

    // dst[29]: bits 841-869 (word 26 pos 9-31, word 27 pos 0-5)
    dst[29] = ((src[26] >> 9) | (src[27] << 23)) & 0x1FFFFFFF;
    
    // dst[30]: bits 870-898 (word 27 pos 6-31, word 28 pos 0-2)
    dst[30] = ((src[27] >> 6) | (src[28] << 26)) & 0x1FFFFFFF;

    // dst[31]: bits 899-927 (word 28, pos 3-31)
    dst[31] = (src[28] >> 3) & 0x1FFFFFFF;
}

// 32 values * 29 bits = 928 bits = 29 words
pub fn compress_29_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] & 0x1FFFFFFF) << 0    // bits 0-28
        | (src[1] & 0x7) << 29;            // bits 29-31 (lower 3 bits of value 1)

    // Word 1: bits 32-63
    dst[1] = ((src[1] >> 3) & 0x3FFFFFF) << 0 // bits 0-25 (upper 26 bits of value 1)
        | (src[2] & 0x3F) << 26;           // bits 26-31 (lower 6 bits of value 2)

    // Word 2: bits 64-95
    dst[2] = ((src[2] >> 6) & 0x7FFFFF) << 0 // bits 0-22 (upper 23 bits of value 2)
        | (src[3] & 0x1FF) << 23;          // bits 23-31 (lower 9 bits of value 3)

    // Word 3: bits 96-127
    dst[3] = ((src[3] >> 9) & 0xFFFFF) << 0 // bits 0-19 (upper 20 bits of value 3)
        | (src[4] & 0xFFF) << 20;          // bits 20-31 (lower 12 bits of value 4)

    // Word 4: bits 128-159
    dst[4] = ((src[4] >> 12) & 0x1FFFF) << 0 // bits 0-16 (upper 17 bits of value 4)
        | (src[5] & 0x7FFF) << 17;         // bits 17-31 (lower 15 bits of value 5)

    // Word 5: bits 160-191
    dst[5] = ((src[5] >> 15) & 0x3FFF) << 0 // bits 0-13 (upper 14 bits of value 5)
        | (src[6] & 0x3FFFF) << 14;        // bits 14-31 (lower 18 bits of value 6)

    // Word 6: bits 192-223
    dst[6] = ((src[6] >> 18) & 0x7FF) << 0 // bits 0-10 (upper 11 bits of value 6)
        | (src[7] & 0x1FFFFF) << 11;       // bits 11-31 (lower 21 bits of value 7)

    // Word 7: bits 224-255
    dst[7] = ((src[7] >> 21) & 0xFF) << 0  // bits 0-7 (upper 8 bits of value 7)
        | (src[8] & 0xFFFFFF) << 8;        // bits 8-31 (lower 24 bits of value 8)

    // Word 8: bits 256-287
    dst[8] = ((src[8] >> 24) & 0x1F) << 0  // bits 0-4 (upper 5 bits of value 8)
        | (src[9] & 0x7FFFFFF) << 5;       // bits 5-31 (lower 27 bits of value 9)

    // Word 9: bits 288-319
    dst[9] = ((src[9] >> 27) & 0x3) << 0   // bits 0-1 (upper 2 bits of value 9)
        | (src[10] & 0x1FFFFFFF) << 2      // bits 2-30
        | (src[11] & 0x1) << 31;           // bit 31 (lower 1 bit of value 11)

    // Word 10: bits 320-351
    dst[10] = ((src[11] >> 1) & 0xFFFFFFF) << 0 // bits 0-27 (upper 28 bits of value 11)
        | (src[12] & 0xF) << 28;           // bits 28-31 (lower 4 bits of value 12)

    // Word 11: bits 352-383
    dst[11] = ((src[12] >> 4) & 0x1FFFFFF) << 0 // bits 0-24 (upper 25 bits of value 12)
        | (src[13] & 0x7F) << 25;          // bits 25-31 (lower 7 bits of value 13)

    // Word 12: bits 384-415
    dst[12] = ((src[13] >> 7) & 0x3FFFFF) << 0 // bits 0-21 (upper 22 bits of value 13)
        | (src[14] & 0x3FF) << 22;         // bits 22-31 (lower 10 bits of value 14)

    // Word 13: bits 416-447
    dst[13] = ((src[14] >> 10) & 0x7FFFF) << 0 // bits 0-18 (upper 19 bits of value 14)
        | (src[15] & 0x1FFF) << 19;        // bits 19-31 (lower 13 bits of value 15)

    // Word 14: bits 448-479
    dst[14] = ((src[15] >> 13) & 0xFFFF) << 0 // bits 0-15 (upper 16 bits of value 15)
        | (src[16] & 0xFFFF) << 16;        // bits 16-31 (lower 16 bits of value 16)

    // Word 15: bits 480-511
    dst[15] = ((src[16] >> 16) & 0x1FFF) << 0 // bits 0-12 (upper 13 bits of value 16)
        | (src[17] & 0x7FFFF) << 13;       // bits 13-31 (lower 19 bits of value 17)

    // Word 16: bits 512-543
    dst[16] = ((src[17] >> 19) & 0x3FF) << 0 // bits 0-9 (upper 10 bits of value 17)
        | (src[18] & 0x3FFFFF) << 10;      // bits 10-31 (lower 22 bits of value 18)

    // Word 17: bits 544-575
    dst[17] = ((src[18] >> 22) & 0x7F) << 0 // bits 0-6 (upper 7 bits of value 18)
        | (src[19] & 0x1FFFFFF) << 7;      // bits 7-31 (lower 25 bits of value 19)

    // Word 18: bits 576-607
    dst[18] = ((src[19] >> 25) & 0xF) << 0 // bits 0-3 (upper 4 bits of value 19)
        | (src[20] & 0xFFFFFFF) << 4;      // bits 4-31 (lower 28 bits of value 20)

    // Word 19: bits 608-639
    dst[19] = ((src[20] >> 28) & 0x1) << 0 // bit 0 (upper 1 bit of value 20)
        | (src[21] & 0x1FFFFFFF) << 1      // bits 1-29
        | (src[22] & 0x3) << 30;           // bits 30-31 (lower 2 bits of value 22)

    // Word 20: bits 640-671
    dst[20] = ((src[22] >> 2) & 0x7FFFFFF) << 0 // bits 0-26 (upper 27 bits of value 22)
        | (src[23] & 0x1F) << 27;          // bits 27-31 (lower 5 bits of value 23)

    // Word 21: bits 672-703
    dst[21] = ((src[23] >> 5) & 0xFFFFFF) << 0 // bits 0-23 (upper 24 bits of value 23)
        | (src[24] & 0xFF) << 24;          // bits 24-31 (lower 8 bits of value 24)

    // Word 22: bits 704-735
    dst[22] = ((src[24] >> 8) & 0x1FFFFF) << 0 // bits 0-20 (upper 21 bits of value 24)
        | (src[25] & 0x7FF) << 21;         // bits 21-31 (lower 11 bits of value 25)

    // Word 23: bits 736-767
    dst[23] = ((src[25] >> 11) & 0x3FFFF) << 0 // bits 0-17 (upper 18 bits of value 25)
        | (src[26] & 0x3FFF) << 18;        // bits 18-31 (lower 14 bits of value 26)

    // Word 24: bits 768-799
    dst[24] = ((src[26] >> 14) & 0x7FFF) << 0 // bits 0-14 (upper 15 bits of value 26)
        | (src[27] & 0x1FFFF) << 15;       // bits 15-31 (lower 17 bits of value 27)

    // Word 25: bits 800-831
    dst[25] = ((src[27] >> 17) & 0xFFF) << 0 // bits 0-11 (upper 12 bits of value 27)
        | (src[28] & 0xFFFFF) << 12;       // bits 12-31 (lower 20 bits of value 28)

    // Word 26: bits 832-863
    dst[26] = ((src[28] >> 20) & 0x1FF) << 0 // bits 0-8 (upper 9 bits of value 28)
        | (src[29] & 0x7FFFFF) << 9;       // bits 9-31 (lower 23 bits of value 29)

    // Word 27: bits 864-895
    dst[27] = ((src[29] >> 23) & 0x3F) << 0 // bits 0-5 (upper 6 bits of value 29)
        | (src[30] & 0x3FFFFFF) << 6;      // bits 6-31 (lower 26 bits of value 30)

    // Word 28: bits 896-927
    dst[28] = ((src[30] >> 26) & 0x7) << 0 // bits 0-2 (upper 3 bits of value 30)
        | (src[31] & 0x1FFFFFFF) << 3;     // bits 3-31
}

// 32 values * 30 bits = 960 bits = 30 words
pub fn decompress_30_bit(src: &[u32], dst: &mut [u32]) {
    // dst[0]: bits 0-29 (in src[0] bits 0-29)
    dst[0]  = (src[0] >> 0)  & 0x3FFFFFFF;
    
    // dst[1]: bits 30-59 (spans src[0] bits 30-31 + src[1] bits 0-27)
    dst[1]  = ((src[0] >> 30) | (src[1] << 2)) & 0x3FFFFFFF;

    // dst[2]: bits 60-89 (spans src[1] bits 28-31 + src[2] bits 0-25)
    dst[2]  = ((src[1] >> 28) | (src[2] << 4)) & 0x3FFFFFFF;
    
    // dst[3]: bits 90-119 (spans src[2] bits 26-31 + src[3] bits 0-23)
    dst[3]  = ((src[2] >> 26) | (src[3] << 6)) & 0x3FFFFFFF;

    // dst[4]: bits 120-149 (spans src[3] bits 24-31 + src[4] bits 0-21)
    dst[4]  = ((src[3] >> 24) | (src[4] << 8)) & 0x3FFFFFFF;
    
    // dst[5]: bits 150-179 (spans src[4] bits 22-31 + src[5] bits 0-19)
    dst[5]  = ((src[4] >> 22) | (src[5] << 10)) & 0x3FFFFFFF;

    // dst[6]: bits 180-209 (spans src[5] bits 20-31 + src[6] bits 0-17)
    dst[6]  = ((src[5] >> 20) | (src[6] << 12)) & 0x3FFFFFFF;
    
    // dst[7]: bits 210-239 (spans src[6] bits 18-31 + src[7] bits 0-15)
    dst[7]  = ((src[6] >> 18) | (src[7] << 14)) & 0x3FFFFFFF;

    // dst[8]: bits 240-269 (spans src[7] bits 16-31 + src[8] bits 0-13)
    dst[8]  = ((src[7] >> 16) | (src[8] << 16)) & 0x3FFFFFFF;
    
    // dst[9]: bits 270-299 (spans src[8] bits 14-31 + src[9] bits 0-11)
    dst[9]  = ((src[8] >> 14) | (src[9] << 18)) & 0x3FFFFFFF;

    // dst[10]: bits 300-329 (spans src[9] bits 12-31 + src[10] bits 0-9)
    dst[10] = ((src[9] >> 12) | (src[10] << 20)) & 0x3FFFFFFF;
    
    // dst[11]: bits 330-359 (spans src[10] bits 10-31 + src[11] bits 0-7)
    dst[11] = ((src[10] >> 10) | (src[11] << 22)) & 0x3FFFFFFF;

    // dst[12]: bits 360-389 (spans src[11] bits 8-31 + src[12] bits 0-5)
    dst[12] = ((src[11] >> 8) | (src[12] << 24)) & 0x3FFFFFFF;
    
    // dst[13]: bits 390-419 (spans src[12] bits 6-31 + src[13] bits 0-3)
    dst[13] = ((src[12] >> 6) | (src[13] << 26)) & 0x3FFFFFFF;

    // dst[14]: bits 420-449 (spans src[13] bits 4-31 + src[14] bits 0-1)
    dst[14] = ((src[13] >> 4) | (src[14] << 28)) & 0x3FFFFFFF;
    
    // dst[15]: bits 450-479 (in src[14] bits 2-31)
    dst[15] = (src[14] >> 2)  & 0x3FFFFFFF;

    // dst[16]: bits 480-509 (in src[15] bits 0-29)
    dst[16] = (src[15] >> 0)  & 0x3FFFFFFF;
    
    // dst[17]: bits 510-539 (spans src[15] bits 30-31 + src[16] bits 0-27)
    dst[17] = ((src[15] >> 30) | (src[16] << 2)) & 0x3FFFFFFF;

    // dst[18]: bits 540-569 (spans src[16] bits 28-31 + src[17] bits 0-25)
    dst[18] = ((src[16] >> 28) | (src[17] << 4)) & 0x3FFFFFFF;
    
    // dst[19]: bits 570-599 (spans src[17] bits 26-31 + src[18] bits 0-23)
    dst[19] = ((src[17] >> 26) | (src[18] << 6)) & 0x3FFFFFFF;

    // dst[20]: bits 600-629 (spans src[18] bits 24-31 + src[19] bits 0-21)
    dst[20] = ((src[18] >> 24) | (src[19] << 8)) & 0x3FFFFFFF;
    
    // dst[21]: bits 630-659 (spans src[19] bits 22-31 + src[20] bits 0-19)
    dst[21] = ((src[19] >> 22) | (src[20] << 10)) & 0x3FFFFFFF;

    // dst[22]: bits 660-689 (spans src[20] bits 20-31 + src[21] bits 0-17)
    dst[22] = ((src[20] >> 20) | (src[21] << 12)) & 0x3FFFFFFF;
    
    // dst[23]: bits 690-719 (spans src[21] bits 18-31 + src[22] bits 0-15)
    dst[23] = ((src[21] >> 18) | (src[22] << 14)) & 0x3FFFFFFF;

    // dst[24]: bits 720-749 (spans src[22] bits 16-31 + src[23] bits 0-13)
    dst[24] = ((src[22] >> 16) | (src[23] << 16)) & 0x3FFFFFFF;
    
    // dst[25]: bits 750-779 (spans src[23] bits 14-31 + src[24] bits 0-11)
    dst[25] = ((src[23] >> 14) | (src[24] << 18)) & 0x3FFFFFFF;

    // dst[26]: bits 780-809 (spans src[24] bits 12-31 + src[25] bits 0-9)
    dst[26] = ((src[24] >> 12) | (src[25] << 20)) & 0x3FFFFFFF;
    
    // dst[27]: bits 810-839 (spans src[25] bits 10-31 + src[26] bits 0-7)
    dst[27] = ((src[25] >> 10) | (src[26] << 22)) & 0x3FFFFFFF;

    // dst[28]: bits 840-869 (spans src[26] bits 8-31 + src[27] bits 0-5)
    dst[28] = ((src[26] >> 8) | (src[27] << 24)) & 0x3FFFFFFF;
    
    // dst[29]: bits 870-899 (spans src[27] bits 6-31 + src[28] bits 0-3)
    dst[29] = ((src[27] >> 6) | (src[28] << 26)) & 0x3FFFFFFF;

    // dst[30]: bits 900-929 (spans src[28] bits 4-31 + src[29] bits 0-1)
    dst[30] = ((src[28] >> 4) | (src[29] << 28)) & 0x3FFFFFFF;
    
    // dst[31]: bits 930-959 (in src[29] bits 2-31)
    dst[31] = (src[29] >> 2)  & 0x3FFFFFFF;
}

// Compress 32 values of 30 bits each into 30 words of 32 bits
pub fn compress_30_bit(src: &[u32], dst: &mut [u32]) {
    // dst[0]: src[0] bits 0-29 at positions 0-29, src[1] bits 0-1 at positions 30-31
    dst[0] = (src[0] & 0x3FFFFFFF) | ((src[1] & 0x3) << 30);
    
    // dst[1]: src[1] bits 2-29 at positions 0-27, src[2] bits 0-3 at positions 28-31
    dst[1] = ((src[1] >> 2) & 0xFFFFFFF) | ((src[2] & 0xF) << 28);
    
    // dst[2]: src[2] bits 4-29 at positions 0-25, src[3] bits 0-5 at positions 26-31
    dst[2] = ((src[2] >> 4) & 0x3FFFFFF) | ((src[3] & 0x3F) << 26);
    
    // dst[3]: src[3] bits 6-29 at positions 0-23, src[4] bits 0-7 at positions 24-31
    dst[3] = ((src[3] >> 6) & 0xFFFFFF) | ((src[4] & 0xFF) << 24);
    
    // dst[4]: src[4] bits 8-29 at positions 0-21, src[5] bits 0-9 at positions 22-31
    dst[4] = ((src[4] >> 8) & 0x3FFFFF) | ((src[5] & 0x3FF) << 22);
    
    // dst[5]: src[5] bits 10-29 at positions 0-19, src[6] bits 0-11 at positions 20-31
    dst[5] = ((src[5] >> 10) & 0xFFFFF) | ((src[6] & 0xFFF) << 20);
    
    // dst[6]: src[6] bits 12-29 at positions 0-17, src[7] bits 0-13 at positions 18-31
    dst[6] = ((src[6] >> 12) & 0x3FFFF) | ((src[7] & 0x3FFF) << 18);
    
    // dst[7]: src[7] bits 14-29 at positions 0-15, src[8] bits 0-15 at positions 16-31
    dst[7] = ((src[7] >> 14) & 0xFFFF) | ((src[8] & 0xFFFF) << 16);
    
    // dst[8]: src[8] bits 16-29 at positions 0-13, src[9] bits 0-17 at positions 14-31
    dst[8] = ((src[8] >> 16) & 0x3FFF) | ((src[9] & 0x3FFFF) << 14);
    
    // dst[9]: src[9] bits 18-29 at positions 0-11, src[10] bits 0-19 at positions 12-31
    dst[9] = ((src[9] >> 18) & 0xFFF) | ((src[10] & 0xFFFFF) << 12);
    
    // dst[10]: src[10] bits 20-29 at positions 0-9, src[11] bits 0-21 at positions 10-31
    dst[10] = ((src[10] >> 20) & 0x3FF) | ((src[11] & 0x3FFFFF) << 10);
    
    // dst[11]: src[11] bits 22-29 at positions 0-7, src[12] bits 0-23 at positions 8-31
    dst[11] = ((src[11] >> 22) & 0xFF) | ((src[12] & 0xFFFFFF) << 8);
    
    // dst[12]: src[12] bits 24-29 at positions 0-5, src[13] bits 0-25 at positions 6-31
    dst[12] = ((src[12] >> 24) & 0x3F) | ((src[13] & 0x3FFFFFF) << 6);
    
    // dst[13]: src[13] bits 26-29 at positions 0-3, src[14] bits 0-27 at positions 4-31
    dst[13] = ((src[13] >> 26) & 0xF) | ((src[14] & 0xFFFFFFF) << 4);
    
    // dst[14]: src[14] bits 28-29 at positions 0-1, src[15] bits 0-29 at positions 2-31
    dst[14] = ((src[14] >> 28) & 0x3) | ((src[15] & 0x3FFFFFFF) << 2);
    
    // dst[15]: src[16] bits 0-29 at positions 0-29, src[17] bits 0-1 at positions 30-31
    dst[15] = (src[16] & 0x3FFFFFFF) | ((src[17] & 0x3) << 30);
    
    // dst[16]: src[17] bits 2-29 at positions 0-27, src[18] bits 0-3 at positions 28-31
    dst[16] = ((src[17] >> 2) & 0xFFFFFFF) | ((src[18] & 0xF) << 28);
    
    // dst[17]: src[18] bits 4-29 at positions 0-25, src[19] bits 0-5 at positions 26-31
    dst[17] = ((src[18] >> 4) & 0x3FFFFFF) | ((src[19] & 0x3F) << 26);
    
    // dst[18]: src[19] bits 6-29 at positions 0-23, src[20] bits 0-7 at positions 24-31
    dst[18] = ((src[19] >> 6) & 0xFFFFFF) | ((src[20] & 0xFF) << 24);
    
    // dst[19]: src[20] bits 8-29 at positions 0-21, src[21] bits 0-9 at positions 22-31
    dst[19] = ((src[20] >> 8) & 0x3FFFFF) | ((src[21] & 0x3FF) << 22);
    
    // dst[20]: src[21] bits 10-29 at positions 0-19, src[22] bits 0-11 at positions 20-31
    dst[20] = ((src[21] >> 10) & 0xFFFFF) | ((src[22] & 0xFFF) << 20);
    
    // dst[21]: src[22] bits 12-29 at positions 0-17, src[23] bits 0-13 at positions 18-31
    dst[21] = ((src[22] >> 12) & 0x3FFFF) | ((src[23] & 0x3FFF) << 18);
    
    // dst[22]: src[23] bits 14-29 at positions 0-15, src[24] bits 0-15 at positions 16-31
    dst[22] = ((src[23] >> 14) & 0xFFFF) | ((src[24] & 0xFFFF) << 16);
    
    // dst[23]: src[24] bits 16-29 at positions 0-13, src[25] bits 0-17 at positions 14-31
    dst[23] = ((src[24] >> 16) & 0x3FFF) | ((src[25] & 0x3FFFF) << 14);
    
    // dst[24]: src[25] bits 18-29 at positions 0-11, src[26] bits 0-19 at positions 12-31
    dst[24] = ((src[25] >> 18) & 0xFFF) | ((src[26] & 0xFFFFF) << 12);
    
    // dst[25]: src[26] bits 20-29 at positions 0-9, src[27] bits 0-21 at positions 10-31
    dst[25] = ((src[26] >> 20) & 0x3FF) | ((src[27] & 0x3FFFFF) << 10);
    
    // dst[26]: src[27] bits 22-29 at positions 0-7, src[28] bits 0-23 at positions 8-31
    dst[26] = ((src[27] >> 22) & 0xFF) | ((src[28] & 0xFFFFFF) << 8);
    
    // dst[27]: src[28] bits 24-29 at positions 0-5, src[29] bits 0-25 at positions 6-31
    dst[27] = ((src[28] >> 24) & 0x3F) | ((src[29] & 0x3FFFFFF) << 6);
    
    // dst[28]: src[29] bits 26-29 at positions 0-3, src[30] bits 0-27 at positions 4-31
    dst[28] = ((src[29] >> 26) & 0xF) | ((src[30] & 0xFFFFFFF) << 4);
    
    // dst[29]: src[30] bits 28-29 at positions 0-1, src[31] bits 0-29 at positions 2-31
    dst[29] = ((src[30] >> 28) & 0x3) | ((src[31] & 0x3FFFFFFF) << 2);
}

// 32 values * 31 bits = 992 bits = 31 words
pub fn decompress_31_bit(src: &[u32], dst: &mut [u32]) {
    // dst[0]: bits 0-30 (word 0, pos 0-30)
    dst[0]  = (src[0] >> 0)  & 0x7FFFFFFF;
    
    // dst[1]: bits 31-61 (word 0 pos 31, word 1 pos 0-29)
    dst[1]  = ((src[0] >> 31) | (src[1] << 1)) & 0x7FFFFFFF;

    // dst[2]: bits 62-92 (word 1 pos 30-31, word 2 pos 0-28)
    dst[2]  = ((src[1] >> 30) | (src[2] << 2)) & 0x7FFFFFFF;
    
    // dst[3]: bits 93-123 (word 2 pos 29-31, word 3 pos 0-27)
    dst[3]  = ((src[2] >> 29) | (src[3] << 3)) & 0x7FFFFFFF;

    // dst[4]: bits 124-154 (word 3 pos 28-31, word 4 pos 0-26)
    dst[4]  = ((src[3] >> 28) | (src[4] << 4)) & 0x7FFFFFFF;
    
    // dst[5]: bits 155-185 (word 4 pos 27-31, word 5 pos 0-25)
    dst[5]  = ((src[4] >> 27) | (src[5] << 5)) & 0x7FFFFFFF;

    // dst[6]: bits 186-216 (word 5 pos 26-31, word 6 pos 0-24)
    dst[6]  = ((src[5] >> 26) | (src[6] << 6)) & 0x7FFFFFFF;

    // dst[7]: bits 217-247 (word 6 pos 25-31, word 7 pos 0-23)
    dst[7]  = ((src[6] >> 25) | (src[7] << 7)) & 0x7FFFFFFF;
    
    // dst[8]: bits 248-278 (word 7 pos 24-31, word 8 pos 0-22)
    dst[8]  = ((src[7] >> 24) | (src[8] << 8)) & 0x7FFFFFFF;

    // dst[9]: bits 279-309 (word 8 pos 23-31, word 9 pos 0-21)
    dst[9]  = ((src[8] >> 23) | (src[9] << 9)) & 0x7FFFFFFF;
    
    // dst[10]: bits 310-340 (word 9 pos 22-31, word 10 pos 0-20)
    dst[10] = ((src[9] >> 22) | (src[10] << 10)) & 0x7FFFFFFF;

    // dst[11]: bits 341-371 (word 10 pos 21-31, word 11 pos 0-19)
    dst[11] = ((src[10] >> 21) | (src[11] << 11)) & 0x7FFFFFFF;

    // dst[12]: bits 372-402 (word 11 pos 20-31, word 12 pos 0-18)
    dst[12] = ((src[11] >> 20) | (src[12] << 12)) & 0x7FFFFFFF;

    // dst[13]: bits 403-433 (word 12 pos 19-31, word 13 pos 0-17)
    dst[13] = ((src[12] >> 19) | (src[13] << 13)) & 0x7FFFFFFF;

    // dst[14]: bits 434-464 (word 13 pos 18-31, word 14 pos 0-16)
    dst[14] = ((src[13] >> 18) | (src[14] << 14)) & 0x7FFFFFFF;
    
    // dst[15]: bits 465-495 (word 14 pos 17-31, word 15 pos 0-15)
    dst[15] = ((src[14] >> 17) | (src[15] << 15)) & 0x7FFFFFFF;

    // dst[16]: bits 496-526 (word 15 pos 16-31, word 16 pos 0-14)
    dst[16] = ((src[15] >> 16) | (src[16] << 16)) & 0x7FFFFFFF;

    // dst[17]: bits 527-557 (word 16 pos 15-31, word 17 pos 0-13)
    dst[17] = ((src[16] >> 15) | (src[17] << 17)) & 0x7FFFFFFF;

    // dst[18]: bits 558-588 (word 17 pos 14-31, word 18 pos 0-12)
    dst[18] = ((src[17] >> 14) | (src[18] << 18)) & 0x7FFFFFFF;

    // dst[19]: bits 589-619 (word 18 pos 13-31, word 19 pos 0-11)
    dst[19] = ((src[18] >> 13) | (src[19] << 19)) & 0x7FFFFFFF;
    
    // dst[20]: bits 620-650 (word 19 pos 12-31, word 20 pos 0-10)
    dst[20] = ((src[19] >> 12) | (src[20] << 20)) & 0x7FFFFFFF;

    // dst[21]: bits 651-681 (word 20 pos 11-31, word 21 pos 0-9)
    dst[21] = ((src[20] >> 11) | (src[21] << 21)) & 0x7FFFFFFF;

    // dst[22]: bits 682-712 (word 21 pos 10-31, word 22 pos 0-8)
    dst[22] = ((src[21] >> 10) | (src[22] << 22)) & 0x7FFFFFFF;

    // dst[23]: bits 713-743 (word 22 pos 9-31, word 23 pos 0-7)
    dst[23] = ((src[22] >> 9) | (src[23] << 23)) & 0x7FFFFFFF;

    // dst[24]: bits 744-774 (word 23 pos 8-31, word 24 pos 0-6)
    dst[24] = ((src[23] >> 8) | (src[24] << 24)) & 0x7FFFFFFF;
    
    // dst[25]: bits 775-805 (word 24 pos 7-31, word 25 pos 0-5)
    dst[25] = ((src[24] >> 7) | (src[25] << 25)) & 0x7FFFFFFF;

    // dst[26]: bits 806-836 (word 25 pos 6-31, word 26 pos 0-4)
    dst[26] = ((src[25] >> 6) | (src[26] << 26)) & 0x7FFFFFFF;

    // dst[27]: bits 837-867 (word 26 pos 5-31, word 27 pos 0-3)
    dst[27] = ((src[26] >> 5) | (src[27] << 27)) & 0x7FFFFFFF;

    // dst[28]: bits 868-898 (word 27 pos 4-31, word 28 pos 0-2)
    dst[28] = ((src[27] >> 4) | (src[28] << 28)) & 0x7FFFFFFF;

    // dst[29]: bits 899-929 (word 28 pos 3-31, word 29 pos 0-1)
    dst[29] = ((src[28] >> 3) | (src[29] << 29)) & 0x7FFFFFFF;
    
    // dst[30]: bits 930-960 (word 29 pos 2-31, word 30 pos 0)
    dst[30] = ((src[29] >> 2) | (src[30] << 30)) & 0x7FFFFFFF;

    // dst[31]: bits 961-991 (word 30 pos 1-31)
    dst[31] = (src[30] >> 1) & 0x7FFFFFFF;
}

// Compress 32 values of 31 bits each into 31 words of 32 bits
pub fn compress_31_bit(src: &[u32], dst: &mut [u32]) {
    
    // dst[0]: src[0] bits 0-30 at positions 0-30, src[1] bit 0 at position 31
    dst[0] = (src[0] & 0x7FFFFFFF) | ((src[1] & 0x1) << 31);
    
    // dst[1]: src[1] bits 1-30 at positions 0-29, src[2] bits 0-1 at positions 30-31
    dst[1] = ((src[1] >> 1) & 0x3FFFFFFF) | ((src[2] & 0x3) << 30);
    
    // dst[2]: src[2] bits 2-30 at positions 0-28, src[3] bits 0-2 at positions 29-31
    dst[2] = ((src[2] >> 2) & 0x1FFFFFFF) | ((src[3] & 0x7) << 29);
    
    // dst[3]: src[3] bits 3-30 at positions 0-27, src[4] bits 0-3 at positions 28-31
    dst[3] = ((src[3] >> 3) & 0xFFFFFFF) | ((src[4] & 0xF) << 28);
    
    // dst[4]: src[4] bits 4-30 at positions 0-26, src[5] bits 0-4 at positions 27-31
    dst[4] = ((src[4] >> 4) & 0x7FFFFFF) | ((src[5] & 0x1F) << 27);
    
    // dst[5]: src[5] bits 5-30 at positions 0-25, src[6] bits 0-5 at positions 26-31
    dst[5] = ((src[5] >> 5) & 0x3FFFFFF) | ((src[6] & 0x3F) << 26);
    
    // dst[6]: src[6] bits 6-30 at positions 0-24, src[7] bits 0-6 at positions 25-31
    dst[6] = ((src[6] >> 6) & 0x1FFFFFF) | ((src[7] & 0x7F) << 25);
    
    // dst[7]: src[7] bits 7-30 at positions 0-23, src[8] bits 0-7 at positions 24-31
    dst[7] = ((src[7] >> 7) & 0xFFFFFF) | ((src[8] & 0xFF) << 24);
    
    // dst[8]: src[8] bits 8-30 at positions 0-22, src[9] bits 0-8 at positions 23-31
    dst[8] = ((src[8] >> 8) & 0x7FFFFF) | ((src[9] & 0x1FF) << 23);
    
    // dst[9]: src[9] bits 9-30 at positions 0-21, src[10] bits 0-9 at positions 22-31
    dst[9] = ((src[9] >> 9) & 0x3FFFFF) | ((src[10] & 0x3FF) << 22);
    
    // dst[10]: src[10] bits 10-30 at positions 0-20, src[11] bits 0-10 at positions 21-31
    dst[10] = ((src[10] >> 10) & 0x1FFFFF) | ((src[11] & 0x7FF) << 21);
    
    // dst[11]: src[11] bits 11-30 at positions 0-19, src[12] bits 0-11 at positions 20-31
    dst[11] = ((src[11] >> 11) & 0xFFFFF) | ((src[12] & 0xFFF) << 20);
    
    // dst[12]: src[12] bits 12-30 at positions 0-18, src[13] bits 0-12 at positions 19-31
    dst[12] = ((src[12] >> 12) & 0x7FFFF) | ((src[13] & 0x1FFF) << 19);
    
    // dst[13]: src[13] bits 13-30 at positions 0-17, src[14] bits 0-13 at positions 18-31
    dst[13] = ((src[13] >> 13) & 0x3FFFF) | ((src[14] & 0x3FFF) << 18);
    
    // dst[14]: src[14] bits 14-30 at positions 0-16, src[15] bits 0-14 at positions 17-31
    dst[14] = ((src[14] >> 14) & 0x1FFFF) | ((src[15] & 0x7FFF) << 17);
    
    // dst[15]: src[15] bits 15-30 at positions 0-15, src[16] bits 0-15 at positions 16-31
    dst[15] = ((src[15] >> 15) & 0xFFFF) | ((src[16] & 0xFFFF) << 16);
    
    // dst[16]: src[16] bits 16-30 at positions 0-14, src[17] bits 0-16 at positions 15-31
    dst[16] = ((src[16] >> 16) & 0x7FFF) | ((src[17] & 0x1FFFF) << 15);
    
    // dst[17]: src[17] bits 17-30 at positions 0-13, src[18] bits 0-17 at positions 14-31
    dst[17] = ((src[17] >> 17) & 0x3FFF) | ((src[18] & 0x3FFFF) << 14);
    
    // dst[18]: src[18] bits 18-30 at positions 0-12, src[19] bits 0-18 at positions 13-31
    dst[18] = ((src[18] >> 18) & 0x1FFF) | ((src[19] & 0x7FFFF) << 13);
    
    // dst[19]: src[19] bits 19-30 at positions 0-11, src[20] bits 0-19 at positions 12-31
    dst[19] = ((src[19] >> 19) & 0xFFF) | ((src[20] & 0xFFFFF) << 12);
    
    // dst[20]: src[20] bits 20-30 at positions 0-10, src[21] bits 0-20 at positions 11-31
    dst[20] = ((src[20] >> 20) & 0x7FF) | ((src[21] & 0x1FFFFF) << 11);
    
    // dst[21]: src[21] bits 21-30 at positions 0-9, src[22] bits 0-21 at positions 10-31
    dst[21] = ((src[21] >> 21) & 0x3FF) | ((src[22] & 0x3FFFFF) << 10);
    
    // dst[22]: src[22] bits 22-30 at positions 0-8, src[23] bits 0-22 at positions 9-31
    dst[22] = ((src[22] >> 22) & 0x1FF) | ((src[23] & 0x7FFFFF) << 9);
    
    // dst[23]: src[23] bits 23-30 at positions 0-7, src[24] bits 0-23 at positions 8-31
    dst[23] = ((src[23] >> 23) & 0xFF) | ((src[24] & 0xFFFFFF) << 8);
    
    // dst[24]: src[24] bits 24-30 at positions 0-6, src[25] bits 0-24 at positions 7-31
    dst[24] = ((src[24] >> 24) & 0x7F) | ((src[25] & 0x1FFFFFF) << 7);
    
    // dst[25]: src[25] bits 25-30 at positions 0-5, src[26] bits 0-25 at positions 6-31
    dst[25] = ((src[25] >> 25) & 0x3F) | ((src[26] & 0x3FFFFFF) << 6);
    
    // dst[26]: src[26] bits 26-30 at positions 0-4, src[27] bits 0-26 at positions 5-31
    dst[26] = ((src[26] >> 26) & 0x1F) | ((src[27] & 0x7FFFFFF) << 5);
    
    // dst[27]: src[27] bits 27-30 at positions 0-3, src[28] bits 0-27 at positions 4-31
    dst[27] = ((src[27] >> 27) & 0xF) | ((src[28] & 0xFFFFFFF) << 4);
    
    // dst[28]: src[28] bits 28-30 at positions 0-2, src[29] bits 0-28 at positions 3-31
    dst[28] = ((src[28] >> 28) & 0x7) | ((src[29] & 0x1FFFFFFF) << 3);
    
    // dst[29]: src[29] bits 29-30 at positions 0-1, src[30] bits 0-29 at positions 2-31
    dst[29] = ((src[29] >> 29) & 0x3) | ((src[30] & 0x3FFFFFFF) << 2);
    
    // dst[30]: src[30] bit 30 at position 0, src[31] bits 0-30 at positions 1-31
    dst[30] = ((src[30] >> 30) & 0x1) | ((src[31] & 0x7FFFFFFF) << 1);
}

// 32 values * 32 bits = 1024 bits = 32 words
pub fn decompress_32_bit(src: &[u32], dst: &mut [u32]) {
    // dst[0]: bits 0-31 (in src[0] bits 0-31)
    dst[0]  = src[0];
    
    // dst[1]: bits 32-63 (in src[1] bits 0-31)
    dst[1]  = src[1];

    // dst[2]: bits 64-95 (in src[2] bits 0-31)
    dst[2]  = src[2];
    
    // dst[3]: bits 96-127 (in src[3] bits 0-31)
    dst[3]  = src[3];

    // dst[4]: bits 128-159 (in src[4] bits 0-31)
    dst[4]  = src[4];
    
    // dst[5]: bits 160-191 (in src[5] bits 0-31)
    dst[5]  = src[5];

    // dst[6]: bits 192-223 (in src[6] bits 0-31)
    dst[6]  = src[6];
    
    // dst[7]: bits 224-255 (in src[7] bits 0-31)
    dst[7]  = src[7];

    // dst[8]: bits 256-287 (in src[8] bits 0-31)
    dst[8]  = src[8];
    
    // dst[9]: bits 288-319 (in src[9] bits 0-31)
    dst[9]  = src[9];

    // dst[10]: bits 320-351 (in src[10] bits 0-31)
    dst[10] = src[10];
    
    // dst[11]: bits 352-383 (in src[11] bits 0-31)
    dst[11] = src[11];

    // dst[12]: bits 384-415 (in src[12] bits 0-31)
    dst[12] = src[12];
    
    // dst[13]: bits 416-447 (in src[13] bits 0-31)
    dst[13] = src[13];

    // dst[14]: bits 448-479 (in src[14] bits 0-31)
    dst[14] = src[14];
    
    // dst[15]: bits 480-511 (in src[15] bits 0-31)
    dst[15] = src[15];

    // dst[16]: bits 512-543 (in src[16] bits 0-31)
    dst[16] = src[16];
    
    // dst[17]: bits 544-575 (in src[17] bits 0-31)
    dst[17] = src[17];

    // dst[18]: bits 576-607 (in src[18] bits 0-31)
    dst[18] = src[18];
    
    // dst[19]: bits 608-639 (in src[19] bits 0-31)
    dst[19] = src[19];

    // dst[20]: bits 640-671 (in src[20] bits 0-31)
    dst[20] = src[20];
    
    // dst[21]: bits 672-703 (in src[21] bits 0-31)
    dst[21] = src[21];

    // dst[22]: bits 704-735 (in src[22] bits 0-31)
    dst[22] = src[22];
    
    // dst[23]: bits 736-767 (in src[23] bits 0-31)
    dst[23] = src[23];

    // dst[24]: bits 768-799 (in src[24] bits 0-31)
    dst[24] = src[24];
    
    // dst[25]: bits 800-831 (in src[25] bits 0-31)
    dst[25] = src[25];

    // dst[26]: bits 832-863 (in src[26] bits 0-31)
    dst[26] = src[26];
    
    // dst[27]: bits 864-895 (in src[27] bits 0-31)
    dst[27] = src[27];

    // dst[28]: bits 896-927 (in src[28] bits 0-31)
    dst[28] = src[28];
    
    // dst[29]: bits 928-959 (in src[29] bits 0-31)
    dst[29] = src[29];

    // dst[30]: bits 960-991 (in src[30] bits 0-31)
    dst[30] = src[30];
    
    // dst[31]: bits 992-1023 (in src[31] bits 0-31)
    dst[31] = src[31];
}

// 32 values * 32 bits = 1024 bits = 32 words
pub fn compress_32_bit(src: &[u32], dst: &mut [u32]) {
    // src[0]: bits 0-31 (to dst[0] bits 0-31)
    dst[0]  = src[0];
    
    // src[1]: bits 32-63 (to dst[1] bits 0-31)
    dst[1]  = src[1];

    // src[2]: bits 64-95 (to dst[2] bits 0-31)
    dst[2]  = src[2];
    
    // src[3]: bits 96-127 (to dst[3] bits 0-31)
    dst[3]  = src[3];

    // src[4]: bits 128-159 (to dst[4] bits 0-31)
    dst[4]  = src[4];
    
    // src[5]: bits 160-191 (to dst[5] bits 0-31)
    dst[5]  = src[5];

    // src[6]: bits 192-223 (to dst[6] bits 0-31)
    dst[6]  = src[6];
    
    // src[7]: bits 224-255 (to dst[7] bits 0-31)
    dst[7]  = src[7];

    // src[8]: bits 256-287 (to dst[8] bits 0-31)
    dst[8]  = src[8];
    
    // src[9]: bits 288-319 (to dst[9] bits 0-31)
    dst[9]  = src[9];

    // src[10]: bits 320-351 (to dst[10] bits 0-31)
    dst[10] = src[10];
    
    // src[11]: bits 352-383 (to dst[11] bits 0-31)
    dst[11] = src[11];

    // src[12]: bits 384-415 (to dst[12] bits 0-31)
    dst[12] = src[12];
    
    // src[13]: bits 416-447 (to dst[13] bits 0-31)
    dst[13] = src[13];

    // src[14]: bits 448-479 (to dst[14] bits 0-31)
    dst[14] = src[14];
    
    // src[15]: bits 480-511 (to dst[15] bits 0-31)
    dst[15] = src[15];

    // src[16]: bits 512-543 (to dst[16] bits 0-31)
    dst[16] = src[16];
    
    // src[17]: bits 544-575 (to dst[17] bits 0-31)
    dst[17] = src[17];

    // src[18]: bits 576-607 (to dst[18] bits 0-31)
    dst[18] = src[18];
    
    // src[19]: bits 608-639 (to dst[19] bits 0-31)
    dst[19] = src[19];

    // src[20]: bits 640-671 (to dst[20] bits 0-31)
    dst[20] = src[20];
    
    // src[21]: bits 672-703 (to dst[21] bits 0-31)
    dst[21] = src[21];

    // src[22]: bits 704-735 (to dst[22] bits 0-31)
    dst[22] = src[22];
    
    // src[23]: bits 736-767 (to dst[23] bits 0-31)
    dst[23] = src[23];

    // src[24]: bits 768-799 (to dst[24] bits 0-31)
    dst[24] = src[24];
    
    // src[25]: bits 800-831 (to dst[25] bits 0-31)
    dst[25] = src[25];

    // src[26]: bits 832-863 (to dst[26] bits 0-31)
    dst[26] = src[26];
    
    // src[27]: bits 864-895 (to dst[27] bits 0-31)
    dst[27] = src[27];

    // src[28]: bits 896-927 (to dst[28] bits 0-31)
    dst[28] = src[28];
    
    // src[29]: bits 928-959 (to dst[29] bits 0-31)
    dst[29] = src[29];

    // src[30]: bits 960-991 (to dst[30] bits 0-31)
    dst[30] = src[30];
    
    // src[31]: bits 992-1023 (to dst[31] bits 0-31)
    dst[31] = src[31];
}