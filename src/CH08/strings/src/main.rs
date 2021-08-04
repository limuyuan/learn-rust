fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

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
}
