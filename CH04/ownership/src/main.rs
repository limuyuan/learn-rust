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
    code_under_f4_3();
}

fn code_under_f4_3 () {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
