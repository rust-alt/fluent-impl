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
        pub fn append<S: Borrow<str>>(&mut self, arg: S, counter: &mut usize) {
            self.text += arg.borrow();
            *counter += 1;
        }
    }
}

fn main() {
    use m::St;
    let mut s1 = St::default();
    let mut c1 = 0;
    let mut c2 = 0;
    s1.append("foo", &mut c1);
    assert_eq!(s1.get_text(), "foo");
    assert_eq!(s1, St::default().with_appended("foo", &mut c2));
    assert_eq!(c1, 1);
    assert_eq!(c1, c2);
}
