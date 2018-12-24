use pyo3::prelude::*;
use pyo3::types::PyDict;

use std::f64;
use std::fs::File;
use std::io::{self, prelude::*};
use rand::Rng;

use std::fmt::Write;

// Теоретические частоты попадания случайной величины в интервалы
fn laplass(mw: f64, sd: f64, x: f64) -> PyResult<f64> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let locals = PyDict::new(py);
    locals.set_item("stats", py.import("scipy.stats")?)?;

    let res = py.eval(&format!("stats.norm.cdf({})", (x - mw) / sd), None, Some(&locals))?;

    let res: f64 = res.to_string().parse::<f64>().unwrap();
    Ok(res)
}

pub fn laplace(mw: f64, sd: f64, x: f64) -> f64 {
    laplass(mw, sd, x).unwrap()
}

fn isf(q: f64, dfn: f64, dfd: f64) -> PyResult<f64> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let locals = PyDict::new(py);
    locals.set_item("stats", py.import("scipy.stats")?)?;

    let res = py.eval(&format!("stats.f.isf({}, {}, {})", q, dfn, dfd), None, Some(&locals))?;

    let res: f64 = if res.to_string() == "nan" { f64::NAN } else { res.to_string().parse::<f64>().unwrap() };
    Ok(res)
}

pub fn is_f(q: f64, dfn: f64, dfd: f64) -> f64 {
    isf(q, dfn, dfd).unwrap()
}

fn chi2_isf(q: f64, dfn: f64) -> PyResult<f64> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let locals = PyDict::new(py);
    locals.set_item("stats", py.import("scipy.stats")?)?;

    let res = py.eval(&format!("stats.chi2.isf({}, {})", q, dfn), None, Some(&locals))?;

    let res: f64 = if res.to_string() == "nan" { f64::NAN } else { res.to_string().parse::<f64>().unwrap() };

    Ok(res)
}

pub fn chi2_is_f(q: f64, dfn: f64) -> f64 {
    chi2_isf(q, dfn).unwrap()
}

pub fn math_waiting(nums: &Vec<f64>) -> f64 {
    nums.iter().fold(0f64, |acc, a| acc + a) / nums.len() as f64
}

pub fn standart_deviation(nums: &Vec<f64>) -> f64 {
    let mw = math_waiting(nums);
    let res = nums.iter().fold(0f64, |acc, a| acc + (a - mw).powi(2));

    (res / (nums.len() - 1) as f64).sqrt()
}

pub fn range(nums: &Vec<f64>) -> f64 {
    let f = |x: &&f64, y: &&f64| x.partial_cmp(y).unwrap();
    nums.iter().max_by(f).unwrap() - nums.iter().min_by(f).unwrap()
}

pub fn sturges(nums: &Vec<f64>) -> f64 {
    range(nums) / (1 as f64 + 3.22 * (nums.len() as f64).log10())
}

pub fn coordinates_of_intervals(nums: &Vec<f64>) -> Vec<f64> {
    let mut coords: Vec<f64> = Vec::new();

    let min_val = *nums.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
    let sturges_val = sturges(nums);

    for i in 0..=nums.len() {
        coords.push(min_val + i as f64 * sturges_val);
    }

    coords
}

pub fn asymmetry_coefficient(nums: &Vec<f64>) -> f64 {
    let mw = math_waiting(nums);
    let numerator = nums.iter().fold(0f64, |acc, x| acc + (x - mw).powi(3));
    let denominator = (nums.len() - 1) as f64 * standart_deviation(nums).powi(3);
    numerator / denominator
}

// Коэффицент асимметрии А служит для характеристики распределения
pub fn skewness(nums: &Vec<f64>) -> f64 {
    let mw = math_waiting(nums);
    let numerator = (1f64 / nums.len() as f64) * nums.iter().fold(0f64, |acc, x| acc + (x - mw).powi(3));
    let denominator = ((1f64 / (nums.len() as f64)) *
     nums.iter().fold(0f64, |acc, x| acc + (x - mw).powi(2))).powf(3f64 / 2f64);
    // println!("numerator: {} denominator: {}", numerator, denominator);
    if denominator == 0.0 { 0.0 } else { numerator / denominator }
}

pub fn excess_coefficient(nums: &Vec<f64>) -> f64 {
    let mw = math_waiting(nums);
    let numerator = nums.iter().fold(0f64, |acc, x| acc + (x - mw).powi(4));
    let denominator = (nums.len() - 1) as f64 * standart_deviation(nums).powi(4);
    println!("numerator: {} denominator: {}", numerator, denominator);
    if denominator == 0f64 { 0f64 } else { numerator / denominator - 3f64 }
}

// Коэфицент эксцесса E служит для характеристики крутости, т.е. остовершинности распределения
pub fn kurtosis(nums: &Vec<f64>) -> f64 {
    let mw = math_waiting(nums);
    let numerator = (1f64 / nums.len() as f64) * nums.iter().fold(0f64, |acc, x| acc + (x - mw).powi(4));
    let denominator = ((1f64 / (nums.len() as f64)) *
     nums.iter().fold(0f64, |acc, x| acc + (x - mw).powi(2))).powi(2);
    if denominator == 0f64 { 0f64 } else { numerator / denominator - 3f64 }
}

pub fn emperical_law(nums: &Vec<f64>) -> Vec<((f64, f64), f64)> {
    let mut emperical_arr = Vec::new();
    let coords = coordinates_of_intervals(nums);
    let len = nums.len() as f64;
    for i in 0..coords.len() - 1 {
        let mut hits = 0;
        for j in nums {
            if *j >= coords[i] && *j < coords[i + 1] {
                hits += 1;
            }
        }
        emperical_arr.push(((coords[i], coords[i + 1]), (hits as f64 / len)));
    }
    emperical_arr
}

pub fn statical_distribution_function(nums: &Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let emp = emperical_law(&nums);
    let x = coordinates_of_intervals(nums);
    let mut y = vec![0.0];
    for i in 0..x.len() - 1 {
        y.push(emp[i].1 + y[i]);
    }
    (x, y)
}

pub fn pierson_criteria(nums: &Vec<f64>) -> f64 {
    let sdf = statical_distribution_function(nums).1;
    let mut res = 0.0;
    let len = nums.len() as f64;
    for i in 0..nums.len() {
        if sdf[i] == 0.0 { continue; }
        res += (nums[i] - len * sdf[i]).powi(2) / (len * sdf[i])
    }
    res
}

pub fn hypotesis_check(nums: &Vec<f64>, signnificance_level: f64) -> String {
    let pirs = pierson_criteria(nums).powf(0.5);
    let table_value = chi2_is_f(signnificance_level, (nums.len() - 1) as f64);
    println!("X = {:e}, X**2k,2 = {}", pirs, table_value);
    if pirs < table_value {
        format!("Так как X < X**2k,a, распределение нормальное")
    } else {
        format!("Так как X > X**2k,a, распределение не нормальное")
    }
}

pub fn hit_chances(nums: &Vec<f64>) -> Vec<f64> {
    let mw = math_waiting(&nums);
    let sd = standart_deviation(&nums);
    let intervals = coordinates_of_intervals(&nums);
    let mut p = Vec::new();
    for i in 0..intervals.len() - 1 {
        p.push(laplace(mw, sd, intervals[i + 1]) - laplace(mw, sd, intervals[i]))
    }
    p
}

pub fn join(a: &Vec<f64>) -> String {
    let s: String = a.iter().skip(1).fold(String::new(),|mut s,&n| {write!(s,", {}",n).ok(); s});
    format!("{}{}", a[0], s)
}

pub fn scan<T: std::str::FromStr>() -> Result<T, <T as std::str::FromStr>::Err> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().parse::<T>()
}


pub fn fill_sequence() -> Vec<f64> {
    println!("1. Автоматическое заполнение");
    println!("2. Ручное заполнение");
    println!("3. Загрузить последовательность");

    let mut nums: Vec<f64> = Vec::new();

    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    let mut is_save_needed = false;

    match line.trim().parse::<i64>() {
        Ok(1) => {
            println!("Введите количество значений: ");
            line.clear();
            stdin.lock().read_line(&mut line).unwrap();
            // println!("{:?}", line);
            let len = line.trim().parse::<i64>().unwrap();
            
            println!("Определите интервал появления значений. ОТ: ");
            line.clear();
            stdin.lock().read_line(&mut line).unwrap();
            let answer_from = line.trim().parse::<i64>().unwrap();
            println!("Определите интервал появления значений. ДО: ");
            line.clear();
            stdin.lock().read_line(&mut line).unwrap();
            let answer_to = line.trim().parse::<i64>().unwrap();

            let mut rng = rand::thread_rng();

            for _ in 0..len {
                nums.push(rng.gen_range(answer_from as f64, answer_to as f64));
            }

            is_save_needed = true;
        },
        Ok(2) => {
            println!("Введите значания (через запятую и пробел):");
            line.clear();
            stdin.lock().read_line(&mut line).unwrap();
            for i in line.trim().split(", ") {
                // println!("{:?}", i);
                nums.push(i.parse::<f64>().unwrap());
            }

            is_save_needed = true
        },
        Ok(3) => {
            println!("Введите имя файла:");
            line.clear();
            stdin.lock().read_line(&mut line).unwrap();
            println!("{:?}", line.trim());
            let mut f = File::open(line.trim()).expect("file not found");
            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect("something went wrong reading the file");

            for i in contents.trim().split(", ") {
                // println!("{:?}", i);
                nums.push(i.parse::<f64>().unwrap());
            }
        },
        Ok(i) => { println!("Было введено не правилное число {}", i); },
        Err(_) => { panic!("error!"); },
    }

    if is_save_needed {
        println!("Сохранить последовательность? 1 - Да, 2 - Нет: ");
        line.clear();
        stdin.lock().read_line(&mut line).unwrap();
        if line.trim().parse::<i32>().unwrap() == 1 {
            println!("Введите название последовательности: ");
            line.clear();
            stdin.lock().read_line(&mut line).unwrap();

            let mut file = match File::create(&line.trim()) {
                Err(_) => panic!("couldn't create"),
                Ok(file) => file,
            };
    
            {
                use std::io::Write;
                match file.write_all(join(&nums).as_bytes()) {
                    Err(_) => panic!(),
                    Ok(_) => {},
                }
            }
        }
    }

    nums
}
