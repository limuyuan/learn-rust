fn main() {
    let mut v: Vec<i32> = Vec::new();

    {
        let _v1 = vec![1, 2, 3];
    }

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(9);

    let _first = &v[0];

    v.push(10);

    // adding element to end of vec may also relocate full vector, so there's ownership
    // println!("{:?}", first);

    println!("The second element is: {}", &v[1]);
    match v.get(2) {
        None => println!("There's no element"),
        Some(third) => println!("The third element is: {}", third),
    }

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    println!("{:#?}", &v);

    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let ev = vec![
        SpreadSheetCell::Int(12345),
        SpreadSheetCell::Float(1.234),
        SpreadSheetCell::Text(String::from("Test")),
        SpreadSheetCell::Text(String::from("Just a test")),
    ];

    println!("{:#?}", ev);
}
