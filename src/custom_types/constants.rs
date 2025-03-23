use crate::{ADULT_AGE, LANGUAGE};

fn is_adult(n: u32) -> bool {
    n >= ADULT_AGE
}

pub(super) fn constants() {
    println!("--Language: {}", LANGUAGE);
    println!("--Is 18 an adult? {}", is_adult(18));
}
