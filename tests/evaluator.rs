#![feature(test)]

#[macro_use]
extern crate lazy_static;

extern crate test;
use pokerlib::cards_type::CardsType;
use pokerlib::card::Card;
use std::path::Path;
use pokerlib::Evaluator;
use pokerlib::{NativeEvaluator, CactusKevEvaluator, TwoPlusTwoEvaluator};
use pokerlib::tools::twoplustwo::generate_data_file;


lazy_static! {
    static ref NATIVE_EVALUATOR: NativeEvaluator = NativeEvaluator::new();
    static ref CACTUSKEV_EVALUATOR: CactusKevEvaluator = CactusKevEvaluator::new();
    static ref TWOPLUSTWO_EVALUATOR: TwoPlusTwoEvaluator = {
        // generate data file
        if !Path::new("TptHandRank.dat").exists() {
            generate_data_file(Path::new("TptHandRank.dat")).ok().unwrap();
        }
        TwoPlusTwoEvaluator::with_data_file("TptHandRank.dat")
    };
}

fn cards_from_str(s: &str) -> Vec<Card> {
    let mut ret: Vec<Card> = Vec::new();
    let chars = s.chars();
    for i in 0..chars.count()/2 {
        let cs = s.get(i*2..i*2+2).unwrap();
        ret.push(Card::from_str(cs).ok().unwrap());
    }
    return ret;
}

macro_rules! cards_case {
    ($name:ident, $evaluator:ident) => {
        #[test]
        fn $name() {
            for (cards, cards_type, major_cards) in [
                ("As7dKh8c3h2d9c", CardsType::High, vec!["A", "K", "9", "8", "7"]),
                ("QsAsKsJsTs9s", CardsType::StraightFlush, vec!["A","K","Q","J","T"]),

                ("Qs7sKsJsTs9s", CardsType::StraightFlush, vec!["K","Q","J","T","9"]),
                ("2sAs3s4s5s8s", CardsType::StraightFlush, vec!["5","4","3","2","A"]),
                ("2sAs3s4s7s8s", CardsType::Flush, vec!["A", "8", "7","4","3"]),
                ("AsAdAhAc8hKd", CardsType::Four, vec!["A", "A","A","A","K"]),
                ("KsKdKhKcAd", CardsType::Four, vec!["K", "K","K","K","A"]),
                ("AsAdAh7c8hKd", CardsType::Three, vec!["A", "A","A","K", "8"]),

                ("AsAdAh8c8hKd", CardsType::Full, vec!["A", "A","A","8","8"]),
                ("AsAdKh8c8hKd", CardsType::Pair2, vec!["A", "A","K","K", "8"]),
                ("As7dKh8c3h2d9c", CardsType::High, vec!["A", "K", "9", "8", "7"]),
                ("As7dKh2c3h2d", CardsType::Pair, vec!["2", "2","A", "K", "7"]),
                ("Qs7dKhJcTh9d", CardsType::Straight, vec!["K","Q","J","T","9"]),
                ("QsAdKhJcTh9d", CardsType::Straight, vec!["A","K","Q","J","T"]),
                ("2sAd3h4c5h3d", CardsType::Straight, vec!["5","4","3","2","A"]),
            ].iter(){
                let cmp_value = $evaluator.simple_eval(&cards_from_str(cards));
                let (tp, tp_cards) = $evaluator.unpack_eval_value(cmp_value).unwrap();
                assert_eq!(tp, *cards_type);
                assert_eq!(tp_cards.iter()
                           .map(|&x| x.to_string().to_string())
                           .collect::<Vec<String>>(),
                           major_cards.iter().map(|&x| x.to_string()).collect::<Vec<String>>());
            }
        }
    };
}

cards_case!(native_cards_cases, NATIVE_EVALUATOR);
cards_case!(cactuskev_cards_cases, CACTUSKEV_EVALUATOR);
cards_case!(twoplustwo_cards_cases, TWOPLUSTWO_EVALUATOR);
