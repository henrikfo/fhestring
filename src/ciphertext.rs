use tfhe::integer::{RadixCiphertext};

#[derive(Serialize, Deserialize, Debug)]
struct FheAsciiChar{
    pub ctxt: RadixCiphertext,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FheString{
    pub str: Vec<FheAsciiChar>,
}

impl encrypt for FheString {

    pub fn encrypt(
        client_key: &RadixClientKey,
        s: &str,
    ) -> Result<FheString, Box<dyn std::error::Error>> {
        if !s.is_ascii() {
            return Err("content contains non-ascii characters".into());
        }
        Ok(s.as_bytes()
            .iter()
            .map(|byte| client_key.encrypt(*byte as u64))
            .collect())
    }

    // pub fn decrypt(encstring: FheString) -> str{
    //     let mut plainstring = vec![];
    // }
    // pub fn contains(&self, s: &str) -> bool{   
    // }
    // pub fn starts_with(&self, s: &str) -> bool{
    // }
    // pub fn ends_with(&self, s: &str) -> bool{
    // }
}