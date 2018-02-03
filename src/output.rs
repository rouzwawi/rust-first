pub trait OutputWriter: Sync {
    fn write(&self);
}

#[derive(Default)]
pub struct PlainOutput;
#[derive(Default)]
pub struct WildOutput;

pub static PLAIN: &'static OutputWriter = &PlainOutput;
pub static WILD: &'static OutputWriter = &WildOutput;

impl OutputWriter for PlainOutput {
    fn write(&self) {
        println!("Plain");
    }
}

impl OutputWriter for WildOutput {
    fn write(&self) {
        println!("Wild");
    }
}
