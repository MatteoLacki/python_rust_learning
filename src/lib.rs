use pyo3::prelude::*;
use rand::Rng;
use std::cmp::Ordering;
use numpy::{PyArray1, IntoPyArray};
// use numpy::{PyArray1, IntoPyArray, PyArrayDyn};
use std::io;
use rayon::prelude::*;


#[pyfunction]
fn guess_the_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


#[pyfunction]
fn process_array(py: Python, arr: &PyArray1<f64>) -> PyResult<Py<PyArray1<f64>>> {
    let mut rust_vec = unsafe{ arr.as_array().to_owned() };
    // Here, perform some operations on the rust_vec, for example:
    for item in rust_vec.iter_mut() {
        *item *= 2.0;
    }
    Ok(rust_vec.into_pyarray(py).to_owned())
}

// #[pyfunction]
// fn process_array_inplace(arr: &PyArray1<f64>) -> PyResult<()> {
//     let mut rust_vec = unsafe{ arr.as_array_mut().as_slice_mut().unwrap() };
//     // Here, perform some operations on the rust_vec, for example:
//     for item in rust_vec.iter_mut() {
//         *item *= 2.0;
//     }
//     Ok(())
// }



// #[pyfunction]
// fn process_array_double(arr: &PyArray1<f64>) -> PyResult<()> {
//     // SAFETY: Here, we ensure that we have exclusive access to the data by borrowing it mutably.
//     // This operation is marked as unsafe because you're accessing raw memory which the compiler cannot verify.
//     let rust_slice: &[f64] = unsafe { arr.as_array().as_slice().unwrap() };

//     let mut doubled: Vec<f64> = rust_slice.to_vec();
//     doubled.par_iter_mut().for_each(|item| {
//         *item *= 2.0;
//     });

//     Ok(())
// }


// #[pyfunction]
// fn process_array_double(py: Python, arr: &PyArray1<f64>) -> PyResult<PyObject> {
//     // Extract a slice from the original array
//     let rust_slice: &[f64] = unsafe { arr.as_array().as_slice().unwrap() };

//     // Use parallel map to double the values
//     let doubled: Vec<f64> = rust_slice.par_iter().map(|&item| item * 2.0).collect();

//     // Convert the Rust Vec<f64> back into a numpy PyArray1 and return it
//     Ok(doubled.into_pyarray(py).to_object(py))
// }


#[pyfunction]
fn parallel_double_and_return(py: Python, arr: &PyArray1<f64>) -> PyResult<PyObject> {
    // Extract a slice from the original array
    let temp_array = unsafe{ arr.as_array() };
    let rust_slice: &[f64] = temp_array.as_slice().unwrap();

    // Use parallel map to double the values
    let doubled: Vec<f64> = rust_slice.par_iter().map(|&item| item * 2.0).collect();

    // Convert the Rust Vec<f64> back into a numpy PyArray1 and return it
    Ok(doubled.into_pyarray(py).to_object(py))
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn guessing_game_maturin(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(guess_the_number, m)?)?;
    m.add_function(wrap_pyfunction!(process_array, m)?)?;
    // m.add_function(wrap_pyfunction!(process_array_inplace, m)?)?;
    // m.add_function(wrap_pyfunction!(process_array_double, m)?)?;
    m.add_function(wrap_pyfunction!(parallel_double_and_return, m)?)?;
    Ok(())
}
