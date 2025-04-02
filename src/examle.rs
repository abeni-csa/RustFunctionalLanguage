fn add_one_v1(x: u32) -> u32 {
    x + 1
}
pub fn call_me() {
    let add_one_v1 = |x: u32| -> u32 { x + 1 };
    let example_closure = |y| y;
    let str = example_closure(String::from("Some String"));
    let list = vec![1, 2, 3, 4];
    print!("Before defining the closure: {list:?}");
    let only_borrows = || println!("From clousure: {list:?}");
    println!("berfore calling clousure: {list:?}");
    only_borrows();
    println!("After calling clousure: {list:?}");
}
