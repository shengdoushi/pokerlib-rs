use crate::Evaluator;
use crate::evaluator::CactusKevEvaluator;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

struct DataFileGenerator {
    //    ids: [i64;612978],
    ids: Vec<i64>, 
    //    hr: [i32;32487834],
    hr: Vec<i32>,
    num_ids: i32,
    max_id: i64,
    numcards: usize,
    max_hr: i32,
}

impl DataFileGenerator {
    fn new() -> DataFileGenerator{
        let mut hr: Vec<i32> = Vec::with_capacity(32487834);
        hr.resize(32487834, 0);

        let mut ids: Vec<i64> = Vec::new();
        ids.resize(612978, 0);
        DataFileGenerator {
//            ids: [0;612978],
            ids: ids,
//            hr: [0;32487834],
            hr: hr,
            num_ids: 1,
            max_id: 0,
            numcards: 0,
            max_hr: 0,
        }
    }
    
    /*
    int maxHR = 0;
     */

    fn make_id(&mut self, id_min: i64, newcard: i32) -> i64{
        // returns a 64-bit hand ID, for up to 8 cards, stored 1 per byte.

        let mut suitcount: [i32;4 + 1] = [0;5];
        let mut rankcount: [i32;13 + 1] = [0;14];
        let mut wk: [i32;8] = [0;8];  // intentially keeping one as a 0 end
        let mut newcard = newcard;

        // can't have more than 6 cards!
        for cardnum in 0..6 {
            // leave the 0 hole for new card
            wk[cardnum as usize + 1] =  ((id_min >> (8 * cardnum)) & 0xff) as i32;
        }

        // my cards are 2c = 1, 2d = 2  ... As = 52
        newcard-=1;  // make 0 based!

        // add next card. formats card to rrrr00ss
        wk[0] = (((newcard >> 2) + 1) << 4) + (newcard & 3) + 1;

        let mut getout = false;
        self.numcards = 0;
        while wk[self.numcards] != 0 {
            // need to see if suit is significant
            suitcount[wk[self.numcards] as usize & 0xf]+= 1;
            // and rank to be sure we don't have 4!
            rankcount[(wk[self.numcards] >> 4) as usize & 0xf]+=1;
            if self.numcards != 0 {
                // can't have the same card twice
                // if so need to get out after counting self.numcards
                if wk[0] == wk[self.numcards]{
                    getout = true;
                }
            }
            self.numcards += 1;
        }

        if getout {return 0;} // duplicated another card (ignore this one)
        
        // (MakeID)

        // for suit to be significant, need to have n-2 of same suit
        let needsuited = self.numcards as i32 - 2;
        if self.numcards > 4 {
            for rank in 1..14{
                // if I have more than 4 of a rank then I shouldn't do this one!!
                // can't have more than 4 of a rank so return an ID that can't be!
                if rankcount[rank] > 4 {return 0;}
            }
        }

        // However in the ID process I prefered that
        // 2s = 0x21, 3s = 0x31,.... Kc = 0xD4, Ac = 0xE4
        // This allows me to sort in Rank then Suit order

        // if we don't have at least 2 cards of the same suit for 4,
        // we make this card suit 0.
        if needsuited > 1 {
            for cardnum in 0..self.numcards {// for each card
                if suitcount[wk[cardnum] as usize & 0xf] < needsuited as i32 {
	            // check suitcount to the number I need to have suits significant
	            // if not enough - 0 out the suit - now this suit would be a 0 vs 1-4
	            wk[cardnum] &= 0xf0;
                }
            }
        }
        
        // (MakeID)

        // Sort Using XOR.  Netwk for N=7, using Bose-Nelson Algorithm:
        // Thanks to the thread!
        for &(i, j) in [
            (0, 4), (1, 5), (2, 6), (0, 2), (1, 3),
            (4, 6), (2, 4), (3, 5), (0, 1), (2, 3),
            (4, 5), (1, 4), (3, 6), (1, 2), (3, 4),
            (5, 6),
        ].iter() {
            if wk[i] < wk[j] {wk[i]^=wk[j]; wk[j]^=wk[i]; wk[i]^=wk[j];}
        }

        // long winded way to put the pieces into a int64
        // cards in bytes --66554433221100
        // the resulting ID is a 64 bit value with each card represented by 8 bits.
        (0..7).fold(0, |x, i| x | ((wk[i] as i64)<<(8*i)))
    }

    fn save_id(&mut self, id: i64) -> i32 {
        // this inserts a hand ID into the IDs array.

        if id == 0 {return 0;} // don't use up a record for a 0!

        // take care of the most likely first goes on the end...
        if id >= self.max_id {
            if id > self.max_id { // greater than create new else it was the last one!
                self.ids[self.num_ids as usize] = id;  // add the new ID
                self.num_ids += 1;
                self.max_id = id;
            }
            return self.num_ids - 1;
        }

        // find the slot (by a pseudo bsearch algorithm)
        let mut low: i32 = 0;
        let mut high: i32 = self.num_ids - 1;

        while high - low > 1 {
            let holdtest = (high + low + 1) / 2;
            let testval = self.ids[holdtest as usize] - id;
            if testval > 0 {high = holdtest;}
            else if testval < 0 {low = holdtest;}
            else {return holdtest;}   // got it!!
        }
        // it couldn't be found so must be added to the current location (high)
        // make space...  // don't expect this much!
        unsafe {
            let dst = self.ids.as_mut_ptr().add(high as usize +1);
            let src = self.ids.as_ptr().add(high as usize);
            std::ptr::copy(src, dst, (self.num_ids-high) as usize);
        }

        self.ids[high as usize] = id;   // do the insert into the hole created

        self.num_ids+=1;
        return high;
    }

    fn do_eval(&mut self, idin: i64, evaluator: &CactusKevEvaluator) -> i32 {
        // converts a 64bit handID to an absolute ranking.

        // I guess I have some explaining to do here...
        // I used the Cactus Kevs Eval ref http://www.suffecool.net/poker/evaluator.html
        // I Love the pokersource for speed, but I needed to do some tweaking
        // to get it my way and Cactus Kevs stuff was easy to tweak ;-)
        let mut result: i32 = 0;
        let mut mainsuit: i32 = 20;  // just something that will never hit...
        // TODO: need to eliminate the main suit from the iterator
        let mut suititerator: i32 = 1; // changed as per Ray Wotton's comment at http://archives1.twoplustwo.com/showflat.php?Cat=0&Number=8513906&page=0&fpart=18&vc=1
        let mut holdrank: i32 = 0;
        let mut wk: [u32;8] = [0;8];  // "work" intentially keeping one as a 0 end
        let mut holdcards: [i32;8] = [0;8];
        let mut numevalcards: i32 = 0;

        // See Cactus Kevs page for explainations for this type of stuff...
        const PRIMES: [i32;13] = [ 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41 ];

        if idin != 0 { // if I have a good ID then do it...
            for cardnum in 0..7 {
                // convert all 7 cards (0s are ok)
                holdcards[cardnum] =  ((idin >> (8 * cardnum)) & 0xff) as i32;
                if holdcards[cardnum] == 0 { break;}	// once I hit a 0 I know I am done
                numevalcards+=1;
                // if not 0 then count the card
                let suit = holdcards[cardnum] & 0xf;
                if suit != 0 {
	            // find out what suit (if any) was significant and remember it
	            mainsuit = suit;
                }
            }
            
            // (DoEval)
            for cardnum in 0..numevalcards {
                // just have numcards...
                let wkcard = holdcards[cardnum as usize];

                // convert to cactus kevs way!!
                // ref http://www.suffecool.net/poker/evaluator.html
                //   +--------+--------+--------+--------+
                //   |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
                //   +--------+--------+--------+--------+
                //   p = prime number of rank (deuce=2,trey=3,four=5,five=7,...,ace=41)
                //   r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
                //   cdhs = suit of card
                //   b = bit turned on depending on rank of card

                let rank = (wkcard >> 4) - 1;	 // my rank is top 4 bits 1-13 so convert
                let mut suit = wkcard & 0xf;  // my suit is bottom 4 bits 1-4, order is different, but who cares?
                if suit == 0 {
	            // if suit wasn't significant though...
	            suit = suititerator;   // Cactus Kev needs a suit!
                    suititerator += 1;
	            if suititerator == 5{	 // loop through available suits
	                suititerator = 1;
                    }
	            if suit == mainsuit {   // if it was the sigificant suit...  Don't want extras!!
	                suit = suititerator;    // skip it
                        suititerator += 1;
	                if suititerator == 5{	  // roll 1-4
	                    suititerator = 1;
                        }
	            }
                }
                // now make Cactus Kev's Card
                wk[cardnum as usize] = (PRIMES[rank as usize] | (rank << 8) | (1 << (suit + 11)) | (1 << (16 + rank))) as u32;
            }

            // (DoEval)
            // James Devlin: replaced all calls to Cactus Kev's eval_5cards with calls to
            // Senzee's improved eval_5hand_fast

            match numevalcards {  // run Cactus Keys routines
                5 | 6 | 7 => {
                    holdrank = evaluator.eval(&wk[0..numevalcards as usize]) as i32
                }
                _ => {// problem!!  shouldn't hit this...
                    println!("    Problem with numcards = {}!!\n", self.numcards);
                }
            }
            
            // (DoEval)
            // I would like to change the format of Catus Kev's ret value to:
            // hhhhrrrrrrrrrrrr   hhhh = 1 high card -> 9 straight flush
            //                    r..r = rank within the above	1 to max of 2861
            //            result = 7463 - holdrank;  // now the worst hand = 1
            result = holdrank;

            if      result < 1278 { result = result -    0 + 4096 * 1;  }// 1277 high card
            else if result < 4138 { result = result - 1277 + 4096 * 2;  }// 2860 one pair
            else if result < 4996 { result = result - 4137 + 4096 * 3;  }//  858 two pair
            else if result < 5854 { result = result - 4995 + 4096 * 4;  }//  858 three-kind
            else if result < 5864 { result = result - 5853 + 4096 * 5;  }//   10 straights
            else if result < 7141 { result = result - 5863 + 4096 * 6;  }// 1277 flushes
            else if result < 7297 { result = result - 7140 + 4096 * 7;  }//  156 full house
            else if result < 7453 { result = result - 7296 + 4096 * 8;  }//  156 four-kind
            else                  { result = result - 7452 + 4096 * 9;  }//   10 str.flushes
        }
        return result;  // now a handrank that I like
    }

    fn generate(&mut self){
        // step through the ID array - always shifting the current ID and
        // adding 52 cards to the end of the array.
        // when I am at 7 cards put the Hand Rank in!!
        // stepping through the ID array is perfect!!

        // main()

        println!("\nGetting Card IDs!\n");

        // Jmd: Okay, this loop is going to fill up the IDs[] array which has
        // 612,967 slots. as this loops through and find new combinations it
        // adds them to the end. I need this list to be stable when I set the
        // handranks (next set)  (I do the insertion sort on new IDs these)
        // so I had to get the IDs first and then set the handranks
        let mut id_num = 0;
        while self.ids[id_num as usize] != 0 || id_num == 0 {
            // start at 1 so I have a zero catching entry (just in case)
            for card in 1..=52 {
                // the ids above contain cards upto the current card.  Now add a new card
                let id = self.make_id(self.ids[id_num as usize], card);   // get the new ID for it
                // and save it in the list if I am not on the 7th card
                if self.numcards < 7 { self.save_id(id); }
            }
            print!("\rID - {}", id_num);	  // show progress -- this counts up to 612976
            id_num += 1;
        }

        // main()
        println!("\nSetting HandRanks!\n");


        let evaluator = CactusKevEvaluator::new();

        // this is as above, but will not add anything to the ID list, so it is stable
        id_num = 0;
        while self.ids[id_num as usize] != 0 || id_num == 0 {
            // start at 1 so I have a zero catching entry (just in case)
            for card in 1..=52 {
                let id = self.make_id(self.ids[id_num as usize], card);

                let id_slot = 
                if self.numcards < 7 {
	            // when in the index mode (< 7 cards) get the id to save
	            self.save_id(id) * 53 + 53
                } else {
	            // if I am at the 7th card, get the equivalence class ("hand rank") to save
	            self.do_eval(id, &evaluator)
                };

                self.max_hr = id_num * 53 + card + 53;	// find where to put it
                self.hr[self.max_hr as usize] = id_slot; // and save the pointer to the next card or the handrank
            }

            if self.numcards == 6 || self.numcards == 7 {
                // an extra, If you want to know what the handrank when there is 5 or 6 cards
                // you can just do HR[u3] or HR[u4] from below code for Handrank of the 5 or
                // 6 card hand
                // this puts the above handrank into the array
                self.hr[id_num as usize * 53 + 53] = self.do_eval(self.ids[id_num as usize], &evaluator);
            }

	    print!("\rID - {}", id_num);
            id_num += 1;
        }

        println!("\nNumber IDs = {}\nmaxHR = {}\n", self.num_ids, self.max_hr);  // for warm fuzzys

        // another algorithm right off the thread

        // Store the count of each type of hand (One Pair, Flush, etc)
        let mut hand_type_sum: [i32;10] = [0;10];

        let mut count = 0;
        // QueryPerformanceCounter(&timings);
        // start High Precision clock
        for c0 in 1..=52 {
            let u0 = self.hr[53+c0] as usize;
            for c1 in c0+1..=52 {
                let u1 = self.hr[u0+c1] as usize;
                for c2 in c1+1..=52 {
	            let u2 = self.hr[u1+c2] as usize;
                    for c3 in c2+1..=52{
	                let u3 = self.hr[u2+c3] as usize;
                        for c4 in c3+1..=52{
	                    let u4 = self.hr[u3+c4] as usize;
                            for c5 in c4+1..=52{
	                        let u5 = self.hr[u4+c5] as usize;
	                        for c6 in c5+1..=52 {
		                    hand_type_sum[self.hr[u5+c6] as usize >> 12]+=1;
		                    count+=1;
	                        }
	                    }
	                }
	            }
                }
            }
        }

        //	QueryPerformanceCounter(&endtimings);
        // end the high precision clock
        let hand_ranks: [&str;10] = [
            "BAD!!",//0
            "High Card",//1
            "Pair",//2
            "Two Pair",//3
            "Three of a Kind",//4
            "Straight",//5
            "Flush",//6
            "Full House",//7
            "Four of a Kind",//8
            "Straight Flush"//9
        ];
        
        for i in 0..9 {
            print!("\n{} = {}", hand_ranks[i], hand_type_sum[i]);
        }
        
        print!("\nTotal Hands = {}\n", count);
    }
}

// 生成 twoplustwo 表格文件
pub fn generate_data_file(path: &Path) -> std::io::Result<()>{
    let mut generater = DataFileGenerator::new();
    generater.generate();

    // output the array now that I have it!!
    let mut file = File::create(path)?;
    
    let ptr :*const i32 = generater.hr.as_ptr();
    let ptr :*const u8 = ptr as *const u8;

    unsafe {
        let slice = std::slice::from_raw_parts(ptr, generater.hr.len()*4);
        file.write(slice)?;
    }
    Ok(())
}
