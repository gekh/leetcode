pub struct Solution;

impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(mut words: Vec<String>) -> Vec<String> {
        let mut for_cut = words
            .iter()
            .enumerate()
            .map(|(i, w)| (i, w.clone()))
            .collect::<Vec<(usize, String)>>();
        let mut words_sorted = words.clone();
        words_sorted.sort_unstable_by(|a, b| a.len().cmp(&b.len()));
        words_sorted.reverse();
        for i in 0..words_sorted.len() {
            for j in 0..for_cut.len() {
                if for_cut[j].1 == words_sorted[i] {
                    continue;
                }
                if for_cut[j].1.contains(&words_sorted[i]) {
                    for_cut.push((for_cut[j].0, for_cut[j].1.replace(&words_sorted[i], "0")));
                }
            }
        }


        let mut for_cut = for_cut
            .iter()
            .filter(|el| {
                let nulls = el.1.chars().filter(|&c| c == '0').count();
                if nulls == el.1.len() && nulls > 1 {
                    true
                } else {
                    false
                }
            })
            .map(|el| el.0)
            .collect::<Vec<_>>();
        for_cut.sort_unstable();
        for_cut.dedup();

        return words
            .into_iter()
            .enumerate()
            .filter_map(|(i, el)| if for_cut.contains(&i) { Some(el) } else { None })
            .collect::<Vec<_>>();
    }
}
