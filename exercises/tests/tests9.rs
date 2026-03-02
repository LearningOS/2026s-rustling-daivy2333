// tests9.rs

extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    
    // Add this attribute to link the alias to the original function's symbol
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // Add this attribute to prevent name mangling so the symbol is exactly "my_demo_function"
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}