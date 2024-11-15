use crate::startup::startup;
pub mod startup;
mod window;

pub fn core(){
    startup();
    window::main();
}