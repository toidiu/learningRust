#![allow(unused)]

fn main() {}



//========== sub-typing
struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse<'b>(&'b self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
    // Ok(())
}


//========== lifetime bounds
struct Ref<'a, T: 'a>(&'a T);
struct StaticRef<T: 'static>(&'static T);


//========== trait obj lifetime bounds
fn trait_obj_lifetimes() {

    trait Foo { }
    struct Bar<'a> {
        x: &'a i32,
    }
    impl<'a> Foo for Bar<'a> { }

    let num = 5;
    //trait obj!
    let obj = Box::new(Bar { x: &num }) as Box<Foo>;
    let explicit_obj =  Box::new(Bar { x: &num }) as Box<Foo>;
}
