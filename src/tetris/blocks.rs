use super::{tetris_block::TetrisBlock, position::Position};

pub enum BlockType {
    IBlock(IBlock),
    JBlock(JBlock),
    LBlock(LBlock),
    OBlock(OBlock),
    SBlock(SBlock),
    TBlock(TBlock),
    ZBlock(ZBlock),
}

impl BlockType {
    // BlockTypeからTetrisBlockに変換するメソッドを実装
    pub fn to_tetris_block(&self) -> TetrisBlock {
        match self {
            BlockType::IBlock(block) => block.block.clone(),
            BlockType::JBlock(block) => block.block.clone(),
            BlockType::LBlock(block) => block.block.clone(),
            BlockType::OBlock(block) => block.block.clone(),
            BlockType::SBlock(block) => block.block.clone(),
            BlockType::TBlock(block) => block.block.clone(),
            BlockType::ZBlock(block) => block.block.clone(),
        }
    }
}

// Lブロック
pub struct LBlock {
    pub block: TetrisBlock,
}

impl LBlock {
    pub fn new() -> Self {
        let mut block = TetrisBlock::new();
        block.id = 1;
        block.cells.push(vec![
            Position { row: 0, column: 2 },
            Position { row: 1, column: 0 },
            Position { row: 1, column: 1 },
            Position { row: 1, column: 2 },
        ]);
        block.cells.push(vec![
            Position { row: 0, column: 1 },
            Position { row: 1, column: 1 },
            Position { row: 2, column: 1 },
            Position { row: 2, column: 2 },
        ]);
        block.cells.push(vec![
            Position { row: 1, column: 0 },
            Position { row: 1, column: 1 },
            Position { row: 1, column: 2 },
            Position { row: 2, column: 0 },
        ]);
        block.cells.push(vec![
            Position { row: 0, column: 0 },
            Position { row: 0, column: 1 },
            Position { row: 1, column: 1 },
            Position { row: 2, column: 1 },
        ]);
        block.move_block(0, 3);
        LBlock { block }
    }
}

// Jブロック
pub struct JBlock {
    pub block: TetrisBlock,
}

impl JBlock {
    pub fn new() -> Self {
        let mut block = TetrisBlock::new();
        block.id = 2;
        block.cells.push(vec![
            Position { row: 0, column: 0 },
            Position { row: 1, column: 0 },
            Position { row: 1, column: 1 },
            Position { row: 1, column: 2 },
        ]);
        block.cells.push(vec![
            Position { row: 0, column: 1 },
            Position { row: 0, column: 2 },
            Position { row: 1, column: 1 },
            Position { row: 2, column: 1 },
        ]);
        block.cells.push(vec![
            Position { row: 1, column: 0 },
            Position { row: 1, column: 1 },
            Position { row: 1, column: 2 },
            Position { row: 2, column: 2 },
        ]);
        block.cells.push(vec![
            Position { row: 0, column: 1 },
            Position { row: 1, column: 1 },
            Position { row: 2, column: 0 },
            Position { row: 2, column: 1 },
        ]);
        block.move_block(0, 3);
        JBlock { block }
    }
}

// Iブロック
pub struct IBlock {
    pub block: TetrisBlock,
}

impl IBlock {
    pub fn new() -> Self {
        let mut block = TetrisBlock::new();
        block.id = 3;
        block.cells.push(vec![
            Position { row: 1, column: 0 },
            Position { row: 1, column: 1 },
            Position { row: 1, column: 2 },
            Position { row: 1, column: 3 },
        ]);
        block.cells.push(vec![
            Position { row: 0, column: 2 },
            Position { row: 1, column: 2 },
            Position { row: 2, column: 2 },
            Position { row: 3, column: 2 },
        ]);
        block.cells.push(vec![
            Position { row: 2, column: 0 },
            Position { row: 2, column: 1 },
            Position { row: 2, column: 2 },
            Position { row: 2, column: 3 },
        ]);
        block.cells.push(vec![
            Position { row: 0, column: 1 },
            Position { row: 1, column: 1 },
            Position { row: 2, column: 1 },
            Position { row: 3, column: 1 },
        ]);
        block.move_block(-1, 3);
        IBlock { block }
    }
}

// Oブロック
pub struct OBlock {
    pub block: TetrisBlock,
}

impl OBlock {
    pub fn new() -> Self {
        let mut block = TetrisBlock::new();
        block.id = 4;
        block.cells.push(vec![
            Position { row: 0, column: 0 },
            Position { row: 0, column: 1 },
            Position { row: 1, column: 0 },
            Position { row: 1, column: 1 },
        ]);
        block.move_block(0, 4);
        OBlock { block }
    }
}

// Sブロック
pub struct SBlock {
    pub block: TetrisBlock,
}

impl SBlock {
    pub fn new() -> Self {
        let mut block = TetrisBlock::new();
        block.id = 5;
        block.cells.push(vec![
            Position { row: 0, column: 1 },
            Position { row: 0, column: 2 },
            Position { row: 1, column: 0 },
            Position { row: 1, column: 1 },
        ]);
        block.cells.push(vec![
            Position { row: 0, column: 1 },
            Position { row: 1, column: 1 },
            Position { row: 1, column: 2 },
            Position { row: 2, column: 2 },
        ]);
        block.cells.push(vec![
            Position { row: 1, column: 1 },
            Position { row: 1, column: 2 },
            Position { row: 2, column: 0 },
            Position { row: 2, column: 1 },
        ]);
        block.cells.push(vec![
            Position { row: 0, column: 0 },
            Position { row: 1, column: 0 },
            Position { row: 1, column: 1 },
            Position { row: 2, column: 1 },
        ]);
        block.move_block(0, 3);
        SBlock { block }
    }
}

// Tブロック
pub struct TBlock {
    pub block: TetrisBlock,
}

impl TBlock {
    pub fn new() -> Self {
        let mut block = TetrisBlock::new();
        block.id = 6;
        block.cells.push(vec![
            Position { row: 0, column: 1 },
            Position { row: 1, column: 0 },
            Position { row: 1, column: 1 },
            Position { row: 1, column: 2 },
        ]);
        block.cells.push(vec![
            Position { row: 0, column: 1 },
            Position { row: 1, column: 1 },
            Position { row: 1, column: 2 },
            Position { row: 2, column: 1 },
        ]);
        block.cells.push(vec![
            Position { row: 1, column: 0 },
            Position { row: 1, column: 1 },
            Position { row: 1, column: 2 },
            Position { row: 2, column: 1 },
        ]);
        block.cells.push(vec![
            Position { row: 0, column: 1 },
            Position { row: 1, column: 0 },
            Position { row: 1, column: 1 },
            Position { row: 2, column: 1 },
        ]);
        block.move_block(0, 3);
        TBlock { block }
    }
}

// Zブロック
pub struct ZBlock {
    pub block: TetrisBlock,
}

impl ZBlock {
    pub fn new() -> Self {
        let mut block = TetrisBlock::new();
        block.id = 7;
        block.cells.push(vec![
            Position { row: 0, column: 0 },
            Position { row: 0, column: 1 },
            Position { row: 1, column: 1 },
            Position { row: 1, column: 2 },
        ]);
        block.cells.push(vec![
            Position { row: 0, column: 2 },
            Position { row: 1, column: 1 },
            Position { row: 1, column: 2 },
            Position { row: 2, column: 1 },
        ]);
        block.cells.push(vec![
            Position { row: 1, column: 0 },
            Position { row: 1, column: 1 },
            Position { row: 2, column: 1 },
            Position { row: 2, column: 2 },
        ]);
        block.cells.push(vec![
            Position { row: 0, column: 1 },
            Position { row: 1, column: 0 },
            Position { row: 1, column: 1 },
            Position { row: 2, column: 0 },
        ]);
        block.move_block(0, 3);

        ZBlock { block }
    }
}

pub fn get_block_position(id: u32) -> Vec<Vec<Position>> {
    match id {
        1 => {
            vec![
                vec![
                    Position { row: 0, column: 2 },
                    Position { row: 1, column: 0 },
                    Position { row: 1, column: 1 },
                    Position { row: 1, column: 2 },
                ],
                vec![
                    Position { row: 0, column: 1 },
                    Position { row: 1, column: 1 },
                    Position { row: 2, column: 1 },
                    Position { row: 2, column: 2 },
                ],
                vec![
                    Position { row: 1, column: 0 },
                    Position { row: 1, column: 1 },
                    Position { row: 1, column: 2 },
                    Position { row: 2, column: 0 },
                ],
                vec![
                    Position { row: 0, column: 0 },
                    Position { row: 0, column: 1 },
                    Position { row: 1, column: 1 },
                    Position { row: 2, column: 1 },
                ],
            ]
        }
        2 => {
            vec![
                vec![
                    Position { row: 0, column: 0 },
                    Position { row: 1, column: 0 },
                    Position { row: 1, column: 1 },
                    Position { row: 1, column: 2 },
                ],
                vec![
                    Position { row: 0, column: 1 },
                    Position { row: 0, column: 2 },
                    Position { row: 1, column: 1 },
                    Position { row: 2, column: 1 },
                ],
                vec![
                    Position { row: 1, column: 0 },
                    Position { row: 1, column: 1 },
                    Position { row: 1, column: 2 },
                    Position { row: 2, column: 2 },
                ],
                vec![
                    Position { row: 0, column: 1 },
                    Position { row: 1, column: 1 },
                    Position { row: 2, column: 0 },
                    Position { row: 2, column: 1 },
                ],
            ]
        }
        3 => {
            vec![
                vec![
                    Position { row: 1, column: 0 },
                    Position { row: 1, column: 1 },
                    Position { row: 1, column: 2 },
                    Position { row: 1, column: 3 },
                ],
                vec![
                    Position { row: 0, column: 2 },
                    Position { row: 1, column: 2 },
                    Position { row: 2, column: 2 },
                    Position { row: 3, column: 2 },
                ],
                vec![
                    Position { row: 2, column: 0 },
                    Position { row: 2, column: 1 },
                    Position { row: 2, column: 2 },
                    Position { row: 2, column: 3 },
                ],
                vec![
                    Position { row: 0, column: 1 },
                    Position { row: 1, column: 1 },
                    Position { row: 2, column: 1 },
                    Position { row: 3, column: 1 },
                ]
            ]
        }
        4 => {
            vec![
                vec![
                    Position { row: 0, column: 0 },
                    Position { row: 0, column: 1 },
                    Position { row: 1, column: 0 },
                    Position { row: 1, column: 1 },
                ],
            ]
        }
        5 => {
            vec![
                vec![
                    Position { row: 0, column: 1 },
                    Position { row: 0, column: 2 },
                    Position { row: 1, column: 0 },
                    Position { row: 1, column: 1 },
                ],
                vec![
                    Position { row: 0, column: 1 },
                    Position { row: 1, column: 1 },
                    Position { row: 1, column: 2 },
                    Position { row: 2, column: 2 },
                ],
                vec![
                    Position { row: 1, column: 1 },
                    Position { row: 1, column: 2 },
                    Position { row: 2, column: 0 },
                    Position { row: 2, column: 1 },
                ],
                vec![
                    Position { row: 0, column: 0 },
                    Position { row: 1, column: 0 },
                    Position { row: 1, column: 1 },
                    Position { row: 2, column: 1 },
                ],
            ]
        }
        6 => {
            vec![
                vec![
                    Position { row: 0, column: 1 },
                    Position { row: 1, column: 0 },
                    Position { row: 1, column: 1 },
                    Position { row: 1, column: 2 },
                ],
                vec![
                    Position { row: 0, column: 1 },
                    Position { row: 1, column: 1 },
                    Position { row: 1, column: 2 },
                    Position { row: 2, column: 1 },
                ],
                vec![
                    Position { row: 1, column: 0 },
                    Position { row: 1, column: 1 },
                    Position { row: 1, column: 2 },
                    Position { row: 2, column: 1 },
                ],
                vec![
                    Position { row: 0, column: 1 },
                    Position { row: 1, column: 0 },
                    Position { row: 1, column: 1 },
                    Position { row: 2, column: 1 },
                ],
            ]
        }
        7 => {
            vec![
                vec![
                    Position { row: 0, column: 0 },
                    Position { row: 0, column: 1 },
                    Position { row: 1, column: 1 },
                    Position { row: 1, column: 2 },
                ],
                vec![
                    Position { row: 0, column: 2 },
                    Position { row: 1, column: 1 },
                    Position { row: 1, column: 2 },
                    Position { row: 2, column: 1 },
                ],
                vec![
                    Position { row: 1, column: 0 },
                    Position { row: 1, column: 1 },
                    Position { row: 2, column: 1 },
                    Position { row: 2, column: 2 },
                ],
                vec![
                    Position { row: 0, column: 1 },
                    Position { row: 1, column: 0 },
                    Position { row: 1, column: 1 },
                    Position { row: 2, column: 0 },
                ],
            ]
        }
        _ => {
            vec![
                vec![
                    Position { row: 0, column: 0 },
                    Position { row: 0, column: 1 },
                    Position { row: 1, column: 0 },
                    Position { row: 1, column: 1 },
                ],
            ]
        }
    }
}