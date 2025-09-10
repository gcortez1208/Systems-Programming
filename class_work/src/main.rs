fn main() {
    let x = 10;
    
    {
        let x = x + 10;
        println!("{}",x);
    }

    {
        let x = x + 10;
        println!("{}",x);
    }
    
    {
        let x = x + 10;
        println!("{}",x);
    }
    drop(x);
    println!("{}",x);
}
