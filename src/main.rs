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
    let mut a = 0_f32; 
    let mut b = 0_f32; 

        // Радиусы
    let r1 = 1_f32; // Радиус маленькой окружности 
    let r2 = 2_f32; // Радиус большей окружности 

        // Углы закругления 
    let mut i: f32 = 0_f32;
    let mut j: f32 = 0_f32;
    if i == 0.0 || j == 0.0 {} // Заглушка

    let step = 0.07_f32; // шаг для перебора i и j
    let offset = 4.5_f32;



   
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
                let y = r1 * is;
                let z = (r2 + r1 * ic) * js;

                    // Вычисляем нормаль до поворота
                let nx = jc * ic; 
                let ny = is;
                let nz = ic * js;

                let rotate_tor = rotate(a, b, x, y, z);
                let rotate_normal = rotate(a, b, nx, ny, nz);

                    // Поворачиваем нормаль 
                let nx2 =rotate_normal.0;
                //let ny2 =rotate_normal.1;
                //let nz2 =rotate_normal.2;

                    // Яркость 
                let brightness = clamp(nx2, 0.0, 1.0);
                let gradient = [' ', '.', ',', '-', '~', ':', ';', ':', '=', '!', '*', '#', '$', '@']; // @%#*+=-:. 
                let gradient_size = gradient.len();
                let idx = (brightness * (gradient_size - 1) as f32).round() as usize;
                let pixel = gradient[idx];
                   
                    // Поворачиваем точки
                let x2 = rotate_tor.0;
                let y2 = rotate_tor.1;
                let z2 = rotate_tor.2;

                let d = 1.0 / (z2 + offset);

                let sx = clamp(center_x + scale_x * d * x2, 0_f32, (width-1) as f32) as usize;
                let sy = clamp(center_y - scale_y * d * y2, 0_f32, (height-1) as f32) as usize;

                
                let index = sy * width + sx;
                if d > z_buffer[index]{
                    z_buffer[index] = d;
                    screen[index] = pixel; // Записываем символ
                }
                
                j += step;
            }
            
            i += step;
            j = 0.0;
        }

        
        let output: String = screen.iter().collect(); 
        println!("{}", output);
        a += 0.04; b += 0.02;
        std::thread::sleep(std::time::Duration::from_millis(30));
        
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


fn clamp(value: f32, min: f32, max: f32) -> f32{ // переписал функцию
    return min.max(value.min(max));
}
