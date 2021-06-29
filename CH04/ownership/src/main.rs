fn main() {

    //println!("the value of s before curly bracket is: {}", s);

    {                      // s is not valid here, it’s not yet declared
        //let s = "hello";   // s is valid from this point forward


        let mut s = String::from("hello");
        // do stuff with s
    
        s.push_str(", world!"); // push_str() appends a literal to a String
        
        println!("the value of s in scope is: {}", s);
    
    
    }                      // this scope is now over, and s is no longer valid

    //println!("the value of s after curly bracket is: {}", s);
    //code_under_f4_3();
    clone_string();
}

fn code_under_f4_3 () {
    let s1 = String::from("hello");
    //let s2 = s1; move s1 to s2 here

    println!("{}, world!", s1);
}

fn clone_string() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    let x = 5;
    //let y = x.clone(); no need to use clone() for stack-only data
    let y = x;

    println!("s1 = {}, s2 = {}", s1, s2);
    println!("x = {}, y = {}", x, y);
}
