use crate::board::*;

impl Board {
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
