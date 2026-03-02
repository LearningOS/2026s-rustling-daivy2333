// Define the macro FIRST
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // Now the macro is known and can be used
    my_macro!();
}