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
