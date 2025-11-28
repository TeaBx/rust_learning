fn main() {
    {   
        //This code will compile because r is immutable reference to s(ownership isn't moved), so both s and r can be used (any number of immutable references is allowed) 
        let s = String::from("hello");
        let r = &s;
        println!("{}", s);
        println!("{}", r); 
    }

    {   // This code won't compile because there are both mutable and immutable references to s(within the same lifetime), which is not allowed
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &mut s;
        println!("{}", r1);
    }

    {   // This code won't compile because there are 2 mutable references, only one mutable reference at a time is allowed
        let mut s = String::from("hello");
        let r1 = &mut s;
        let r2 = &mut s;
        println!("{}", r1);
    }

}
