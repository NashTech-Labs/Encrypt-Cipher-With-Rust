use std::fmt::Write;
use std::mem;

fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (x & z) | (y & z)
}

fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

macro_rules! md4round1 {
    ( $a:expr, $b:expr, $c:expr, $d:expr, $i:expr, $s:expr, $x:expr) => {{
        $a = ($a.wrapping_add(f($b, $c, $d)).wrapping_add($x[$i])).rotate_left($s);
    }};
}

macro_rules! md4round2 {
    ( $a:expr, $b:expr, $c:expr, $d:expr, $i:expr, $s:expr, $x:expr) => {{
        $a = ($a
            .wrapping_add(g($b, $c, $d))
            .wrapping_add($x[$i])
            .wrapping_add(0x5a827999_u32))
        .rotate_left($s);
    }};
}

macro_rules! md4round3 {
    ( $a:expr, $b:expr, $c:expr, $d:expr, $i:expr, $s:expr, $x:expr) => {{
        $a = ($a
            .wrapping_add(h($b, $c, $d))
            .wrapping_add($x[$i])
            .wrapping_add(0x6ed9eba1_u32))
        .rotate_left($s);
    }};
}

fn convert_byte_vec_to_u32(mut bytes: Vec<u8>) -> Vec<u32> {
    bytes.shrink_to_fit();
    let num_bytes = bytes.len();
    let num_words = num_bytes / 4;
    unsafe {
        let words = Vec::from_raw_parts(bytes.as_mut_ptr() as *mut u32, num_words, num_words);
        mem::forget(bytes);
        words
    }
}

pub fn md4<T: Into<Vec<u8>>>(input: T) -> [u32; 4] {
    let mut bytes = input.into().to_vec();
    let initial_bit_len = (bytes.len() << 3) as u64;

    bytes.push(0x80_u8);
    while (bytes.len() % 64) != 56 {
        bytes.push(0_u8);
    }

    let mut w = convert_byte_vec_to_u32(bytes);

    w.push(initial_bit_len as u32); // Push low-order bytes first
    w.push((initial_bit_len >> 32) as u32);

    let mut a = 0x67452301_u32;
    let mut b = 0xefcdab89_u32;
    let mut c = 0x98badcfe_u32;
    let mut d = 0x10325476_u32;

    let n = w.len();
    for i in 0..n / 16 {
        let x = &w[i * 16..i * 16 + 16];

        let aa = a;
        let bb = b;
        let cc = c;
        let dd = d;

        md4round1!(a, b, c, d, 0, 3, x); // [A B C D 0 3]
        md4round1!(d, a, b, c, 1, 7, x); // [D A B C 1 7]
        md4round1!(c, d, a, b, 2, 11, x); // [C D A B 2 11]
        md4round1!(b, c, d, a, 3, 19, x); // [B C D A 3 19]
        md4round1!(a, b, c, d, 4, 3, x); // [A B C D 4 3]
        md4round1!(d, a, b, c, 5, 7, x); // [D A B C 5 7]
        md4round1!(c, d, a, b, 6, 11, x); // [C D A B 6 11]
        md4round1!(b, c, d, a, 7, 19, x); // [B C D A 7 19]
        md4round1!(a, b, c, d, 8, 3, x); // [A B C D 8 3]
        md4round1!(d, a, b, c, 9, 7, x); // [D A B C 9 7]
        md4round1!(c, d, a, b, 10, 11, x); // [C D A B 10 11]
        md4round1!(b, c, d, a, 11, 19, x); // [B C D A 11 19]
        md4round1!(a, b, c, d, 12, 3, x); // [A B C D 12 3]
        md4round1!(d, a, b, c, 13, 7, x); // [D A B C 13 7]
        md4round1!(c, d, a, b, 14, 11, x); // [C D A B 14 11]
        md4round1!(b, c, d, a, 15, 19, x); // [B C D A 15 19]

        md4round2!(a, b, c, d, 0, 3, x); //[A B C D 0  3]
        md4round2!(d, a, b, c, 4, 5, x); //[D A B C 4  5]
        md4round2!(c, d, a, b, 8, 9, x); //[C D A B 8  9]
        md4round2!(b, c, d, a, 12, 13, x); //[B C D A 12 13]
        md4round2!(a, b, c, d, 1, 3, x); //[A B C D 1  3]
        md4round2!(d, a, b, c, 5, 5, x); //[D A B C 5  5]
        md4round2!(c, d, a, b, 9, 9, x); //[C D A B 9  9]
        md4round2!(b, c, d, a, 13, 13, x); //[B C D A 13 13]
        md4round2!(a, b, c, d, 2, 3, x); //[A B C D 2  3]
        md4round2!(d, a, b, c, 6, 5, x); //[D A B C 6  5]
        md4round2!(c, d, a, b, 10, 9, x); //[C D A B 10 9]
        md4round2!(b, c, d, a, 14, 13, x); //[B C D A 14 13]
        md4round2!(a, b, c, d, 3, 3, x); //[A B C D 3  3]
        md4round2!(d, a, b, c, 7, 5, x); //[D A B C 7  5]
        md4round2!(c, d, a, b, 11, 9, x); //[C D A B 11 9]
        md4round2!(b, c, d, a, 15, 13, x); //[B C D A 15 13]

        md4round3!(a, b, c, d, 0, 3, x); //[A B C D 0  3]
        md4round3!(d, a, b, c, 8, 9, x); //[D A B C 8  9]
        md4round3!(c, d, a, b, 4, 11, x); //[C D A B 4  11]
        md4round3!(b, c, d, a, 12, 15, x); //[B C D A 12 15]
        md4round3!(a, b, c, d, 2, 3, x); //[A B C D 2  3]
        md4round3!(d, a, b, c, 10, 9, x); //[D A B C 10 9]
        md4round3!(c, d, a, b, 6, 11, x); //[C D A B 6  11]
        md4round3!(b, c, d, a, 14, 15, x); //[B C D A 14 15]
        md4round3!(a, b, c, d, 1, 3, x); //[A B C D 1  3]
        md4round3!(d, a, b, c, 9, 9, x); //[D A B C 9  9]
        md4round3!(c, d, a, b, 5, 11, x); //[C D A B 5  11]
        md4round3!(b, c, d, a, 13, 15, x); //[B C D A 13 15]
        md4round3!(a, b, c, d, 3, 3, x); //[A B C D 3  3]
        md4round3!(d, a, b, c, 11, 9, x); //[D A B C 11 9]
        md4round3!(c, d, a, b, 7, 11, x); //[C D A B 7  11]
        md4round3!(b, c, d, a, 15, 15, x); //[B C D A 15 15]

        a = a.wrapping_add(aa);
        b = b.wrapping_add(bb);
        c = c.wrapping_add(cc);
        d = d.wrapping_add(dd);
    }

    [
        u32::from_be(a),
        u32::from_be(b),
        u32::from_be(c),
        u32::from_be(d),
    ]
}

pub fn encrypt_cipher(digest: &[u32]) -> String {
    let mut s = String::new();
    for &word in digest {
        write!(&mut s, "{:08x}", word).unwrap();
    }
    s
}
