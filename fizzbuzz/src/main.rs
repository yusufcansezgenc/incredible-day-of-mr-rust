fn main() {
    for number in 1..100 {
        decision(number);
    }  
}

fn decision(value: i32) {
    if value % 15 == 0 {
        println!("fizzbuzz");
    }
    else if value % 5 == 0 {
        println!("buzz");
    } 
    else if value % 3 == 0 {
        println!("fizz");
    } 
    else {
        println!("{value}");
    }
}
