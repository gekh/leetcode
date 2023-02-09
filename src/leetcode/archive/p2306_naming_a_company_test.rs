use super::p2306_naming_a_company::Solution;

pub fn test() {
    assert_eq!(
        call(vec![ String::from("coffee"), String::from("donuts"), String::from("time"), String::from("toffee"), ]),
        6
    );

    assert_eq!(
        call(vec![ String::from("soth"), String::from("cdmxv"), String::from("kec"), String::from("gxac"), String::from("f"), String::from("dkg"), String::from("fbjasepo"), String::from("ffpfv"), String::from("lkorxv"), String::from("fiogt"), String::from("yydiiz"), String::from("mbdkgsle"), String::from("fxk"), String::from("dex"), String::from("cysl"), String::from("bjtacetyh"), String::from("fkieusm"), String::from("vz"), String::from("tfpfv"), String::from("jn"), String::from("kzrsdsykpb"), String::from("y"), String::from("roedo"), String::from("rzbyvkeyhr"), String::from("cq"), ]),
        530
    );
}

fn call(ideas: Vec<String>) -> i64 {
    Solution::distinct_names(ideas)
}
