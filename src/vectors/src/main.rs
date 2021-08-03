fn main() {
    let mut v: Vec<i32> = Vec::new();

    {
        let _v1 = vec![1, 2, 3];
    }

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(9);

    //v.push(1);

    println!("{:?}", v);
}
