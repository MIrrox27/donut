// author https://github.com/MIrrox27/donut
// main.rs

use std::io::{self, BufRead};


fn main() {
    let width: usize = 120; // ширина экрана
    let height: usize = 30; // высота экрана

    let mut screen = vec![' '; width * height]; // Создаем массив с символами размещенными по всему экрану 
    let mut nums: Vec<usize> = vec![0; width * height];
    let aspect = width as f32 / height as f32 * 11.0 / 24.0; // соотношение сторон, для исправления неровностей. 11/24 - соотношение сторон 1 символа. для окна 120х30 примерно равно 1.833
    let r = 0.7; // отдаление круга от камеры (или же квадрат радиуса, круга, который отображается в консоли)
    
    for t in 1..10000000{
        for i in 0..width{
            for j in 0..height{
                let x: f32 = (i as f32 / width as f32 * 2.0 - 1.0) * aspect + (t as f32 * 0.0005).sin(); // координата X с нормированием (-1;1)
                let y: f32 = j as f32 / height as f32 * 2.0 - 1.0; // координата Y с нормированием (-1;1)

                let mut pixel = ' ';
                let index = i + j * width;
                
                if (x*x + y*y) < r {pixel = '@';}  // рисуем символ 
                screen[index] = pixel;
                nums[index] = index;

                
            }

        }
        let output: String = screen.iter().collect(); 
        println!("{}", output);
    }

    


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
