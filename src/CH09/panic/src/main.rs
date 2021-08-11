use std::fs::File;

fn main() {
    //panic!("Hello, world!");

    //let v = vec![1, 2, 3];
    // v[99];
    
    let f = File::open("test");

    let f = match f {
        Ok(file) => file,
        Err(err) => panic!("Problem opening the file: {:#?}", err),
    };
}
