fn main() {
    let _hello_arabic = String::from("السلام‬");
    let _hello_polish = String::from("Dobrý den");
    let _hello_eng = String::from("Hello");
    let _hello_amharic = String::from("‫לוֹם‬ ָ‫;)שׁ‬");
    let _hello_hindi = String::from("नम े ");
    let _hello_japanese = String::from("こんにちは");
    let _hello_korean = String::from("안녕하세요");
    let _hello_chinese = String::from("你好");
    let _hello_spanish = String::from("Olá");
    let _hello_russian = String::from("Здравствуйте");
    let _hello_sp = String::from("Hola");

    let s = format!("{}-{}-{}", _hello_eng, _hello_amharic, _hello_japanese);
    println!("{}", s);
    for c in _hello_russian.chars() {
        println!("{}", c);
    }
    for b in _hello_chinese.bytes() {
        println!("{}", b);
    }
}
