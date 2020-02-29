pub mod card;
pub mod cards_type;
pub mod tools;

pub use card::Card;
pub use cards_type::CardsType;


/// An Evaluator trait 
///
/// Example:
/// ```
/// use crate::pokerlib::Evaluator;
/// 
/// let cards = &pokerlib::Card::one_desk_cards()[0..7];
/// let evaluator = pokerlib::NativeEvaluator::new();
/// let value = evaluator.simple_eval(cards);
/// ```
/// 
pub trait Evaluator {
    /// evaluator's card type
    ///
    /// Every evaluator's card type maybe different: u32, u8, etc...
    type CardType;

    /// make evaluator's card from Card
    fn make_card(&self, card: &Card) -> Self::CardType;

    /// eval cards's cmp value
    ///
    /// You can use return value to compare.
    /// if a>b, then a is winner.
    fn eval(&self, cards: &[Self::CardType]) -> u32;

    /// get the cardstype from eval_value.
    fn eval_value_type(&self, eval_value: u32) -> Option<CardsType>;

    /// eval's human interface: use `&[Card]` as argument
    fn simple_eval(&self, cards: &[Card]) -> u32 {
        let inner_cards: Vec<Self::CardType> = cards.iter().map(|x| self.make_card(x)).collect();
        self.eval(&inner_cards)
    }
}

pub mod evaluator;
pub use evaluator::NativeEvaluator;
pub use evaluator::CactusKevEvaluator;
pub use evaluator::TwoPlusTwoEvaluator;
