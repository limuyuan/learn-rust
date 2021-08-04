fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    println!("length of {} is {}.", &hello, hello.len());

    let hello = String::from("Dobrý den");
    println!("length of {} is {}.", &hello, hello.len());

    let hello = String::from("Hello");
    println!("length of {} is {}.", &hello, hello.len());

    let hello = String::from("שָׁלוֹם");
    println!("length of {} is {}.", &hello, hello.len());

    let hello = String::from("नमस्ते");
    println!("length of {} is {}.", &hello, hello.len());

    let hello = String::from("こんにちは");
    println!("length of {} is {}.", &hello, hello.len());

    let hello = String::from("안녕하세요");
    println!("length of {} is {}.", &hello, hello.len());

    let hello = String::from("你好");
    println!("length of {} is {}.", &hello, hello.len());

    let hello = String::from("Olá");
    println!("length of {} is {}.", &hello, hello.len());

    let hello = String::from("Здравствуйте");
    println!("length of {} is {}.", &hello, hello.len());

    let hello = String::from("Hola");
    println!("length of {} is {}.", &hello, hello.len());

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut lo = String::from("lo");
    lo.push('l');
    println!("{}", lo);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //s1 is moved here so cannot use s1 later
                       // println!("s1 is {}", s1);
    println!("s2 is {}, s3 is {}", s2, s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    //let c1 = s1 + "-" + &s2 + "-" + &s3;
    let c2 = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", c2);
    println!("{}{}{}", s1, s2, s3); //format! won't take ownership!

    let s1 = String::from("Hello");
    //let h = s1[0];
}
