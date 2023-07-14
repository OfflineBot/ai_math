use ndarray::{Array1, Array2};

// return highest value in vector
pub fn max(x: &Vec<f64>) -> f64 {
    let mut max_val = f64::MIN;
    for i in x.iter() {
        if i > &max_val {
            max_val = *i;
        }
    }
    max_val
}

// return highest value in Array1
pub fn nd_max(x: &Array1<f64>) -> f64 {
    let mut max_val = f64::MIN;
    x.map(|y| if y > &max_val { max_val = *y });
    max_val
}

// return index of highest value in vector
pub fn argmax(x: &Vec<f64>) -> usize {
    let mut index = 0;
    let mut max_val = f64::MIN;
    for (i, item) in x.iter().enumerate() {
        if item > &max_val {
            max_val = *item;
            index = i;
        }
    }
    index
}

// return index of highest value in Array1
pub fn nd_argmax(x: &Array1<f64>) -> usize {
    let mut index = 0;
    let mut max_val = f64::MIN;
    for (i, item) in x.iter().enumerate() {
        if item > &max_val {
            max_val = *item;
            index = i;
        }
    }
    index
}

// return quotient and modulo of 2 numbers
pub fn divmod(x: usize, y: usize) -> (usize, usize) {
    let div = x / y;
    let modulo = x % y;
    (div, modulo)
}

// get x and y coordinate of special char in 2d vector
pub fn find_item(arr2: &Vec<Vec<char>>, item: char) -> Result<(usize, usize), String> {
    let some_pos = arr2
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row
                .iter()
                .enumerate()
                .find_map(|(j, col)| {
                    if col == &item {
                        return Some((i, j));
                    } else {
                        None
                    }
                })
        });

    let pos = match some_pos {
        Some(value) => value,
        None => return Err(String::from("Item not found!"))
    };
    Ok(pos)
}

//get x and y coordinate of special char in Array2
pub fn nd_find_item(arr2: &Array2<char>, item: char) -> Result<(usize, usize), String> {
    let pos = arr2.indexed_iter().find_map(|((x, y), my_item)| {
        if my_item == &item {
            return Some((x, y));
        } else {
            None
        }
    });
    let position = match pos {
        Some(value) => value,
        None => return Err(String::from("Item not found!")), 
    };
    Ok(position)
}

// get size of 2d vector
pub fn grid_size(arr2: &Vec<Vec<char>>) -> usize {
    arr2[0].len() * arr2.len()
}

// get size of Array2<char>
pub fn nd_grid_size(arr2: &Array2<char>) -> usize {
    arr2.shape()[0] * arr2.shape()[1]
}