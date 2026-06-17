fn main() {
    let mut width: usize = 120;
    let mut height: usize = 30;

    let mut screen = vec!['@'; width * height + 1]; // Создаем массив с символами размещенными по всему экрану 
    screen[width * height] = '\0';

    

    for i in 0..width{
        for j in 0..height{
            screen[i + j * width] = '@';

        }
    }

    print!(screen[])
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
