use std::fmt::Display;

pub trait Summary {
    fn author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Similar articles by {}", self.author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn author(&self) -> String {
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn author(&self) -> String {
        format!("@{}", self.username)
    }
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}
// //takes a value that implements the summary trait
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// //similar to impl trait without syntactic sugar
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }
// //using + to add more trait bounds

// pub fn notify(item: &(impl Summary + Display)) {}
//this is cumbersome
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// //this is simpler
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where T: Display + Clone,
// U: Clone + Debug
// {}

struct Pair<T> {
    x:T,
    y:T,
}

impl <T> Pair<T>{
    fn new(x:T,y:T) -> Self{
        Self{x,y}
    }
}

impl <T: Display + PartialOrd> Pair<T>{
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x {}",self.x);
        }else{
            println!("The largest number is y {}",self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("you have 1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        author: String::from("Martin Thuranira"),
        content: String::from("Getting inot the weeds with rust"),
        location: "Nairobi".to_string(),
        headline: "Breaking News".to_string(),
    };
    println!("you have 1 new article :{}", article.summarize());
}
