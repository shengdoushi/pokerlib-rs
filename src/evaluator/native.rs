use crate::Evaluator;
use crate::card;
use crate::cards_type::CardsType;

/// a native evaluator
///
/// evaluate immediately, without use any table
pub struct NativeEvaluator {}

impl NativeEvaluator {
    /// new a native evaluator
    pub fn new() -> Self {
        NativeEvaluator{}
    }

    // 1-13
    #[inline]
    fn card_value(&self, card: u32) -> u32 {
        card & 0xF
    }

    /// unpack a eval value
    ///
    /// result:
    ///   - cardstype
    ///   - result card's values
    pub fn unpack_eval_value(&self, eval_value: u32) -> Option<(CardsType, Vec<card::Value>)>{
        let ctype = self.eval_value_type(eval_value);
        if ctype.is_none(){
            return None;
        }
        let ctype = ctype.unwrap();
        let mut values: Vec<card::Value> = Vec::new();
        let space_diff = if ctype == CardsType::Straight || ctype == CardsType::StraightFlush {1} else {0};
        let mut last_v = 0;
        for i in  0..5 {
            let v = ((eval_value>>((4-i)*4)) & 0xF) as u8;
            if v != 0 {
                values.push(card::Value::from_index(v - 1));
                last_v = v;
            }else{
                last_v = ((last_v-1+13-space_diff) % 13) + 1;
                values.push(card::Value::from_index(last_v - 1));
            }
        }
        Some((ctype, values))
    }
}

impl Evaluator for NativeEvaluator {
    /// card format:
    /// u32
    ///
    /// +--------+--------+--------+--------+
    /// |ttbbbbbb|bbbbbbbb|xxsxxhxx|dxxcrrrr|
    /// +--------+--------+--------+--------+
    /// r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
    /// cdhs = suit of card (bit turned on based on suit of card)
    /// b = bit turned on depending on rank of card, note: first tag and last is A
    /// t = suit value (0,1,2,3)
    type CardType = u32;


    fn make_card(&self, card: &card::Card) -> u32 {
        let suit_index = card.suit_index() as u32;
        let value_index = card.value_index() as u32;
        (value_index+1)
            | ((1<<(3*suit_index)) << 4)
            | ((1<< (value_index+1)) << 16)
            | (if value_index==12 {0x10000} else {0})
            | (suit_index << 30)
    }

    fn eval_value_type(&self, eval_value: u32) -> Option<CardsType> {
        match eval_value >> 20 {
            1 => Some(CardsType::High),
            2 => Some(CardsType::Pair),
            3 => Some(CardsType::Pair2),
            4 => Some(CardsType::Three),
            5 => Some(CardsType::Straight),
            6 => Some(CardsType::Flush),
            7 => Some(CardsType::Full),
            8 => Some(CardsType::Four),
            9 => Some(CardsType::StraightFlush),
            _ => None,
        }
    }

    fn eval(&self, input_cards: &[u32]) -> u32 {
        if input_cards.len() > 7 || input_cards.len() < 5 {
            return 0;
        }
        let mut suit_sum = 0;

        // sort
        let mut all_sorted_cards: [u32;7] = [0;7];
        let cards_len = input_cards.len();
        for i in 0..7{
            if i >= cards_len {break;}
            let c = input_cards[i];
            suit_sum += (c >> 4) & 0xFFF;
            all_sorted_cards[i] = c;
        }
        all_sorted_cards.sort_by(|&a, &b| self.card_value(b).partial_cmp(&self.card_value(a)).unwrap() );
        let cards = &all_sorted_cards[0..cards_len];
        
        // 判断花色
        let mut flush_suit: u32 = 0;
        for i in 0..4{
            if (suit_sum&0x7) > 4 {
                flush_suit=(1<<(i*3)) as u32;
                break
            }
            suit_sum >>= 3;
        }

        if flush_suit != 0 {
            let mut flush_card_values = 0;
            let mut flush_card_count = 0;
            let mut cards_value = 0;

            for &c in cards.iter(){
                if (c>>4)&0xFFF == flush_suit {
                    flush_card_count+=1;
                    flush_card_values<<=4; flush_card_values|=self.card_value(c);
                    cards_value |= (c>>16)&0x3FFFF;
                }
            }

            // straight-flush 9
            for i in 0..=flush_card_count-4 {
                let cv = (flush_card_values>>(4*(flush_card_count-i-1))) & 0xF;
                if cv < 4 {
                    break
                }
                if (0x1F & (cards_value >> (cv-4))) == 0x1F{
                    return (9 << 20) | (cv << 16);
                }
            }
            // flush 6
            return (6 << 20) | (flush_card_values>>(4*(flush_card_count-5)));
        }

        // 四条 8
        for i in 0..=cards.len()-4 {
            if self.card_value(cards[i]) == self.card_value(cards[i+3]) {
                return (8<<20) | (self.card_value(cards[i]) << 16)
                    | (self.card_value(cards[if i==0 {4} else {0}]));
            }
        }

        let mut total_cards_value = 0;
        let mut three = 0;         // 只记录一个即可
        let mut twos = 0;          // 只记录两个即可
        let mut ones = 0;
        let mut ones_count = 0;
        let mut i = 0;
        while i < cards.len() {
            let cv = self.card_value(cards[i]);
            total_cards_value |= (cards[i]>>16) & 0x3FFF;
            i += if (i+2)<cards.len() && cv == self.card_value(cards[i+2]) {
                if three != 0 {
                    twos <<= 4; twos |= cv;
                }else{
                    three = cv;
                }
                3
            }else if (i+1)<cards.len() && cv == self.card_value(cards[i+1]) {
                if (twos >> 4) != 0 {
                    ones <<= 4; ones |= cv; ones_count+=1;
                }else{
                    twos <<= 4; twos |= cv;
                }
                2
            }else {
                ones <<= 4; ones |= cv; ones_count+=1;
                1
            };
        }

        // 葫芦 7
        if three != 0 && twos != 0{
            return (7 << 20) | (three << 16) | (twos << 4)
        }
        
        // 顺子 5
        for i in 0..=cards.len()-4 {
            let cv = self.card_value(cards[i]);
            if cv < 4 {
                break
            }
            if (0x1F & (total_cards_value >> (cv-4))) == 0x1F{
                return (5 << 20) | (cv << 16);
            }
        }

        // 三条 4
        if three != 0 {
            return (4 << 20) | (three << 16) | (ones>>((ones_count-2)*4))
        }
        // 二对 3
        if (twos >> 4) != 0 {
            return (3 << 20) | ((twos >> 4) <<16) | ((twos&0xF) << 8) | (((ones>>((ones_count-1)*4))&0xF))
        }
        // 一对 2
        if twos != 0 {
            return (2 << 20) | (twos << 16) | (ones>>((ones_count-3)*4))
        }
        // 高牌 1
        (1 << 20) | (ones>>((ones_count-5)*4))
    }

}
