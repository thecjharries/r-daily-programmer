fn password_protected(username: &str, password: &str) -> bool {
    username == "AzureDiamond" && password == "hunter2"
}

fn main() {
    let username = "AzureDiamond";
    let password = "hunter2";
    println!("{}", password_protected(username, password));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_protected() {
        assert_eq!(password_protected("AzureDiamond", "hunter2"), true);
        assert_eq!(password_protected("AzureDiamond", "hunter3"), false);
        assert_eq!(password_protected("qqq", "hunter2"), false);
        assert_eq!(password_protected("aaa", "hunter22"), false);
    }
}
