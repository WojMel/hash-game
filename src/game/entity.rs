pub enum EntityBlock {
    Player(u8),
    Trap,
}

pub struct Entity {
    pub pos: (u16, u16),
    pub block: EntityBlock,
}

impl Entity {
    pub fn new(block: EntityBlock, pos: (u16, u16)) -> Self {
        Self { block, pos }
    }
}
