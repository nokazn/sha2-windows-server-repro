mod macros;
use sha2::{Digest, Sha256};
use std::{fmt::Debug, fs, io, path::Path, result};

#[derive(Debug)]

struct Error(String);

fn to_error<E: Debug>(error: E) -> Error {
  Error(format!("{:?}", error))
}

type Result<T> = result::Result<T, Error>;

fn generate_hash(path: impl AsRef<Path>) -> Result<String> {
  let contents = fs::read(path).map_err(to_error)?;
  let mut hasher = Sha256::new();
  hasher.update(&contents);
  let raw_hash = hasher.finalize();
  let hash = base16::encode_lower(&raw_hash);
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
    "binary" => TestCase{
      input: "./tests/fixtures/binary/bun.lockb",
      expected: "85033ff2d163840fd691a57776940012f35656cca8bb51a9ab2848c707ca8930"
    },
    "empty" => TestCase{
      input: "./tests/fixtures/empty/.gitkeep",
      expected: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    },
    "utf8-crlf" => TestCase{
      input: "./tests/fixtures/json/utf8-crlf.json",
      expected: "ca3d163bab055381827226140568f3bef7eaac187cebd76878e0b63e9e442356"
    },
    "utf8-lf" => TestCase{
      input: "./tests/fixtures/json/utf8-lf.json",
      expected: "ca3d163bab055381827226140568f3bef7eaac187cebd76878e0b63e9e442356"
    },
    "utf8withbom-crlf" => TestCase{
      input: "./tests/fixtures/json/utf8withbom-crlf.json",
      expected: "ec2f00596f643c3181e6f3c6bfb64bb07cf8994f019df013681c886bee0d8b6a"
    },
    "utf8withbom-lf" => TestCase{
      input: "./tests/fixtures/json/utf8withbom-lf.json",
      expected: "ec2f00596f643c3181e6f3c6bfb64bb07cf8994f019df013681c886bee0d8b6a"
    },
    "utf16be-crlf" => TestCase{
      input: "./tests/fixtures/json/utf16be-crlf.json",
      expected: "d381222dd118d2d0c2af76c3f1a79259d707a57b35c60f0ebe72b6b3dda5e533"
    },
    "utf16be-lf" => TestCase{
      input: "./tests/fixtures/json/utf16be-lf.json",
      expected: "7ef7c1a42351608778a8a1562e7acb200f5b2c336f304b9d8ec896c3fabcf99c"
    },
    "utf16le-crlf" => TestCase{
      input: "./tests/fixtures/json/utf16le-crlf.json",
      expected: "d381222dd118d2d0c2af76c3f1a79259d707a57b35c60f0ebe72b6b3dda5e533"
    },
    "utf16le-lf" => TestCase{
      input: "./tests/fixtures/json/utf16le-lf.json",
      expected: "7ef7c1a42351608778a8a1562e7acb200f5b2c336f304b9d8ec896c3fabcf99c"
    },
  );
}
