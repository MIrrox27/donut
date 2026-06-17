fn main() {
    let width: usize = 120;
    let height: usize = 30;

    let mut screen = vec!['@'; width * height + 1]; // Создаем массив с символами размещенными по всему экрану 
   

    

    for i in 0..width{
        for j in 0..height{
            let idx = i + j * width;
            screen[idx] = '@';

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
}
