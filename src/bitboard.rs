use crate::board::{self,RANKS_ITER,FILES_ITER,FILES,fr_to_sq,SQUARE_120_TO_64,SQUARE_64_TO_120};

const BIT_TABLE: [usize; 64] = [63, 30, 3, 32, 25, 41, 22, 33, 15, 50, 42, 13, 11, 53, 19, 34, 61, 29, 2, 51, 21, 43, 45, 10, 18, 47, 1, 54, 9, 57, 0, 35, 62, 31, 40, 4, 49, 5, 52, 26, 60, 6, 23, 44, 46, 27, 56, 16, 7, 39, 48, 24, 59, 14, 12, 55, 38, 28, 58, 20, 37, 17, 36, 8];

static mut FILE_BB_MASKS: [u64; 8] = [0; 8];
static mut RANK_BB_MASKS: [u64; 8] = [0; 8];

static mut BLACK_PASSED_MASK: [u64; 64] = [0; 64];
static mut WHITE_PASSED_MASK: [u64; 64] = [0; 64];
// Believe this one only depends on the file, and so could be stored
// with 8 values instead of 64.
static mut ISOLATED_MASK: [u64; 64] = [0; 64];

#[derive(Clone)]
pub struct Bitboard {
    val: u64,
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard{ val: 0 }
    }

    pub fn nonzero(&self) -> bool {
        self.val != 0
    }

    pub fn set_bit(&mut self, index: usize) {
        let mask: u64 = 1 << index;
        self.val |= mask;
    }

    pub fn clear_bit(&mut self, index: usize) {
        let mask: u64 = !(1 << index);
        self.val &= mask;
    }

    pub fn count(&self) -> i32 {
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

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let one: u64 = 1;
        let mut sq;
        let mut sq64: usize;

        let mut s = String::new();
        for rank in RANKS_ITER.rev() {
            for file in FILES_ITER {
                sq = fr_to_sq(file, rank);
                sq64 = SQUARE_120_TO_64[sq as usize];
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

pub fn init_eval_masks() {
    let mut sq;
    let mut sq64: usize;
    for rank in RANKS_ITER.rev() {
        for file in FILES_ITER {
            sq = fr_to_sq(file, rank);
            sq64 = SQUARE_120_TO_64[sq as usize];
            unsafe {
                FILE_BB_MASKS[file as usize] |= 1 << sq64;
                RANK_BB_MASKS[rank as usize] |= 1 << sq64;
            }
        }
    }

    let mut tsq: i32;
    for sq64 in 0..64 as usize {
        tsq = sq64 as i32 + 8;
        while tsq < 64 {
            unsafe {
                WHITE_PASSED_MASK[sq64] |= 1 << tsq;
            }
            tsq += 8;
        }

        tsq = sq64 as i32 - 8;
        while tsq >= 0 {
            unsafe {
                BLACK_PASSED_MASK[sq64] |= 1 << tsq;
            }
            tsq -= 8;
        }

        if FILES[SQUARE_64_TO_120[sq64] as usize] > board::FILE_A {
            unsafe {
                ISOLATED_MASK[sq64] |= FILE_BB_MASKS[(FILES[SQUARE_64_TO_120[sq64] as usize] - 1) as usize];
            }

            tsq = sq64 as i32 + 7;
            while tsq < 64 {
                unsafe {
                    WHITE_PASSED_MASK[sq64] |= 1 << tsq;
                }
                tsq += 8;
            }

            tsq = sq64 as i32 - 9;
            while tsq >= 0 {
                unsafe {
                    BLACK_PASSED_MASK[sq64] |= 1 << tsq;
                }
                tsq -= 8;
            }
        }

        if FILES[SQUARE_64_TO_120[sq64] as usize] < board::FILE_H {
            unsafe {
                ISOLATED_MASK[sq64] |= FILE_BB_MASKS[(FILES[SQUARE_64_TO_120[sq64] as usize] + 1) as usize];
            }

            tsq = sq64 as i32 + 9;
            while tsq < 64 {
                unsafe {
                    WHITE_PASSED_MASK[sq64] |= 1 << tsq;
                }
                tsq += 8;
            }

            tsq = sq64 as i32 - 7;
            while tsq >= 0 {
                unsafe {
                    BLACK_PASSED_MASK[sq64] |= 1 << tsq;
                }
                tsq -= 8;
            }
        }
    }

    // let mut bb = Bitboard::new();
    // for sq64 in 0..64 as usize {
    //     unsafe {
    //         bb.val = ISOLATED_MASK[sq64];
    //     }
    //     println!("{}", bb.to_string());
    // }

    // let mut bb = Bitboard::new();
    // unsafe {
    //     bb.val = RANK_BB_MASKS[1];
    //     println!("{}", bb.to_string());
    // }
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
        assert_eq!(bb.count(), 2);
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
