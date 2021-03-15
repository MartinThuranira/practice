fn main() {
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);
    // }
    // fn calculate_length(s: &String) -> usize {
    // s.len()
    //}

    //string slices

    //let s = String::from("My name is @the_new_guy");
    //let slice = &s[0..2];
    //let slice = &s[..2];
    // let len = s.len();
    // let slice = &s[3..len];

    // println!("{}", slice);

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
        }

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 5,
        };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        //active: user1.active,
        //sign_in_count: user1.sign_in_count,
        ..user1
        };

        println!("user {} has signed in {} times", user2.username,user2.sign_in_count);
    }
