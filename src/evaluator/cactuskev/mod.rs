use crate::Evaluator;
use crate::CardsType;
use crate::card;

const CARD_PRIMES: [u32;13] = [2,3,5,7,11,13,17,19,23,29,31,37,41];

// 一些表格
const FLUSHES: [u16;0x1F01] = include!("tbl_flushes");
const UNIQUE5: [u16;0x1F01] = include!("tbl_unique5");
//const OTHER_PRODUCTS: [u32;4888] = include!("tbl_other_products");
//const OTHER_VALUES: [u16;4888] = include!("tbl_other_values");
const HASH_ADJUST: [u16;512] = include!("tbl_hash_adjust");
const HASH_VALUES: [u16;8191] = include!("tbl_hash_values");
const VALUE_CONTENTS: [u32;7462] = include!("tbl_value_contents");

pub struct CactusKevEvaluator {}

/*
fn binary_search_position(arr: &[i32], value: i32) -> Option<usize> {
    let mut start = 0;
    let mut end = arr.len();
    while end >= start {
        let pos = start + (end-start)/2;
        if arr[pos] == value {
            return Some(pos);
        }else if arr[pos] < value {
            start = pos+1;
        }else{
            end = pos;
        }
    }
    None
}
*/

// perfect hash
pub fn find_fast(u: u32) -> u32 {
    let mut u = u as u64 + 0xe91aaa35;
    u ^= u >> 16;
    u += u << 8;
    u ^= ((u as u32) >> 4) as u64;
    let b: u64 = (u >> 8) & 0x1FF;
    let a: u64 = (u + (u << 2) & 0xFFFFFFFF) >> 19;
    let r: u32 = (a as u32) ^ (HASH_ADJUST[b as usize] as u32);
    r
}

impl CactusKevEvaluator {
    pub fn new() -> Self {
        CactusKevEvaluator{}
    }

    fn calc_cards_type_value_5_ex(&self, input_cards: &[u32], max_cmp_type: u32) -> u32 {
        if input_cards.len() != 5 {
            return 0;
        }

        let q: usize = (0..5).fold(0, |a, i| a | input_cards[i] as usize) >> 16;
        if 0 != (0..5).fold(0xF000, |a, i| a & input_cards[i]) {
            // straight-flush/flush
            return 7463-FLUSHES[q] as u32;
        }
        if max_cmp_type >= 9 {
            return 0;
        }

        let unique = UNIQUE5[q];
        if unique != 0 {
            return 7463-unique as u32;
        }

        let lookup = find_fast(
            (0..5).fold(1, |acc, b| acc * (input_cards[b]&0xFF)) as u32
        );
        return 7463-HASH_VALUES[lookup as usize] as u32;
    }

    fn calc_cards_type_value_5(&self, input_cards: &[u32]) -> u32 {
        self.calc_cards_type_value_5_ex(input_cards, 0)
    }

    pub fn unpack_eval_value(&self, value: u32) -> Option<(CardsType, Vec<card::Value>)>{
        let cptype = self.eval_value_type(value);
        if cptype.is_none() {
            return None
        }
        let value = VALUE_CONTENTS[value as usize-1];
        let mut cards: Vec<card::Value> = Vec::new();
        for i in 0..5 {
            let v = (value >> (4-i)*4) & 0xF;
            cards.push(card::Value::from_index(v as u8 - 1));
        }
        Some((cptype.unwrap(), cards))
    }
}

impl Evaluator for CactusKevEvaluator {
    type CardType = u32;
    
    // +--------+--------+--------+--------+
    // |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
    // +--------+--------+--------+--------+
    // p = prime number of rank (deuce=2,trey=3,four=5,...,ace=41)
    // r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
    // cdhs = suit of card (bit turned on based on suit of card)
    // b = bit turned on depending on rank of card
    fn make_card(&self, card: &card::Card) -> u32 {
        let value_index = card.value_index() as u32;
        let suit_index = card.suit_index() as u32;
        
        ((1 << value_index) << 16) |
        ((1 << suit_index) << 12) |
        (value_index << 8) |
        CARD_PRIMES[value_index as usize]
    }

    fn eval_value_type(&self, eval_value: u32) -> Option<CardsType> {
        match eval_value-1 {
            0..=1276 => Some(CardsType::High),
            1277..=4136 => Some(CardsType::Pair),
            4137..=4994 => Some(CardsType::Pair2),
            4995..=5852 => Some(CardsType::Three),
            5853..=5862 => Some(CardsType::Straight),
            5863..=7139 => Some(CardsType::Flush),
            7140..=7295 => Some(CardsType::Full),
            7296..=7451 => Some(CardsType::Four),
            7452..=7461 => Some(CardsType::StraightFlush),
            _ => None
        }
    }
    
    fn eval(&self, input_cards: &[u32]) -> u32 {
        let len = input_cards.len();
        if 5 == len {
            return self.calc_cards_type_value_5(input_cards);
        }

        let mut cards: [u32;7] = [0;7];
        for i in 0..len {
            cards[i] = input_cards[i];
        }
        
        let mut max_cmp_value = 0;
        // 6
        if 6 == len{
            max_cmp_value = self.calc_cards_type_value_5(&cards[0..5]);
            for i in 0..5 {
                cards.swap(i,5);
                let cmp_value = self.calc_cards_type_value_5_ex(&cards[0..5], max_cmp_value>>20);
                if cmp_value > max_cmp_value {
                    max_cmp_value = cmp_value;
                }
                cards.swap(i,5);
            }
        }else if 7 == len{
            for &(c1, c2) in &[
                (0,1),(0,2),(0,3),(0,4),(5,0),(0,6),
                (1,2),(1,3),(1,4),(5,1),(1,6),
                (2,3),(2,4),(5,2),(2,6),
                (3,4),(5,3),(3,6),
                (5,4),(4,6),
                (5,6)
            ] {
                cards.swap(c1, 5);
                cards.swap(c2, 6);
                let cmp_value = self.calc_cards_type_value_5_ex(&cards[0..5], max_cmp_value>>20);
                if cmp_value > max_cmp_value {
                    max_cmp_value = cmp_value;
                }
                cards.swap(c2, 6);
                cards.swap(c1, 5);
            }
        }
        
        return max_cmp_value;
    }
}



#[cfg(test)]
mod tests {
    use crate::Evaluator;
    use super::CactusKevEvaluator;

    #[test]
    fn test_bug_cards(){
        let evaluator: CactusKevEvaluator = CactusKevEvaluator::new();

        // +--------+--------+--------+--------+
        // |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
        // +--------+--------+--------+--------+
        for (cards, should_value) in &[
            (vec![0x10140b, 0x81307, 0x41205, 0x21103, 0x22103, 0x11002, 0x14002], 7454),
            (vec![1053707,529159,266757,135427,139523,147715,69634], 7454),
        ]{
            let value = evaluator.eval(cards.as_slice());
            assert_eq!(value, *should_value);
        }
    }
}
