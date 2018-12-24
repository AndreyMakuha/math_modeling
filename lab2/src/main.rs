use helper_fn::*;

fn main() {
    let nums = fill_sequence();
    println!("Коэффициент ассиметрии: {}", skewness(&nums));
    println!("Эксцесс: {}", kurtosis(&nums));
}
