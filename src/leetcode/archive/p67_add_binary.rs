/**
 * Given two binary strings a and b, return their sum as a binary string.
 * https://leetcode.com/problems/add-binary/submissions/865332996/
 */
pub fn add_binary(mut a: String, mut b: String) -> String {
  let mut i = std::cmp::max(a.len(), b.len());
  let mut carry = 0;
  let mut r: Vec<u8> = vec![];

  loop {
    i -= 1;

    let x = a.pop().unwrap_or('0');
    let y = b.pop().unwrap_or('0');

    match (x,y) {
      ('0','0') => { r.push(0 + carry); carry = 0;},
      ('1','0') | ('0','1') => if carry == 1 { r.push(0); } else { r.push(1) },
      ('1','1') => if carry == 1 { r.push(1); } else { r.push(0); carry = 1; },
      _ => unreachable!("It is impossible"),
    }

    if i == 0 { break; }
  }

  if carry == 1 {
    r.push(1);
  }

  r.reverse();

  return r.into_iter().map(|i| i.to_string()).collect::<String>();
}
