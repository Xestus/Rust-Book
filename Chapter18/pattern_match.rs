struct Point {
    x: i32,
    y: i32,
}
enum Hi {
    Meow(i32),
    Bhau(i32,i32),
}

fn main() {
/*    let x = Some(5);
    let y = 10;
    match x {
         Some(50) => println!("Got 50"),
         Some(y) => println!("Matched, y = {:?}", y),
         _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
*/

    let x = 1;
    match x {
        1 | 2 => println!("1"),
        3 ...7 => println!("2"),
        _ => println!("something else"),
    }

    let y = 'm';
    match y {
        'a' ... 'q' => println!("a"),
        'r' ... 'z' => println!("b"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 2 };
    let Point { x:a,y:b } = p; // =     let Point { x,y } = p;

    match p {
        Point { x, y: 0 } => println!("On x-axis: {}, {}", a, b),
        Point {x: 0,y} => println!("On y-axis:{}, {}", x, y),
        Point{x,y} => println!("Neither: {}, {}", x, y),
    }

    let h = Hi::Bhau(1,2);
    match h {
        Hi::Bhau(x,y) => println!("x: {}, y: {}", x, y),
        Hi::Meow(x) => println!("x: {}", x),
    }

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];
    let sum_of_squares: i32 = points
        .iter()// create an iteration
        .map(|&Point { x, y }| x * x + y * y)//destructure it and calculate distance
        .sum(); // add all of them
}

