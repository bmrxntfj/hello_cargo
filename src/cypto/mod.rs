mod inaccessible;
pub mod nested;
pub fn function() {
    println!("called `cypto::function()`");
}

fn private_function() {
    println!("called `cypto::private_function()`");
}

pub fn indirect_access() {
    print!("called `cypto::indirect_access()`, that\n> ");

    private_function();
}

pub fn test_mod(){
    function();
    indirect_access();
    private_function();
}