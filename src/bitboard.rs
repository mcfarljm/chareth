use crate::board::{ranks,files,fr_to_sq,SQUARE_120_TO_64,SQUARE_64_TO_120};

const BIT_TABLE: [usize; 64] = [63, 30, 3, 32, 25, 41, 22, 33, 15, 50, 42, 13, 11, 53, 19, 34, 61, 29, 2, 51, 21, 43, 45, 10, 18, 47, 1, 54, 9, 57, 0, 35, 62, 31, 40, 4, 49, 5, 52, 26, 60, 6, 23, 44, 46, 27, 56, 16, 7, 39, 48, 24, 59, 14, 12, 55, 38, 28, 58, 20, 37, 17, 36, 8];

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


#[cfg(test)]
mod tests {
    use crate::bitboard::*;
    
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
