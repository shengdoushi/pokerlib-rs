use std::fmt;

/// Cards' value enum, 23456789TJQKA
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nign,
    Ten,
    Jack,
    Queen,
    Kine,
    Ace,
}

const CARD_VALUES: [Value;13] = [
    Value::Two, Value::Three, Value::Four, Value::Five, Value::Six, Value::Seven,
    Value::Eight, Value::Nign, Value::Ten, Value::Jack, Value::Queen, Value::Kine, Value::Ace];

impl Value {
    pub fn to_string(&self) -> &'static str {
        match *self {
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nign => "9",
            Value::Ten => "T",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::Kine => "K",
            Value::Ace => "A",
        }
    }

    pub fn from_char(c: char) -> Result<Value, CardStringConvertError> {
        match c {
            '2' => Ok(Value::Two),
            '3' => Ok(Value::Three),
            '4' => Ok(Value::Four),
            '5' => Ok(Value::Five),
            '6' => Ok(Value::Six),
            '7' => Ok(Value::Seven),
            '8' => Ok(Value::Eight),
            '9' => Ok(Value::Nign),
            'T' => Ok(Value::Ten),
            'J' => Ok(Value::Jack),
            'Q' => Ok(Value::Queen),
            'K' => Ok(Value::Kine),
            'A' => Ok(Value::Ace),
            _ => Err(CardStringConvertError)
        }
    }

    pub fn from_index(index: u8) -> Value {
        CARD_VALUES[index as usize]
    }

    pub fn index(&self) -> u8 {
        (0..13).find(|&x| CARD_VALUES[x] == *self).unwrap() as u8
    }
}

/// Card's suit enum
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}
const CARD_SUITS: [Suit;4] = [Suit::Spade, Suit::Heart, Suit::Club, Suit::Diamond];

impl Suit {
    pub fn to_string(&self) -> &'static str {
        match *self {
            Suit::Spade => "s",
            Suit::Heart => "h",
            Suit::Club => "c",
            Suit::Diamond => "d",
        }
    }
    pub fn from_char(c: char) -> Result<Suit, CardStringConvertError>{
        match c {
            's' => Ok(Suit::Spade),
            'h' => Ok(Suit::Heart),
            'c' => Ok(Suit::Club),
            'd' => Ok(Suit::Diamond),
            _ => Err(CardStringConvertError)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct Card{
    card_index: u8, // policy: GroupByValue
}

/// Card's index policy definition.
pub enum CardIndexPolicy {
    /// Group by value.
    /// 
    /// 0,1,2,3 are Cards which values are Two in different suits.
    ///
    /// |index|value|
    /// |----|-----|
    /// |0,1,2,3|Two|
    /// |4,5,6,7|Three|
    /// |8,9,10,11|Four|
    /// |12,13,14,15|Five|
    /// |16,17,18,19|Six|
    /// |20,21,22,23|Seven|
    /// |24,25,26,27| Eight |
    /// |28,29,30,31| Nign |
    /// |32,33,34,35| Ten |
    /// |36,37,38,39| Jack |
    /// |40,41,42,43| Queen|
    /// |44,45,46,47| Kine|
    /// |48,49,50,51| Ace|
    GroupByValue,
    /// Group by suit.
    ///
    /// 0,1,2,3,...12 are Cards which suit are same in different values.
    /// ...
    /// |index|value|
    /// |----|-----|
    /// | 0,1,2,3,4,5,6,7,8,9,10,11,12| Spade Two-Ace |
    /// | 13,14,15,16,17,18,19,20,21,22,23,24,25 | Heart Two-Ace |
    /// | 26,27,28,29,30,31,32,33,34,35,36,37,38 | Club Two-Ace |
    /// | 39,40,41,42,43,44,45,46,47,48,49,50,51, | Diamond Two-Ace |
    GroupBySuit,
}

pub struct CardStringConvertError;

impl Card {
    pub fn new(value: Value, suit: Suit)-> Card{
        let v_idx = (0..13).find(|&x| CARD_VALUES[x] == value).unwrap();
        let s_idx = (0..4).find(|&x| CARD_SUITS[x] == suit).unwrap();
        Card{
            card_index: (v_idx * 4 + s_idx) as u8,
        }
    }

    pub fn with_index(value_index: u8, suit_index: u8) -> Card {
        if value_index >= 13 || suit_index >= 4 {
            panic!("Card::with_index argument error {},{}", value_index, suit_index);
        }
        Card{card_index: value_index * 4 + suit_index}
    }

    pub fn with_card_index(card_index: u8, policy: CardIndexPolicy) -> Card{
        if card_index >= 52 {
            panic!("Card::with_card_index argument error: {}", card_index);
        }
        Card{card_index: match policy {
            CardIndexPolicy::GroupByValue => card_index,
            CardIndexPolicy::GroupBySuit => (card_index%13) * 4 + (card_index/13),
        }}
    }

    #[inline]
    pub fn value(&self) -> Value {
        CARD_VALUES[self.card_index as usize/4]
    }

    #[inline]
    pub fn value_index(&self) -> u8 {
        self.card_index / 4
    }

    #[inline]
    pub fn suit(&self) -> Suit {
        CARD_SUITS[self.card_index as usize %4]
    }

    #[inline]
    pub fn suit_index(&self) -> u8 {
        self.card_index % 4
    }

    #[inline]
    pub fn card_index(&self, policy: CardIndexPolicy) -> u8 {
        match policy {
            CardIndexPolicy::GroupByValue => self.card_index,
            CardIndexPolicy::GroupBySuit => self.suit_index() * 13 + self.value_index()
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::from("");
        s.push_str(self.value().to_string());
        s.push_str(self.suit().to_string());
        s
    }

    pub fn from_str(card_str: &str) -> Result<Card, CardStringConvertError>{
        if card_str.len() < 2 {
            return Err(CardStringConvertError);
        }
        let mut chars = card_str.chars();
        let value = Value::from_char(chars.next().unwrap())?;
        let suit = Suit::from_char(chars.next().unwrap())?;
        Ok(Card::new(value, suit))
    }

    pub fn one_desk_cards() -> [Card;52] {
        [Card{card_index:0},Card{card_index:1},Card{card_index:2},Card{card_index:3},Card{card_index:4},Card{card_index:5},Card{card_index:6},Card{card_index:7},Card{card_index:8},Card{card_index:9},Card{card_index:10},Card{card_index:11},Card{card_index:12},Card{card_index:13},Card{card_index:14},Card{card_index:15},Card{card_index:16},Card{card_index:17},Card{card_index:18},Card{card_index:19},Card{card_index:20},Card{card_index:21},Card{card_index:22},Card{card_index:23},Card{card_index:24},Card{card_index:25},Card{card_index:26},Card{card_index:27},Card{card_index:28},Card{card_index:29},Card{card_index:30},Card{card_index:31},Card{card_index:32},Card{card_index:33},Card{card_index:34},Card{card_index:35},Card{card_index:36},Card{card_index:37},Card{card_index:38},Card{card_index:39},Card{card_index:40},Card{card_index:41},Card{card_index:42},Card{card_index:43},Card{card_index:44},Card{card_index:45},Card{card_index:46},Card{card_index:47},Card{card_index:48},Card{card_index:49},Card{card_index:50},Card{card_index:51}]
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
