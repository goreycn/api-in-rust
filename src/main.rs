extern crate cc;

use cc::Hello;


#[derive(Hello)]
struct GG {
    pub username: String,
    pub size: u32,
}

pub trait Hello {
    fn hi();
}

fn main() {
    GG::hi()
}
