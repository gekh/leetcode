use super::p944_delete_columns_to_make_sorted::Solution;

pub fn test() {

    let strs = vec![
        String::from("abc"),
        String::from("dyf"),
        String::from("xez"),
    ];
    assert!(call(strs) == 1);

    let strs = vec![
        String::from("abc"),
        String::from("bce"),
        String::from("cae"),
    ];
    assert!(call(strs) == 1);

    let strs = vec![
        String::from("abc"),
        String::from("cce"),
        String::from("bae"),
    ];
    assert!(call(strs) == 2);

    let strs = vec![
        String::from("cba"),
        String::from("daf"),
        String::from("ghi"),
    ];
    assert!(call(strs) == 1);

    let strs = vec![
        String::from("a"),
        String::from("b"),
    ];
    assert!(call(strs) == 0);

    let strs = vec![
        String::from("zyx"),
        String::from("wvu"),
        String::from("tsr"),
    ];
    assert!(call(strs) == 3);

    let strs = vec![
        String::from("rrjk"),
        String::from("furt"),
        String::from("guzm"),
    ];
    assert!(call(strs) == 2);
}

fn call(strs: Vec<String>) -> i32 {
    Solution::min_deletion_size(strs)
}
