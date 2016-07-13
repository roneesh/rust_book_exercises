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

    //Conclusions:
    //1. functions are easy, 
    //2. functions take types in parens()
    //3. functions return types with -> :type:
    //3. variables can point to functions

// 4.3 Primitive Types
    let x = true;
    let y : bool = false;
    let x = 'x';
    let two_hearts = '💕';
    println!("{}, {}, {}", x, y, two_hearts);

    let x = 42; // x has type of i32 now
    let y = 1.0; // y has type of f64 now

    let a = [1,2,3]; // array of types i32, 3 elements
    let mut m = [1,2,3]; //array of types i32, 3 elements, but it is mutable
    let a = [0; 20]; // arry of 20 i32 elements
    let a = [1,2,3];
    println!("a has {} elements", a.len());

    let names = ["Ron", "Carl"];
    println!("name 1 is: {}", names[1]);
    
    let a = [0,1,2,3]; //i32 arary of 4 len()
    let complete = &a[..];
    let middle = &a[1..4];
    println!("array slices complete, {:?} and middle {:?}", complete, middle);
    
    let tup = (1, "hello");
    let tup2 : (i32, &str) = (1, "hello");

    let (x,y,z) = (1, "hello", 3);
    println!("z is: {}", z);
    
    let tuppy = (1, 2, 3);

    let x = tuppy.0;
    let y = tuppy.1;
    let z = tuppy.2;

    println!("x is {}", x);

    //Conclusions:
    //1. char types are a single quote
    //2. isize and usize are signed and unsigned types that are appropriate for the architecture
    //3. A slice is a view into a piece of data, right now it seems you only slice arrays, but I'm sure that changes later on
    //4. Rust's str type is more primitive than it's char type? I guess so.
    //5 Read &str as a string slice
}
