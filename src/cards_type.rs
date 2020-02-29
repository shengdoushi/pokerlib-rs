#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CardsType {
    High,
    Pair,
    Pair2,
    Three,
    Straight,
    Flush,
    Full,
    Four,
    StraightFlush,
}

// 一些统计信息

// 全部的牌型组合数量
pub const TOTAL_RANK_COUNT: u16 = 7462;

pub const HIGH_RANK_COUNT: u16 = 1277;
pub const PAIR_RANK_COUNT: u16 = 2860;
pub const PAIR2_RANK_COUNT: u16 = 858;
pub const THREE_RANK_COUNT: u16 = 858;
pub const FULL_RANK_COUNT: u16 = 156;
pub const FOUR_RANK_COUNT: u16 = 156;
pub const FLUSH_RANK_COUNT: u16 = 1277;
pub const STRAIGHT_RANK_COUNT: u16 = 10;
pub const STRAIGHT_FLUSH_RANK_COUNT: u16 = 10;
