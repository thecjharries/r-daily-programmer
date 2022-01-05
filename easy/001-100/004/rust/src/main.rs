use rand::prelude::*;
use rand::distributions::Alphanumeric;
use rand_pcg::Pcg64;

fn main() {
    println!("Hello, world!");
}

fn random_password(rng: Pcg64,length: u32) -> String {
    let password: String = rng
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect();
    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_password() {
        let mut rng = Pcg64::seed_from_u64(0);
        assert_eq!(random_password(rng, 10), "1KDsyHVtHF");
        rng = Pcg64::seed_from_u64(0);
        assert_eq!(random_password(rng, 30), "1KDsyHVtHFHpJInAwXCUqgDpkqy8qD");
    }
}
