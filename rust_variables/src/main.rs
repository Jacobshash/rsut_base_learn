fn main() {
    // 指定了类型
    let init_count: i64 = 0;
    println!("count is {}", init_count);
    // 未指定类型，rust可以推导类型
    let init_number =  5;
    println!("number is {}", init_number);
    
    // 无法对不可变的变量进行重复赋值，会在编译期报错
    // init_count = 123;

    // 声明一个 mut 的变量，mut 是 mutable 的缩写，表示可变的，可以对该可变变量进行重新赋值
    let mut _count = init_count;
    _count = 12;
    println!("count is {}", _count);

    // 或者我们可以重新声明一下该不可变量，对其重新赋值，此时可以改变该变量的类型，此时也不会报错，该行为在Rust中称为shadowing，即覆盖，此外，和变量的作用域也有关系
    let _variable: f64 = 12.5;
    println!("_variable is {}", _variable);

    let _variable = "dasdad";
    println!("_variable is {}", _variable);

    let _variable = 12;
    println!("_variable is {}", _variable);

    // 变量作用域，猜测一下，x_number 在作用域内，在作用域外，x_number 的值会不会改变
    let x_number = 5;
    {
        // x_number 在作用域内
        let x_number = x_number + 1;
        println!("x_number in inner scope is {}", x_number);
    }
    println!("x_number in outer scope is {}", x_number);

    let x_number = "5_str";
    println!("x_number is {}", x_number);

    // 变量作用域，猜测一下，x_number_mut 在作用域内，在作用域外，x_number_mut 的值会不会改变
    let mut  x_number_mut = 5;
    {
        // x_number_mut 在作用域内
        x_number_mut = x_number_mut * 3;
        println!("x_number_mut in inner scope is {}", x_number_mut);
    }
    // 此时该值发生了变化
    println!("x_number_mut in outer scope is {}", x_number_mut);

    let x_number_mut = "5_str";
    println!("x_number_mut is {}", x_number_mut);
}
