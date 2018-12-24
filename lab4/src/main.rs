use helper_fn::*;

fn main() {
    let nums = fill_sequence();

    println!("Введите уровень значимости: ");
    let significance_level_answer = scan::<f64>().unwrap();

    println!("Вывести таблицу значимости: 1 - Да, 2 - Нет");
    let table_answer = scan::<i32>().unwrap();
    if table_answer == 1 {
        let mut el = Vec::new();
        for i in emperical_law(&nums) {
            el.push(i.1);
        }

        let clust_data = hit_chances(&nums);

        let coords = coordinates_of_intervals(&nums);
        println!("Теоретические частоты попадания случайной величины в интервалы");
        // for i in 0..coords.len() - 1 {
        //     println!("от {:.2} - до {:.2}: {:.2} {:.2}", coords[i], coords[i + 1], clust_data[i], el[i]);
        // }
            let mut s = String::new();

    for i in 0..coords.len() - 1 {
        s += &format!("| {:.3} - {:.3} ", coords[i], coords[i + 1]);
    }

    println!("+{}+", "-".repeat(s.len()-1));

    for i in 0..coords.len() - 1 {
        print!("| {:.3} - {:.3} ", coords[i], coords[i + 1]);
    }
    println!("|");
    println!("+{}+", "-".repeat(s.len()-1));
    for i in 0..coords.len() - 1 {
        print!("|{:^14.3} ", clust_data[i]);
    }
    println!("|");
    println!("+{}+", "-".repeat(s.len()-1));
    }
    println!("{}", hypotesis_check(&nums, significance_level_answer));
}
