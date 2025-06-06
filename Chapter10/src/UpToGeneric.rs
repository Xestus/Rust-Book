fn main()
{
    let mut ve = vec![1,2,8,2321,112];
    let inty = Name{x:"MEow", y: "BhaU", z:"A"};
    let stri = Name{x: 2, y: 4, z: 6};
    println!("{:?}",stri.new());
    let helow = inty.up(stri);
    println!("{:?}",helow.new());

    // let x = largest(&ve);
}

#[derive(Debug)]
struct Name<A,B,C> {
    x: A,
    y: B,
    z: C,
}

// Generic type speed == concrete/normal type speed due to MONOMORPHIZATION
impl <A,B,C> Name<A,B,C> {
    fn new(&self) -> (&A, &B, &C) {
        (&self.x, &self.y, &self.z)
    }
}

impl <A,B,C> Name<A,B,C> {
    fn up<P,Q,R>(self, other: Name<P,Q,R>) -> Name<A,Q,R> {
        Name {
            x: self.x,
            y: other.y,
            z: other.z
        }
    }
}



/*fn largest<T>(list: &[T]) -> T {
    let mut l = list[0];
    for &item in list.iter() {
        if item > l {
            l = item;
        }
    }
    l
}*/
/*
    let mut ve = vec![1,2,8,2321,112];
    let l = large(&ve); // 1

    fn large(vec: &[i32]) ->i32 { // 2
        let mut l = vec[0];
        for &number in vec.iter() {
            if number > l {
                l = number;
            }
        }
        l
    }
}*/

// While passing ownership to the function, the only datatype applicable is Vec<i32> but
// when we're borrowing the variable (&ve) in 1, we can write both &Vec<i32> and &[i32]
// but &i32 doesn't work.

    // &Vec[i32] -> reference of the whole `Vec` object (contains all of its info i.e. length, capacity))
    // &[i32] -> reference to just the list of values inside.
    // In most of the cases, &Vec[T] gets converted into &[T] automatically.
    // &i32 -> reference to just a single number. Hence, cannot be used.

    // Vec<i32> passed the whole `Vec` object.
    // [i32] doesn't work as all values in rust must have a known, fixed size at compile time.

// But, why is &[i32] allowed but not [i32] when both of them are slices?
    // Because [i32] doesn't have a known size at compile time but &[i32] has.
    // &[i32] -> passing a reference to a slice -> 2 metadatas -> pointer + length AKA fat pointer.
        // -> size is known (2 usize values)
// If &[i32] carries a pointer and length (2 × usize), why can’t [i32] be treated similarly?
    // ONLY REFERENCES (&[i32]) CARRY A LENGTH INFO, VALUES ([i32]) DOESN'T INCLUDE POINTER OR LENGTH
    // ITS A GROUP OF SOME UNKNOWN NUMBER OF DATATYPE `i32` (UNSIZED).



// But we can't have &[str] on the function argument while borrowing &str as follows:
    /*
    let mut str = vec!["Hello", "Hi", "Apple"];
    let B = big(&str);
    fn big(str: &[str] -> str {

    }
    */

// The vec![//values] are `&str` values, not `str`. So,Vec<&str>!= Vec<str>
// [str] -> unsized slice of string slice.
    // Why does i32([i32]) work but not str([str])?
    // Because `i32` is a sized type and slice([i32]) of sized type makes sense.
    // `str` -> primitive unsized type -> never use `str` by itself because its unsized
    //  and slice([str]) of unsized type doesn't make sense.
    // Its like saying “I want a list of raw strings, where each one is unsized.”
// &[str] -> Slice of unsized types
    // Its like saying “I want a reference to a slice, where each element is a str (unsized string).”
    // As rust needs to know size of each element in order to compute indexing, where next item start etc.
// &[&str] works because its a slice of string reference.
    // &str -> sized (16 bytes: pointer + length).
    // &[&str] is a reference to 16 byte chunks.



// Also learn the difference between arrangements of:
//  let B = big(****str****);
//  fn big(str:**** Vec<&str>)**** -> str {


//     let mut str = vec!["Hello", "Hi", "Apple"];
//     big(str);

// fn big(str: Vec<&str>) -> str {

//     let mut l = str[0]; ---- 1
        // Adding & before str[0] increases the number of "&" before "str" datatype a
        // s &str, &&str, &&&str ...

//     for &&number in str.iter() {
        // Adding & before number decreases the number of "&" upto a certain point and
        // starts increasing after the datatype of `number` becomes `str` before "str"
        // datatype as &&str, &str, str, &str, &&str. WHY?
//         if number > l {
//             l = number;
//         }
//     }
//     l
// }

// - Takes a Vec<&str> -> each element is already in reference (&str)
// For 1
    // let mut l = &str[0];
    // l:&&str -> reference to string slice.
// For 2
    // Calling an .iter() on Vec<&str> produces iteration that is references to each element.
// As each element is `&str`, the value generated by .iter is `&&str`.
// number -> number is in type of `&&str` -> No destructuring.
// &number -> number is in type of `&str` -> 1 Destructure.
// &&number -> number is in type of `str` -> 2 Destructure.
// &&&number -> number is in type of `&str`  -> Trying to 3 Destructure -> Only 2 Destructure exist -> Take one reference -> ERROR
// And so on....


