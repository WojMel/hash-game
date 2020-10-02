use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};

#[derive(Clone, Copy)]
pub enum Block {
    Wall,
    Empty,
}

pub struct Chunk {
    pub blocks: [Block; 3 * 3],
}

pub struct Board {
    width: u8,
    height: u8,
    chunks: Vec<Chunk>,
}

pub enum EntityBlock {
    Player(u8),
    Trap,
}

pub struct Entity {
    pub pos: (u16, u16),
    pub block: EntityBlock,
}

impl Board {
    pub fn new(width: u8, height: u8) -> Self {
        let mut chunks = Vec::<Chunk>::with_capacity((width * height).into());
        (0..height).for_each(|_y| {
            (0..height).for_each(|_x| {
                chunks.push(Chunk {
                    blocks: [Block::Wall; 9],
                })
            })
        });
        Self {
            width,
            height,
            chunks,
        }
    }

    pub fn random_rng(width: u8, height: u8, rng: &mut ThreadRng) -> Self {
        let mut chunks = Vec::<Chunk>::with_capacity((width * height).into());
        (0..height).for_each(|_y| (0..height).for_each(|_x| chunks.push(Chunk::random(rng))));
        Self {
            width,
            height,
            chunks,
        }
    }

    pub fn chunks<'a>(&'a self) -> &'a Vec<Chunk> {
        &self.chunks
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }
}

impl Chunk {
    pub fn random(rng: &mut ThreadRng) -> Self {
        lazy_static! {
            static ref DISTR: Uniform<u8> = Uniform::from(1..16);
            static ref IDX: [usize; 4] = [1, 3, 5, 7];
        }
        let mut blocks = [
            Block::Wall,
            Block::Empty,
            Block::Wall,
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Wall,
            Block::Empty,
            Block::Wall,
        ];
        for _ in 0..((DISTR.sample(rng) as f32).log2() as i32 - 4).abs() {
            blocks[IDX[DISTR.sample(rng) as usize / 4]] = Block::Wall;
        }

        Self { blocks }
    }
}

impl Entity {
    pub fn new(block: EntityBlock, pos: (u16, u16)) -> Self {
        Self { block, pos }
    }
}
