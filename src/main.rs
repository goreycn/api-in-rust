extern crate cc;
extern crate bean_check;
extern crate bean_check_lib;
extern crate regex;

use regex::Regex;

use cc::Hello;
use bean_check::BeanCheck;
use bean_check_lib::BeanCheck;

#[derive(Hello)]
struct GG {
    #[nullable]
    #[size(1, 3)]
    pub username: String,

    #[reg(r"^\d*$")]
    pub size: u32,
}

pub trait Hello {
    fn hi();
}

#[derive(BeanCheck)]
struct UserDO {
    #[Range(1, 100)]
    pub age: u16,
    #[Length(1, 10)]
    pub username: String,
    #[Pattern(r"^\d{1,10}$")]
    pub password: String,
}

fn main() {
    // GG::hi()
    let u = UserDO {
        age: 20,
        username: "gorey".to_string(),
        password: "12a345".to_string()
    };

    // let t = UserDO::is_valid();
    let t = u.is_valid();
    println!("{}", t);

    // let t = String::from("hello");
    // let reg = r"^\d+$";
    // let min = 1;
    // let max = 10;
    //
    // let isa = { t.len() >= min && t.len() <= max };
    // let isb = { Regex::new(reg).unwrap().is_match(t.as_str()) };
    //
    // let ttt = { t.len() >= min && t.len() <= max } && { Regex::new(reg).unwrap().is_match(t.as_str()) };
    //
    // println!("--> {} : {}  ==> {}", isa, isb, ttt);
}
