
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

// const MAXIMUM: i32 = 100;

// #[test]
// fn constant() {
//     const MINIMUM: i32 = 10;

//     println!("{}", MAXIMUM);
//     println!("{}", MINIMUM);
// }

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

// #[test]
// fn stack_heap() {
//     f_a();
//     f_b();
// }

// fn f_a() {
//     let a =  10;
//     let b = String::from("ahmad");

//     println!("{} {}", a, b);
// }

// fn f_b() {
//     let a =  10;
//     let b = String::from("ucup");

//     println!("{} {}", a, b);
// }

#[test]
fn string() {
    let name: &str = "   ahmad maulana yusuf   ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);
    //pointer
}

#[test]
fn string_type() {
    let mut name: String = String::from("ahmad maulana");
    println!("{}", name);
    name.push_str(" yusuf");
    println!("{}", name);
    let _ = name.replace("ahmad", "testing");
    println!("{}", name);
}

#[test]
fn ownership_rules() {
    //var belum dideklarasi
    let a =  10;
    // a bisa diakses
    println!("{}", a);
    {
        // b blm bisa diakses karna belum dideklarasi
        let b = 11;
        println!("{}", a); //var a bisa diakses karena scope paling luar valuenya masih ada
        println!("{}", b);
        // b bisa diakses
    }// scope b selesai dan b dihapus tidak bisa diakses keluar scope
    println!("{}", a);
    //var a masih bisa diakses
} //var a selesai scope dan dihapus tidak bisa diakses lagi keluar scope

#[test]
fn data_copy() {
    let a = 10;
    let b = a;

    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1 = String::from("ahmad");
    let name2 = name1;

    println!("{}", name2);
    //println!("{}", name1);
}

#[test]
fn ownership_clone() {
    let name1 = String::from("ahmad");
    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}

#[test]
fn if_expression() {
    let value = 8;

    if value >= 9{
        println!("good");
    }else{
        println!("sip")
    }
}

#[test]
fn if_else_ex() {
    let value = 9;

    if value >= 10{
        //
    } else if value >= 11 {
        //
    }
}

#[test]
fn let_statement() {
    let value = 8;
    let _result: &str = if value >= 9 {
        "testing"
    }else{
        "else"
    };
}


#[test]
fn loop_ex() {
    let mut counter = 0;

    loop {
        counter += 1;
        if counter > 10 {
            break;
        }else if counter % 2 == 0 {
            continue;
        }

        println!("{}", counter);
    }
}

#[test]
fn loop_return() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10{
            break counter * 3;
        }
    };

    println!("{}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    'luar: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'luar;
            }

            println!("{} x {} = {}", number, i , number * i);
            i += 1;
            if i >10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0 ;
    while counter <= 10 {
        println!("{}", counter);
        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array: [&str; 5] = ["a", "b", "c", "d", "e"];
    let mut index = 0;

    while index < array.len() {
        println!("value : {}", array[index]);
        index += 1;
    } 
}

#[test]
fn array_iteration2() {
    let array: [&str; 5] = ["a", "b", "c", "d", "e"];
    
    for i in array  {
        println!("value : {}", i);
    }
}

#[test]
fn range() {
    let array = ["a", "b", "c", "d", "e"];

    let range = 0..5;

    for i in range {
        println!("value : {}", array[i] );
    }
}

// fn print_number(number: i32) {
//     println!("{}", number);
// }

// fn print_name(name: String) {
//     println!("{}", name);
// }

// #[test]
// fn test_hi() {
//     let number = 10;

//     print_number(number);

//     let name = String::from("eko");

//     print_name(name);
// }

// fn fullname(firstname: String, lastname: String) -> String {
//     let fullname = format!("{} {}", firstname, lastname);

//     return fullname;
// }

// #[test]
// fn test_fullname() {
//     let firstname = String::from("ahmad");
//     let lastname = String::from("ucup");

//     let _full = fullname(firstname, lastname);
// }

// fn change_value(value: String) {
//     value.push("test");
// }

// #[test]
// fn test_change_value() {
//     let value = String::from("ahmad");

//     change_value(value);

//     println!("{}", value)
// }


struct Person {
    firstname: String,
    mid_name: String,
    lastname: String,
    age: u8,
}

fn print_person(person: &Person) {
    println!("{}", person.age);
    println!("{}", person.firstname);
    println!("{}", person.mid_name);
    println!("{}", person.lastname);
}
#[test]
fn struct_person() {
    let firstname = String::from("ahmad");
    let lastname = String::from("xxx");

    let person: Person = Person {
        age: 20,
        firstname,
        mid_name: String::from("xxxx"),
        lastname
    };

    let person2: Person = Person {
        firstname: person.firstname.clone(),
        mid_name: person.mid_name.clone(),
        lastname: person.lastname.clone(),
        ..person
    };
    print_person(&person);
    print_person(&person2);
}

struct GeoPoint(f64, f64);

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-6.2 , 6.6);

    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

struct Nothing;

#[test]
fn test_nothing() {
    let _nothing1: Nothing = Nothing;
}