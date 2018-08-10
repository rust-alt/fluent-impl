#![feature(use_extern_macros)]

extern crate fluent_impl;

pub mod simple {
    use fluent_impl::{fluent_impl, fluent_impl_opts};

    #[derive(Default, PartialEq, Debug)]
    pub struct Simple {
        num: i32,
    }

    #[fluent_impl(prefix = "whatever_")]
    impl Simple {
        pub fn new(n: i32) -> Self {
            Self { num: n }
        }

        pub fn get_num(&self) -> i32 {
            self.num
        }

        #[fluent_impl_opts(name = "ret_with_added_1")]
        pub fn add_1(&mut self) {
            self.num += 1;
        }
    }
}

fn main() {
    use simple::Simple;
    let mut s1 = Simple::default();
    s1.add_1();
    assert_eq!(s1.get_num(), 1);
    assert_eq!(s1, Simple::default().ret_with_added_1());
}
