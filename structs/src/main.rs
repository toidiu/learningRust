fn main() {

    #[derive(Debug)]
    struct Cat {
        color: String,
        age: u32
    }

    impl Cat {
        fn meow(&self) {
            println!("{} colored cat goes 'meow'", self.color);
        }

        fn old_cat() -> Cat {
            Cat{color: String::from("brown"), age: 9000}
        }

        fn make_noise() {
            println!("meow");
        }
    }

    let dolche = Cat {
        color: String::from("blue"),
        age: 4
    };

    println!("{} {}", dolche.age, dolche.color);
    println!("{:?}", dolche);

    let old = Cat::old_cat();
    dolche.meow();
    old.meow();
    Cat::make_noise();
}
