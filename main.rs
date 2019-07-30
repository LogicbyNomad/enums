
#[derive(Debug)]
pub struct Bed{
    size:i32,
    count:u32,  
}


#[derive(Debug)]
pub enum Room{
    Kitchen(i32),
    Bedroom(Bed),
    Lounge
}


fn main() {
    
    let t = Room::Bedroom(Bed{size:50, count:2});
    println!("Hello from the {:?}",t);

    match t{
        Room::Kitchen(n) => println!("The room is a kitchen with {} rooms", n),
        d=> println!("{:?}", d), 
    }
}