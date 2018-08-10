#![feature(use_extern_macros)]

extern crate fluent_impl;

pub mod simple {
    pub trait Add1 {
        fn add_1(&mut self);
    }

    use fluent_impl::{fluent_impl, fluent_impl_opts};

    #[derive(Default, PartialEq, Debug)]
    pub struct Simple {
        num: i32,
    }

    impl Simple {
        pub fn new(n: i32) -> Self {
            Self { num: n }
        }

        pub fn get_num(&self) -> i32 {
            self.num
        }
    }

    #[fluent_impl] //~ ERROR
    impl Add1 for Simple {
        fn add_1(&mut self) {
            self.num += 1;
        }
    }
}

fn main() {
    use simple::{Add1, Simple};
    let mut s1 = Simple::default();
    s1.add_1();
    assert_eq!(s1.get_num(), 1);
    assert_eq!(s1, Simple::default().with_add_1());
}
