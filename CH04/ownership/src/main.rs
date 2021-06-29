fn main() {

    //println!("the value of s before curly bracket is: {}", s);

    {                      // s is not valid here, it’s not yet declared
        //let s = "hello";   // s is valid from this point forward


        let mut s = String::from("hello");
        // do stuff with 
        s.push_str(", world!"); // push_str() appends a literal to a String
        
       
        println!("the value of s in scope is: {}", s);
        takes_ownership(s);

        let mut x = 5;

        makes_copy(x);

        println!("the value of x after copy is: {}", x);
    
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

fn takes_ownership(some_string: String) { 
    println!("{}", some_string);
}

fn makes_copy(mut some_integer: i32) {
    some_integer += 1;
    println!("{}", some_integer);
}
