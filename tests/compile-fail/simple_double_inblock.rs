extern crate fluent_impl;

pub mod simple {
    use fluent_impl::{fluent_impl, fluent_impl_opts};

    #[derive(Default, PartialEq, Debug)]
    pub struct Simple {
        num: i32,
    }

    #[fluent_impl(inblock, docs = "", inblock)] //~ ERROR
    /// Simple impl
    impl Simple {
        /// new()
        pub fn new(n: i32) -> Self {
            Self { num: n }
        }

        /// get_num()
        pub fn get_num(&self) -> i32 {
            self.num
        }

        /// add_1()
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
    assert_eq!(s1, Simple::default().with_add_1());
}
