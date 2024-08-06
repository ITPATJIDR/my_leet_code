use std::{ collections::HashMap};

struct Solution;
impl Solution {
    pub fn minimum_pushes(mut word: String) -> i32 {
        let mut chars:Vec<char> = word.chars().collect();
        chars.sort_unstable();

        let mut numpad: HashMap<char, i32> = HashMap::new();
        let mut start_numpad:i32 = 2;
        let mut result:i32 = 0;

        for i in chars.clone() {
            if !numpad.contains_key(&i) {
                if start_numpad > 8 {
                    numpad.insert(i,9);
                }else {
                    numpad.insert(i,start_numpad);
                } 
                start_numpad += 1;
            }
        }

        for ch in chars {
            for (x,y) in &numpad {
                if *x == ch {
                    println!("{}", x);
                    println!("{}", ch);
                    println!("================================");
                    result += 1;
                }
            }
        }

        println!("{:?}",  result);

        return 12;
    }

    pub fn minimum_pushes_2(word: String) -> i32 {
        let mut letter_freq = vec![0; 26];

        for c in word.chars() {
            println!("{:?}", c as u8 - b'a');
            letter_freq[(c as u8 - b'a') as usize] += 1;
        }

        letter_freq.sort_unstable_by(|a, b| b.cmp(a));
        
        let mut total_presses = 0;
        for (i, &freq) in letter_freq.iter().enumerate() {
            if freq == 0 {
                break;
            }
            total_presses += (i / 8 + 1) as i32 * freq;
        }
        
        total_presses
    }

    pub fn minimum_pushes_3(word: String) -> i32 {
        let mut map = vec![0usize; 26];
        let a = 'a' as usize;
        for c in word.chars() {
            let idx = c as usize - a;
            map[idx] += 1;
        }
        let mut out = 0usize;
        map.sort();
        for (i, v) in map.into_iter().rev().enumerate() {
            out += (i / 8 + 1) * v;
        }
        out as i32
    }
}

fn main() {
    let test_case:Vec<String> = vec![
        "abcde".to_string(),
        "xyzxyzxyzxyz".to_string(),
        "aabbccddeeffgghhiiiiii".to_string()
    ];

    let result = Solution::minimum_pushes_2(test_case[2].clone());

    println!("{:?}",result);
}
