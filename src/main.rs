pub mod implementations;
mod generated_glue {
    include!(concat!(env!("OUT_DIR"), "/generated_glue.rs"));
}

use generated_glue::Object;

pub trait Methods {
    fn func(&self);
}

pub struct Methods_2;
impl Methods for Methods_2 {
    fn func(&self) {
        println!("baz");
    }
}

fn main() {
    Object::new(2).func();
}
