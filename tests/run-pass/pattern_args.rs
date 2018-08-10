#![feature(use_extern_macros)]

extern crate fluent_impl;

pub mod m {
    use fluent_impl::{fluent_impl, fluent_impl_opts};
    use std::borrow::Borrow;

    #[derive(Default, PartialEq, Debug)]
    pub struct TCounter(pub u32);

    #[derive(Default, PartialEq, Debug)]
    pub struct SCounter {
        pub c: u32,
    }

    #[derive(Default, PartialEq, Debug)]
    pub struct St {
        text: String,
    }

    #[fluent_impl]
    impl St {
        pub(crate) const C_TC: u32 = 3;
        pub(crate) const C_SC: u32 = 7;

        pub fn get_text(&self) -> &str {
            &self.text
        }

        #[fluent_impl_opts(rename = "appended")]
        pub fn append<S: Borrow<str>>(&mut self, arg: S, counter: &mut usize) {
            self.text += arg.borrow();
            *counter += 1;
        }

        #[fluent_impl_opts(rename = "appended_tc")]
        pub fn append_tc<S: Borrow<str>>(&mut self, arg: S, TCounter(counter): &mut TCounter) {
            self.text += arg.borrow();
            *counter += 1;
        }

        #[fluent_impl_opts(rename = "appended_sc")]
        pub fn append_sc<S: Borrow<str>>(&mut self, arg: S, SCounter { c: counter }: &mut SCounter) {
            self.text += arg.borrow();
            *counter += 1;
        }
    }
}

fn main() {
    use m::{SCounter, St, TCounter};
    let mut s = St::default();
    let mut s_tc = St::default();
    let mut s_sc = St::default();
    let mut c1 = 0;
    let mut c2 = 0;
    let mut tc1 = TCounter(St::C_TC);
    let mut tc2 = TCounter(St::C_TC);
    let mut sc1 = SCounter { c: St::C_SC };
    let mut sc2 = SCounter { c: St::C_SC };
    s.append("foo", &mut c1);
    assert_eq!(s.get_text(), "foo");
    assert_eq!(s, St::default().with_appended("foo", &mut c2));
    assert_eq!(c1, 1);
    assert_eq!(c1, c2);
    // ========
    s_tc.append_tc("foo", &mut tc1);
    assert_eq!(s_tc.get_text(), "foo");
    assert_eq!(s_tc, St::default().with_appended_tc("foo", &mut tc2));
    assert_eq!(tc1, TCounter(St::C_TC + 1));
    assert_eq!(tc1, tc2);
    // ========
    s_sc.append_sc("foo", &mut sc1);
    assert_eq!(s_sc.get_text(), "foo");
    assert_eq!(s_sc, St::default().with_appended_sc("foo", &mut sc2));
    assert_eq!(sc1, SCounter { c: St::C_SC + 1 });
    assert_eq!(sc1, sc2);
}
