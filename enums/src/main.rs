#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {

    struct Cat {
        sound: String,
        age: u32
    }

    struct Dog {
        sound: String,
    }

    enum AnimalKind {
        CatKind(Cat),
        DogKind(Dog),
    }

    //============Match
    fn make_sound(kind: AnimalKind) {
        match kind {
            AnimalKind::CatKind(c) => println!("{}", c.sound),
            AnimalKind::DogKind(d) => println!("{}", d.sound),
        }
    }

    let cat = Cat{ sound: String::from("meow"), age: 8 };
    let kitty = AnimalKind::CatKind(cat);
    make_sound(kitty);

    //============Option
    let cat1 = Cat{ sound: String::from("meow"), age: 8 };
    let cat_in_box1 = Some(cat1);
    let cat_in_box2: Option<Cat> = None;

    match cat_in_box1 {
        None => println!("dead"),
        Some(c) => println!("alive"),
    }
    match cat_in_box2 {
        None => println!("dead"),
        Some(c) => println!("alive"),
    }

    //===========if let
    let cat2 = Cat{ sound: String::from("meow"), age: 8 };
    let cat_in_box3 = Some(cat2);
    if let Some(kat) = cat_in_box3 {
        println!("hi kitty {}", kat.age);
    } else {
        println!("not there");
    }


    //=======Test ownership
    struct Circle{rad: u32}
    enum Sides {
        One(Circle),
    }

    let cir = Circle{rad: 2};

    {
        let one = Sides::One(cir);
    }

}
