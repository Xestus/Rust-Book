// LIFETIME ANNOTATION IS REQUIRED AS RUST DOESN'T KNOW HOW LONG A REFERENCE WILL LIVE.
// W/O LA RUST WOULDN'T KNOW WHERE THE &str IN THE STRUCT BELOW COMES FROM.
struct ImportantExcerpt<'a> {
    // LA IS NEEDED IF YOU NEED TO STORE A REFERENCE IN STRUCT.
    part: &'a str,
}


fn main() {
    let str = String::from("Hello, world!");
    {
        let s2 = "xyz";

        let r = longest(str.as_str(), s2);
        println!("{}", r);
    }

    let aeiou = String::from("Hello, world!");
    let char = ImportantExcerpt {part: aeiou.as_str()};

    let s: &'static str = "I have a static lifetime."; // Instantiated for whole program.

}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // Both the inputs and return value live at least as long as 'a.
    // We're basically returning s1 or s2, whichever lasts longer.
    if s1.len() > s2.len() {
         s1
    }
    else {
         s2
    }
}


// 3 Lifetime Elision Rules

// 1-  If a function takes multiple reference parameter, assume each one has different lifetime.
    // fn A(x: &i32, y: i32) <=> fn A<'a,'b> (x: &'a i32, y: &b i32)

// 2- If there's only one input reference, its lifetime is the return lifetime.
    // fn A(x: &i32) -> &i32 <=> fn A<'a>(x : &'a i32) -> &'a i32

// 3- If there is a self reference, return uses self's lifetime.
    // fn A(&self) -> &str <=> fn A<'a>(&'a self) -> &'a str
