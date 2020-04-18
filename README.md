api-in-rust
===========
api-in-rust is a rust http api server project.

Whom is this project for
------------------------
Any developer who wants to develop api server in Rust language.

Requirements
------------
currently use rust version 1.42.0

Installing
----------

If you want to use BeanCheck util:

```toml
[dependencies]
bean_check = { path = "bean_check"}
bean_check_lib = { path = "bean_check_lib"}
regex = "1"
```

and white code in your main.rs
```rs
extern crate bean_check;
extern crate bean_check_lib;
extern crate regex;

use regex::Regex;
use bean_check::BeanCheck;
use bean_check_lib::BeanCheck;
use bean_check_lib::CheckError;
use std::str::FromStr;

#[derive(BeanCheck)]
struct UserDO {
    #[Min(30)]
    pub min_age: u32,
    #[Max(20)]
    #[Range(1, 100)]
    pub age: u16,
    #[Length(1, 13)]
    pub username: String,
    #[Pattern(r"^\d{1,5}$")]
    pub password: String,
    #[Email]
    pub email: String,
    pub mobile: String,
}

fn main() {
    let u = UserDO {
        min_age: 325,
        age: 20,
        username: "gorey".to_string(),
        password: "12345".to_string(),
        email: "aa@qq.com".to_string(),
        mobile: "13812341234".to_string()
    };

    match u.validate() {
        Ok(_) => { println!("check pass "); },
        Err(e) => { println!("{}", e); },
    }
}
```

Debug Macro
-----------
You can install cargo-expand util, to see the pre-compiled code for macros.

```shell script
cargo install cargo-expand
# run command on main project where Cargo.toml located
cargo expand
```

#### TODO
---------
[x] Bean Check Macro
[x] Param Bean Sign check
[ ] Log
[ ] MySQL handle
[ ] Redis handle


#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

