use std::fmt;
use std::num::Wrapping;
use bitintr::{Tzcnt,Popcnt,Lzcnt};

use crate::board::{self,RANKS_ITER,FILES_ITER,Square};
use crate::pieces::WHITE;

pub const BB_RANK_4: u64 = 0x00000000FF000000;
pub const BB_RANK_5: u64 = 0x000000FF00000000;
pub const BB_FILE_A: u64 = 0x0101010101010101;
pub const BB_FILE_H: u64 = 0x8080808080808080;

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

    static ref OBS_DIFF_MASKS: [[ObsDiffMask; 64]; 4] = get_obstruction_diff_masks();
}


#[derive(Clone)]
#[derive(Copy)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard(0)
    }

    pub fn nonzero(&self) -> bool {
        self.0 != 0
    }

    pub fn set_bit(&mut self, index: Square) {
        let mask: u64 = 1 << index;
        self.0 |= mask;
    }

    pub fn clear_bit(&mut self, index: Square) {
        let mask: u64 = !(1 << index);
        self.0 &= mask;
    }

    pub fn count(&self) -> i32 {
        self.0.popcnt() as i32
    }

    pub fn pop_bit(&mut self) -> Square {
        let sq = self.0.tzcnt();
        self.0 &= self.0 - 1;
        sq as Square
    }

    pub fn isolated_pawn(&self, sq64: Square) -> bool {
        if ISOLATED_MASK[sq64 as usize] & self.0 == 0 {
            return true;
        } else {
            return false;
        }
    }

    // Checks whether side's pawn at square is passed using the
    // opposing side's pawn bitboard (self)
    pub fn passed_pawn(&self, sq64: Square, side: usize) -> bool {
        if side == WHITE {
            if WHITE_PASSED_MASK[sq64 as usize] & self.0 == 0 {
                return true;
            } else {
                return false;
            }
        } else {
            if BLACK_PASSED_MASK[sq64 as usize] & self.0 == 0 {
                return true;
            } else {
                return false;
            }
        }
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Bitboard::new()
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sq;

        for rank in RANKS_ITER.rev() {
            for file in FILES_ITER {
                sq = board::fr_to_sq(file, rank);
                if (1 << sq) & self.0 != 0 {
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
    type Item = Square;
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
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
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
    for rank in RANKS_ITER.rev() {
        for file in FILES_ITER {
            sq = board::fr_to_sq(file, rank);
            file_bb_masks[file as usize] |= 1 << sq;
            rank_bb_masks[rank as usize] |= 1 << sq;
        }
    }

    let mut tsq: i32;
    for sq64 in 0..64 {
        tsq = sq64 as i32 + 8;
        while tsq < 64 {
            white_passed_mask[sq64 as usize] |= 1 << tsq;
            tsq += 8;
        }

        tsq = sq64 as i32 - 8;
        while tsq >= 0 {
            black_passed_mask[sq64 as usize] |= 1 << tsq;
            tsq -= 8;
        }

        if sq64%8 > board::FILE_A {
            isolated_mask[sq64 as usize] |= file_bb_masks[(sq64%8 - 1) as usize];

            tsq = sq64 as i32 + 7;
            while tsq < 64 {
                white_passed_mask[sq64 as usize] |= 1 << tsq;
                tsq += 8;
            }

            tsq = sq64 as i32 - 9;
            while tsq >= 0 {
                black_passed_mask[sq64 as usize] |= 1 << tsq;
                tsq -= 8;
            }
        }

        if sq64%8 < board::FILE_H {
            isolated_mask[sq64 as usize] |= file_bb_masks[(sq64%8 + 1) as usize];

            tsq = sq64 as i32 + 9;
            while tsq < 64 {
                white_passed_mask[sq64 as usize] |= 1 << tsq;
                tsq += 8;
            }

            tsq = sq64 as i32 - 7;
            while tsq >= 0 {
                black_passed_mask[sq64 as usize] |= 1 << tsq;
                tsq -= 8;
            }
        }
    }

    // let mut bb = Bitboard::new();
    // for sq64 in 0..64 as usize {
    //     unsafe {
    //         bb.0 = isolated_mask[sq64];
    //     }
    //     println!("{}", bb);
    // }

    // let mut bb = Bitboard::new();
    // unsafe {
    //     bb.0 = rank_bb_masks[1];
    //     println!("{}", bb);
    // }

    BitboardArrays(file_bb_masks, rank_bb_masks, white_passed_mask, black_passed_mask, isolated_mask)
}


// Slide piece masks:

// See: https://www.chessprogramming.org/On_an_empty_Board

fn rank_mask(sq: usize) -> u64 {
    0xff << (sq & 56)
}

fn file_mask(sq: usize) -> u64 {
    0x0101010101010101 << (sq & 7)
}

fn diagonal_mask(sq: i32) -> u64 {
    let main_diag: u64 = 0x8040201008040201;
    let diag: i32 = 8*(sq & 7) - (sq & 56);
    let north = -diag & (diag >> 31);
    let south = diag & (-diag >> 31);
    (main_diag >> south) << north
}

fn anti_diagonal_mask(sq: i32) -> u64 {
    let main_diag: u64 = 0x0102040810204080;
    let diag: i32 = 56 - 8*(sq & 7) - (sq & 56);
    let north = -diag & (diag >> 31);
    let south = diag & (-diag >> 31);
    (main_diag >> south) << north
}

fn positive_ray(sq: usize, line: u64) -> u64 {
    line & (Wrapping(0u64) - (Wrapping(2u64) << sq)).0
}

fn negative_ray(sq: usize, line: u64) -> u64 {
    line & ((1 << sq) - 1)
}

// Obstruction difference:

#[derive(Copy,Clone)]
struct ObsDiffMask {
    lower: u64,
    upper: u64,
    line_exc: u64, // lower | upper
}

impl ObsDiffMask {
    fn new(lower: u64, upper: u64) -> ObsDiffMask {
        ObsDiffMask{
            lower: lower,
            upper: upper,
            line_exc: lower | upper,
        }
    }
}

impl Default for ObsDiffMask {
    fn default() -> ObsDiffMask {
        ObsDiffMask::new(0, 0)
    }
}

fn get_obstruction_diff_masks() -> [[ObsDiffMask; 64]; 4] {
    let mut os_masks = [[Default::default(); 64]; 4];
    let mut lower: u64;
    let mut upper: u64;
    let mut line: u64;
    for sq in 0..64 {
        // Rank masks:
        line = rank_mask(sq);
        upper = positive_ray(sq, line);
        lower = negative_ray(sq, line);
        os_masks[0][sq] = ObsDiffMask::new(lower, upper);

        // File masks:
        line = file_mask(sq);
        upper = positive_ray(sq, line);
        lower = negative_ray(sq, line);
        os_masks[1][sq] = ObsDiffMask::new(lower, upper);

        // Diagonal masks:
        line = diagonal_mask(sq as i32);
        upper = positive_ray(sq, line);
        lower = negative_ray(sq, line);
        os_masks[2][sq] = ObsDiffMask::new(lower, upper);

        // Anti-diagonal masks:
        line = anti_diagonal_mask(sq as i32);
        upper = positive_ray(sq, line);
        lower = negative_ray(sq, line);
        os_masks[3][sq] = ObsDiffMask::new(lower, upper);
    }
    os_masks
}

pub fn init_obs_diff_masks() {
    lazy_static::initialize(&OBS_DIFF_MASKS);
}

fn get_line_attacks(occ: u64, os_mask: ObsDiffMask) -> u64 {
    let lower = os_mask.lower & occ;
    let upper = os_mask.upper & occ;
    let ms1b = 0x8000000000000000 >> (lower | 1).lzcnt();
    let ls1b = upper & (-Wrapping(upper)).0;
    let odiff = Wrapping(2)*Wrapping(ls1b) - Wrapping(ms1b);
    os_mask.line_exc & odiff.0
}

pub fn get_rook_attacks(sq: Square, occ: u64) -> u64 {
    get_line_attacks(occ, OBS_DIFF_MASKS[0][sq as usize]) | get_line_attacks(occ, OBS_DIFF_MASKS[1][sq as usize])
}

pub fn get_bishop_attacks(sq: Square, occ: u64) -> u64 {
    get_line_attacks(occ, OBS_DIFF_MASKS[2][sq as usize]) | get_line_attacks(occ, OBS_DIFF_MASKS[3][sq as usize])
}

pub fn get_queen_attacks(sq: Square, occ: u64) -> u64 {
    get_rook_attacks(sq, occ) | get_bishop_attacks(sq, occ)
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
        bb.0 = FILE_BB_MASKS[1];
        assert_eq!(bb.0, 0x202020202020202);        
        bb.0 = FILE_BB_MASKS[7];
        assert_eq!(bb.0, 0x8080808080808080);
        // println!("{}", bb);
        // println!("{:x}", bb.0);
    }

    #[test]
    fn rank_bb_masks() {
        let mut bb = Bitboard::new();
        bb.0 = RANK_BB_MASKS[1];
        assert_eq!(bb.0, 0xff00);
        bb.0 = RANK_BB_MASKS[7];
        assert_eq!(bb.0, 0xff00000000000000);        
    }

    #[test]
    fn white_pawn_passed() {
        let mut bb = Bitboard::new();
        bb.0 = WHITE_PASSED_MASK[0];
        assert_eq!(bb.0, 0x303030303030300);
        bb.0 = WHITE_PASSED_MASK[37];
        assert_eq!(bb.0, 0x7070700000000000);        
    }

    #[test]
    fn black_pawn_passed() {
        let mut bb = Bitboard::new();
        bb.0 = BLACK_PASSED_MASK[63];
        assert_eq!(bb.0, 0xc0c0c0c0c0c0c0);        
    }

    #[test]
    fn isolated_pawn() {
        let mut bb = Bitboard::new();
        bb.0 = ISOLATED_MASK[0];
        assert_eq!(bb.0, 0x202020202020202);
        bb.0 = ISOLATED_MASK[55];
        assert_eq!(bb.0, 0x4040404040404040);
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
