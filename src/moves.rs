use crate::board;
use crate::board::Square;
use crate::pieces::Piece;

use std::fmt;

pub fn square_string(sq: Square) -> String {
    String::from(format!("{}{}",
                         ('a' as u8 + board::FILES[sq as usize] as u8) as char,
                         ('1' as u8 + board::RANKS[sq as usize] as u8) as char))
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub enum MoveFlag {
    None,
    EnPas,
    PawnStart,
    Castle,
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub struct Move {
    from: Square,
    to: Square,
    pub capture: Piece,
    pub promote: Piece,
    flag: MoveFlag,
}



#[allow(dead_code)]
impl Move {
    pub fn new(from: Square, to: Square, capture: Piece, promote: Piece, flag: MoveFlag) -> Move {

        Move {
            from: from,
            to: to,
            capture: capture,
            promote: promote,
            flag: flag,
        }
    }

    pub fn from(&self) -> Square {
        self.from
    }

    pub fn to(&self) -> Square {
        self.to
    }

    pub fn is_capture(&self) -> bool {
        self.capture.exists()
    }

    pub fn is_promotion(&self) -> bool {
        self.promote.exists()
    }

    pub fn is_en_pas(&self) -> bool {
        match self.flag {
            MoveFlag::EnPas => true,
            _ => false,
        }
    }

    pub fn is_castle(&self) -> bool {
        match self.flag {
            MoveFlag::Castle => true,
            _ => false,
        }
    }

    pub fn is_pawn_start(&self) -> bool {
        match self.flag {
            MoveFlag::PawnStart => true,
            _ => false,
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let from = self.from();
        let to = self.to();

        write!(f, "{}{}", square_string(from), square_string(to))?;

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
            write!(f, "{}", pchar)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::moves::*;
    
    #[test]
    fn move_string() {
        let mv = Move::new(board::Position::C1 as Square, board::Position::C3 as Square, Piece::Empty, Piece::WR, MoveFlag::None);
        assert_eq!(mv.to_string(), "c1c3r");
    }
}
