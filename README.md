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
use pokerlib::Evaluator;

let cards = &pokerlib::Card::one_desk_cards()[0..7];

let evaluator = pokerlib::TwoPlusTwoEvaluator::with_data_file("HandRank.dat");
let value = evaluator.simple_eval(cards);

```

# Evaluator

There are 3 evaluators:

- TwoPlusTwoEvaluator, fastest when eval 6, 7 cards
- NativeEvaluator, without use any table, faster than CactusKevEvaluator when eval 7 cards.
- CactusKevEvaluator, fastest only when eval 5 cards

| evaluator           | construct                                     | table size | 5 cards | 6 cards | 7 cards |
|---------------------|-----------------------------------------------|------------|---------|---------|---------|
| TwoPlusTwoEvaluator | TwoPlusTwoEvaluator::with_data_file(filepath) | 130M       | faster  | fastest | fastest |
| NativeEvaluator     | NativeEvaluator::new()                        | 0          | slow    | slow    | faster  |
| CactusKevEvaluator  | CactusKevEvaluator::new()                     | < 1M       | fastest | faster  | slow    |

You can run `cargo bench` to watch. In my machine, the bench is:

```
test cactuskev_select_5_all               ... bench:  19,277,977 ns/iter (+/- 1,948,889)
test cactuskev_select_6_all               ... bench: 882,087,921 ns/iter (+/- 22,191,924)
test cactuskev_select_7_some              ... bench: 187,920,001 ns/iter (+/- 7,446,700)

test native_select_5_all                  ... bench: 102,297,941 ns/iter (+/- 6,075,751)
test native_select_6_all                  ... bench: 962,169,099 ns/iter (+/- 19,223,725)
test native_select_7_some                 ... bench:  50,624,862 ns/iter (+/- 2,889,970)

test twoplustwo_select_5_all              ... bench:  23,767,016 ns/iter (+/- 2,799,717)
test twoplustwo_select_6_all              ... bench: 165,268,480 ns/iter (+/- 13,554,236)
test twoplustwo_select_7_some             ... bench:   6,330,081 ns/iter (+/- 1,095,291)
```

TwoPlusTwoEvaluator's data file (130M size) can generate by `tools::twoplustwo::generate_data_file(path: std::path::Path)`.

```rust
use pokerlib::tools::twoplustwo::generate_data_file;

generate_data_file(Path::new("TptHandRank.dat")).ok().unwrap();
```
