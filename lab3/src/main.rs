use helper_fn::*;

fn main() {
    let nums = fill_sequence();
    let clust_data = hit_chances(&nums);
    let coords = coordinates_of_intervals(&nums);
    println!("Теоретические частоты попадания случайной величины в интервалы");
    // for i in 0..coords.len() - 1 {
    //     println!("от {:.3} - до {:.3}: {:.3}", coords[i], coords[i + 1], clust_data[i]);
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