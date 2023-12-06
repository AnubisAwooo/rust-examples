use anyhow::{Ok, Result};

trait Encode {
    type Target;

    fn encode(&self) -> Result<Self::Target>;
}

struct Event<Id, Data> {
    id: Id,
    data: Data,
}

impl<Id, Data> Event<Id, Data>
where
    Id: Encode<Target = Vec<u8>>,
    Data: Encode<Target = Vec<u8>>,
{
    fn new(id: Id, data: Data) -> Self {
        Self { id, data }
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = self.id.encode()?;
        encoded.extend(self.data.encode()?);
        Ok(encoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        impl Encode for u64 {
            type Target = Vec<u8>;

            fn encode(&self) -> Result<Self::Target> {
                Ok(vec![0, 0, 0, 0, 0, 0, 0, 0])
            }
        }

        impl Encode for String {
            type Target = Vec<u8>;

            fn encode(&self) -> Result<Self::Target> {
                Ok(self.as_bytes().to_vec())
            }
        }

        let event = Event::new(1, "hello world".to_string());

        let _encode = event.encode().unwrap();

        let event = Event::new("id".to_string(), "hello world".to_string());

        let _encode = event.encode().unwrap();
    }
}
