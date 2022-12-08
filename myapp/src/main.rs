use mylib::some_module::*;
use myotherlib::another_module::*;

fn main() {
    let x = mylib::some_module::testing_add(1, 2);
    let y = testing_add(3,4);

    let a = myotherlib::add(1,3);
    let b = another_add_fn(7,8);
    println!("Hello, world! (myapp) {x} {y} {a} {b}");
}
