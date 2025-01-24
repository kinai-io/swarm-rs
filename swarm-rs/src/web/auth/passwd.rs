
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordVerifier,PasswordHasher
};

#[allow(unused)]
pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let res = Argon2::default().hash_password(password.as_bytes(), &salt).unwrap();
    res.to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let hash = PasswordHash::new(hash).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .is_ok()
}

#[test]
pub fn test_hash_password() {
    let password = "my-password";
    let hash = hash_password(password);
    let is_valid = verify_password(password, &hash);
    println!("Hash : {}", hash);
    println!("Valid : {}", is_valid);
    assert!(is_valid);
}