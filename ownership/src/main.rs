fn main() {
    let mut s = String::from("Hello, ");
    s.push_str("world!");

    println!("{}", s);

    /*let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);*/ 

    //s1 and s2 are pointing at the same String on heap 
    //this code blows up because Rust actively tries to run away from "double free" errors, 
    //the moment s2 switch happens s1 is invalid

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    //clone() simply "deep copy" the entire hello string on heap to s2 we now have two hellos and two pointers pointing at it. 
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}