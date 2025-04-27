//////////////// HASHMAPS //////////////////

use std::collections::HashMap;

fn main() {
    let mut a = HashMap::new();
    let ae = String::from("Water");
    let be = String::from("NotWater");
    a.insert(ae,be);
    a.insert(String::from("Water"), String::from("Wa'or")); // Overwrites the value
    // variable ae and be are unusable because hashmap "a" takes the ownership.
    // Borrowing isn't commonly done -> Variables out of scope -> Dangling Pointer

    let apple = String::from("Water");
    let sc = a.get(&apple); // Takes value associated with string "apple" from hashmap.
    println!("{:?}",sc); // Option<T> -> If no match, return none.

    a.entry(String::from("Water")).or_insert(String::from("Meowh"));
    a.entry(String::from("Blue")).or_insert(String::from("WoofWoof"));
    println!("{:?}", a); // Checks the existence of string,
    // if the string doesn't exist, add its value, if it does ignore it.
}



/////////// STRINGS //////////////
/*fn main() {

    let mut s1 = String::from("foo");
    let mut s1c = &s1;
    let s2 = 'l';
    s1.push(s2);
    let s2 = "ish";
    s1.push_str(s2);

    // char has "push" but str has "push_str" as it prevents surprise behaviour and appending "char" is 4 bit
    // max but appending a "string" requires calculation of whole slice length and copy multiple bytes.

    let mut s3 = String::from("lo");
    let mut s4 = s1 + "-" + &s3;
    let s4 = format!("{s1c} - {s3}");

   let data = "meow"; // a reference of string literal in binary (not a heap)
    let mut s = String::new();
     let s = data.to_string(); // allocates heap memory and copies content of "data".
    let s = "meow".to_string(); // allocates heap memory and w/o binding content of "data".
    let s = String::from("hello"); // Same as above step but prefer it -> explicit -> less error.


    let helow = "Здравствуйте";
    let s = &helow[0..4]; // Works -> each letter - 2 bytes -> 0-3 = 4 bytes = 2 chars.
    let s = &helow[0..1]; // Panic -> 0-0 = 1 byte = 0.5 char

    for s in helow.chars() { // .chars return unicode || .bytes return raw byte
        println!("{}", s);
    }

}*/

/////// VECTOR ///////////////
/*fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immutable borrow
    v.push(6); // mutable borrow.
    // println!("{:?}", first); // If not used, rust's NLL will drop variable first early.
    // Error:
    // Hey, first is still alive and you are trying to modify v (push) —
    // what if v reallocates memory under the hood? first would be pointing to garbage

    for i in &mut v {
        *i +=*i;
        println!("{:?}", i);
    }
    enum Spreadsheet {
    Int(i32),
    Float(f64),
    Text(String),
    }

    let a = vec!{Spreadsheet::Int(2), Spreadsheet::Float(2.0), Spreadsheet::Text(String::from("Hello!"))};

    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    let mut v2 = vec![1, 2, 3];
    v2.push(2);
    let b : Option<&i32> = v2.get(2); // Index out of bound will return "none"

    let a: &i32 = &v2[2]; // Panic at index out of bound
    // a is a memory reference of v2[2] or its pointing to the variable v2[2].
    let a1: i32 = v2[2]; // v2[2] still has original value and a1 has copy of v2[2]
    // because we're using "i32" and i32 is a COPY trait. COPY trait -> BTS moving the value is copying the value.
    // a is a copy of v2[2] and v2[2] can be modified without changing a.

}
*/
// Copy trait is a duplication of original value as its simple, fast and no memory complexity.

// Integers have copy because integers have fixed size, **stack-owned** and
// duplicating integers means wasting few bytes (very cheap).

// String & other **heap-owned** doesn't have copy because copying a string will create
// 2 pointers pointing on the same address, dropping both at ones = bad.
