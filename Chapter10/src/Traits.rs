fn main() {
    let mut po = Point { x: 10, y : 20, z: 30};
    println!("{} \n{} \n{}", po.add(), po.sum(), po.mul());
    notify(po);
}

struct Point {
    pub y: i32,
    pub x: i32,
    pub z:i32,
}

pub trait Arithmetic {
    fn add(&self) -> String;
    fn sum(&self) -> String {
        String::from("Hey")
    }
    fn mul(&self) -> String {
        format!("So anyways, here's {}", self.add())
    }
    // Why this no work?
/*    fn div(&self) -> String {
        format!("@{}", self.x())
    }*/
}

impl Arithmetic for Point {
    fn add(&self) -> String {
        format!("{} + {} = {}", self.x, self.y, self.z)
    }
}

pub fn notify<T>(t: T) // Short of <T: Arithmetic>
    where T: Arithmetic
{
    println!("-- notify -- {:?}", t.add());
}
