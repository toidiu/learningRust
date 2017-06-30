fn main() {
    println!("Hello, world!");


    struct Context<'a>(&'a str);

    struct Parser<'c, 's: 'c> {
        context: &'c Context<'s>,
    }

    impl<'c, 's: 'c> Parser<'c, 's> {
        fn parse(&self) -> Result<&'s str, &'s str> {
//            Ok(&self.context.0[1..])
            Err(&self.context.0[1..])
        }
    }


    fn parse_ctx<'a>(context: Context<'a>) -> Result<&'a str, &'a str> {
        Parser{context: &context}.parse()
    }



    let ctx = Context("bla");
    let res = parse_ctx(ctx);
    match res {
        Ok(obj) => println!("success {:?}", obj),
        Err(er) => println!("fail {}", er),
    }
}
