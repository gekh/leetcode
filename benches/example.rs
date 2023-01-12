#[macro_use]
extern crate bencher;

#[path = "../src/leetcode/mod.rs"]
mod leetcode;


use bencher::Bencher;

fn leet(bench: &mut Bencher) {
  bench.iter(|| {
    // for _ in 0..1000 {
      leetcode::p1519_num_of_nodes_in_subtree_with_the_same_label_test::test();
    // }
  });
}

// fn a(bench: &mut Bencher) {
//     bench.iter(|| {
//         (0..1000).fold(0, |x, y| x + y)
//     })
// }

// fn b(bench: &mut Bencher) {
//     const N: usize = 1024;
//     bench.iter(|| {
//         vec![0u8; N]
//     });

//     bench.bytes = N as u64;
// }

benchmark_group!(benches, leet);
benchmark_main!(benches);