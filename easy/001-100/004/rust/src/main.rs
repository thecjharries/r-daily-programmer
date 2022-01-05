fn main() {
    println!("Hello, world!");
}

fn random_password(rng: Pcg64,length: u32) -> String {
    let password: String = rng.sample_iter(&Alphanumeric).take(length as usize).collect();
    password
}
}
