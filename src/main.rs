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

// 4.4 Comments - SKIPPED!

//4.5 If - SKIPPED!

//4.6 Loops

    //commented out so program ends!
    // loop {
    //     println!("loop forever!")
    // }
    let mut x : i32 = 1;
    let mut done : bool = false;
    while !done {
        x = x + 1;
        if x == 21 {
            done = true;
        }
    }
    println!("x after loop is: {}", x);

    for (k, j) in (0..9).enumerate() {
        println!("x^2 is {}, called the {}th time", k*k, j);
    }

    let lines = "hello\nworld".lines();
    println!("lines: {:?}", lines);

    let linesEnum = lines.enumerate();
    println!("lines enum: {:?}", linesEnum);

    for (linenumber, line) in linesEnum {
        println!("{} : {}", linenumber, line);
    }

    let charsEnum = "hello world".chars().enumerate();
    for (charNumber, charItem) in charsEnum {
        println!("{} : {}", charNumber, charItem);
    }

    let mut blah = 5;
    loop {
        blah = blah + 1;
        if blah % 10 == 0 {
            break;
        }
    }
    println!("blah is: {}", blah);

    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // continues the loop over x
            if y % 2 == 0 { continue 'inner; } // continues the loop over y
            println!("x: {}, y: {}", x, y);
        }
    }

    //Conclusions:
    //1. Break loops early with break, skip ahead with continue;
    //2. The idiomatic for loop is a lot like Javascripts for/in syntax
    //3. Use .enumerate when you need to keep track of the index of the enumerated charItem

//4.7

    let v = vec![1,2,3,4,5];// v: Vec<i32>
    let v = vec![0; 10]; // ten i32 zeroes;

    println!("element 3 of v is : {}", v[3]);

    match v.get(7) {
        Some(x) => println!("{}", x),
        None => println!("Sorry this vector is too short!")
    }

    //Doesn't work!
    // for i in v {
    //     println!("Take ownership of the vector and its element {}", i);
    // }

    // for i in v {
    //     println!("Take ownership of the vector and its element {}", i);
    // }

    //works!
    for i in &v {
        println!("This is a reference to {}", i);
    }

    for i in &v {
        println!("This is a reference to {}", i);
    }

    //Conclusions:
    //1. Vectors also use bracket notation to get values
    //2. They are similar to arrays, but unbounded
    //3. You can use macro notation to create them
    //4. You can't directly refer to them more than once, otherwise too many blocks own it

//4.8 Ownership - 4.10 Lifetimes

    let v = 1; //i32

    let v2 = v; 

    //works because i32 has copy trait 
    println!("v is: {}", v);

    let a : i32 = 5; //i32 has copy trait
    let _y = double(a);

    fn double(x: i32) -> i32 {
        x * 2
    }

    let b : bool = false;
    let c : bool = flip(b);

    fn flip(x: bool) -> bool {
        !x
    }

    
    // This is the example in the book of the nasty function, that takes ownership of vectors and then gives them back
    fn boo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
        // do stuff with v1 and v2

        // hand back ownership, and the result of our function
        (v1, v2, 42)
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let (v1, v2, answer) = boo(v1, v2);
    // *END NASTY EXAMPLE*
    // So let's talk through nasty example...
    //1. We have a function foo, and it takes two Vectors, since Vectors don't do the copy trait, owernship is now transferred to this function
    //2. If we don't return the Vectors in our last line, those vectors can never again be used by the main() funciton which called foo()
    //3. So the function returns this tuple of (Vec1, Vec2, i32);
    //4. Our code thne, re-binds v1 and v2 in the tuple line (the line of `let (v1,v2,answer) = foo(v1,v2)`)
    //5. We don't want to have to do this in every function we write!! It would stink!
    //6. So instead we pass to foo two vectors that are "Borrowed" 

    // This is the good version, that uses Borrowing
    fn noo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        // do stuff with v1 and v2

        // return the answer
        42
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let answer = noo(&v1, &v2);
    // *END GOOD EXAMPLE*
    // So let's talk through good example...
    //1. we call foo with '&v1' and '&v2' args, so foo has borrowed v1 and v2
    //2. When it is done, it gives ownership back to the calling function
    //3. And the function then only has to return it's new data in our program, the i32, and doesn't have to return Vecs that get re-binded


    //An example of mutable Borrowing
    let mut x = 5;
    {
        let y = &mut x;
        *y = *y + 1;
    }
    println!("x is now {}", x);

    // Lifetimes are complicated...
    // 'a 'b are lifetimes a and lifetime b

    //Conclusions:
    //1. The reason ownership rules exist is because some data is so complex, it has to be stored between both the stack and the heap, meaning a stack data points to heap data, and if we have two variables with stack data pointing to the same heap data, then they are implicitly in a race condition, you can't access one without know if the other is accessing it first. 
    //2. Simple data like i32 or Booleans have the copy trait, which allows multiple variable bindings to own the data, because the data is so simple it can live only in the stack.
    //3. Borrowed data can't be mutated unless it's borrowed with `&mut`, vs regular `&`
    //4. Right now all I can make sense of with lifetimes is that they are defined with a 'name syntax, and they are used in function defintions

//4.11 Mutability - SKIPPED

//4.12 Structs

    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point {
        x: 0,
        y: 0
    };

    //we can access properties via dot, just like Javascript!
    println!("Origin at: {}, {}", origin.x, origin.y);

    let mut o2 = Point {
        x: 0,
        y: 1
    };
    o2.y = 2;
    let o2 = o2;

    println!("Origin at: {}, {}", o2.x, o2.y);

    // Have to include this compiler flag so Point3d gets a debug trait
    #[derive(Debug)]
    struct Point3d {
        x: i32,
        y: i32,
        z: i32
    }

    let mut point = Point3d {
        x:0, y:0, z:0
    };
    point = Point3d { y: 1, .. point};
    println!("Point3d: {:?}", point);

    // Tuple Structs
    struct Color(i32, i32, i32);
    {
        struct Point(i32, i32, i32);
        let origin = Point(0,0,0);
    }

    let black = Color(0,0,0);
    let white = Color(255,255,255);

    // Almost never use Tubple Structs!
    // Unless your struct has one piece of data only!

    struct Inches(i32);
    let length = Inches(10);

    struct Electron;
    let x = Electron;

    //Conclusions
    //1. A struct is perhaps most like a JS object,
    //   except maybe it's more like a class in Ruby?
    //2. Only the whole object can be mutable, not individual properties, which is good
    //3. Structs need to be given a debug trait to be logable!
    //4. Tuple structs and empty structs are rarer, generally go wtih plain structs!

//4.13 Enums

}
