fn main() {
    let ac = 3;
    println!("Hello, {}!", ll(&ac));
    print!("{}", ac);
}

fn ll(lx: &u32) -> String{
    lx.to_string()
}