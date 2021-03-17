fn main() {
    let mut x = vec![1, 2, 3, 4, 5];
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &mut x {
        *i += 50;
        println!("x is {}", i)
    }
    //let out = vect_enum::row(2);
    let mut input = String::new();
    println!{"input a text"};
    std::io::stdin().read_line(&mut input).expect("Incorrect input");
    let out = vect_enum::input_to_vect(input);
    //println!("enum value is {:?}", out);

}

mod vect_enum {
    #[derive(Debug)]
    pub enum spread {
        num(i32),
        floater(f64),
        text(String),
    }
    pub fn input_to_vect(inset : String){
        let mut row = vec![
            spread::num(5),
            spread::floater(2.6),
            spread::text(inset),
        ];
        println!("output is {:?}",&row[2]);
    }
}
