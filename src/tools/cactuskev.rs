use crate::Evaluator;
use crate::evaluator::NativeEvaluator;
use crate::cards_type::CardsType;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use super::combination::CardsByValueCombination;


const CARD_PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41 ];

const HASH_ADJUST: [u16;512] = [
    0, 5628, 7017, 1298, 2918, 2442, 8070, 6383, 6383, 7425, 2442, 5628, 8044, 7425, 3155, 6383, 
    2918, 7452, 1533, 6849, 5586, 7452, 7452, 1533, 2209, 6029, 2794, 3509, 7992, 7733, 7452, 131, 
    6029, 4491, 1814, 7452, 6110, 3155, 7077, 6675, 532, 1334, 7555, 5325, 3056, 1403, 1403, 3969, 
    4491, 1403, 7592, 522, 8070, 1403, 0, 1905, 3584, 2918, 922, 3304, 6675, 0, 7622, 7017, 
    3210, 2139, 1403, 5225, 0, 3969, 7992, 5743, 5499, 5499, 5345, 7452, 522, 305, 3056, 7017, 
    7017, 2139, 1338, 3056, 7452, 1403, 6799, 3204, 3290, 4099, 1814, 2191, 4099, 5743, 1570, 1334, 
    7363, 1905, 0, 6799, 4400, 1480, 6029, 1905, 0, 7525, 2028, 2794, 131, 7646, 3155, 4986, 
    1858, 2442, 7992, 1607, 3584, 4986, 706, 6029, 5345, 7622, 6322, 5196, 1905, 6847, 218, 1785, 
    0, 4099, 2981, 6849, 4751, 3950, 7733, 3056, 5499, 4055, 6849, 1533, 131, 5196, 2918, 3879, 
    5325, 2794, 6029, 0, 0, 322, 7452, 6178, 2918, 2320, 6675, 3056, 6675, 1533, 6029, 1428, 
    2280, 2171, 6788, 7452, 3325, 107, 4262, 311, 5562, 7857, 6110, 2139, 4942, 4600, 1905, 0, 
    3083, 5345, 7452, 6675, 0, 6112, 4099, 7017, 1338, 6799, 2918, 1232, 3584, 522, 6029, 5325, 
    1403, 6759, 6849, 508, 6675, 2987, 7745, 6870, 896, 7452, 1232, 4400, 12, 2981, 3850, 4491, 
    6849, 0, 6675, 747, 4491, 7525, 6675, 7452, 7992, 6921, 7323, 6849, 3056, 1199, 2139, 6029, 
    6029, 190, 4351, 7891, 4400, 7134, 1533, 1194, 3950, 6675, 5345, 6383, 7622, 131, 1905, 2883, 
    6383, 1533, 5345, 2794, 4303, 1403, 0, 1338, 2794, 992, 4871, 6383, 4099, 2794, 3889, 6184, 
    3304, 1905, 6383, 3950, 3056, 522, 1810, 3975, 7622, 7452, 522, 6799, 5866, 7084, 7622, 6528, 
    2798, 7452, 1810, 7907, 642, 5345, 1905, 6849, 6675, 7745, 2918, 4751, 3229, 2139, 6029, 5207, 
    6601, 2139, 7452, 5890, 1428, 5628, 7622, 2139, 3146, 2400, 578, 941, 7672, 1814, 3210, 1533, 
    4491, 12, 2918, 1900, 7425, 2794, 2987, 3465, 1377, 3822, 3969, 3210, 859, 5499, 6878, 1377, 
    3056, 4027, 8065, 8065, 5207, 4400, 4303, 3210, 3210, 0, 6675, 357, 5628, 5512, 1905, 3452, 
    1403, 7646, 859, 6788, 3210, 2139, 378, 5663, 7733, 870, 0, 4491, 4813, 2110, 578, 2139, 
    3056, 4099, 1905, 1298, 4672, 2191, 3950, 5499, 3969, 4974, 6323, 6029, 7414, 6383, 0, 4974, 
    3210, 795, 4099, 131, 5345, 5345, 6576, 1810, 1621, 4400, 2918, 1905, 2442, 2679, 6322, 7452, 
    2110, 1403, 6383, 2653, 5132, 6856, 7841, 2794, 6110, 2028, 6675, 7425, 6999, 7441, 6029, 183, 
    6675, 4400, 859, 1403, 2794, 5985, 5345, 1533, 322, 4400, 1227, 5890, 4474, 4491, 3574, 8166, 
    6849, 7086, 5345, 5345, 5459, 3584, 6675, 3969, 7579, 8044, 2295, 2577, 1480, 5743, 3304, 5499, 
    330, 4303, 6863, 3822, 4600, 4751, 5628, 3822, 2918, 6675, 2400, 6663, 1403, 6849, 6029, 3145, 
    6110, 3210, 747, 3229, 3056, 2918, 7733, 330, 4055, 7322, 5628, 2987, 3056, 1905, 2903, 669, 
    5325, 2845, 4099, 5225, 6283, 4099, 5000, 642, 4055, 5345, 8034, 2918, 1041, 5769, 7051, 1538, 
    2918, 3366, 608, 4303, 3921, 0, 2918, 1905, 218, 6687, 5963, 859, 3083, 2987, 896, 5056, 
    1905, 2918, 4415, 7966, 7646, 2883, 5628, 7017, 8029, 6528, 4474, 6322, 5562, 6669, 4610, 7006,    
];

// perfect hash
fn find_fast(u: u32) -> u32 {
    let mut u = u as u64 + 0xe91aaa35;
    u ^= u >> 16;
    u += u << 8;
    u ^= ((u as u32) >> 4) as u64;
    let b: u64 = (u >> 8) & 0x1FF;
    let a: u64 = (u + (u << 2) & 0xFFFFFFFF) >> 19;
    let r: u32 = (a as u32) ^ (HASH_ADJUST[b as usize] as u32);
    r
}

// 生成 cactuskev 表格
pub fn gen_cactuskev_tables(dir_path: &Path) -> std::io::Result<()>{
    let evaluator = NativeEvaluator::new();

    // 收集表格
    let mut all_cards_values: Vec<u32> = Vec::with_capacity(7462);
    for cards in CardsByValueCombination::with_collect_5() {
        // flush
        let cards_value = evaluator.simple_eval(&cards);
        all_cards_values.push(cards_value);
    }

    all_cards_values.sort();
    save_tables(dir_path, &all_cards_values, evaluator)
}

fn save_tables(dir_path: &Path, all_cards_values: &[u32], evaluator: NativeEvaluator) -> std::io::Result<()>{
    // flushes
    let mut flushes: [u16;0x1F00+1] = [0;0x1F00+1];
    let mut unique5: [u16;0x1F00+1] = [0;0x1F00+1];
    let mut others: [(u32,u16);4888] = [(0,0);4888];
    let mut others_count = 0;

    for (output_value, &cards_type_value) in all_cards_values.iter().rev().enumerate() {
        let (tp_type, values) = evaluator.unpack_eval_value(cards_type_value).unwrap();
        let output_value = output_value + 1;

        match tp_type {
            CardsType::StraightFlush | CardsType::Flush => {
                let flush_idx = values.iter().fold(0, |acc, &b| acc | (1 << (b.index() as u32-1)));
                flushes[flush_idx] = output_value as u16;
            }
            CardsType::Straight | CardsType::High => {
                let idx = values.iter().fold(0, |acc, &b| acc | (1 << (b.index() as i32)));
                unique5[idx] = output_value as u16;
            }
            _ => {
                others[others_count] = (values.iter().fold(1, |acc, &b| acc * CARD_PRIMES[b.index() as usize]),
                                        output_value as u16);
                others_count += 1;
            }
        }
    }

    assert_eq!(others_count, 4888);
    others.sort_by(|(a,_), (b,_)| a.partial_cmp(b).unwrap());    

    let mut file = File::create(dir_path.join("tbl_flushes"))?;
    file.write_all(b"[")?;
    for value in flushes.iter() {
        file.write(format!("{},", value).as_bytes())?;
    }
    file.write_all(b"]")?;

    let mut file = File::create(dir_path.join("tbl_unique5"))?;
    file.write_all(b"[")?;
    for value in unique5.iter() {
        file.write(format!("{},", value).as_bytes())?;
    }
    file.write_all(b"]")?;
    
    let mut file = File::create(dir_path.join("tbl_other_products"))?;
    file.write_all(b"[")?;
    for (product, _) in others.iter() {
        file.write(format!("{},", product).as_bytes())?;
    }
    file.write_all(b"]")?;

    let mut file = File::create(dir_path.join("tbl_other_values"))?;
    file.write_all(b"[")?;
    for (_, value) in others.iter() {
        file.write(format!("{},", value).as_bytes())?;
    }
    file.write_all(b"]")?;

    let mut file = File::create(dir_path.join("tbl_value_contents"))?;
    file.write_all(b"[")?;
    for &value in all_cards_values.iter() {
        let (_, values) = evaluator.unpack_eval_value(value).unwrap();
        let value = (value & 0xF00000) | values.iter().fold(0, |acc, &b| (acc <<4) | (b.index() as u32+1));
        file.write(format!("{},", value).as_bytes())?;
    }
    file.write_all(b"]")?;

    let mut file = File::create(dir_path.join("tbl_hash_adjust"))?;
    file.write_all(b"[")?;
    for value in HASH_ADJUST.iter() {
        file.write(format!("{},", value).as_bytes())?;
    }
    file.write_all(b"]")?;
    
    let mut file = File::create(dir_path.join("tbl_hash_values"))?;
    file.write_all(b"[")?;
    let mut hash_values: Vec<u16> = Vec::with_capacity(others.len());
    for &(product, value) in others.iter() {
        let hash_key = find_fast(product) as usize;
        if hash_key >= hash_values.len() {
            //println!("resize {}", hash_key+1);
            hash_values.resize(hash_key+1, 166);
        }
        hash_values[hash_key] = value;
    }
    for value in hash_values.iter() {
        file.write(format!("{},", value).as_bytes())?;
    }
    file.write_all(b"]")?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_fast(){
        assert_eq!(1608, find_fast(79052387))
    }
}


