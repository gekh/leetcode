pub struct Solution();

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        word.chars().skip(1).all(|c| c.is_lowercase())
        || word.chars().all(|c| c.is_uppercase())
    }
}
