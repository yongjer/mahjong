use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileType {
    Wan,   // Character
    Tong,  // Dot
    Tiao,  // Bamboo
    Wind,  // East, South, West, North
    Dragon, // Zhong, Fa, Bai
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub current_shanten: i32,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub discard_tile: String,
    pub shanten_after: i32,
    pub ukeire: Vec<String>,
    pub ukeire_count: i32,
    pub score: f64,
}

pub struct Hand {
    pub counts: [i32; 34],
}

impl Hand {
    pub fn from_string(s: &str) -> Self {
        let mut counts = [0; 34];
        let mut chars = s.chars().peekable();
        let mut temp_digits = Vec::new();

        while let Some(c) = chars.next() {
            if c.is_digit(10) {
                temp_digits.push(c.to_digit(10).unwrap() as usize);
            } else {
                let offset = match c {
                    'm' | 'w' => 0,  // Wan
                    'p' | 't' => 9,  // Tong
                    's' | 'l' => 18, // Tiao
                    'z' => 27,       // Honors
                    _ => continue,
                };
                for digit in temp_digits.drain(..) {
                    if digit >= 1 && digit <= 9 {
                        counts[offset + digit - 1] += 1;
                    } else if offset == 27 && digit >= 1 && digit <= 7 {
                        counts[27 + digit - 1] += 1;
                    }
                }
            }
        }
        Hand { counts }
    }

    pub fn to_tile_string(index: usize) -> String {
        if index < 9 {
            format!("{}m", index + 1)
        } else if index < 18 {
            format!("{}p", index - 9 + 1)
        } else if index < 27 {
            format!("{}s", index - 18 + 1)
        } else {
            format!("{}z", index - 27 + 1)
        }
    }
}

pub fn calculate_shanten(counts: &[i32; 34]) -> i32 {
    let mut min_shanten = 10;
    let mut memo = HashMap::new();

    for i in 0..34 {
        if counts[i] >= 2 {
            let mut next_counts = *counts;
            next_counts[i] -= 2;
            let s = calculate_recursive(&mut next_counts, 0, 0, 0, 1, &mut memo);
            if s < min_shanten {
                min_shanten = s;
            }
        }
    }
    
    let s = calculate_recursive(&mut counts.clone(), 0, 0, 0, 0, &mut memo);
    if s < min_shanten {
        min_shanten = s;
    }

    min_shanten
}

fn calculate_recursive(
    counts: &mut [i32; 34], 
    index: usize, 
    melds: i32, 
    taatsu: i32, 
    has_pair: i32,
    memo: &mut HashMap<(usize, i32, i32, i32, [i32; 34]), i32>
) -> i32 {
    if index >= 34 {
        if has_pair == 1 {
            let effective_taatsu = std::cmp::min(taatsu, 5 - melds);
            return 10 - 2 * melds - effective_taatsu - 1;
        } else {
            let effective_taatsu = std::cmp::min(taatsu, 6 - melds);
            return 10 - 2 * melds - effective_taatsu;
        }
    }

    // Memoization key: (index, melds, taatsu, has_pair, counts_from_index_onward)
    // Actually, counts[index..] is enough.
    let mut state_counts = [0i32; 34];
    state_counts[index..34].copy_from_slice(&counts[index..34]);
    let key = (index, melds, taatsu, has_pair, state_counts);
    if let Some(&res) = memo.get(&key) {
        return res;
    }

    if counts[index] == 0 {
        let res = calculate_recursive(counts, index + 1, melds, taatsu, has_pair, memo);
        memo.insert(key, res);
        return res;
    }

    // Option 1: Ignore this tile (but we must move past it if we can't use it anymore)
    // To simplify, if we don't use it now, we won't use it in any block starting at this index.
    let mut res = calculate_recursive(counts, index + 1, melds, taatsu, has_pair, memo);

    // Option 2: Triplet
    if counts[index] >= 3 {
        counts[index] -= 3;
        let s = calculate_recursive(counts, index, melds + 1, taatsu, has_pair, memo);
        if s < res { res = s; }
        counts[index] += 3;
    }

    // Option 3: Sequence
    if index < 27 && index % 9 <= 6 {
        if counts[index+1] > 0 && counts[index+2] > 0 {
            counts[index] -= 1;
            counts[index+1] -= 1;
            counts[index+2] -= 1;
            let s = calculate_recursive(counts, index, melds + 1, taatsu, has_pair, memo);
            if s < res { res = s; }
            counts[index] += 1;
            counts[index+1] += 1;
            counts[index+2] += 1;
        }
    }

    // Option 4: Taatsu (Pair as taatsu)
    if counts[index] >= 2 {
        counts[index] -= 2;
        let s = calculate_recursive(counts, index, melds, taatsu + 1, has_pair, memo);
        if s < res { res = s; }
        counts[index] += 2;
    }

    // Option 5: Taatsu (Sequence types)
    if index < 27 && index % 9 <= 7 {
        if counts[index+1] > 0 {
            counts[index] -= 1;
            counts[index+1] -= 1;
            let s = calculate_recursive(counts, index, melds, taatsu + 1, has_pair, memo);
            if s < res { res = s; }
            counts[index] += 1;
            counts[index+1] += 1;
        }
    }
    if index < 27 && index % 9 <= 6 {
        if counts[index+2] > 0 {
            counts[index] -= 1;
            counts[index+2] -= 1;
            let s = calculate_recursive(counts, index, melds, taatsu + 1, has_pair, memo);
            if s < res { res = s; }
            counts[index] += 1;
            counts[index+2] += 1;
        }
    }

    memo.insert(key, res);
    res
}

pub fn analyze(hand_str: &str, discards: &[String]) -> AnalysisResult {
    let hand = Hand::from_string(hand_str);
    let current_shanten = calculate_shanten(&hand.counts);
    
    // Parse discards into a frequency table as well
    let mut discard_counts = [0i32; 34];
    for d in discards {
        let h = Hand::from_string(d);
        for i in 0..34 {
            discard_counts[i] += h.counts[i];
        }
    }

    let mut recommendations = Vec::new();

    for i in 0..34 {
        if hand.counts[i] > 0 {
            let mut next_counts = hand.counts;
            next_counts[i] -= 1;
            
            let shanten_after = calculate_shanten(&next_counts);
            
            let mut ukeire = Vec::new();
            let mut ukeire_count = 0;
            
            for j in 0..34 {
                let mut test_counts = next_counts;
                test_counts[j] += 1;
                if calculate_shanten(&test_counts) < shanten_after {
                    ukeire.push(Hand::to_tile_string(j));
                    // Remaining = 4 - (in hand) - (in discards)
                    let remaining = 4 - hand.counts[j] - discard_counts[j];
                    if remaining > 0 {
                        ukeire_count += remaining;
                    }
                }
            }

            recommendations.push(Recommendation {
                discard_tile: Hand::to_tile_string(i),
                shanten_after,
                ukeire,
                ukeire_count,
                score: ukeire_count as f64,
            });
        }
    }

    // Sort by shanten then by ukeire count
    recommendations.sort_by(|a, b| {
        a.shanten_after.cmp(&b.shanten_after)
            .then(b.ukeire_count.cmp(&a.ukeire_count))
    });

    AnalysisResult {
        current_shanten,
        recommendations,
    }
}
