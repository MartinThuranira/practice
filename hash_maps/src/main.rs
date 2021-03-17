use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Mike"), 1);
    scores.insert(String::from("Molly"), 2);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let _scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    for (key, value) in &_scores {
        println!("{}: {}", key, value);
    }
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
