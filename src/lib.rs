pub const FILE_A: i32 = 0;
pub const FILE_B: i32 = 1;
pub const FILE_C: i32 = 2;
pub const FILE_D: i32 = 3;
pub const FILE_E: i32 = 4;
pub const FILE_F: i32 = 5;
pub const FILE_G: i32 = 6;
pub const FILE_H: i32 = 7;

pub const RANK_1: i32 = 0;
pub const RANK_2: i32 = 1;
pub const RANK_3: i32 = 2;
pub const RANK_4: i32 = 3;
pub const RANK_5: i32 = 4;
pub const RANK_6: i32 = 5;
pub const RANK_7: i32 = 6;
pub const RANK_8: i32 = 7;

const BIT_TABLE: [usize; 64] = [63, 30, 3, 32, 25, 41, 22, 33, 15, 50, 42, 13, 11, 53, 19, 34, 61, 29, 2, 51, 21, 43, 45, 10, 18, 47, 1, 54, 9, 57, 0, 35, 62, 31, 40, 4, 49, 5, 52, 26, 60, 6, 23, 44, 46, 27, 56, 16, 7, 39, 48, 24, 59, 14, 12, 55, 38, 28, 58, 20, 37, 17, 36, 8];

pub fn fr_to_sq(file: i32, rank: i32) -> usize {
    (21 + file + rank * 10) as usize
}


pub fn ranks() -> std::ops::Range<i32> {
    (RANK_1..RANK_8 + 1)
}

pub fn files() -> std::ops::Range<i32> {
    (FILE_A..FILE_H + 1)
}

pub struct Bitboard {
    val: u64,
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard{ val: 0 }
    }

    pub fn set_bit(&mut self, index: usize) {
        let mask: u64 = 1 << index;
        self.val |= mask;
    }

    pub fn clear_bit(&mut self, index: usize) {
        let mask: u64 = !(1 << index);
        self.val &= mask;
    }

    pub fn count_bits(&self) -> i32 {
        let mut b = self.val;
        let mut r = 0;
        while b != 0 {
            r += 1;
            b &= b - 1;
        }
        r
    }

    pub fn pop_bit(&mut self) -> usize {
        let b: u64 = self.val ^ (self.val-1);
        let fold: u32 = ((b & 0xffffffff) ^ (b >> 32)) as u32;
        self.val &= self.val - 1;
        let i: usize = (fold.wrapping_mul(0x783a9b23) >> 26) as usize;
        BIT_TABLE[i]
    }

    pub fn to_string(&self) -> String {
        let one: u64 = 1;
        let mut sq: usize;
        let mut sq64: usize;

        let mut s = String::new();
        for rank in ranks().rev() {
            for file in files() {
                sq = fr_to_sq(file, rank);
                sq64 = SQUARE_120_TO_64[sq];
                if (one << sq64) & self.val != 0 {
                    s.push('x');
                } else {
                    s.push('-');
                }
            }
            s.push('\n');
        }
        s
    }
}

pub const SQUARE_120_TO_64: [usize; 120] = [
    65, 65, 65, 65, 65, 65, 65, 65, 65, 65,
    65, 65, 65, 65, 65, 65, 65, 65, 65, 65,
    65,  0,  1,  2,  3,  4,  5,  6,  7, 65,
    65,  8,  9, 10, 11, 12, 13, 14, 15, 65,
    65, 16, 17, 18, 19, 20, 21, 22, 23, 65,
    65, 24, 25, 26, 27, 28, 29, 30, 31, 65,
    65, 32, 33, 34, 35, 36, 37, 38, 39, 65,
    65, 40, 41, 42, 43, 44, 45, 46, 47, 65,
    65, 48, 49, 50, 51, 52, 53, 54, 55, 65,
    65, 56, 57, 58, 59, 60, 61, 62, 63, 65,
    65, 65, 65, 65, 65, 65, 65, 65, 65, 65,
    65, 65, 65, 65, 65, 65, 65, 65, 65, 65
];

pub const SQUARE_64_TO_120: [usize; 64] = [
    21, 22, 23, 24, 25, 26, 27, 28,
    31, 32, 33, 34, 35, 36, 37, 38,
    41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58,
    61, 62, 63, 64, 65, 66, 67, 68,
    71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88,
    91, 92, 93, 94, 95, 96, 97, 98
];

#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn bb_string_empty() {
        let bb = Bitboard::new();
        let s = "--------\n\
                 --------\n\
                 --------\n\
                 --------\n\
                 --------\n\
                 --------\n\
                 --------\n\
                 --------\n";
        assert_eq!(bb.to_string(), s);
    }

    #[test]
    fn bb_string_bit9() {
        let mut bb = Bitboard::new();
        bb.set_bit(9);
        let s = "--------\n\
                 --------\n\
                 --------\n\
                 --------\n\
                 --------\n\
                 --------\n\
                 -x------\n\
                 --------\n";
        assert_eq!(bb.to_string(), s);
    }

    #[test]
    fn bb_string_bit9_44() {
        let mut bb = Bitboard::new();
        bb.set_bit(9);
        bb.set_bit(44);
        let s = "--------\n\
                 --------\n\
                 ----x---\n\
                 --------\n\
                 --------\n\
                 --------\n\
                 -x------\n\
                 --------\n";
        assert_eq!(bb.to_string(), s);
    }

    #[test]
    fn bb_clear_bit() {
        let mut bb = Bitboard::new();
        bb.set_bit(9);
        bb.set_bit(44);
        bb.clear_bit(44);
        let s = "--------\n\
                 --------\n\
                 --------\n\
                 --------\n\
                 --------\n\
                 --------\n\
                 -x------\n\
                 --------\n";
        assert_eq!(bb.to_string(), s);
    }

     #[test]
    fn bb_count() {
        let mut bb = Bitboard::new();
        bb.set_bit(9);
        bb.set_bit(44);
        assert_eq!(bb.count_bits(), 2);
    }

    #[test]
    fn bb_pop() {
        let mut bb = Bitboard::new();
        bb.set_bit(9);
        bb.set_bit(44);
        let i = bb.pop_bit();
        assert_eq!(i, 9);
        let s = "--------\n\
                 --------\n\
                 ----x---\n\
                 --------\n\
                 --------\n\
                 --------\n\
                 --------\n\
                 --------\n";
        assert_eq!(bb.to_string(), s);
    }  

}
