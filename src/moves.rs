use crate::board;
use crate::pieces::Piece;

const EN_PAS_FLAG: u8 = 1;
const PAWN_START_FLAG: u8 = 2;
const CASTLE_FLAG: u8 = 4;

pub fn square_string(sq: usize) -> String {
    String::from(format!("{}{}",
                         ('a' as u8 + board::FILES[sq] as u8) as char,
                         ('1' as u8 + board::RANKS[sq] as u8) as char))
}


pub enum MoveFlag {
    None,
    EnPas,
    PawnStart,
    Castle,
}


pub struct Move {
    from: u8,
    to: u8,
    capture: Piece,
    promote: Piece,
    flag: MoveFlag,
    score: i32,
}



#[allow(dead_code)]
impl Move {
    pub fn new(from: usize, to: usize, capture: Piece, promote: Piece, flag: MoveFlag) -> Move {

        Move {
            from: from as u8,
            to: to as u8,
            capture: capture,
            promote: promote,
            flag: flag,
            score: 0
        }
    }

    pub fn from(&self) -> usize {
        self.from as usize
    }

    pub fn to(&self) -> usize {
        self.to as usize
    }

    pub fn is_capture(&self) -> bool {
        self.capture.exists()
    }

    pub fn is_promotion(&self) -> bool {
        self.promote.exists()
    }

    pub fn to_string(&self) -> String {
        let from = self.from();
        let to = self.to();

        let mut s = String::from(format!("{}{}", square_string(from), square_string(to)));

        if self.promote.exists() {
            let mut pchar = 'q';
            if self.promote.is_knight() {
                pchar = 'n'
            }
            else if self.promote.is_rook_or_queen() && ! self.promote.is_bishop_or_queen() {
                pchar = 'r';
            }
            else if self.promote.is_bishop_or_queen() && ! self.promote.is_rook_or_queen() {
                pchar = 'b';
            }
            s.push(pchar);
        }

        s
    }

    pub fn score(&self) -> i32 {
        self.score
    }

}

#[cfg(test)]
mod tests {
    use crate::moves::*;
    
    #[test]
    fn move_string() {
        let mv = Move::new(board::Position::C1 as usize, board::Position::C3 as usize, Piece::Empty, Piece::WR, MoveFlag::None);
        let s = mv.to_string();
        assert_eq!(s, "c1c3r");
    }
}
