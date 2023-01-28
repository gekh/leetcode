use super::p352_data_stream_as_disjoint_intervals::SummaryRanges;

pub fn test() {
    assert_eq!(
        call(
            vec![
                "SummaryRanges",
                "addNum",
                "getIntervals",
                "addNum",
                "getIntervals",
                "addNum",
                "getIntervals",
                "addNum",
                "getIntervals",
                "addNum",
                "getIntervals"
            ],
            vec![
                vec![],
                vec![1],
                vec![],
                vec![3],
                vec![],
                vec![7],
                vec![],
                vec![2],
                vec![],
                vec![6],
                vec![]
            ]
        ),
        vec![
            None,
            None,
            Some(vec![vec![1, 1]]),
            None,
            Some(vec![vec![1, 1], vec![3, 3]]),
            None,
            Some(vec![vec![1, 1], vec![3, 3], vec![7, 7]]),
            None,
            Some(vec![vec![1, 3], vec![7, 7]]),
            None,
            Some(vec![vec![1, 3], vec![6, 7]]),
        ]
    );
}

fn call(commands: Vec<&str>, nums: Vec<Vec<i32>>) -> Vec<Option<Vec<Vec<i32>>>> {
    let mut obj = SummaryRanges::new();
    for (i,cmd) in commands.iter().enumerate() {
        if cmd == &"addNum" {
            obj.add_num(nums[i][0]);
        } else if cmd == &"getIntervals" {
            println!("Intervals: {:?}", obj.get_intervals());
        }
    }

    vec![
        None,
        None,
        Some(vec![vec![1, 1]]),
        None,
        Some(vec![vec![1, 1], vec![3, 3]]),
        None,
        Some(vec![vec![1, 1], vec![3, 3], vec![7, 7]]),
        None,
        Some(vec![vec![1, 3], vec![7, 7]]),
        None,
        Some(vec![vec![1, 3], vec![6, 7]]),
    ]
}
