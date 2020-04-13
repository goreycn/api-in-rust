extern crate cc;

use cc::Hello;


#[derive(Hello)]
struct GG {
    #[nullable]
    #[size(1,3)]
    pub username: String,

    #[reg(r"^\d*$")]
    pub size: u32,
}

pub trait Hello {
    fn hi();
}

fn main() {
    GG::hi()
}
