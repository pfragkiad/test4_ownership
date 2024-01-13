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
    println!("{s3}");

    let sf = String::from("HEre is!!");
    let first: &str = &sf[..first_word(&sf)]; //slice! + borrowed reference
    println!("The first word is at: {}", first);

    let a: [i32; 4] = [1, 2, 3, 1];
    let ap: &i32 = &a[0];
    let ap2: &[i32] = &a[..];

    // let l: &str  = "H";
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

fn first_word(s: &str) -> usize {
    //THE ACTION OF CREATING A REFERENCE IS BORROWING! (&String)
    //reference to owned String

    let chars = s.chars();

    let mut _user1: User = build_user(String::from("someusername123"), String::from("someone@example.com"));

    _user1.username.push_str("KEFTEDSDS!!!");
    _user1.sign_in_count += 1;
    println!(
        "The name is {} and value is {}!",
        _user1.username, _user1.sign_in_count
    );

    let _user2 = User{ email:String::from("asdk@msad.com"), .._user1};

    //println!("{} HERE!!",_user1.email); //string was moved after _user2 assignment!!!!!
    //_user2.active = false; //immutable

    for (i, c) in chars.enumerate() {
        if c == ' ' {
            return i;
        }
    }

    s.len()

    //let bytes: &[u8] = s.as_bytes();
    // for (i, &item) in bytes.iter().enumerate() {
    //     if item == b' ' {
    //         return i;
    //     }
    // }
    // s.len()
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {

    struct Color(i32,i32,i32);
    let c1: Color = Color(1,2,23);
    
    println!("The color is {}!",c1.0); 

/* ttest commsad */
    struct KeftesEqual{}
    let _s1: KeftesEqual=KeftesEqual{};
    // let s2=KeftesEqual;
    // //println!("Are they same: {}", s1);
 

    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}



