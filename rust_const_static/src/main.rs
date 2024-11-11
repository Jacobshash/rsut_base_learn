
// const SECOND_HOUR: i64 = 3600;
// the name `SECOND_HOUR` is defined multiple times `SECOND_HOUR` must be defined only once in the value namespace of this module
// const SECOND_HOUR: usize = 3600;
const SECOND_HOUR: usize = 3600;
const SECOND_DAY: usize = SECOND_HOUR * 24;
const SECOND_MONTH: usize = SECOND_DAY * 30;
// const STR_ING: usize = "hello";
const STR_ING: &str = "hello";

static MY_STATIC: usize = 3600;

fn main() {
    // 在rust中不同的数据类型不能强转
    // const SECOND_HOUR_1: i64 = 3600;
    // mismatched types expected `usize`, found `i64`
    // const SECOND_HOUR_2: usize =SECOND_HOUR_1* 3600;
    const SECOND_HOUR: usize = 3600;
    const SECOND_MONTH: usize = SECOND_DAY * 30;
    // mismatched types expected `usize`, found `&str`
    // const STR_ING: usize = "hello";
    {
        const SECOND_YEAR: usize = SECOND_DAY * 365;
        // 下面这两种写法等价
        println!("{}", SECOND_YEAR);
        println!("{SECOND_YEAR}");
    }
    // cannot find value `SECOND_YEAR` in this scope
    // println!("{}", SECOND_YEAR);
    println!("{}", STR_ING);
    // format arguments must be a string literal
    // 格式参数必须是字符串文字
    // println!(SECOND_DAY);
    println!("{}", SECOND_DAY);

    // const 变量不能用 mut 修饰
    // const mut inte: uszie = "hello";
    static MY_STATIC: usize = 3600;
    // 不能被修改
    // my_static = 36002;
    static mut MY_MUT_STATIC: usize = 3600;
    // 只能在unsafe块中修改
    // MY_MUT_STATIC = 36001;
    unsafe {
        MY_MUT_STATIC = 36003;
        println!("{}", MY_MUT_STATIC);
        // 不加mut修饰的static变量不能被修改
        // my_static = 36004;
    }
    println!("{}", MY_STATIC);
    // 在unsafe中被修改的变量，safe块的代码无法再次使用
    // println!("{MY_MUT_STATIC}")

}
