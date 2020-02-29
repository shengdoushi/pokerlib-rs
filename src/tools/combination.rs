use crate::card::Card;

/*
提供组合相关的类

- 通用的组合迭代器
- 适用于卡牌的迭代器
-- 选牌值的迭代器

 */
pub struct Combination {
    total: usize,                 // 总数
    elem: usize,                  // 收集元素数
}

pub struct CombinationIter{
    total: usize,                 // 总数
    elem: usize,                  // 收集元素数
    indexes: Vec<usize>,
}

impl Combination {
    pub fn new(total: usize, elem: usize) -> Combination {
        if total < elem {
            panic!("Combination total {} should >= elem {}", total, elem)
        }

        Combination{
            total: total,
            elem: elem,
        }
    }

    pub fn iter(&self) -> CombinationIter {
        CombinationIter::new(self.total, self.elem)
    }
}

impl CombinationIter {
    pub fn new(total: usize, elem: usize) -> CombinationIter {
        if total < elem {
            panic!("CombinationIter total {} should >= elem {}", total, elem)
        }

        CombinationIter{
            total: total,
            elem: elem,
            indexes: Vec::with_capacity(elem),
        }
    }

    pub fn move_next(&mut self, result: &mut [usize]) -> bool {
        // first
        if self.indexes.is_empty() {
            self.indexes.extend(0..self.elem);
            for i in 0..self.elem {
                result[i] = i;
            }
            return true;
        }

        // over
        if self.indexes[0] == self.total-self.elem {
            return false;
        }

        // find inc index
        let mut index = self.elem;
        for i in 0..self.elem {
            //self.indexes[self.elem-1-i]+=1;
            if self.indexes[self.elem-1-i] < (self.total-1-i) {
                index = self.elem-1-i;
                break;
            }
        }
        
        self.indexes[index] += 1;
        // index +1,2,3...
        for i in index+1..self.elem {
            self.indexes[i] = self.indexes[index] + (i-index);
        }

        for i in 0..self.elem {
            result[i] = self.indexes[i];
        }
        true
    }
}


impl Iterator for CombinationIter {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item>{
        let mut result: Vec<usize> = Vec::new();
        result.resize(self.elem, 0);
        if self.move_next(&mut result){
            Some(result)
        }else{
            None
        }
    }
}

pub fn for_each_card_indexes(card_count: usize, f: impl Fn(&[usize])){
    let mut card_indexes:[usize;7] = [0;7];
    for i in 0..card_count {
        card_indexes[i] = if i==card_count-1 {i-1} else {i}
    }

    loop {
        let mut target_index = card_count;
        for i in 0..card_count {
            card_indexes[card_count-1-i]+=1;
            if card_indexes[card_count-1-i] < (51-i) {
                target_index = card_count-1-i;
                break;
            }
        }
        if target_index == card_count {
            break
        }
        for i in target_index..card_count{
            card_indexes[i] = card_indexes[target_index] + (i-target_index);
        }

        f(&card_indexes[0..card_count]);
    }
}

enum CardsCollectCount {
    Five,
    Six,
    Seven,
}

pub struct CardsByValueCombination {
    value_indexes: Vec<u8>,          // 0-12
    elems: u8,
    cur_is_flush: Option<bool>,          // 同花状态： 先查同花 true，再变非同花 false
}

impl CardsByValueCombination {
    fn new(collect_count: CardsCollectCount) -> CardsByValueCombination{
        CardsByValueCombination{
            value_indexes: vec![],
            elems: match collect_count{
                CardsCollectCount::Five => 5,
                CardsCollectCount::Six => 6,
                CardsCollectCount::Seven => 7,
            },
            cur_is_flush: None,
        }
    }
    pub fn with_collect_5() -> CardsByValueCombination {
        CardsByValueCombination::new(CardsCollectCount::Five)
    }
    pub fn with_collect_6() -> CardsByValueCombination {
        CardsByValueCombination::new(CardsCollectCount::Six)
    }
    pub fn with_collect_7() -> CardsByValueCombination {
        CardsByValueCombination::new(CardsCollectCount::Seven)
    }
    fn cur_values(&self) -> Vec<Card>{
        self.value_indexes.iter().enumerate()
            .map(|(i, &x)| Card::with_index(x,
                                            if let Some(is_flush) = self.cur_is_flush {
                                                if is_flush {0} else {i as u8%4}
                                            }
                                            else if i >= 3 && self.value_indexes[i-3]==x {3}
                                            else if i >= 2 && self.value_indexes[i-2]==x {2}
                                            else if i >= 1 && self.value_indexes[i-1]==x {1}
                                            else {0}))
            .collect()
    }
}

impl Iterator for CardsByValueCombination {
    type Item = Vec<Card>;

    fn next(&mut self) -> Option<Self::Item>{
        // first
        if self.value_indexes.is_empty() {
            self.value_indexes.extend((0..self.elems).map(|x| x/4));
            return Some(self.cur_values());
        }
        // end
        if self.elems == 0 ||
            (self.value_indexes[0]==(13-(self.elems+3)/4) &&
             (self.elems == 1 || self.value_indexes[self.elems as usize%4] == (self.value_indexes[0]+1))){
                return None;
            }

        // 是否需要同花
        if let Some(true) = self.cur_is_flush {
            self.cur_is_flush = Some(false);
            return Some(self.cur_values());
        }
        self.cur_is_flush = None;

        // inc
        let index = self.value_indexes.iter().enumerate()
            .rposition(|(i, &x)| x< (13-(self.elems+3-i as u8)/4)).unwrap();

        self.value_indexes[index] += 1;
        for i in index+1..self.elems as usize {
            self.value_indexes[i] = self.value_indexes[index] + (i-index) as u8/4;
        }

        // 可以有同花
        if self.elems >= 5 && (1..self.elems as usize).position(|i| self.value_indexes[i]==self.value_indexes[i-1]).is_none() {
            self.cur_is_flush = Some(true);
        }
        return Some(self.cur_values());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination(){
        let v: Vec<Vec<usize>> = CombinationIter::new(3,2).collect();
        assert_eq!(v, vec![
            vec![0,1],
            vec![0,2],
            vec![1,2]
        ]);

        assert_eq!(2598960, CombinationIter::new(52, 5).count());
    }

    #[test]
    fn test_card_combination(){
        assert_eq!(7462, CardsByValueCombination::with_collect_5().count());
    }
}

