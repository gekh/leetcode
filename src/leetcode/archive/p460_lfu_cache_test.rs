use super::p460_lfu_cache::LFUCache;

pub fn test() {
    // call(
    //     vec!["LFUCache", "put", "put", "get", "put", "get", "get", "put", "get", "get", "get", "put", "put", "put", "put", "put"],
    //     vec![vec![2], vec![1, 1], vec![2, 2], vec![1], vec![3, 3], vec![2], vec![3], vec![4, 4], vec![1], vec![3], vec![4],
    //     vec![4, 4], vec![4, 4], vec![4, 4], vec![4, 4], vec![4, 4]],
    // );

    // call (
    //     vec!["LFUCache","put","put","get","put","put","get"],
    //     vec![vec![2],vec![2,1],vec![2,2],vec![2],vec![1,1],vec![4,1],vec![2]],
    // );

    call(
        vec!["LFUCache","put","put","get","put","get","get"],
        vec![vec![2],vec![2,1],vec![1,1],vec![2],vec![4,1],vec![1],vec![2]]
    );
}

fn call(commands: Vec<&str>, nums: Vec<Vec<i32>>) {
    let mut obj = LFUCache::new(nums[0][0]);
    for (i,cmd) in commands.iter().enumerate() {
        if cmd == &"put" {
            println!();
            println!("PUT {:?}", nums[i]);
            obj.put(nums[i][0], nums[i][1]);
        } else if cmd == &"get" {
            println!();
            println!("GET {:?}", nums[i][0]);
            println!("Cache: {:?}", obj.get(nums[i][0]));
        }
    }

}
