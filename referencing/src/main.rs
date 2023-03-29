fn main() {
    let mut s1 = String::from("hello");

    change(&mut s1); // mutable reference
                     // you cannot have more than one mut ref to a value
                     // this prevents "data races" where two pointers point
                     // at the same data where one of them has access to write
                     // and there is no sync in place
                     // if you want you can always open a new scope with {} and create a new mut ref there like so:
                    /*  { 
                        let r1 = &mut s; 
                        }  
                    */
                    // we also CANNOT have and imut and mut reference to the same data
                    /*
                        let r1 = &s; // no problem
                        let r2 = &s; // no problem
                        let r3 = &mut s; // BIG PROBLEM
                    */
                     // having multiple imut refs are fine since they are read-only

    let len = calculate_length(&s1); // we are sending a reference -- ownership of s1 stays on main scope

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

/*
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

This blows up, references are immutable by default, you CANNOT change something that you have reference to
 */