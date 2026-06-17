// author https://github.com/MIrrox27/donut
// main.rs

use std::io::{self, BufRead};


fn main() {
    let width: usize = 120;
    let height: usize = 30;

    let screen = vec!['@'; width * height]; // Создаем массив с символами размещенными по всему экрану 
   

    let output: String = screen.iter().collect(); 

    println!("{}", output);


    /*  Нужно для отрисовки пончика 
    let r1: i32; // Радиус маленькой окружности (внетренность)
    let r2: i32; // Радиус большей окружности 

    let i: i32;
    let j: i32;

    let x = (r2 + r1 * i.cos()) * j.cos();
    let y = r1 * i.sin();
    let z = (r2 + r1 * i.cos()) * j.sin();
    */

    let stdin = io::stdin(); // чтобы сразу не закрывалась 
    let _ = stdin.lock().read_line(&mut String::new());
}
