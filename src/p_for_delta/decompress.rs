pub type DecompressorFn = fn(&[u32],&mut [u32]);

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

// 32 values * 13 bits = 416 bits = 13 words
pub fn decompress_13_bit(src: &[u32], dst: &mut [u32]) {
    // Word 0: bits 0-31
    dst[0] = (src[0] >> 0) & 0x1FFF;
    dst[1] = (src[0] >> 13) & 0x1FFF;
    
    // Value 2 spans words 0-1
    dst[2] = ((src[0] >> 26) & 0x3F) | ((src[1] & 0x7F) << 6);

    // Word 1: bits 32-63
    dst[3] = (src[1] >> 7) & 0x1FFF;
    dst[4] = (src[1] >> 20) & 0x1FFF;
    
    // Value 5 spans words 1-2
    dst[5] = ((src[1] >> 33 - 32) & 0x1) | ((src[2] & 0xFFF) << 1);

    // Word 2: bits 64-95
    dst[6] = (src[2] >> 12) & 0x1FFF;
    
    // Value 7 spans words 2-3
    dst[7] = ((src[2] >> 25) & 0x7F) | ((src[3] & 0x3F) << 7);

    // Word 3: bits 96-127
    dst[8] = (src[3] >> 6) & 0x1FFF;
    dst[9] = (src[3] >> 19) & 0x1FFF;

    // Word 4: bits 128-159
    dst[10] = (src[4] >> 0) & 0x1FFF;
    dst[11] = (src[4] >> 13) & 0x1FFF;
    
    // Value 12 spans words 4-5
    dst[12] = ((src[4] >> 26) & 0x3F) | ((src[5] & 0x7F) << 6);

    // Word 5: bits 160-191
    dst[13] = (src[5] >> 7) & 0x1FFF;
    dst[14] = (src[5] >> 20) & 0x1FFF;
    
    // Value 15 spans words 5-6
    dst[15] = ((src[5] >> 33 - 32) & 0x1) | ((src[6] & 0xFFF) << 1);

    // Word 6: bits 192-223
    dst[16] = (src[6] >> 12) & 0x1FFF;
    
    // Value 17 spans words 6-7
    dst[17] = ((src[6] >> 25) & 0x7F) | ((src[7] & 0x3F) << 7);

    // Word 7: bits 224-255
    dst[18] = (src[7] >> 6) & 0x1FFF;
    dst[19] = (src[7] >> 19) & 0x1FFF;

    // Word 8: bits 256-287
    dst[20] = (src[8] >> 0) & 0x1FFF;
    dst[21] = (src[8] >> 13) & 0x1FFF;
    
    // Value 22 spans words 8-9
    dst[22] = ((src[8] >> 26) & 0x3F) | ((src[9] & 0x7F) << 6);

    // Word 9: bits 288-319
    dst[23] = (src[9] >> 7) & 0x1FFF;
    dst[24] = (src[9] >> 20) & 0x1FFF;
    
    // Value 25 spans words 9-10
    dst[25] = ((src[9] >> 33 - 32) & 0x1) | ((src[10] & 0xFFF) << 1);

    // Word 10: bits 320-351
    dst[26] = (src[10] >> 12) & 0x1FFF;
    
    // Value 27 spans words 10-11
    dst[27] = ((src[10] >> 25) & 0x7F) | ((src[11] & 0x3F) << 7);

    // Word 11: bits 352-383
    dst[28] = (src[11] >> 6) & 0x1FFF;
    dst[29] = (src[11] >> 19) & 0x1FFF;

    // Word 12: bits 384-415
    dst[30] = (src[12] >> 0) & 0x1FFF;
    dst[31] = (src[12] >> 13) & 0x1FFF;
}

pub fn decompress_14_bit(src: &[u32], dst: &mut [u32]) {
    dst[0] = (src[0] >> 0) & 0x3FFF;
    dst[1] = (src[0] >> 14) & 0x3FFF;
    dst[2] = ((src[0] >> 28) & 0xF) | ((src[1] & 0x3FF) << 4);
    
    dst[3] = (src[1] >> 10) & 0x3FFF;
    dst[4] = ((src[1] >> 24) & 0xFF) | ((src[2] & 0x3F) << 8);
    
    dst[5] = (src[2] >> 6) & 0x3FFF;
    dst[6] = (src[2] >> 20) & 0x3FFF;
    dst[7] = ((src[2] >> 2) & 0x3) | ((src[3] & 0xFFF) << 2); 
    
    dst[8] = (src[3] >> 12) & 0x3FFF;
    dst[9] = ((src[3] >> 26) & 0x3F) | ((src[4] & 0xFF) << 6);
    
    dst[10] = (src[4] >> 8) & 0x3FFF;
    dst[11] = (src[4] >> 22) & 0x3FFF;
    dst[12] = ((src[4] >> 4) & 0xF) | ((src[5] & 0x3FF) << 4);
    
    dst[13] = (src[5] >> 10) & 0x3FFF;
    dst[14] = ((src[5] >> 24) & 0xFF) | ((src[6] & 0x3F) << 8);
    
    dst[15] = (src[6] >> 6) & 0x3FFF;
    dst[16] = (src[6] >> 20) & 0x3FFF;
    dst[17] = ((src[6] >> 2) & 0x3) | ((src[7] & 0xFFF) << 2);
    
    dst[18] = (src[7] >> 12) & 0x3FFF;
    dst[19] = ((src[7] >> 26) & 0x3F) | ((src[8] & 0xFF) << 6);
    
    dst[20] = (src[8] >> 8) & 0x3FFF;
    dst[21] = (src[8] >> 22) & 0x3FFF;
    dst[22] = ((src[8] >> 4) & 0xF) | ((src[9] & 0x3FF) << 4);
    
    dst[23] = (src[9] >> 10) & 0x3FFF;
    dst[24] = ((src[9] >> 24) & 0xFF) | ((src[10] & 0x3F) << 8);
    
    dst[25] = (src[10] >> 6) & 0x3FFF;
    dst[26] = (src[10] >> 20) & 0x3FFF;
    dst[27] = ((src[10] >> 2) & 0x3) | ((src[11] & 0xFFF) << 2);
    
    dst[28] = (src[11] >> 12) & 0x3FFF;
    dst[29] = ((src[11] >> 26) & 0x3F) | ((src[12] & 0xFF) << 6);
    
    dst[30] = (src[12] >> 8) & 0x3FFF;
    dst[31] = (src[12] >> 22) & 0x3FFF;
}

pub fn decompress_15_bit(src: &[u32], dst: &mut [u32]) {
    dst[0] = (src[0] >> 0) & 0x7FFF;
    dst[1] = (src[0] >> 15) & 0x7FFF;
    dst[2] = ((src[0] >> 30) & 0x3) | ((src[1] & 0x1FFF) << 2);

    dst[3] = (src[1] >> 13) & 0x7FFF;
    dst[4] = ((src[1] >> 28) & 0xF) | ((src[2] & 0x7FF) << 4);

    dst[5] = (src[2] >> 11) & 0x7FFF;
    dst[6] = ((src[2] >> 26) & 0x3F) | ((src[3] & 0x1FF) << 6);

    dst[7] = (src[3] >> 9) & 0x7FFF;
    dst[8] = ((src[3] >> 24) & 0xFF) | ((src[4] & 0x7F) << 8);

    dst[9] = (src[4] >> 7) & 0x7FFF;
    dst[10] = ((src[4] >> 22) & 0x3FF) | ((src[5] & 0x1F) << 10);

    dst[11] = (src[5] >> 5) & 0x7FFF;
    dst[12] = (src[5] >> 20) & 0x7FFF;
    dst[13] = ((src[5] >> 3) & 0x7) | ((src[6] & 0xFFF) << 3);

    dst[14] = (src[6] >> 12) & 0x7FFF;
    dst[15] = ((src[6] >> 27) & 0x1F) | ((src[7] & 0x3FF) << 5);

    dst[16] = (src[7] >> 10) & 0x7FFF;
    dst[17] = ((src[7] >> 25) & 0x7F) | ((src[8] & 0xFF) << 7);

    dst[18] = (src[8] >> 8) & 0x7FFF;
    dst[19] = ((src[8] >> 23) & 0x1FF) | ((src[9] & 0x3F) << 9);

    dst[20] = (src[9] >> 6) & 0x7FFF;
    dst[21] = (src[9] >> 21) & 0x7FFF;
    dst[22] = ((src[9] >> 4) & 0xF) | ((src[10] & 0x7FF) << 4);

    dst[23] = (src[10] >> 11) & 0x7FFF;
    dst[24] = ((src[10] >> 26) & 0x3F) | ((src[11] & 0x1FF) << 6);

    dst[25] = (src[11] >> 9) & 0x7FFF;
    dst[26] = ((src[11] >> 24) & 0xFF) | ((src[12] & 0x7F) << 8);

    dst[27] = (src[12] >> 7) & 0x7FFF;
    dst[28] = ((src[12] >> 22) & 0x3FF) | ((src[13] & 0x1F) << 10);

    dst[29] = (src[13] >> 5) & 0x7FFF;
    dst[30] = (src[13] >> 20) & 0x7FFF;
    dst[31] = ((src[13] >> 3) & 0x7) | ((src[14] & 0xFFF) << 3); 
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