// transform username to first letter uppercase and rest lowercase
pub fn username_camelcase(username: String) -> String {
    let binding = username.to_lowercase();
    let mut username_chars = binding.chars();
    let head = username_chars.next().unwrap().to_uppercase();
    let body: String = username_chars.collect();
    return format!("{}{}", head, body);
}