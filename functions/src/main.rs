fn main() {
    println!("Hello, world!");
    
    let three_times = add_one(give_me_two_times());
    another_function("yay :3", three_times);
}

fn another_function(value: &str, times: i32) {
    println!("Another function with value: {value} times {times}");
}

fn give_me_two_times() -> i32 {
    println!("Giving it two times!");

    let two_times = 2;
    two_times
}

fn add_one(value: i32) -> i32 {
    println!("Adding one!");
    value + 1
}
