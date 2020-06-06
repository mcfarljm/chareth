use crate::board::*;
use crate::moves::Move;
use crate::pieces::Piece;

impl Board {
    pub fn parse_move(&self, input: &str) -> Option<Move> {
        if input.len() < 4 {
            return None;
        }
        if input.chars().nth(0).unwrap() as u8 > 'h' as u8 ||
            (input.chars().nth(0).unwrap() as u8) < 'a' as u8 {
            return None;
        }
        if input.chars().nth(2).unwrap() as u8 > 'h' as u8 ||
            (input.chars().nth(2).unwrap() as u8) < 'a' as u8 {
            return None;
        }
        if input.chars().nth(1).unwrap() as u8 > '8' as u8 ||
            (input.chars().nth(1).unwrap() as u8) < '1' as u8 {
            return None;
        }
        if input.chars().nth(3).unwrap() as u8 > '8' as u8 ||
            (input.chars().nth(3).unwrap() as u8) < '1' as u8 {
            return None;
        }

        let from = fr_to_sq(input.chars().nth(0).unwrap() as Square - 'a' as Square, input.chars().nth(1).unwrap() as Square - '1' as Square);
        let to = fr_to_sq(input.chars().nth(2).unwrap() as Square - 'a' as Square, input.chars().nth(3).unwrap() as Square - '1' as Square);
        assert!(square_on_board(from));
        assert!(square_on_board(to));

        let move_list = self.generate_all_moves();

        for smv in move_list.moves.into_iter() {
            let mv = smv.mv;
            if mv.from() == from && mv.to() == to {
                let prom_piece = mv.promote;
                if prom_piece.exists() {
                    match prom_piece {
                        Piece::WR | Piece::BR if input.chars().nth(4).unwrap() == 'r' => { return Some(mv); }
                        Piece::WB | Piece::BB if input.chars().nth(4).unwrap() == 'b' => { return Some(mv); }
                        Piece::WQ | Piece::BQ if input.chars().nth(4).unwrap() == 'q' => { return Some(mv); }
                        Piece::WN | Piece::BN if input.chars().nth(4).unwrap() == 'n' => { return Some(mv); }
                        _ => continue,
                    }
                } else {
                    return Some(mv);
                }
            }
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_move() {
        let mut board = Board::from_fen(START_FEN);
        let mut mv = board.parse_move("e2e4");
        assert_eq!(mv.unwrap().to_string(), "e2e4");

        mv = board.parse_move("e2e5");
        assert!(mv.is_none());

        mv = board.parse_move("b1c3");
        assert_eq!(mv.unwrap().to_string(), "b1c3");

        // Promotion:
        board = board.update_from_fen("6k1/P7/8/6r1/8/2b5/5n2/2K5 w - - 0 1");
        mv = board.parse_move("a7a8q");
        assert_eq!(mv.unwrap().to_string(), "a7a8q");
    }
}
