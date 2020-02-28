use crate::board::*;

impl Board {
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
            self.pawns[color].clear_bit(sq64);
            self.pawns[BOTH].clear_bit(sq64);
        }

        // Remove from piece list
        let mut i_piece = 0;
        let mut found = false;
        for (i, t_sq) in self.piece_lists[piece as usize].iter().enumerate() {
            if *t_sq == sq {
                found = true;
                i_piece = i;
                break;
            }
        }
        assert!(found);
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
            self.pawns[color].set_bit(sq64);
            self.pawns[BOTH].set_bit(sq64);
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
            self.pawns[color].clear_bit(from64);
            self.pawns[BOTH].clear_bit(from64);
            self.pawns[color].set_bit(to64);
            self.pawns[BOTH].set_bit(to64);
        }

        let mut found = false;
        for (i, t_sq) in self.piece_lists[piece as usize].iter().enumerate() {
            if *t_sq == from {
                found = true;
                self.piece_lists[piece as usize][i] = to;
                break;
            }
        }
        assert!(found);
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
