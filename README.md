# elicit-rs

## Build Status

- [main](https://github.com/hanepjiv/elicit-rs/tree/main): [![CI Rust](https://github.com/hanepjiv/elicit-rs/actions/workflows/ci-rust.yml/badge.svg)](https://github.com/hanepjiv/elicit-rs/actions/workflows/ci-rust.yml)

# Examples

## Elicit

```
pub(crate) mod mine {
    use elicit::{elicit_define, Elicit};

    #[elicit_define(mine_elicit)]
    pub(crate) trait Mine {
    fn action(&self) -> i32;
    fn action_mut(&mut self) -> i32;
    }

    // pub(crate) mine_elicit::author as elicit_author;
    pub(crate) use mine_elicit::user as elicit_user;

    #[derive(Debug, Default, Clone, Elicit)]
    #[elicit_mod_author(mine_elicit::author)]
    pub(crate) struct X {}

    impl Mine for X {
    fn action(&self) -> i32 {
        0i32
    }
    fn action_mut(&mut self) -> i32 {
        0i32
    }
    }

    #[derive(Debug, Clone, Elicit)]
    #[elicit_mod_author(mine_elicit::author)]
    // #[elicit_from_self_field(_fsf)] // here
    pub(crate) struct Y {
    #[elicit_from_self_field] // or here
    _fsf: mine_elicit::author::ElicitFromSelfField,
    i: i32,
    }

    impl Y {
    pub(crate) fn new(a: i32) -> Self {
        Y {
        _fsf: Default::default(),
        i: a,
        }
    }
    }

    impl Mine for Y {
    fn action(&self) -> i32 {
        self.i
    }
    fn action_mut(&mut self) -> i32 {
        self.i += 1;
        self.i
    }
    }
}

pub(crate) fn fire() -> elicit::Result<()> {
    use mine::elicit_user::Elicit as MineElicit;
    use mine::{X, Y};

    let mut e: MineElicit;

    e = MineElicit::new(X::default())?;

    e.try_with(|m| -> elicit::Result<()> {
    println!("{:?}", m);
    assert!(m.action() == 0);
    Ok(())
    })?;

    let y = Y::new(1);
    e = MineElicit::new(y)?;

    e.try_with_mut(|m| -> elicit::Result<()> {
    println!("{:?}", m);
    assert!(m.action_mut() == 2);
    Ok(())
    })?;

    Ok(())
}

fire().expect("Doc-tests");
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE-2.0) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
