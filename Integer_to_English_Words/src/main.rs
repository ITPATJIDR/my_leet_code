struct Solution;

impl Solution {
    pub fn number_to_words(mut num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let mp = vec![
            (1_000_000_000, "Billion"),
            (1_000_000, "Million"),
            (1_000, "Thousand"),
            (100, "Hundred"),
            (90, "Ninety"),
            (80, "Eighty"),
            (70, "Seventy"),
            (60, "Sixty"),
            (50, "Fifty"),
            (40, "Forty"),
            (30, "Thirty"),
            (20, "Twenty"),
            (19, "Nineteen"),
            (18, "Eighteen"),
            (17, "Seventeen"),
            (16, "Sixteen"),
            (15, "Fifteen"),
            (14, "Fourteen"),
            (13, "Thirteen"),
            (12, "Twelve"),
            (11, "Eleven"),
            (10, "Ten"),
            (9, "Nine"),
            (8, "Eight"),
            (7, "Seven"),
            (6, "Six"),
            (5, "Five"),
            (4, "Four"),
            (3, "Three"),
            (2, "Two"),
            (1, "One"),
        ];

        fn helper(num: i32, mp: &[(i32, &str)]) -> String {
            for &(value, name) in mp {
                if num >= value {
                    let a = if num >= 100 {
                        Solution::number_to_words(num / value) + " "
                    } else {
                        "".to_string()
                    };

                    let b = name.to_string();

                    let c = if num % value != 0 {
                        " ".to_string() + &Solution::number_to_words(num % value)
                    } else {
                        "".to_string()
                    };

                    return a + &b + &c;
                }
            }
            "".to_string()
        }

        helper(num, &mp)
    }
}

fn main() {
    let num = 1234567891;
    let result = Solution::number_to_words(num);
    println!("{}", result);
}
