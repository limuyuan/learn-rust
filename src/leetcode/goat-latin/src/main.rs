// from leetcode: https://leetcode-cn.com/problems/goat-latin/

fn main() {
    let mut test_case = Solution {
        sentence: String::from("test"),
    };
    println!("{}", Solution::to_goat_latin(test_case.sentence));
}

impl Solution {
    // TODO: unfinished, cannot understand concept of deref and don't know why method `push_str()`
    // cannot be used here
    pub fn to_goat_latin(sentence: String) -> String {
        let v: Vec<&str> = sentence.split(" ").collect();

        for i in 0..(v.len()) {
            match &v[i][0..1] {
                "a"|"e"|"i"|"o"|"u" => {
                    v[i].push_str("ma")
                }
            }
        }

        String::from("no result")
    }
}

struct Solution {
    sentence: String,
}
