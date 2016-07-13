fn main() {

// 4.1 Variable Bindings
	let x = 5;
	let (x, y, roneesh) = (1, 2, 3);

	let z : i32 = 5;

	let mut a = 5;
	a = 10;

	let b: i32;
	// won't work!
	// println!("the value of b is: {}", b);

	let c: i32 = 17;
	{
		let d : i32 = 3;
		println!("The value of c is {} and the value of d is {}", c, d);		
	}
	//won't work!	
	// println!("The value of c is {} and the value of d is {}", c, d);		


	let x : i32 = 8;
	{
		println!("{}", x);
		let x = 12;
		println!("{}", x);
	}
	println!("{}", x);
	let x = 42;
	println!("{}", x);

	let mut x: i32 = 1;
	x = 7;
	let x = x;

	let y = 4;
	let y = "I can also be bound to text!"; 

	//Conclusions:
	//1. I can re-define variables!
	//2. scope is heavily enforced!
	//3. i32 is a basic number type!

// 4.2 Functions

	fn foo() {

	}
    fn print_number(x: i32) {
        println!("x is: {}", x);
    }
    print_number(5);

    fn print_sum(x: i32, y:i32) {
        println!("sum is: {}", x + y);
    }    
    print_sum(5, 6);

    //won't work!
    //fn print_sum(x, y) { ... }

    fn add_one(x: i32) -> i32 {
        //won't work!
        // x + 1;
        //will work
        x + 1
    }
    add_one(10);

    {fn foo(x: i32) -> i32 {
        return x;

        // never runs
        x + 1
    }}

    //poor form (put in a block becasue you can't double define foo)
    {fn foo(x: i32) -> i32 {
        return x + 1;
    }}

    fn diverges() -> ! {
        panic!("This fn never returns!")
    }
    // diverges();

    // let x : i32 = diverges();
    // let x : String = diverges();
    // println!("{}", x);

    let f : fn(i32) -> i32;

    fn plus_one(i: i32) -> i32 {
        i + 1
    }

    let f: fn(i32) -> i32 = plus_one;
    let f = plus_one;
    let six = f(5);
}
