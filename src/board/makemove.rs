use crate::board::*;
use crate::moves::Move;
use crate::validate::side_valid;

impl Board {
    // Return false if in check after making the move
    pub fn make_move(&mut self, mv: &Move) -> bool {
        debug_assert!(self.check());

        let from = mv.from();
        let to = mv.to();
        let side = self.side;

        debug_assert!(square_on_board(from));
        debug_assert!(square_on_board(to));
        debug_assert!(side_valid(side));
        debug_assert!(self.pieces[from as usize].exists());

        let prev_hash = self.hash;

        if mv.is_en_pas() {
            if side == WHITE {
                self.clear_piece(to - 10);
            } else {
                self.clear_piece(to + 10);
            }
        }
        else if mv.is_castle() {
            if to == Position::C1 as Square {
                self.move_piece(Position::A1 as Square, Position::D1 as Square);
            }
            else if to == Position::C8 as Square {
                self.move_piece(Position::A8 as Square, Position::D8 as Square);
            }
            else if to == Position::G1 as Square {
                self.move_piece(Position::H1 as Square, Position::F1 as Square);
            }
            else if to == Position::G8 as Square {
                self.move_piece(Position::H8 as Square, Position::F8 as Square);
            }
            else {
                panic!("invalid to position in move with castle flag");
            }
        }

        if self.en_pas != Position::NONE as Square {
            self.hash_en_pas();
        }
        // Hash out current state
        self.hash_castle();

        let undo = Undo{
            mv: mv.clone(),
            fifty_move: self.fifty_move,
            en_pas: self.en_pas,
            castle_perm: self.castle_perm,
            hash: prev_hash,
        };

        // Todo: verify this can just go at the end...
        self.history.push(undo);

        self.castle_perm &= CASTLE_PERM[from as usize];
        self.castle_perm &= CASTLE_PERM[to as usize];
        self.en_pas = Position::NONE as Square;
            
        // Hash in new state of castling permission
        self.hash_castle();

        self.fifty_move += 1;

        if mv.is_capture() {
            self.clear_piece(to);
            self.fifty_move = 0;
        }

        self.ply += 1;
        self.hist_ply += 1;

        if self.pieces[from as usize].is_pawn() {
            self.fifty_move = 0;
            if mv.is_pawn_start() {
                if side == WHITE {
                    self.en_pas = from + 10;
                    debug_assert!(RANKS[self.en_pas as usize] == RANK_3);
                } else {
                    self.en_pas = from - 10;
                    debug_assert!(RANKS[self.en_pas as usize] == RANK_6);
                }
                self.hash_en_pas();
            }
        }

        self.move_piece(from, to);

        if mv.is_promotion() {
            debug_assert!(mv.promote.exists() && ! mv.promote.is_pawn());
            self.clear_piece(to);
            self.add_piece(mv.promote, to);
        }

        if self.pieces[to as usize].is_king() {
            self.king_sq[self.side] = to;
        }

        self.side ^= 1;
        self.hash_side();

        debug_assert!(self.check());

        if self.square_attacked(self.king_sq[side], self.side) {
            self.undo_move();
            return false;
        }
        
        true
    }

    pub fn undo_move(&mut self) {
        debug_assert!(self.check());

        self.hist_ply -= 1;
        self.ply -= 1;

        let undo = self.history.pop().unwrap();
        let mv = &undo.mv;
        let from = mv.from();
        let to = mv.to();

        debug_assert!(square_on_board(from));
        debug_assert!(square_on_board(to));

        if self.en_pas != Position::NONE as Square {
            self.hash_en_pas();
        }
        self.hash_castle();

        self.castle_perm = undo.castle_perm;
        self.fifty_move = undo.fifty_move;
        self.en_pas = undo.en_pas;

        if self.en_pas != Position::NONE as Square {
            self.hash_en_pas();
        }
        self.hash_castle();

        self.side ^= 1;
        self.hash_side();

        if mv.is_en_pas() {
            if self.side == WHITE {
                self.add_piece(Piece::BP, to-10);
            } else {
                self.add_piece(Piece::WP, to+10);
            }
        } else if mv.is_castle() {
            if to == Position::C1 as Square {
                self.move_piece(Position::D1 as Square, Position::A1 as Square);
            }
            else if to == Position::C8 as Square {
                self.move_piece(Position::D8 as Square, Position::A8 as Square);
            }
            else if to == Position::G1 as Square {
                self.move_piece(Position::F1 as Square, Position::H1 as Square);
            }
            else if to == Position::G8 as Square {
                self.move_piece(Position::F8 as Square, Position::H8 as Square);
            }
            else {
                panic!("invalid to position in undo move with castle flag");
            }
        }

        self.move_piece(to, from);

        if self.pieces[from as usize].is_king() {
            self.king_sq[self.side] = from;
        }

        if mv.is_capture() {
            self.add_piece(mv.capture, to);
        }

        if mv.is_promotion() {
            debug_assert!(mv.promote.exists() && ! mv.promote.is_pawn());
            self.clear_piece(from);
            self.add_piece(if mv.promote.color() == WHITE { Piece::WP } else { Piece::BP }, from)
        }

        debug_assert!(self.check());
    }
    
    fn clear_piece(&mut self, sq: Square) {
        debug_assert!(square_on_board(sq));
        let piece = self.pieces[sq as usize];
        debug_assert!(piece.exists());

        let color = piece.color();

        self.hash_piece(piece, sq);

        self.pieces[sq as usize] = Piece::Empty;
        self.material[color] -= piece.value();


        if piece.is_big() {
            self.num_big_piece[color] -= 1;
            if piece.is_major() {
                self.num_major_piece[color] -= 1;
            } else {
                self.num_minor_piece[color] -= 1;
            }
        } else {
            let sq64 = SQUARE_120_TO_64[sq as usize];
            self.bitboards[piece as usize].clear_bit(sq64);
        }

        // Remove from piece list
        let i_piece = self.piece_lists[piece as usize]
            .iter()
            .position(|x| *x == sq)
            .expect("piece must exist in piece list");
        self.piece_lists[piece as usize].swap_remove(i_piece);
    }

    fn add_piece(&mut self, piece: Piece, sq: Square) {
        debug_assert!(piece.exists());
        debug_assert!(square_on_board(sq));

        let color = piece.color();

        self.hash_piece(piece, sq);
        self.pieces[sq as usize] = piece;

        if piece.is_big() {
            self.num_big_piece[color] += 1;
            if piece.is_major() {
                self.num_major_piece[color] += 1;
            } else {
                self.num_minor_piece[color] += 1;
            }
        } else {
            let sq64 = SQUARE_120_TO_64[sq as usize];
            self.bitboards[piece as usize].set_bit(sq64);
        }

        self.material[color] += piece.value();
        self.piece_lists[piece as usize].push(sq);
    }

    fn move_piece(&mut self, from: Square, to: Square) {
        debug_assert!(square_on_board(from));
        debug_assert!(square_on_board(to));
        let piece = self.pieces[from as usize];
        let color = piece.color();

        self.hash_piece(piece, from);
        self.pieces[from as usize] = Piece::Empty;

        self.hash_piece(piece, to);
        self.pieces[to as usize] = piece;

        if ! piece.is_big() {
            let from64 = SQUARE_120_TO_64[from as usize];
            let to64 = SQUARE_120_TO_64[to as usize];
            self.bitboards[piece as usize].clear_bit(from64);
            self.bitboards[piece as usize].set_bit(to64);
        }

        let sq = self.piece_lists[piece as usize]
            .iter_mut()
            .find(|sq| **sq == from)
            .expect("from square must have a piece");
        *sq = to;
    }

    fn hash_piece(&mut self, piece: Piece, sq: Square) {
        self.hash ^= self.hash_keys.piece_keys[piece as usize][sq as usize];
    }

    fn hash_side(&mut self) {
        self.hash ^= self.hash_keys.side_key;
    }

    fn hash_en_pas(&mut self) {
        self.hash ^= self.hash_keys.piece_keys[Piece::Empty as usize][self.en_pas as usize];
    }

    fn hash_castle(&mut self) {
        self.hash ^= self.hash_keys.castle_keys[self.castle_perm as usize];
    }
}

const CASTLE_PERM: [u8; 120] = [
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 13, 15, 15, 15, 12, 15, 15, 14, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15,  7, 15, 15, 15,  3, 15, 15, 11, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15
];
