use crate::Evaluator;
use crate::card;
use crate::cards_type::CardsType;
use crate::cards_type;
use std::fs::File;
use std::io::prelude::*;

const VALUE_CONTENTS: [u32;7462] = include!("../cactuskev/tbl_value_contents");

pub struct TwoPlusTwoEvaluator{
    hr: Vec<u32>,
}

impl TwoPlusTwoEvaluator {
    pub fn with_data_file(filename: &str) -> Self{
        let mut f = File::open(filename).expect("twoplustwo data file not found");
        let mut hru8: Vec<u8> = Vec::with_capacity(4*32487834);
        f.read_to_end(&mut hru8).ok().unwrap();

        let ptr = hru8.as_ptr();
        TwoPlusTwoEvaluator{
            hr: unsafe {
                std::mem::forget(hru8);
                Vec::from_raw_parts(ptr as *mut u32, 32487834, 32487834)
            },
        }
    }

    #[inline]
    fn get_at(&self, pos: u32) -> u32 {
        unsafe {
            *self.hr.as_ptr().add(pos as usize)
        }
    }
    /// 解析一个结果： 包含牌型以及每个牌值（没有花色）
    pub fn unpack_eval_value(&self, eval_value: u32) -> Option<(CardsType, Vec<card::Value>)>{
        let ctype = self.eval_value_type(eval_value);
        if ctype.is_none(){
            return None;
        }
        let ctype = ctype.unwrap();

        let mut idx: u16 = (eval_value&0xFFF) as u16;
        // 表中 从小到大排列的， 而 eval_value 中存的是本类型内从大到小的排列
        idx = match ctype {
            CardsType::High => idx,
            CardsType::Pair =>
                cards_type::HIGH_RANK_COUNT + idx,
            CardsType::Pair2 =>
                cards_type::HIGH_RANK_COUNT + cards_type::PAIR_RANK_COUNT
                + idx,
            CardsType::Three => 
                cards_type::HIGH_RANK_COUNT + cards_type::PAIR_RANK_COUNT + cards_type::PAIR2_RANK_COUNT
                + idx,
            CardsType::Straight => 
                cards_type::HIGH_RANK_COUNT + cards_type::PAIR_RANK_COUNT + cards_type::PAIR2_RANK_COUNT + cards_type::THREE_RANK_COUNT
                + idx,
            CardsType::Flush =>
                cards_type::HIGH_RANK_COUNT + cards_type::PAIR_RANK_COUNT + cards_type::PAIR2_RANK_COUNT + cards_type::THREE_RANK_COUNT + cards_type::STRAIGHT_RANK_COUNT 
                + idx,
            CardsType::Full => 
                cards_type::HIGH_RANK_COUNT + cards_type::PAIR_RANK_COUNT + cards_type::PAIR2_RANK_COUNT + cards_type::THREE_RANK_COUNT + cards_type::STRAIGHT_RANK_COUNT + cards_type::FLUSH_RANK_COUNT
                + idx,
            CardsType::Four => 
                cards_type::HIGH_RANK_COUNT + cards_type::PAIR_RANK_COUNT + cards_type::PAIR2_RANK_COUNT + cards_type::THREE_RANK_COUNT + cards_type::STRAIGHT_RANK_COUNT + cards_type::FLUSH_RANK_COUNT + cards_type::FULL_RANK_COUNT 
                + idx,
            CardsType::StraightFlush => 
                cards_type::HIGH_RANK_COUNT + cards_type::PAIR_RANK_COUNT + cards_type::PAIR2_RANK_COUNT + cards_type::THREE_RANK_COUNT + cards_type::STRAIGHT_RANK_COUNT + cards_type::FLUSH_RANK_COUNT + cards_type::FULL_RANK_COUNT + cards_type::FOUR_RANK_COUNT 
                + idx,
        } - 1;
        let value = VALUE_CONTENTS[idx as usize];
        let mut cards: Vec<card::Value> = Vec::new();
        for i in 0..5 {
            let v = (value >> (4-i)*4) & 0xF;
            cards.push(card::Value::from_index(v as u8 - 1));
        }
        Some((ctype, cards))
    }
}

impl Evaluator for TwoPlusTwoEvaluator {
    type CardType = u32;

    // 1-52
    fn make_card(&self, card: &card::Card) -> u32 {
        card.card_index(card::CardIndexPolicy::GroupByValue) as u32 + 1
    }

    fn eval_value_type(&self, eval_value: u32)-> Option<CardsType> {
        match eval_value >> 12 {
            1 => Some(CardsType::High),
            2 => Some(CardsType::Pair),
            3 => Some(CardsType::Pair2),
            4 => Some(CardsType::Three),
            5 => Some(CardsType::Straight),
            6 => Some(CardsType::Flush),
            7 => Some(CardsType::Full),
            8 => Some(CardsType::Four),
            9 => Some(CardsType::StraightFlush),
            _ => return None,
        }
    }
    
    fn eval(&self, input_cards: &[u32]) -> u32 {
        let mut p: u32 = 53;
        for i in 0..input_cards.len() {
            p = self.get_at(p + input_cards[i]);
        }
        if input_cards.len() < 7 {
            p = self.get_at(p);
        }
        p
    }
}
