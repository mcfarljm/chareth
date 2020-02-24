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

}
