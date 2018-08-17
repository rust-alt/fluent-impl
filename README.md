# fluent-impl [![Crate](https://img.shields.io/crates/v/fluent-impl.svg)](https://crates.io/crates/fluent-impl) [![Travis Build Status](https://api.travis-ci.org/rust-alt/fluent-impl.svg?branch=master)](https://travis-ci.org/rust-alt/fluent-impl) [![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/rust-alt/fluent-impl?svg=true&branch=master)](https://ci.appveyor.com/project/MoSal/fluent-impl)


[**Documentation**](https://rust-alt.github.io/fluent-impl/doc/fluent_impl/)


A procedural macro that generates chaining methods from non-chaining ones in an impl block.

When applied to an impl block, `#[fluent_impl]` will scan all methods in the block
in search for chain-able methods, and generate chaining methods from them.

Chain-able methods are the ones with `&mut self` as a first argument, and return nothing.
That's it, there are no other restrictions.

# Detailed Build Status

`fluent_impl` remains nightly-only, for now. The feature `use_extern_macros` is
[supposedly days away from stabilisation](https://github.com/rust-lang/rust/pull/50911).


## Travis

| ___Linux___ | ___OSX___ |
|:----:|:----:|
| ![linux-nightly](https://badges.herokuapp.com/travis/rust-alt/fluent-impl?env=BADGE=linux-nightly&label=linux%20nightly) | ![osx-nightly](https://badges.herokuapp.com/travis/rust-alt/fluent-impl?env=BADGE=osx-nightly&label=%20%20osx%20nightly) |
| ![linux-beta](https://badges.herokuapp.com/travis/rust-alt/fluent-impl?env=BADGE=linux-beta&label=%20linux%20beta%20%20) | ![osx-beta](https://badges.herokuapp.com/travis/rust-alt/fluent-impl?env=BADGE=osx-beta&label=%20%20%20osx%20beta%20%20) |
| ![linux-stable](https://badges.herokuapp.com/travis/rust-alt/fluent-impl?env=BADGE=linux-stable&label=linux%20stable%20) | ![osx-stable](https://badges.herokuapp.com/travis/rust-alt/fluent-impl?env=BADGE=osx-stable&label=%20osx%20%20stable%20) |

## AppVeyor

Failure could be caused by a bug in [compiletest-rs](https://github.com/laumann/compiletest-rs)
(I'm not sure). Feedback from Windows users would be greatly appreciated.

| Windows | x86_64 | i686 |
|:-------:|:------:|:----:|
| __nightly__ | ![nightly x86_64](https://appveyor-matrix-badges.herokuapp.com/repos/MoSal/fluent-impl/branch/master/1) | ![nightly i686](https://appveyor-matrix-badges.herokuapp.com/repos/MoSal/fluent-impl/branch/master/2) |
| __beta__   | ![beta x86_64](https://appveyor-matrix-badges.herokuapp.com/repos/MoSal/fluent-impl/branch/master/5) | ![beta i686](https://appveyor-matrix-badges.herokuapp.com/repos/MoSal/fluent-impl/branch/master/6) | 
| __stable__ | ![satble x86_64](https://appveyor-matrix-badges.herokuapp.com/repos/MoSal/fluent-impl/branch/master/3) | ![stable i686](https://appveyor-matrix-badges.herokuapp.com/repos/MoSal/fluent-impl/branch/master/4) |


# Usage

Add `fluent-impl` to the dependencies in `Cargo.toml`:

``` toml
[dependencies]
fluent-impl = "0.1"
```

Then add the following to the top of `src/lib.rs`:

``` rust ignore
// Will be stabilized soon
#![feature(use_extern_macros)]

extern crate fluent_impl;

use fluent_impl::{fluent_impl, fluent_impl_opts};

```

# Examples

If we have a simple struct with a simple impl block:

``` rust
#[derive(Default, PartialEq, Debug)]
pub struct Simple {
    num: i32,
}

impl Simple {
    // ...
    pub fn add_1(&mut self) {
        self.num +=1;
    }
}
```

Then we add the macro attribute to the impl block:

``` rust ignore
#[fluent_impl]
impl Simple {
    // ...
    pub fn add_1(&mut self) {
        self.num +=1;
    }
}
```

The macro will generate a new impl block with the content:

``` rust ignore
#[doc = "Chaining (fluent) methods for [`Simple`]."]
impl Simple {
    #[doc = "The chaining (fluent) equivalent of [`add_1()`].\n\n [`add_1`]: Simple::add_1\n [`add_1()`]: Simple::add_1"]
    pub fn with_add_1(mut self) -> Self {
        self.add_1();
        self
    }
}
```

A full more involved example can be found bellow the *Attribute Configuration* section.

# Attribute Configuration

`#[fluent_impl]` is configurable with comma-separated options passed to the attribute
itself, and options passed to a method-level attribute `#[fluent_impl_opts]`.

## `#[fluent_impl]` Attribute Options
*(`inblock`, `non_public`, `prefix`, `impl_doc`, `doc`)*

 *impl block*-level configuration.

 ### Example

 ``` rust ignore
 #[fluent_impl(inblock, non_public, prefix="chain_")]
 impl Simple {
     // ...
 }
 ```

 ### Options

 * **`inblock`** (default: unset)

   By default, a new impl block is generated, and chaining methods are added there.
   If `inblock` is passed, every chaining method will be generated right below
   the chain-able one.

   The order in which methods appear on docs is probably the only reason why you
   should care about this.

   There is a corresponding method-level *`inblock`* option which will selectively enable
   this behavior for individual methods.

 * **`non_public`** (default: unset)

   By default, non fully-public methods are skipped. If this option is passed, the macro
   will generate chaining equivalents for chain-able private or partially-public methods.

   There is a corresponding method-level *`non_public`* option which will selectively enable
   this behavior for individual methods.

 * **`prefix`** (default: "with_")

   The default chaining method name is this prefix appended by the chain-able method name.

   * *`prefix`* is not allowed to be an empty string. Check the *`name`* method-level option
   if you want to name a chaining method to whatever you like.

   There is a corresponding method-level *`prefix`* option which will selectively override
   the value set here (or the default).

 * **`impl_doc`** (default: "Chaining (fluent) methods for [\`%t%\`].")

   If a new block is generated for the chaining methods, this is the doc string template
   for it. `%t%` is replaced with the type path.

 * **`doc`** (default: "The chaining (fluent) equivalent of [\`%f%()\`].")

   Chaining method doc string template. `%t%` is replaced with the type path. `%f%` is
   replaced with the chain-able method name.

   Additionally, the following is effectively appended at the end:
   ``` text
    ///
    /// [`%f%`]: %t%::%f%
    /// [`%f%()`]: %t%::%f%
   ```

   This allows proper hyper-linking of ``[`%t%`]`` and ``[`%t%()`]``.

   There is a corresponding method-level *`doc`* option which will selectively override
   the value set here (or the default).

## `#[fluent_impl_opts]` Attribute Options
*(`inblock`, `non_public`, `skip`, `prefix`, `rename`, `name`, `doc`)*

Options passed to override block-level defaults, or set method-specific
configurations.

Unlike `#[fluent_impl]`, this attribute:
 1. Applies to methods instead of impl blocks.
 2. Can be passed multiple times to the same method if you please.

### Example

``` rust ignore
#[fluent_impl]
impl Simple {
    #[fluent_impl_opts(non_public, inblock)]
    #[fluent_impl_opts(prefix="chain_", rename="added_1")]
    fn add_1(&mut self) {
        // ...
    }
}
```

 ### Options

 #### Inherited

 * **`inblock`** (default: inherit)

   Set *`inblock`* for this specific method if it's not set for the block already.

 * **`non_public`** (default: inherit)

   Set *`non_public`* for this specific method if it's not set for the block already.

   This allows generating chaining methods for specific private methods, or
   partially public ones (e.g. `pub(crate)` methods).

 * **`prefix`** (default: inherit)

   Override the default, or the block value if set.

   * *`prefix`* is not allowed to be an empty string.
   * Method-specific *`prefix`* is not allowed to be set if *`name`*(see below) is set.

 * **`doc`** (default: inherit)

   Override the default, or the block value if set.

 #### Method Specific

 * **`skip`** (default: unset)

   Skip this method. Don't generate anything from it.

 * **`rename`** (default: chain-able name)

   The default chaining method name is the prefix appended by the chain-able method
   name. This option allows you to rename the name that gets added to the prefix.

   * *`rename`* is not allowed to be an empty string.
   * *`rename`* is not allowed to be set if *`name`*(see below) is set and vise versa.

 * **`name`** (default: unset)

   Set the name of the chaining method.

   * *`name`* is not allowed to be set if method-specific *`prefix`* or *`rename`* is set.


# Full Example

``` rust
#![feature(use_extern_macros)]

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
```
