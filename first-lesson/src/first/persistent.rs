use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
enum Gender {
    Unspecified = 0,
    Male = 1,
    Female = 2,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct User {
    name: String,
    age: u8,
    gender: Gender,
}

#[allow(dead_code)]
impl User {
    fn new(name: String, age: u8, gender: Gender) -> Self {
        Self { name, age, gender }
    }

    fn persist(&self, file: &str) -> Result<usize, std::io::Error> {
        let mut file = std::fs::File::create(file)?;

        let data = serde_json::to_string(&self)?;

        use std::io::Write;
        file.write_all(data.as_bytes())?;

        Ok(data.len())
    }

    fn read(file: &str) -> Result<Self, std::io::Error> {
        let mut file = std::fs::File::open(file)?;

        let mut data = String::new();

        use std::io::Read;
        file.read_to_string(&mut data)?;

        let user = serde_json::from_str(&data)?;

        Ok(user)
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "Unknown user".into(),
            age: 0,
            gender: Gender::Unspecified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let user = User::new("Test".into(), 1, Gender::Female);
        user.persist("./test_persistent_user.json").unwrap();

        let user2 = User::read("./test_persistent_user.json").unwrap();

        assert_eq!(user, user2);
    }
}
