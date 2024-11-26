fn main() {
    println!("Hello, world!");
}


#[test]
fn test(){
    println!("test func test")
}

#[test]
fn test_variable() {
    let name = "ahmad";

    println!("hello {}", name)
    //one line comment
    /*
    
    two or more line comment */
}

#[test]
fn test_type() {
    let int: i32 =  10;

    println!("int : {}", int)
}

#[test]
fn test_operations() {
    let a =  10;
    let b = 20;
    
    let c = a + b;
    let d = a - b;
    let e =  a * b;
    let f = a / b;

    println!("{} {} {} {}", c,d,e,f)
}

#[test]
fn augmented_assigment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 20;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a =  true;
    let b: bool = false;

    println!("boolean :{} :{}", a , b)
}

#[test]
fn tuple() {
    let data: (i32, bool, f64) = (1, true, 0.1);
    println!("data : {:?}", data);


    let a = data.0;
    let b = data.1;

    println!("data 0 : {}", a);
    println!("data 0 : {}", b);

    let (c , d , e) =  data;
    println!("list : {} {} {}", c, d, e);
}

// fn unit() {
//     println!("hello!")
// }

// #[test]
// fn test_unit() {
//     let result: () = unit();
//     println!("{:?}", result);
// }


#[test]
fn array() {
    let array: [i32; 5] =  [1,2,3,4,5];
    //let array =  [1,2,3,4,5];

    println!("{:?}", array);

    let a =  array[0];
    let b = array[1];

    println!("{} :  {}", a,b);

    let length = array.len();
    println!("{}", length)
}

#[test]
fn two_dimensional_array() {
    let matrix = [
        [1,2],
        [3,4]
    ];

    println!("{:?}", matrix);
    println!("{}", matrix[0][1]);
    println!("{:?}", matrix[1]);
}

const MAXIMUM: i32 = 100;

#[test]
fn constant() {
    const MINIMUM: i32 = 10;

    println!("{}", MAXIMUM);
    println!("{}", MINIMUM);
}

#[test]
fn variable_scope() {
    let mut nama = "ahmad";

    {
        println!("nama : {}", nama);
        nama = "maul";
        //let nama2 = "ys";
        println!("nama : {}", nama);
    }

    println!("nama : {}", nama);
    //println!("nama : {}", nama2);
}

#[test]
fn stack_heap() {
    f_a();
    f_b();
}

fn f_a() {
    let a =  10;
    let b = String::from("ahmad");

    println!("{} {}", a, b);
}

fn f_b() {
    let a =  10;
    let b = String::from("ucup");

    println!("{} {}", a, b);
}