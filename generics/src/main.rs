#[derive(Debug)]
struct point<T, Y> {
    x: T,
    y: Y,
}

struct point_self<T, Y> {
    x: T,
    y: Y,
}

impl point_self<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> point<T, U> {
    fn mixup<V, W>(self, other: point<V, W>) -> point<T, W> {
        point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let big = largest (&number_list);
    println!("larget number is: {}", big);
    //perform test that largets is 100
    assert_eq!(big, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let big = largest_i32(&number_list);
    println!("larget number is: {}", big);
    //perform test that largets is 100
    assert_eq!(*big, 6000);

    let char_list = vec!['m', 'a', 'r', 't', 'i', 'n'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let st = point { x: 't', y: 5 };
    println!("struct values are : {:#?}", st);

    let point_1 = point_self { x: 15.5, y: 12.9 };
    point_1.distance_from_origin();
    println!("point_1.x = {}, point_1.y = {}", point_1.x, point_1.y);

    let p1 = point { x: 5, y: 10.4 };
    let p2 = point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//generic function
fn largest<T : PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
