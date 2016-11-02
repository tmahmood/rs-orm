use ::rand::{thread_rng, Rng};

enum Sources {
    Random,
    Database,
}

pub fn random_number(prefix:&str, len:usize) -> String {
    let mut rng = thread_rng();
    // collects 10 digits in a vector
    let mut phone_number = String::from(prefix);
    loop {
        let x = rng.gen::<u32>().to_string();
        phone_number.push_str(&x);
        if phone_number.len() >= len {
            let ts = phone_number.clone();
            phone_number.clear();
            phone_number.push_str(&ts[..len]);
            if number_called_before(&phone_number) {
                continue;
            }
            break;
        }
    }
    phone_number
}

pub fn number_called_before(number:&str) -> bool{
    return false;
}


