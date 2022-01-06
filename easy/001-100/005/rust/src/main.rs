fn password_protected(username: &str, password: &str) -> bool {
    username == "AzureDiamond" && password == "hunter2"
}

fn main() {
    let username = "AzureDiamond";
    let password = "hunter2";
    println!("{}", password_protected(username, password));
}
