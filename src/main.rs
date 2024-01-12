//rust-analyzer
//for debugging : CodeLLdb
fn main() {
    //let s: &str = "Hello";
    let s: String = String::from("Hello");
    let mut s2 = s.clone(); //s is considered INVALID if there is no clone, after this call!! MOVE operation is here!
                            //s.push_str(" mpiftekia!");

    println!("{s2}");

    let e1 = &mut s2;
    let e2 = e1;

    //let mut s2 = "Hello"; //works too

    //https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

    println!("{}", s.len());

    println!("The size of the string is: {}!", calc2(&mut s2));

    let s3: String = calc_length(s2);
    println!("{s3}")
}

fn calc_length(s: String) -> String {
    // s.len()
    // let mut s2  = s;
    // s2.push_str("KEFTES!");
    s
    // s2
}

fn calc2(s: &mut String) -> usize {
    s.push_str("KEFTES"); //we can only change mutable references (s: &mut String)
    s.len()
}
