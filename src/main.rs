
fn main() {
    println!("Hello, world!");
    let z = add_numbers(45, 55);
    println!("Anwser is {z}");
    let  h = 90;
    let h = h + 1;
    println!("{h}");
    let mut s = String::from("MTCSan Francisco");
    let word = first_word(&s);
    println!("The value is {}",word);
    s.clear();
}

fn add_numbers(x: u32,y: u32)-> u32{
    x + y
}
fn first_word(s: &String) -> &str {

    let bytes = s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}


