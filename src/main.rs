// 関数
fn fizzbuzz(num: i32) -> String {
    if num % 15 == 0 {
        "FizzBuzz".to_string()
    } else if num % 3 == 0 {
        "Fizz".to_string()
    } else if num % 5 == 0 {
        "Buzz".to_string()
    } else {
        num.to_string()
    }
}

fn main() {
    // // Hello, world!
    // println!("Hello, world!");

    // // 変数宣言
    // let answer: &str = "cram";
    // println!("How can a clam {} in a clean cram can?", answer);

    // // 変数への再代入
    // let mut x = 1;
    // x = 2;
    // println!("{}", x);

    // // FizzBuzz
    // // if式
    // let num = 0;
    // let fizzbuzz = if num % 15 == 0 {
    //     "FizzBuzz".to_string()
    // } else if num % 3 == 0 {
    //     "Fizz".to_string()
    // } else if num % 5 == 0 {
    //     "Buzz".to_string()
    // } else {
    //     num.to_string()
    // };
    // println!("{}", fizzbuzz);

    // // for文
    // for num in 0..100 {
    //     println!("{}", fizzbuzz(num));
    // }

    // 関数型プログラミング的なアプローチ
    let result = (0..100)
        .map(fizzbuzz)
        .fold(String::from(""), |acc, line| format!("{}\n{}", acc, line));
    println!("{}", result);
}
