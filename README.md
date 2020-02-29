# pokerlib-rs

A poker eval lib. 

# Usage

Add this to your `Cargo.toml`:

```
[dependencies]
rand = { git = "https://github.com/shengdoushi/pokerlib-rs" }
```

# Example

```rust
use crate::pokerlib::Evaluator;

let cards = &pokerlib::Card::one_desk_cards()[0..7];

let evaluator = pokerlib::TwoPlusTwoEvaluator::with_data_file("HandRank.dat");
let value = evaluator.simple_eval(cards);

```
