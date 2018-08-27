extern crate fluent_impl;

pub mod m {
    use fluent_impl::{fluent_impl, fluent_impl_opts};
    use std::borrow::Borrow;
    use std::ops::AddAssign;

    #[derive(PartialEq, Debug)]
    pub struct TCounter(pub u32);

    #[derive(PartialEq, Debug)]
    pub struct St<A: AddAssign> {
        value: A,
        text: String,
    }

    #[fluent_impl]
    // impl block with generic arguments works
    impl<A: AddAssign> St<A> {
        // Constants (or any other items) in impl block are okay
        pub(crate) const C_TC: u32 = 100;

        pub fn new(value: A, text: String) -> Self {
            Self { value, text }
        }

        pub fn get_value(&self) -> &A {
            &self.value
        }

        pub fn get_text(&self) -> &str {
            &self.text
        }

        #[fluent_impl_opts(rename = "added_value")]
        // Destructuring patterns in method arguments are okay
        pub fn add_value(
            &mut self,
            to_be_added: A,
            TCounter(counter): &mut TCounter,
        ) {
            self.value += to_be_added;
            *counter += 1;
        }

        #[fluent_impl_opts(rename = "appended_text")]
        // Generic method arguments are okay
        pub fn append_text<S: Borrow<str>>(&mut self, arg: S) {
            self.text += arg.borrow();
        }

        #[fluent_impl_opts(rename = "appended_text_impl_trait")]
        // Needless to say, impl Trait method arguments are also okay
        pub fn append_text_impl_trait(&mut self, arg: impl Borrow<str>) {
            self.text += arg.borrow();
        }
    }
}

fn main() {
    use m::{St, TCounter};
    // ========
    let mut tc1 = TCounter(St::<u32>::C_TC);
    let mut s1 = St::new(0u32, "".into());
    s1.append_text("simple ");
    s1.append_text::<&str>("turbo fish ");
    s1.append_text_impl_trait("impl trait");
    s1.add_value(5, &mut tc1);
    assert_eq!(s1.get_text(), "simple turbo fish impl trait");
    assert_eq!(tc1, TCounter(St::<u32>::C_TC + 1));
    // ========
    let mut tc2 = TCounter(St::<u32>::C_TC);
    let s2 = St::new(0u32, "".into())
        .with_appended_text("simple ")
        .with_appended_text::<&str>("turbo fish ")
        .with_appended_text_impl_trait("impl trait")
        .with_added_value(5, &mut tc2);
    assert_eq!(s2, s1);
    assert_eq!(tc2, tc1);
}
