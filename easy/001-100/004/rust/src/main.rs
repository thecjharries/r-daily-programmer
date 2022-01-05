use rand::prelude::*;
use rand::distributions::Alphanumeric;
use rand_pcg::Pcg64;

fn main() {
    println!("Hello, world!");
}

fn random_password(rng: Pcg64,length: u32) -> String {
    let password: String = rng.sample_iter(&Alphanumeric).take(length as usize).collect();
    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_password() {
        let mut rng = Pcg64::from_seed(0);
        assert_eq!(random_password(rng, 10), "qqq");
    }
}
