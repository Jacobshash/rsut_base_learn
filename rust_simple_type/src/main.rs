fn main() {
    
    // int 类型
    let shi_jin_zhi = 255;
    let ba_jin_zhi = 0o377;
    let shi_liu_jin_zhi = 0xFF;
    let er_jin_zhi = 0b11111111;

    println!("{} {} {} {}", shi_jin_zhi, ba_jin_zhi, shi_liu_jin_zhi, er_jin_zhi);

    println!("u32 max: {}",u32::MAX);
    println!("u32 min: {}",u32::MIN);
    println!("u32 is {} bits",std::mem::size_of::<u32>());
    
    println!("i32 max: {}",i32::MAX);
    println!("i32 min: {}",i32::MIN);
    println!("i32 is {} bits",std::mem::size_of::<i32>());
    
    println!("uszie max: {}",usize::MAX);
    println!("uszie min: {}",usize::MIN);
    println!("usize is {} bits",std::mem::size_of::<usize>());
    
    println!("isize max: {}",isize::MAX);
    println!("isize min: {}",isize::MIN);
    println!("isize is {} bits",std::mem::size_of::<isize>());

    // float 类型
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("{} {}",x,y);

    let x = 2.9999999; // f64
    let y: f32 = 3.1231231; // f32
    println!("{:.2} {:.2}",x,y);


    // 布尔类型
    let t = true;
    let f = false;
    println!("{} {}",t,f);
    println!("t {} {}",t,t as i32);
    println!("f {} {}",f,f as i32);
    
    // 字符类型
    let c = 'z';
    let zhongwen = '中';
    let emoji = '🤣';
    println!("{} {} {}",c,zhongwen,emoji);
    println!("{}",c as i32);
    println!("{}",c as u32);
    println!("{}",emoji as i32);
    println!("{}",emoji as u32);
    println!("{}",zhongwen as i32);
    println!("{}",zhongwen as u32);

    // 字符串类型
    let s1 = "hello";
    let s2 = "world";
    let s3 = c.to_string();
    let s4 = zhongwen.to_string();
    let s5 = emoji.to_string();
    println!("{} {} {} {} {}",s1,s2,s3,s4,s5);
}
