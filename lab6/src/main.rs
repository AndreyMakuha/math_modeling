use rand::Rng;
use helper_fn::*;

fn main() {
    println!("Заполнение первой последовательности: ");
    let mut x = fill_sequence();
    println!("Заполнение второй последовательности: ");
    let mut y = fill_sequence();
    println!("Заполнение третьей последовательности: ");
    let mut z = fill_sequence();
    println!("Максимум элементов для выборки: {}", ([x.len(), y.len(), z.len()]).iter().min().unwrap());
    println!("Введите количество элементов для выборки из последовательностей: ");
    let n = scan::<usize>().unwrap();

    let mut x1 = Vec::new();
    let mut y1 = Vec::new();
    let mut z1 = Vec::new();

    let mut rnd = rand::thread_rng();
    for _ in 0..n {
        let extr = rnd.gen_range(0, x.len() - 1);
        x1.push(x[extr]);
        x.remove(extr);
        let extr = rnd.gen_range(0, y.len() - 1);
        y1.push(y[extr]);
        y.remove(extr);
        let extr = rnd.gen_range(0, z.len() - 1);
        z1.push(z[extr]);
        z.remove(extr);
    }

    let no_x = math_waiting(&x1);
    let no_y = math_waiting(&y1);
    let no_z = math_waiting(&z1);

    let mut xy_numerator = 0f64;
    let mut xz_numerator = 0f64;
    let mut yz_numerator = 0f64;

    for i in 0..n {
        xy_numerator += (x1[i] - no_x) * (y1[i] - no_y);
        xz_numerator += (x1[i] - no_x) * (z1[i] - no_z);
        yz_numerator += (y1[i] - no_y) * (z1[i] - no_z);
    }

    let r_xy = xy_numerator/(n - 1) as f64 * standart_deviation(&x1) * standart_deviation(&y1);
    let r_xz = xz_numerator/(n - 1) as f64 * standart_deviation(&x1) * standart_deviation(&z1);
    let r_yz = yz_numerator/(n - 1) as f64 * standart_deviation(&y1) * standart_deviation(&z1);


    println!("    | {: ^6.3} 1 {: ^6.3} |", r_xy, r_xz);
    println!("f = |        1 {: ^6.3} |", r_yz);
    println!("    |             1   |")
}
