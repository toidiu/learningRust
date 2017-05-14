fn main() {
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}",s);


    let s = String::from("hello");
    let s1 = s.clone();
    println!("{} {}", s, s1);

    let mut st = String::from("hello");
    {
      let blu = by_ref(&mut st);
      println!("{}", blu);
    }
    println!("{}", st);


    let s = get_part(&st[..]);
    println!("{}", s);
}


fn by_ref(s: &mut String) -> &String {
    s.push_str(" workd");
    s
}

fn get_part(s: &str) -> &str {
    let len = s.len() - 2;
    &s[4..len]
}
