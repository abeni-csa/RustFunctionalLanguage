fn add_one_v1(x: u32) -> u32 {
    x + 1
}
pub fn call_me() {
    let _add_one_v1 = |x: u32| -> u32 { x + 1 };
    let example_closure = |y| y;
    let _str = example_closure(String::from("Some String"));
    let mut list = vec![1, 2, 3, 4];
    add_one_v1(1);
    print!("Before defining the closure: {list:?}");
    let mut borrows_mutable = || list.push(7);
    borrows_mutable();
    println!("After calling clousure: {list:?}");
}
