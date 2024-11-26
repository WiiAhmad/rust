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