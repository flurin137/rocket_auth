use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

pub fn salt_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod test
{
    use super::*;


    #[test]
    fn salt_verify_valid()
    {
        let password = "ASDF";
        let password_hash = salt_password(password);

        assert!(verify_password(password, &password_hash));
    }
    
    #[test]
    fn salt_verify_invalid()
    {
        let password = "ASDF";
        let password_hash = salt_password(password);

        assert!(!verify_password("ASDF1", &password_hash));
        assert!(!verify_password("", &password_hash));
        assert!(!verify_password("ASD", &password_hash));
        assert!(!verify_password("eqztr", &password_hash));
        assert!(!verify_password("____", &password_hash));
    }
}