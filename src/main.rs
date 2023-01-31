use std::io;
use rand::Rng;
use std::cmp::Ordering;//枚举类型 分别是小于大于和等于
fn main() {
    println!("Guess a number!! ");
 
    let secret_number = rand::thread_rng().gen_range(1, 101);
 
    println!("The secret number is:{}", secret_number);
 
    println!("Guess a number: ");
 
    //let foo = 1;//let为定义值操作
 
    //let bar = foo;
 
    //foo = 2;//报错 因为rust中被定义的变量不能改变类似java中的"常量"
 
    let mut guess = String::new();//let mut means the number can be mutable
 
    io::stdin().read_line(&mut guess).expect("can not read in line");//方法的参数是用引用进行传递的
    //read_line()具有返回值 OK||Err  expect函数若read_line()返回值为Err就输出内部字符串 OK就赋值给guess
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    //前面guess是新定义的整数类型 后面guess是就定义的字符串类型 trim是去掉两边空白 parse把字符串变为数值类型
    println!("The number you guess is:{}", guess);
    //String 和 io 均为库 ::代表库下的函数
    match guess.cmp(&secret_number){//cmp返回的是Ordering类型
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}