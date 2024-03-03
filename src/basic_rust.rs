pub fn run() {
    stdout();
    vbcs();
    print_sum(1, 2);
    primitive_dt();
    operation();
    control_flow();
    vector();
    structt();
    tuple_struct();
    unit_struct();
    enums();
    generic_in_action();
}

fn stdout () {
    // Print text to the console.
    println!("Hello World!");
    println!("{}, {}!", "Hello", "world"); // Hello, world!
    println!("{0}, {1}!", "Hello", "world"); // Hello, world!
    println!("{greeting}, {name}!", greeting = "Hello", name = "world"); // Hello, world!
    let (greeting, name) = ("Hello", "world");
    println!("{greeting}, {name}!"); // Hello, world!
    // 🔎 The format! macro is used to store the formatted string.
    let x = format!("{}, {}!", "Hello", "world");
    println!("{}", x); // Hello, world!
    // 💡 Rust has a print!() macro as well
    print!("Hello, world!"); // Without new line
    println!(); // A new line
    print!("Hello, world!\n"); // With new line
}

fn vbcs() {
    // variable binding, constant, static
    // let keyword is used in binding expressions.
    let a = true;
    let b: bool = true;
    let (x, y) = (1, 2);
    let mut z = 5;
    println!("{}", z);
    z = 6;
    // const keyword is used to define constants.
    // It lives for the entire lifetime of a program but have no fixed address in memory.
    const N: i32 = 5;
    // static keyword is used to define ‘global variable’ type facility.
    // There is only one instance for each value, and it’s at a fixed location in memory.
    // Usually statics are placed at top of the code file, outside the functions.
    static M: i32 = 15;
    // Always use const, instead of static.
    // It’s pretty rare that you actually want a memory location associated with your constant,
    // and using a const allows for optimizations like constant propagation
    // not only in your crate but also in downstream crates.
    println!("{} {} {} {} {} {} {}", a, b, x, y, z, N, M);
}

fn print_sum(a: i8, b: i8) {
    println!("sum one is: {}", sum_one(a, b));
    println!("sum two is: {}", sum_two(a, b));
    //️ Function pointers, Usage as a Data Type
    let c = sum_one;
    let d = c(a, b);
    println!("sum one pointer: {}", d);
    //same, with type inference
    let c: fn(i8, i8) -> i8 = sum_one;
    let d = c(a, b);
    println!("sum one inline: {}", d);
    // Function pointers, Usage as a Data Type
    let x = sum_one;
    println!("{}", sum_three(x, 2, 4)); // 8
}

fn sum_one(a: i8, b: i8) -> i8 {
    // There is no ending ; in the above line.
    // It means this is an expression which equals to `return a+1;`
    a + b
}

fn sum_two(a: i8, b: i8) -> i8 {
    //return a+2 but bad practice,
    //should use only on conditional returnes, except it's last expression
    return a + b;
}

fn sum_three(b: fn(i8, i8) -> i8, x: i8, y: i8) -> i8 {
    b(x, b(x, y))
}

fn primitive_dt() {
    // boolean
    let x = true;
    let y: bool = false;
    println!("{} {}", x, y);
    // char
    let x = 'x';
    let y = '😎';
    println!("{} {}", x, y);
    // i8, i16, i32, i64, i128 : fixed size(bit) signed(+/-) integer types
    // i8 : -128 to 127
    // i16 : -32,768 to 32,767
    // i32 : -2,147,483,648 to 2,147,483,647
    // i64 : -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
    // i128 : -170,141,183,460,469,231,731,687,303,715,884,105,728 to 170,141,183,460,469,231,731,687,303,715,884,105,727
    //
    // u8, u16, u32, u64, u128 : fixed size(bit) unsigned(+) integer types
    // u8 : 0 to 255
    // u16 : 0 to 65,535
    // u32 : 0 to 4,294,967,295
    // u64 : 0 to 18,446,744,073,709,551,615
    // u128 : 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455
    // isize, usize : fixed size(bit) signed(+) or unsigned(+) integer types
    //
    // isize, usize : pointer sized signed and unsigned integer types
    //      The actual bit size depends on the computer architecture
    //      you are compiling your program for.
    //      By default, the sizes are equal to 32 bit on
    //      32-bit platforms and 64 bit on 64-bit platforms.
    //
    // max and min values
    println!("{} {}", std::i8::MIN, u8::max_value());

    // f32, f64 : fixed size(bit) floating point types
    // f32 : single precision
    // f64 : double precision
    // max and min values
    println!("{} {}", std::f32::MIN, f64::MAX);

    // arrays
    let a = [1, 2, 3];
    println!("{:?}", a); // [1, 2, 3]
    println!("{:#?}", a); /*
        [
            1,
            2,
            3
        ]
    */
    let b: [i32; 0] = []; //empty array
    println!("{:?}", b);
    // [Type; NO of elements] let c: [int; 3] = [1, 2, 3];
    // str : unsized UTF-8 sequence of Unicode string slices
    // str is an immutable/statically allocated slice holding an unknown sized sequence
    // of UTF-8 code points stored in somewhere in memory.
    // &str can be used to borrow and assign the whole array to the given variable binding.
    let d: [&str; 3] = ["my value", "my value", "my value"];
    println!("{:?}", d);
    // A String is a heap-allocated string.
    // This string is growable, and is also guaranteed to be UTF-8.
    // They are commonly created by converting from a string slice using
    // the to_string() and String::from() methods.
    // ex: “Hello”.to_string();, String::from(“Hello”);
    let d: [String; 3] = [
        String::from("my value"),
        String::from("my value"),
        "my value".to_string(),
    ];
    //  In general, you should use String when you need ownership,
    //  and &str when you just need to borrow a string.

    println!("{:?}", d);
    // Arrays are immutable by default and also even with mut,
    // its element count can not be changed.
    //  If you are looking for a dynamic/ growable array, you can use Vec.
    // Vectors can contain any type of elements but all elements must be in the same data type.

    // tuples: fixed-size ordered list of elements of different(or same) data types
    let ta = (1, 1.5, true, 'a', "Hello, world!");
    println!("{:?}", ta);
    println!("{:#?}", ta);
    let tb: (i32, f64) = (1, 1.5);
    let (c, d) = tb;
    println!("{} {}", c, d);
    // Tuples are also immutable by default and even with mut,
    // its element count can not be changed.
    // Also if you want to change an element’s value,
    // new value should have the same data type of previous value.

    // slice : dynamically-sized reference to another data structure
    // Think you want to get/pass a part of an array or any other data structure.
    // Instead of copy it to another array (or same data structure),
    // Rust allows to create a view/reference to access only that part of data.
    // And it can be mutable or not.
    let a: [i32; 4] = [1, 2, 3, 4];//Parent Array
    println!("{:?}", a);
    let b: &[i32] = &a; //Slicing whole array
    println!("{:?}", b);
    let c = &a[0..4]; // From 0th position to 4th(excluding)
    println!("{:?}", c);
    let d = &a[..]; //Slicing whole array
    println!("{:?}", d);
    let e = &a[1..3]; //[2, 3]
    println!("{:?}", e);
    let e = &a[1..]; //[2, 3, 4]
    println!("{:?}", e);
    let e = &a[..3]; //[1, 2, 3]
    println!("{:?}", e);

}

fn operation() {
    let a = 5;
    println!("{}", a);
    let b = a + 1; //6
    println!("{}", b);
    let c = a - 1; //4
    println!("{}", c);
    let d = a * 2; //10
    println!("{}", d);
    let e = a / 2; // ⭐️ 2 not 2.5
    println!("{}", e);
    let f = a % 2; //1
    println!("{}", f);
    let g = 5.0 / 2.0; //2.5
    println!("{}", g);
    // Comparison Operators : == != < > <= >=
    let c = a == b; //false
    println!("{}", c);
    let d = a != b; //true
    println!("{}", d);
    let e = a < b; //true
    println!("{}", e);
    let f = a > b; //false
    println!("{}", f);
    let g = a <= a; //true
    println!("{}", g);
    let h = a >= a; //true
    println!("{}", h);
    let i = true > false; //true
    println!("{}", i);
    let j = 'a' > 'A'; //true
    println!("{}", j);
    // Logical Operators : ! && ||
    let a = true;
    let b = false;
    let c = !a; //false
    println!("{}", c);
    let d = a && b; //false
    println!("{}", d);
    let e = a || b; //true
    println!("{}", e);
    // On integer types,
    // ! inverts the individual bits in
    // the two’s complement representation of the value.
    let a = !-2; //1
    println!("{}", a);
    let b = !-1; //0
    println!("{}", b);
    let c = !0; //-1
    println!("{}", c);
    let d = !1; //-2
    println!("{}", d);
    // Bitwise Operators : & | ^ << >>
    let a = 1;
    let b = 2;
    let c = a & b; //0  (01 && 10 -> 00)
    println!("{}", c);
    let d = a | b; //3  (01 || 10 -> 11)
    println!("{}", d);
    let e = a ^ b; //3  (01 != 10 -> 11)
    println!("{}", e);
    let f = a << b; //4  (add 2 positions to the end -> '01'+'00' -> 100)
    println!("{}", f);
    let g = a >> a; //0  (remove 2 positions from the end -> o̶1̶ -> 0)
    println!("{}", g);
    // Assignment and Compound Assignment Operators
    // The = operator is used to assign a name to a value or a function.
    // Compound Assignment Operators are created by composing one of:
    //  + - * / % & | ^ << >> operators with = operator.
    let mut a = 2;
    a += 5; //2 + 5 = 7
    println!("{}", a);
    a -= 2; //7 - 2 = 5
    println!("{}", a);
    a *= 5; //5 * 5 = 25
    println!("{}", a);
    a /= 2; //25 / 2 = 12 not 12.5
    println!("{}", a);
    a %= 5; //12 % 5 = 2
    println!("{}", a);
    a &= 2; //10 && 10 -> 10 -> 2
    println!("{}", a);
    a |= 5; //010 || 101 -> 111 -> 7
    println!("{}", a);
    a ^= 2; //111 != 010 -> 101 -> 5
    println!("{}", a);
    a <<= 1; //'101'+'0' -> 1010 -> 10
    println!("{}", a);
    a >>= 2; //101̶0̶ -> 10 -> 2
    println!("{}", a);
    // Type Casting Operator : as
    let a = 15;
    let b = (a as f64) / 2.0; //7.5
    println!("{}", b);
}

fn control_flow() {
    // Simplest Example
    let team_size = 7;
    if team_size < 5 {
        println!("Small");
    } else if team_size < 10 {
        println!("Medium");
    } else {
        println!("Large");
    }
    // partially refactored code
    let team_size = 7;
    let team_size_in_text;
    if team_size < 5 {
        team_size_in_text = "Small";
    } else if team_size < 10 {
        team_size_in_text = "Medium";
    } else {
        team_size_in_text = "Large";
    }
    println!("Current team size : {}", team_size_in_text);
    //optimistic code
    let team_size = 10;
    let team_size_in_text = if team_size < 5 {
        "Small" //⭐️no ;
    } else if team_size < 10 {
        "Medium"
    } else {
        "Large"
    };
    println!("Current team size : {}", team_size_in_text);
    // inline if else
    let is_below_eighteen = if team_size < 18 { true } else { false };
    println!("Is below eighteen : {}", is_below_eighteen);
    // Return data type should be same on each block, when using this as an expression.
    // match
    let tshirt_width = 20;
    let tshirt_size = match tshirt_width {
        16 => "S", // check 16
        17 | 18 => "M", // check 17 and 18
        // 19...21 deprecated move to ..=
        19..=21 => "L", // check from 19 to 21 (19,20,21)
        22 => "XL",
        _ => "Not Available",
    };
    println!("{}", tshirt_size); // L

    let is_allowed = false;
    let list_type = match is_allowed {
        true => "Full",
        false => "Restricted"
        // no default/ _ condition can be skipped
        // Because data type of is_allowed is boolean and all possibilities checked on conditions
    };
    println!("{}", list_type); // Restricted

    let marks_paper_a: u8 = 25;
    let marks_paper_b: u8 = 30;
    let output = match (marks_paper_a, marks_paper_b) {
        (50, 50) => "Full marks for both papers",
        (50, _) => "Full marks for paper A",
        (_, 50) => "Full marks for paper B",
        (x, y) if x > 25 && y > 25 => "Good",
        (_, _) => "Work hard"
    };
    println!("{}", output); // Work hard

    // while
    let mut a = 1;
    while a <= 10 {
        println!("Current value : {}", a);
        a += 1; //no ++ or -- in Rust
    }
    // Usage of break and continue
    let mut b = 0;
    while b < 5 {
    	if b == 0 {
    		println!("while Skip value : {}", b);
    		b += 1;
    		continue;
    	} else if b == 2 {
    		println!("while Break At : {}", b);
    		break;
    	}
    	println!("while Current value : {}", b);
    	b += 1;
    }
    // Outer break
    let mut c1 = 1;
        'outer_while: while c1 < 6 { //set label outer_while
    	let mut c2 = 1;
    	'inner_while: while c2 < 6 {
    		println!("while Current Value : [{}][{}]", c1, c2);
    		if c1 == 2 && c2 == 2 { break 'outer_while; } //kill outer_while
    		c2 += 1;
    	}
    	c1 += 1;
    }
    // loop
    // forever loop
    // loop {
    //    println!("Loop forever!");
    // }
    //
    // Usage of break and continue
    let mut a = 0;
    loop {
    	if a == 0 {
    		println!("Loop Skip Value : {}", a);
    		a += 1;
    		continue;
    	} else if a == 2 {
    		println!("Loop Break At : {}", a);
    		break;
    	}
    	println!("Loop Current Value : {}", a);
    	a += 1;
    }
    // Outer break
    let mut b1 = 1;
    'outer_loop: loop { //set label outer_loop
      let mut b2 = 1;
      'inner_loop: loop {
        println!("Loop Current Value : [{}][{}]", b1, b2);
        if b1 == 2 && b2 == 2 {
            break 'outer_loop; // kill outer_loop
        } else if b2 == 5 {
        	break;
        }
        b2 += 1;
      }
      b1 += 1;
    }
    // for
    for a in 0..10 { //(a = 0; a <10; a++) // 0 to 10(exclusive)
      println!("For Current value : {}", a);
    }
    // Usage of break and continue
    for b in 0..6 {
      if b == 0 {
        println!("For Skip Value : {}", b);
        continue;
      } else if b == 2 {
        println!("For Break At : {}", b);
        break;
      }
      println!("For Current value : {}", b);
    }
    // Outer break
    'outer_for: for c1 in 1..6 { //set label outer_for
      'inner_for: for c2 in 1..6 {
        println!("Current Value : [{}][{}]", c1, c2);
        if c1 == 2 && c2 == 2 { break 'outer_for; } //kill outer_for
      }
    }
    // Working with arrays/vectors
    let group : [&str; 4] = ["Mark", "Larry", "Bill", "Steve"];
    for n in 0..group.len() { //group.len() = 4 -> 0..4 👎 check group.len()on each iteration
      println!("For Current Person : {}", group[n]);
    }
    for person in group.iter() { //👍 group.iter() turn the array into a simple iterator
      println!("For Current Person : {}", person);
    }
}

fn vector() {
    // Vector is kind of a re-sizable array but all elements must be in the same type.
    // It is a generic type, written as Vec<T> . T can have any type,
    // ex. The type of a Vec of i32s is Vec<i32> .
    // Also Vectors always allocate their data in dynamically allocated heap.
    //Creating vectors - 2 ways
    // let mut a = Vec::new(); //1.with new() keyword
    // let mut b = vec![]; //2.using the vec! macro
    //Creating with data types
    let mut a2: Vec<i32> = Vec::new();
    let mut b2: Vec<i32> = vec![];
    let mut b3 = vec![1i32, 2, 3];//sufixing 1st value with data type
    //Creating with data
    let mut b4 = vec![1, 2, 3];
    let mut b5: Vec<i32> = vec![1, 2, 3];
    let mut b6  = vec![1i32, 2, 3];
    let mut b7 = vec![0; 10]; //ten zeroes
    //Accessing and changing exsisting data
    let mut c = vec![5, 4, 3, 2, 1];
    c[0] = 1;
    c[1] = 2;
    //c[6] = 2; can't assign values this way, index out of bounds
    println!("{:?}", c); //[1, 2, 3, 2, 1]
    //push and pop
    let mut d: Vec<i32> = Vec::new();
    d.push(1); //[1] : Add an element to the end
    d.push(2); //[1, 2]
    d.pop(); //[1] : : Remove an element from the end
    // 🔎 Capacity and reallocation
    let mut e: Vec<i32> = Vec::with_capacity(10);
    println!("Length: {}, Capacity : {}", e.len(), e.capacity()); //Length: 0, Capacity : 10
    // These are all done without reallocating...
    for i in 0..10 {
        e.push(i);
    }
    e.push(11);
    println!("{:?}", e);
    // Mainly a vector represents 3 things; a pointer to the data, No of elements currently
    // have(length), capacity (Amount of space allocated for any future elements).
    // If the length of a vector exceeds its capacity,
    // its capacity will be increased automatically.
    // But its elements will be reallocated(which can be slow).
    // So always use Vec::with_capacity whenever it’s possible.
    // String data type is a UTF-8 encoded vector.
    // But you can not index into a String because of encoding.
    //  Vectors can be used with iterators in three ways,
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("A reference to {}", i);
    }
    for i in &mut v {
        println!("A mutable reference to {}", i);
    }
    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8
}
fn structt() {
    // Structs are used to encapsulate related properties into one unified datatype.
    // By convention, the name of the struct starts with a capital letter and follows CamelCase
    // There are 3 variants of structs,
    // 01. C-like structs
    // ▸ one or more comma separated name:value pairs
    // ▸ brace-enclosed list
    // ▸ similar to classes (without it’s methods) in other languages like Java
    // ▸ because fields have names, we can access them through dot notation
    // 02. Tuple structs
    // ▸ one or more comma separated values
    // ▸ parenthesized list like tuples
    // ▸ looks like a named tuples
    // 03. Unit structs
    // ▸ a struct with no members at all
    // ▸ it defines a new type but it resembles an empty tuple, ()
    // ▸ rarely in use, useful with generics
    //
    // When regarding OOP in Rust, attributes and methods are placed separately on
    // structs and traits. Structs contain only attributes,
    // traits contain only methods. They are getting connected via impls .

    //
    // C-like struct
    //
    // creating an instance
    let black = Color {red: 0, green: 0, blue: 0};
    println!("Black = rgb({}, {}, {})", black.red, black.green, black.blue); //Black = rgb(0, 0, 0)
    // structs are immutable by default, use `mut` to make it mutable but doesn't support field level mutability
    let mut link_color = Color {red: 0,green: 0,blue: 255};
    link_color.blue = 238;
    println!("Link Color = rgb({}, {}, {})", link_color.red, link_color.green, link_color.blue); //Link Color = rgb(0, 0, 238)
    // copy elements from another instance
    let blue = Color {blue: 255, .. link_color};
    println!("Blue = rgb({}, {}, {})", blue.red, blue.green, blue.blue); //Blue = rgb(0, 0, 255)
    // destructure the instance using a `let` binding, this will not destruct blue instance
    let Color {red: r, green: g, blue: b} = blue;
    println!("Blue = rgb({}, {}, {})", r, g, b); //Blue = rgb(0, 0, 255)
    // creating an instance via functions & accessing it's fields
    let midnightblue = get_midnightblue_color();
    println!("Midnight Blue = rgb({}, {}, {})", midnightblue.red, midnightblue.green, midnightblue.blue); //Midnight Blue = rgb(25, 25, 112)
    // destructure the instance using a `let` binding
    let Color {red: r, green: g, blue: b} = get_midnightblue_color();
    println!("Midnight Blue = rgb({}, {}, {})", r, g, b); //Midnight Blue = rgb(25, 25, 112)
}

fn get_midnightblue_color() -> Color {
    Color {red: 25, green: 25, blue: 112}
}

struct ColorN (u8, u8, u8);
struct Kilometers(i32);
fn tuple_struct() {
    // creating an instance
    let black = ColorN (0, 0, 0);
    // destructure the instance using a `let` binding, this will not destruct black instance
    let ColorN (r, g, b) = black;
    println!("Black = rgb({}, {}, {})", r, g, b); //black = rgb(0, 0, 0);
    //newtype pattern
    let distance = Kilometers(20);
    // destructure the instance using a `let` binding
    let Kilometers(distance_in_km) = distance;
    println!("The distance: {} km", distance_in_km); //The distance: 20 km
}

struct MXU;
fn unit_struct() {
    let x = MXU;
}


enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}
enum FlashMessage {
  Success, //a unit variant
  Warning{ category: i32, message: String }, //a struct variant
  Error(String) //a tuple variant
}
fn enums() {
    println!("Sunday is the {} day of the week", Day::Sunday as i32); //Sunday is the 0 day of the week
    let mut form_status = FlashMessage::Success;
    print_flash_message(form_status);
    form_status = FlashMessage::Warning {category: 2, message: String::from("Field X     required")};
    print_flash_message(form_status);
    form_status = FlashMessage::Error(String::from("Connection Error"));
    print_flash_message(form_status);
}

fn print_flash_message(m : FlashMessage) {
  // pattern matching with enum
  match m {
    FlashMessage::Success =>
      println!("Form Submitted correctly"),
    FlashMessage::Warning {category, message} => //Destructure, should use same field names
      println!("Warning : {} - {}", category, message),
    FlashMessage::Error(msg) =>
      println!("Error : {}", msg)
  }
}

// genercs
//  x has type T, T is a generic type
// fn takes_anything<T>(x: T) {}
//
// both x and y has same type
// fn takes_two_of_the_same_things<T>(x: T, y: T) {}
//
// multiple types
// fn takes_two_things<T, U>(x: T, y: U) {}
//
// When addding an implementation for a generic struct,
// the type parameters should be declared after the impl as well
//
// impl<T> Point<T> { }
struct Point<T> {
  x: T,
  y: T,
}
// generalizing enums
//-------------------
// enum Option<T> {
//     Some(T),
//     None,
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
fn generic_in_action() {
    let point_a = Point { x: 0, y: 0 }; // T is a int type
    let point_b = Point { x: 0.0, y: 0.0 }; // T is a float type
    println!("Point A: x = {}, y = {}", point_a.x, point_a.y); //Point A: x = 0, y = 0
    println!("Point B: x = {}, y = {}", point_b.x, point_b.y); //Point B: x = 0, y = 0
}
