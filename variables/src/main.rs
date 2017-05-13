fn main() {

    let mut x = 5;
    println!("the valie of x is: {}", x);
    x = 6;
    println!("the valie of x is: {}", x);

    //shadow
    let y = "hi";
    println!("the valie of y is: {}", y);
    let y = "bye";
    println!("the valie of y is: {}", y);
    let y = 5;
    println!("the valie of y is: {}", y);


    let z: u32 = 4;
    println!("val of z: {}", z);

    //float
    let f = 2.2;
    let y: f32 = 4.4;
    println!("val of f: {} and val of y: {}", f, y);

    //other
    let t: bool = true;
    let tup = (1,2,3);
    let (x, y, z) = tup;
    println!("tup val is: {} {} {}", x, tup.1, tup.2);

    let arr = [4,5,6];
    println!("arr val is: {} {} {}", arr[0], arr[1], arr[2]);

    a_function(3);
    println!("{}", five(4));


    //loop and while loop
    let mut num = 6;
    while(num > 1) {
        num = num - 1;
        println!("{}", num);
    }

    //for each
    let arr = ["qwe","rty","uio"];
    for elem in arr.iter() {
        println!("{}", elem);
    }

     for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn a_function(a: u32) {
    println!("this nother one {}", a);
}

fn five(a: i32) -> i32 {
    a+1
}
