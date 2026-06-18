// author https://github.com/MIrrox27/donut
// main.rs

use std::io::{self, BufRead};


fn main() {
    let width: usize = 120;
    let height: usize = 30;

    let mut screen = vec![' '; width * height]; // Создаем массив с символами размещенными по всему экрану 
    let mut nums: Vec<usize> = vec![0; width * height];
    let aspect = height as f32 / height as f32;

    for i in 0..width{
        for j in 0..height{
            

            let x: f32 = (i as f32 / width as f32 * 2.0 - 1.0) * aspect;
            let y: f32 = j as f32 / height as f32 * 2.0 - 1.0;

            let mut pixel = ' ';
            let index = i + j * width;


            if (x*x + y*y) < 0.5 {pixel = '@';}  
            screen[index] = pixel;
            nums[index] = index;

            
        }

    }


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

    let _ = io::stdin().lock().read_line(&mut String::new()); // чтобы окно сразу не закрывалось 
}
