// author https://github.com/MIrrox27/donut
// main.rs


fn main() {
    let width: usize = 120; // ширина экрана
    let height: usize = 30; // высота экрана

    let center_x = width as f32 / 2_f32;
    let center_y = height as f32 / 2_f32;

    let scale_x = 30_f32;
    let scale_y = 15_f32;

    let mut screen = vec![' '; width * height]; // Создаем массив с символами размещенными по всему экрану 
    let mut z_buffer = vec![0.0; width * height];
    //let pi = std::f64::consts::PI as f32;
    let pi_2 = std::f64::consts::PI as f32 * 2_f32;
    
    //let r = 0.7; // отдаление круга от камеры (или же квадрат радиуса, круга, который отображается в консоли)


        // Углы вращения вокруг осей X и Z
    let a = 0_f32; 
    let b = 0_f32; 

        // Радиусы
    let r1 = 1_f32; // Радиус маленькой окружности 
    let r2 = 2_f32; // Радиус большей окружности 

        // Углы закругления 
    let mut i: f32 = 0_f32;
    let mut j: f32 = 0_f32;

    let step = 0.07_f32; // шаг для перебора i и j
    let offset = 5_f32;



   
    loop {
            // Обнуляю все списки и углы в начале каждого кадра
        i = 0.0; j = 0.0; 
        screen.fill(' ');
        z_buffer.fill(0.0);

        while  i <= pi_2{
            while j <= pi_2 {
                let is = i.sin();
                let ic = i.cos();

                let js = j.sin();
                let jc = j.cos();


                    // координаты точек
                let x = (r2 + r1 * ic) * jc;
                let y = r1 * i.sin();
                let z = (r2 + r1 * ic) * js;

                    // Вычисляем нормаль
                let nx = jc * ic; 
                let ny = is;
                let nz = ic * js;

                let rotate_tor = rotate(a, b, x, y, z);
                let rotate_normal = rotate(a, b, nx, ny, nz);

                    // Поворачиваем нормаль 
                let nx2 =rotate_normal.0;
                let ny2 =rotate_normal.1;
                let nz2 =rotate_normal.2;

                let x2 = rotate_tor.0;
                let y2 = rotate_tor.1;
                let z2 = rotate_tor.2;

                let d = 1_f32 / (z2 + offset);

                let sx = clamp(center_x + scale_x * d * x2, 0_f32, width as f32) as usize;
                let sy = clamp(center_y - scale_y * d * y2, 0_f32, height as f32) as usize;

                let pixel = ' ';
                let index = sy * width + sx;

                if d > z_buffer[index]{
                    z_buffer[index] = d;
                    screen[index] = pixel;
                }
                
               
                

                

                j += step;
            }
            i += step;
        }



    
        
        let output: String = screen.iter().collect(); 
        println!("{}", output);
    }
}



fn rotate(a:f32, b:f32, x:f32, y:f32, z:f32) -> (f32, f32, f32){
    let x1 = x;
    let y1 = y * a.cos() - z * a.sin();
    let z1 = y * a.sin() + z * a.cos();

    let x2 = x1 * b.cos() - y1 * b.sin();
    let y2 = x1 * b.sin() + y1 * b.cos();
    let z2 = z1;
    return (x2, y2, z2);
}


fn x(i: usize, width: usize, height: usize) -> f32{
    let aspect = width as f32 / height as f32 * 11.0 / 24.0; // соотношение сторон, для исправления неровностей. 11/24 - соотношение сторон 1 символа. для окна 120х30 примерно равно 1.833
    let x: f32 = (i as f32 / width as f32 * 2.0 - 1.0) * aspect;
    return x;
}


fn y(j: usize, height: usize) -> f32{
    let y: f32 = j as f32 / height as f32 * 2.0 - 1.0;
    return y;

}



fn clamp(value: f32, min: f32, max: f32) -> f32{ // переписал функцию
    return min.max(value.min(max));
}



fn get_color(x: f32, y: f32) -> i32{
    let dist = (x*x + y*y).sqrt();
    let color = (1.0 / dist) as i32;
    return color;

}

fn get_char(x: f32, y: f32) -> char{
    let gradient = [' ', '.', ',', '-', '~', ':', ';', ':', '=', '!', '*', '#', '$', '@']; // @%#*+=-:. 
    let gradient_size = gradient.len() - 2;

    let mut pixel: char = ' ';
    if pixel == ' ' {} // Заглушка
    let mut color = get_color(x, y) as f32;

    color = clamp(color, 0_f32, gradient_size as f32);
    pixel = gradient[color as usize];
    return pixel

}
