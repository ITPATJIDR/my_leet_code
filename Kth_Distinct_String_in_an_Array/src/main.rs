use std::collections::HashMap;

fn main() {
    let mut arr: Vec<String> = Vec::new();
    arr.push("aaa".to_string());
    arr.push("aa".to_string());
    arr.push("a".to_string());

    // arr.push("a".to_string());
    // arr.push("b".to_string());
    // arr.push("a".to_string());


    // // arr.push("d".to_string());
    // // arr.push("b".to_string());
    // // arr.push("c".to_string());
    // // arr.push("b".to_string());
    // // arr.push("c".to_string());
    // // arr.push("a".to_string());

    let k: i32 = 1;

    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut hash = HashMap::new();
        for item in &arr {
            *hash.entry(item).or_insert(0) += 1;
        }
        let (mut result,mut count) = ("".to_string(),0);
        for item in &arr {
            if hash[&item] == 1 {
                count += 1;
                if count == k {
                    result = item.clone();
                    break
                }
                
            }
        }
        result
    }


    let test = kth_distinct(arr, k);
    println!("{}",test)
}
