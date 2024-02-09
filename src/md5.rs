// S specifies the per-round shift amounts
const S: [u8; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

// Use binary integer part of the sines of integers (Radians) as constants:
const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

// Rotate left
fn rol(x: u32, n: u8) -> u32 {
    (x << n) | (x >> (32 - n))
}

pub fn digest(input: &[u8]) -> String {
    // Initialize variables:
    let mut a0: u32 = 0x67452301; // A
    let mut b0: u32 = 0xefcdab89; // B
    let mut c0: u32 = 0x98badcfe; // C
    let mut d0: u32 = 0x10325476; // D

    // Use binary integer part of the sines of integers (Radians) as constants
    // let mut K: [u32; 64] = [0; 64];
    // (0..64).for_each(|i| {
    //     K[i] = ((1u64 << 32) as f64 * (i as f64 + 1.0).sin().abs()).floor() as u32;
    // });

    let mut input = input.to_vec();
    let input_len = input.len();

    // Pre-processing: adding a single 1 bit
    input.push(0x80);

    // Pre-processing: padding with zeros
    while input.len() % 64 != 56 {
        input.push(0);
    }

    // Append the original message length in bits modulo 2^64 in little-endian format:
    input.extend((8u64 * input_len as u64).to_le_bytes());

    // Process the message in successive 512-bit chunks:
    for chunk in input.chunks(64) {
        // Initialize hash value for this chunk:
        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;

        // Main loop:
        for j in 0..64 {
            let mut f;
            let g;

            if j < 16 {
                f = (b & c) | (!b & d);
                g = j;
            } else if j < 32 {
                f = (d & b) | (!d & c);
                g = (5 * j + 1) % 16;
            } else if j < 48 {
                f = b ^ c ^ d;
                g = (3 * j + 5) % 16;
            } else {
                f = c ^ (b | !d);
                g = 7 * j % 16;
            }

            let m = u32::from_le_bytes(chunk[4 * g..4 * (g + 1)].try_into().unwrap());

            f = f.wrapping_add(a).wrapping_add(K[j]).wrapping_add(m);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(rol(f, S[j]));
        }

        // Add this chunk's hash to result so far:
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    // Output is in little-endian
    let mut result = String::new();
    for x in [a0, b0, c0, d0] {
        for c in x.to_le_bytes() {
            result += format!("{:02x}", c).as_str();
        }
    }

    result
}
