use crate::board;
use crate::board::Square;
use crate::pieces::Piece;

pub fn square_string(sq: Square) -> String {
    String::from(format!("{}{}",
                         ('a' as u8 + board::FILES[sq as usize] as u8) as char,
                         ('1' as u8 + board::RANKS[sq as usize] as u8) as char))
}

#[derive(Clone)]
#[derive(Copy)]
pub enum MoveFlag {
    None,
    EnPas,
    PawnStart,
    Castle,
}

#[derive(Clone)]
#[derive(Copy)]
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
}

#[cfg(test)]
mod tests {
    use crate::moves::*;
    
    #[test]
    fn move_string() {
        let mv = Move::new(board::Position::C1 as Square, board::Position::C3 as Square, Piece::Empty, Piece::WR, MoveFlag::None);
        let s = mv.to_string();
        assert_eq!(s, "c1c3r");
    }
}
