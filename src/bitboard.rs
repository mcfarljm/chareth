use std::fmt;

use crate::board::{self,RANKS_ITER,FILES_ITER,FILES,fr_to_sq,SQUARE_120_TO_64,SQUARE_64_TO_120};
use crate::pieces::WHITE;

const BIT_TABLE: [usize; 64] = [63, 30, 3, 32, 25, 41, 22, 33, 15, 50, 42, 13, 11, 53, 19, 34, 61, 29, 2, 51, 21, 43, 45, 10, 18, 47, 1, 54, 9, 57, 0, 35, 62, 31, 40, 4, 49, 5, 52, 26, 60, 6, 23, 44, 46, 27, 56, 16, 7, 39, 48, 24, 59, 14, 12, 55, 38, 28, 58, 20, 37, 17, 36, 8];

// A return value to facilitate a single function that initializes
// multiple bitboard arrays
struct BitboardArrays([u64; 8], [u64; 8], [u64; 64], [u64; 64], [u64; 64]);

// Enable initialization of static arrays.  Once it becomes part of
// stable rust, a better approach may be to use const_fn.
lazy_static! {
    // Tuple struct instance serves only as a holder to retrieve the
    // values from a single function.
    static ref BITBOARD_ARRAYS: BitboardArrays = get_eval_masks();

    // This can also be done without using references (&'static), but
    // believe that would result in additional memory
    static ref FILE_BB_MASKS: &'static[u64; 8] = &BITBOARD_ARRAYS.0;
    static ref RANK_BB_MASKS: &'static[u64; 8] = &BITBOARD_ARRAYS.1;
    static ref WHITE_PASSED_MASK: &'static[u64; 64] = &BITBOARD_ARRAYS.2;
    static ref BLACK_PASSED_MASK: &'static[u64; 64] = &BITBOARD_ARRAYS.3;
    static ref ISOLATED_MASK: &'static[u64; 64] = &BITBOARD_ARRAYS.4;
}

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

    pub fn isolated_pawn(&self, sq64: usize) -> bool {
        if ISOLATED_MASK[sq64] & self.val == 0 {
            return true;
        } else {
            return false;
        }
    }

    // Checks whether side's pawn at square is passed using the
    // opposing side's pawn bitboard (self)
    pub fn passed_pawn(&self, sq64: usize, side: usize) -> bool {
        if side == WHITE {
            if WHITE_PASSED_MASK[sq64] & self.val == 0 {
                return true;
            } else {
                return false;
            }
        } else {
            if BLACK_PASSED_MASK[sq64] & self.val == 0 {
                return true;
            } else {
                return false;
            }
        }
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sq;
        let mut sq64: usize;

        for rank in RANKS_ITER.rev() {
            for file in FILES_ITER {
                sq = fr_to_sq(file, rank);
                sq64 = SQUARE_120_TO_64[sq as usize];
                if (1 << sq64) & self.val != 0 {
                    write!(f, "x")?;
                } else {
                    write!(f, "-")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl IntoIterator for Bitboard {
    type Item = usize;
    type IntoIter = BitboardIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIterator {
            bitboard: self,
        }
    }
}

pub struct BitboardIterator {
    bitboard : Bitboard,
}

impl Iterator for BitboardIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.bitboard.nonzero() {
            Some(self.bitboard.pop_bit())
        } else {
            None
        }
    }
}

// This isn't strictly necessary, as the statics will be automatically
// initialized, but this way we can force them to be initialized at
// the start of the program
pub fn init_eval_masks() {
    lazy_static::initialize(&BITBOARD_ARRAYS);
}

// Initialize and return pawn evaluation mask arrays
fn get_eval_masks() -> BitboardArrays {
    let mut file_bb_masks: [u64; 8] = [0; 8];
    let mut rank_bb_masks: [u64; 8] = [0; 8];
    let mut white_passed_mask: [u64; 64] = [0; 64];
    let mut black_passed_mask: [u64; 64] = [0; 64];
    let mut isolated_mask: [u64; 64] = [0; 64];
    
    let mut sq;
    let mut sq64: usize;
    for rank in RANKS_ITER.rev() {
        for file in FILES_ITER {
            sq = fr_to_sq(file, rank);
            sq64 = SQUARE_120_TO_64[sq as usize];
            file_bb_masks[file as usize] |= 1 << sq64;
            rank_bb_masks[rank as usize] |= 1 << sq64;
        }
    }

    let mut tsq: i32;
    for sq64 in 0..64 as usize {
        tsq = sq64 as i32 + 8;
        while tsq < 64 {
            white_passed_mask[sq64] |= 1 << tsq;
            tsq += 8;
        }

        tsq = sq64 as i32 - 8;
        while tsq >= 0 {
            black_passed_mask[sq64] |= 1 << tsq;
            tsq -= 8;
        }

        if FILES[SQUARE_64_TO_120[sq64] as usize] > board::FILE_A {
            isolated_mask[sq64] |= file_bb_masks[(FILES[SQUARE_64_TO_120[sq64] as usize] - 1) as usize];

            tsq = sq64 as i32 + 7;
            while tsq < 64 {
                white_passed_mask[sq64] |= 1 << tsq;
                tsq += 8;
            }

            tsq = sq64 as i32 - 9;
            while tsq >= 0 {
                black_passed_mask[sq64] |= 1 << tsq;
                tsq -= 8;
            }
        }

        if FILES[SQUARE_64_TO_120[sq64] as usize] < board::FILE_H {
            isolated_mask[sq64] |= file_bb_masks[(FILES[SQUARE_64_TO_120[sq64] as usize] + 1) as usize];

            tsq = sq64 as i32 + 9;
            while tsq < 64 {
                white_passed_mask[sq64] |= 1 << tsq;
                tsq += 8;
            }

            tsq = sq64 as i32 - 7;
            while tsq >= 0 {
                black_passed_mask[sq64] |= 1 << tsq;
                tsq -= 8;
            }
        }
    }

    // let mut bb = Bitboard::new();
    // for sq64 in 0..64 as usize {
    //     unsafe {
    //         bb.val = isolated_mask[sq64];
    //     }
    //     println!("{}", bb);
    // }

    // let mut bb = Bitboard::new();
    // unsafe {
    //     bb.val = rank_bb_masks[1];
    //     println!("{}", bb);
    // }

    BitboardArrays(file_bb_masks, rank_bb_masks, white_passed_mask, black_passed_mask, isolated_mask)
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

    #[test]
    fn file_bb_masks() {
        let mut bb = Bitboard::new();
        bb.val = FILE_BB_MASKS[1];
        assert_eq!(bb.val, 0x202020202020202);        
        bb.val = FILE_BB_MASKS[7];
        assert_eq!(bb.val, 0x8080808080808080);
        // println!("{}", bb);
        // println!("{:x}", bb.val);
    }

    #[test]
    fn rank_bb_masks() {
        let mut bb = Bitboard::new();
        bb.val = RANK_BB_MASKS[1];
        assert_eq!(bb.val, 0xff00);
        bb.val = RANK_BB_MASKS[7];
        assert_eq!(bb.val, 0xff00000000000000);        
    }

    #[test]
    fn white_pawn_passed() {
        let mut bb = Bitboard::new();
        bb.val = WHITE_PASSED_MASK[0];
        assert_eq!(bb.val, 0x303030303030300);
        bb.val = WHITE_PASSED_MASK[37];
        assert_eq!(bb.val, 0x7070700000000000);        
    }

    #[test]
    fn black_pawn_passed() {
        let mut bb = Bitboard::new();
        bb.val = BLACK_PASSED_MASK[63];
        assert_eq!(bb.val, 0xc0c0c0c0c0c0c0);        
    }

    #[test]
    fn isolated_pawn() {
        let mut bb = Bitboard::new();
        bb.val = ISOLATED_MASK[0];
        assert_eq!(bb.val, 0x202020202020202);
        bb.val = ISOLATED_MASK[55];
        assert_eq!(bb.val, 0x4040404040404040);
    }

    #[test]
    fn empty_iter() {
        let bb = Bitboard::new();
        let squares: Vec<_> = bb.into_iter().collect();
        assert_eq!(squares.len(), 0);
    }

    #[test]
    fn iter() {
        let mut bb = Bitboard::new();
        bb.set_bit(9);
        bb.set_bit(25);
        bb.set_bit(44);
        let mut squares: Vec<_> = bb.into_iter().collect();
        squares.sort();
        assert_eq!(squares, &[9, 25, 44]);
    }

}
