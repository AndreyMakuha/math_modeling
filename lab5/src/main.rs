use helper_fn::*;
use rand::Rng;

fn main() {
    let mut x = fill_sequence();
    let mut y = fill_sequence();

    println!("Элементов в первой последовательности: {}", x.len());
    println!("Введите количество элементов для выборки из первой последовательности: ");
    let x_n = scan::<usize>().unwrap();
    println!("Элементов в второй последовательности: {}", y.len());
    println!("Введите количество элементов для выборки из первой последовательности: ");
    let y_n = scan::<usize>().unwrap();
    println!("Введите уровень значимости: ");
    let significanse_level = scan::<f64>().unwrap();

    let mut x1 = Vec::new();
    let mut y1 = Vec::new();

    let mut rnd = rand::thread_rng();
    for _ in 0..x_n {
        let extr = rnd.gen_range(0, x.len() - 1);
        x1.push(x[extr]);
        x.remove(extr);
    }

    for _ in 0..y_n {
        let extr = rnd.gen_range(0, y.len() - 1);
        y1.push(y[extr]);
        y.remove(extr);
    }

    let s1 = standart_deviation(&x1).powi(2);
    let s2 = standart_deviation(&y1).powi(2);

    let relation = if s1 > s2 { s1 / s2 } else { s2 / s1 };

    let fisher = is_f(significanse_level, (x1.len() - 1) as f64, (y1.len() - 1) as f64);

    if relation > fisher {
        println!("Дисперсии не равны");
    } else {
        println!("Дисперсии равны");
    }
}
