use std::{
    error::{self, Error},
    fmt,
};

#[derive(Debug)]
struct SignedCommandMemoError;

impl fmt::Display for SignedCommandMemoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InvalidMemo")
    }
}

impl Error for SignedCommandMemoError {}

struct SignedCommandMemo(pub Vec<u8>);

impl SignedCommandMemo {
    pub fn encode_memo(s: impl AsRef<[u8]>) -> Result<String, Box<dyn error::Error>> {
        const USER_COMMAND_MEMO: u8 = 0x14;

        const DIGEST_LEN: usize = 32;
        const MAX_INPUT_STRING_LENGTH: usize = DIGEST_LEN;
        const MEMO_LEN: usize = DIGEST_LEN + 2;
        const TAG_INDEX: usize = 0;
        const LEN_INDEX: usize = 1;
        const BYTES_TAG: u8 = 1;
        let s = s.as_ref();
        if s.len() > MAX_INPUT_STRING_LENGTH {
            return Err(Box::new(SignedCommandMemoError));
        }
        let mut v = vec![0; MEMO_LEN];
        v[TAG_INDEX] = BYTES_TAG;
        v[LEN_INDEX] = s.len() as u8;
        for (i, &b) in s.iter().enumerate() {
            v[i + 2] = b;
        }
        let hash = bs58::encode(v)
            .with_check_version(USER_COMMAND_MEMO)
            .into_string();
        Ok(hash)
    }

    pub fn decode_memo(s: impl AsRef<[u8]>) -> Result<String, Box<dyn error::Error>> {
        let decoded = bs58::decode(s).into_vec().unwrap();
        let value = &decoded[3..decoded[2] as usize + 3];
        Ok(String::from_utf8_lossy(value).to_string())
    }
}

#[test]
fn roundTrip() {
    println!("{}", SignedCommandMemo::encode_memo(String::from("no mip3").as_bytes()).unwrap());
    println!(
        "{}",
        SignedCommandMemo::decode_memo(
            String::from("E4YbUmaZZqAoURqWNQSpdwiFZURyPFxvfHRXHC1k7nufTxYGAc8cA").as_bytes()
        )
        .unwrap()
    );
}
