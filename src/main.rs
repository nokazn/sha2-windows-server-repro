mod macros;

use std::{fmt::Debug, fs, io, path::Path, result};

use data_encoding::BASE32HEX_NOPAD;
use sha2::{Digest, Sha256};

#[derive(Debug)]
enum Error {
  Any(String),
}

fn to_error<E: Debug>(error: E) -> Error {
  Error::Any(format!("{:?}", error))
}

type Result<T> = result::Result<T, Error>;

fn generate_hash(path: impl AsRef<Path>) -> Result<String> {
  let mut file = fs::File::open(path).map_err(to_error)?;
  dbg!(file.metadata().unwrap());
  let mut hasher = Sha256::new();
  io::copy(&mut file, &mut hasher).map_err(to_error)?;
  let raw_hash = hasher.finalize();
  let hash = BASE32HEX_NOPAD.encode(&raw_hash).to_lowercase();
  Ok(hash)
}

fn main() {
  generate_hash("./tests/fixtures/empty.txt").unwrap();
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestCase {
    input: &'static str,
    expected: &'static str,
  }

  fn test_generate_hash_each(case: TestCase) {
    let actual = generate_hash(case.input).unwrap();
    assert_eq!(actual, case.expected);
  }

  test_each!(
    test_generate_hash,
    test_generate_hash_each,
    "utf8" => TestCase{
      input: "./tests/fixtures/empty-utf8.txt",
      expected: "seoc8gkovge196nruj49irtp4gjqsgf4cidp6j54imchmu2in1ag"
    },
    "utf16be" => TestCase{
      input: "./tests/fixtures/empty-utf16be.txt",
      expected: "u6bmia0gqhbu55vst72maeo2b0fvj6iggkhn1sktfpfu8vct6vj0"
    },
    "utf16le" => TestCase{
      input: "./tests/fixtures/empty-utf16le.txt",
      expected: "mfah1ro44teahpkosmpsnc7csea9tu955s6dp0sujrhkeg4q484g"
    },
  );
}
