use super::p93_restore_ip_addresses::Solution;

pub fn test() {
    assert_eq!(call("00000"), Vec::<String>::new());
    assert_eq!(call("000256"), Vec::<String>::new());
    assert_eq!(call("1111"), vec![String::from("1.1.1.1")]);
    assert_eq!(call("11111"), vec![
        String::from("1.1.1.11"),
        String::from("1.1.11.1"),
        String::from("1.11.1.1"),
        String::from("11.1.1.1")
    ]);
    assert_eq!( call("25525511135"), vec![ String::from("255.255.11.135"), String::from("255.255.111.35") ] );
    assert_eq!(call("0000"), vec![String::from("0.0.0.0")]);
    assert_eq!(
        call("101023"),
        vec![
            String::from("1.0.10.23"),
            String::from("1.0.102.3"),
            String::from("10.1.0.23"),
            String::from("10.10.2.3"),
            String::from("101.0.2.3"),
        ]
    );
}

fn call(s: &str) -> Vec<String> {
    Solution::restore_ip_addresses(String::from(s))
}
