#![feature(test)]

#[macro_use]
extern crate lazy_static;

extern crate test;
use test::Bencher;
use pokerlib::cards_type::CardsType;
use pokerlib::card::Card;
use std::path::Path;
use pokerlib::Evaluator;
use pokerlib::{NativeEvaluator, CactusKevEvaluator, TwoPlusTwoEvaluator};
use pokerlib::tools::combination::CombinationIter;
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

pub fn all_combination_cards_ex<T: Copy,E: Evaluator<CardType=T>>(
    evaluator: &E,
    cards_count: usize,
    cards_type: Option<CardsType>,
    limit: Option<usize>,
) -> Vec<Vec<T>>

where E: Evaluator {
    let cards: Vec<T> = Card::one_desk_cards().iter().map(|x| evaluator.make_card(x)).collect();
    let it = CombinationIter::new(52, cards_count).map(|card_indexes: Vec<usize>|{
        card_indexes.iter().map(|&i| cards[i as usize]).collect::<Vec<T>>()
    }).filter(|cards|{
        match cards_type {
            Some(tp) => {
                let eval_value = evaluator.eval(&cards);
                tp == evaluator.eval_value_type(eval_value).unwrap()
            }
            _ => true
        }
    });

    if let Some(limit) = limit {
        it.take(limit).collect()
    }else{
        it.collect()
    }
}

pub fn all_combination_cards<T: Copy,E: Evaluator<CardType=T>>(
    evaluator: &E,
    cards_count: usize,
) -> Vec<Vec<T>>

where E: Evaluator {
    let cards: Vec<T> = Card::one_desk_cards().iter().map(|x| evaluator.make_card(x)).collect();
    CombinationIter::new(52, cards_count).map(|card_indexes: Vec<usize>|{
        card_indexes.iter().map(|&i| cards[i as usize]).collect::<Vec<T>>()
    }).collect()
}

pub fn get_test_cards_for_type<T: Copy,E: Evaluator<CardType=T>>(
    evaluator: &E,
    cards_type: CardsType) -> Vec<T> {
    match cards_type {
        CardsType::High => vec!["As","Kh","8d","2s","7h","4d","9s"],
        CardsType::Pair => vec!["As","Kh","8d","8s","7h","4d","9s"],
        CardsType::Pair2 => vec!["As","Ah","8d","8s","7h","4d","9s"],
        CardsType::Three => vec!["As","Ah","Ad","8s","7h","4d","9s"],
        CardsType::Full => vec!["As","Ah","Ad","8s","8h","4d","9s"],
        CardsType::Flush => vec!["As","8s","7s","2s","5s","4d","9s"],
        CardsType::Four => vec!["As","Ah","Ad","Ac","8h","4d","9s"],
        CardsType::Straight => vec!["As","Kh","Jd","Qc","Th","4d","9s"],
//        CardsType::StraightFlush => vec!["2s","Ks","Js","Qs","Ts","4d","9s"],
        CardsType::StraightFlush => vec!["As","Ks","Js","Qs","Ts","4d","9s"],
    }.iter().map(|&x| evaluator.make_card(&Card::from_str(x).ok().unwrap())).collect()
}

macro_rules! fix_cards_type {
    ($name:ident, $evaluator:ident, $cardstype:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let i_cards = get_test_cards_for_type(&*$evaluator, $cardstype);
            b.iter(|| $evaluator.eval(&i_cards))
        }
    };
}

fix_cards_type!(native_cards_type_high, NATIVE_EVALUATOR, CardsType::High);
fix_cards_type!(native_cards_type_pair, NATIVE_EVALUATOR, CardsType::Pair);
fix_cards_type!(native_cards_type_pair2, NATIVE_EVALUATOR, CardsType::Pair2);
fix_cards_type!(native_cards_type_three, NATIVE_EVALUATOR, CardsType::Three);
fix_cards_type!(native_cards_type_flush, NATIVE_EVALUATOR, CardsType::Flush);
fix_cards_type!(native_cards_type_full, NATIVE_EVALUATOR, CardsType::Full);
fix_cards_type!(native_cards_type_four, NATIVE_EVALUATOR, CardsType::Four);
fix_cards_type!(native_cards_type_straight, NATIVE_EVALUATOR, CardsType::Straight);
fix_cards_type!(native_cards_type_straight_flush, NATIVE_EVALUATOR, CardsType::StraightFlush);

fix_cards_type!(cactuskev_cards_type_high, CACTUSKEV_EVALUATOR, CardsType::High);
fix_cards_type!(cactuskev_cards_type_pair, CACTUSKEV_EVALUATOR, CardsType::Pair);
fix_cards_type!(cactuskev_cards_type_pair2, CACTUSKEV_EVALUATOR, CardsType::Pair2);
fix_cards_type!(cactuskev_cards_type_three, CACTUSKEV_EVALUATOR, CardsType::Three);
fix_cards_type!(cactuskev_cards_type_flush, CACTUSKEV_EVALUATOR, CardsType::Flush);
fix_cards_type!(cactuskev_cards_type_full, CACTUSKEV_EVALUATOR, CardsType::Full);
fix_cards_type!(cactuskev_cards_type_four, CACTUSKEV_EVALUATOR, CardsType::Four);
fix_cards_type!(cactuskev_cards_type_straight, CACTUSKEV_EVALUATOR, CardsType::Straight);
fix_cards_type!(cactuskev_cards_type_straight_flush, CACTUSKEV_EVALUATOR, CardsType::StraightFlush);


fix_cards_type!(twoplustwo_cards_type_high, TWOPLUSTWO_EVALUATOR, CardsType::High);
fix_cards_type!(twoplustwo_cards_type_pair, TWOPLUSTWO_EVALUATOR, CardsType::Pair);
fix_cards_type!(twoplustwo_cards_type_pair2, TWOPLUSTWO_EVALUATOR, CardsType::Pair2);
fix_cards_type!(twoplustwo_cards_type_three, TWOPLUSTWO_EVALUATOR, CardsType::Three);
fix_cards_type!(twoplustwo_cards_type_flush, TWOPLUSTWO_EVALUATOR, CardsType::Flush);
fix_cards_type!(twoplustwo_cards_type_full, TWOPLUSTWO_EVALUATOR, CardsType::Full);
fix_cards_type!(twoplustwo_cards_type_four, TWOPLUSTWO_EVALUATOR, CardsType::Four);
fix_cards_type!(twoplustwo_cards_type_straight, TWOPLUSTWO_EVALUATOR, CardsType::Straight);
fix_cards_type!(twoplustwo_cards_type_straight_flush, TWOPLUSTWO_EVALUATOR, CardsType::StraightFlush);


macro_rules! select_cards_all {
    ($name:ident, $evaluator:ident, $count:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let all_test_cards = all_combination_cards(&*$evaluator, $count);
            b.iter(||{
                all_test_cards.iter().for_each(|tmp_cards|{
                    $evaluator.eval(&tmp_cards);
                });
            })
        }
    };
}

macro_rules! select_cards_some {
    ($name:ident, $evaluator:ident, $count:expr, $somecount:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let all_test_cards = all_combination_cards_ex(&*$evaluator, $count, None, Some($somecount));
            b.iter(||{
                all_test_cards.iter().for_each(|tmp_cards|{
                    $evaluator.eval(&tmp_cards);
                });
            })
        }
    };
}

select_cards_all!(native_select_5_all, NATIVE_EVALUATOR, 5);
select_cards_all!(cactuskev_select_5_all, CACTUSKEV_EVALUATOR, 5);
select_cards_all!(twoplustwo_select_5_all, TWOPLUSTWO_EVALUATOR, 5);

select_cards_all!(native_select_6_all, NATIVE_EVALUATOR, 6);
select_cards_all!(cactuskev_select_6_all, CACTUSKEV_EVALUATOR, 6);
select_cards_all!(twoplustwo_select_6_all, TWOPLUSTWO_EVALUATOR, 6);

select_cards_some!(native_select_7_some, NATIVE_EVALUATOR, 7, 1_000_000);
select_cards_some!(cactuskev_select_7_some, CACTUSKEV_EVALUATOR, 7, 1_000_000);
select_cards_some!(twoplustwo_select_7_some, TWOPLUSTWO_EVALUATOR, 7, 1_000_000);
