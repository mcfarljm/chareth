use crate::board;
use crate::pieces;

const EN_PAS_FLAG: u32 = 0x40000;
const PAWN_START_FLAG: u32 = 0x80000;
const CASTLE_FLAG: u32 = 0x100000;

pub struct Move {
    /*   
    0000 0000 0000 0000 0000 0111 1111 -> From 0x7F
    0000 0000 0000 0011 1111 1000 0000 -> To >> 7, 0x7F
    0000 0000 0011 1100 0000 0000 0000 -> Captured >> 14, 0xF
    0000 0000 0100 0000 0000 0000 0000 -> EP 0x40000
    0000 0000 1000 0000 0000 0000 0000 -> Pawn Start 0x80000
    0000 1111 0000 0000 0000 0000 0000 -> Promoted Piece >> 20, 0xF
    0001 0000 0000 0000 0000 0000 0000 -> Castle 0x1000000
     */
    val: u32,
}

pub fn square_string(sq: usize) -> String {
    String::from(format!("{}{}",
                         ('a' as u8 + board::FILES[sq] as u8) as char,
                         ('1' as u8 + board::RANKS[sq] as u8) as char))
}

#[allow(dead_code)]
impl Move {
    pub fn new(from: usize, to: usize, capture: usize, promote: usize, en_pas: bool, pawn_start: bool, castle: bool) -> Move {
        let mut val: u32;
        val = (from | (to << 7) | (capture << 14) | (promote << 20)) as u32;

        if en_pas { val |= EN_PAS_FLAG; }
        if pawn_start { val |= PAWN_START_FLAG; }
        if castle { val |= CASTLE_FLAG; }
        
        Move{val}
    }

    pub fn from(&self) -> usize {
        (self.val & 0x7F) as usize
    }

    pub fn to(&self) -> usize {
        ( (self.val >> 7) & 0x7F ) as usize
    }

    pub fn captured_piece(&self) -> usize {
        ((self.val >> 14) & 0xF) as usize
    }

    pub fn promoted_piece(&self) -> usize {
        ((self.val >> 20) & 0xF) as usize
    }

    pub fn en_pas(&self) -> bool {
        (self.val & EN_PAS_FLAG) != 0
    }

    pub fn pawn_start(&self) -> bool {
        (self.val & PAWN_START_FLAG) != 0
    }

    pub fn castle(&self) -> bool {
        (self.val & CASTLE_FLAG) != 0
    }

    pub fn is_capture(&self) -> bool {
        (self.val & 0x7c000) != 0
    }

    pub fn is_promotion(&self) -> bool {
        (self.val & 0xF00000) != 0
    }

    pub fn to_string(&self) -> String {
        let from = self.from();
        let to = self.to();

        let promoted_piece = self.promoted_piece();

        let mut s = String::from(format!("{}{}", square_string(from), square_string(to)));

        if promoted_piece != 0 {
            let mut pchar = 'q';
            if pieces::PIECE_IS_KNIGHT[promoted_piece] {
                pchar = 'n'
            }
            else if pieces::PIECE_IS_ROOK_OR_QUEEN[promoted_piece] && ! pieces::PIECE_IS_BISHOP_OR_QUEEN[promoted_piece] {
                pchar = 'r';
            }
            else if pieces::PIECE_IS_BISHOP_OR_QUEEN[promoted_piece] && ! pieces::PIECE_IS_ROOK_OR_QUEEN[promoted_piece] {
                pchar = 'b';
            }
            s.push(pchar);
        }

        s
    }

}

#[cfg(test)]
mod tests {
    use crate::moves::*;
    
    #[test]
    fn move_string() {
        let mv = Move::new(board::Position::C1 as usize, board::Position::C3 as usize, 0, pieces::Piece::WR, false, false, false);
        let s = mv.to_string();
        assert_eq!(s, "c1c3r");
    }
}
