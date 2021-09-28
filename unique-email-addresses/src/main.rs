use std::collections::HashSet;

pub mod parse;

fn main() {
    let emails = vec![
        "test.email+alex@leetcode.com",
        "test.e.mail+bob.cathy@leetcode.com",
        "testemail+david@lee.tcode.com",
    ]
    .into_iter()
    .map(|email| parse::email(email).map(|(_, email)| email))
    .collect::<Result<HashSet<parse::Email>, _>>()
    .unwrap();

    println!("{}", emails.len());
}
