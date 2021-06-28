fn main() {

    //println!("the value of s before curly bracket is: {}", s);

    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
        println!("the value of s in scope is: {}", s);
    }                      // this scope is now over, and s is no longer valid

    //println!("the value of s after curly bracket is: {}", s);
}
