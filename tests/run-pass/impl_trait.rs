extern crate fluent_impl;

pub mod m {
    use fluent_impl::{fluent_impl, fluent_impl_opts};
    use std::borrow::Borrow;

    #[derive(Default, PartialEq, Debug)]
    pub struct St {
        text: String,
    }

    #[fluent_impl]
    impl St {
        pub fn get_text(&self) -> &str {
            &self.text
        }

        #[fluent_impl_opts(rename = "appended")]
        pub fn append(&mut self, arg: impl Borrow<str>) {
            self.text += arg.borrow();
        }
    }
}

fn main() {
    use m::St;
    let mut s1 = St::default();
    s1.append("foo");
    assert_eq!(s1.get_text(), "foo");
    assert_eq!(s1, St::default().with_appended("foo"));
}
