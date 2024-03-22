mod macros;

use std::{fmt::Debug, fs, io, path::Path, result};

use data_encoding::{BASE32HEX, BASE32HEX_NOPAD};
use sha2::{Digest, Sha256};

#[derive(Debug)]
enum Error {
  Any(String),
}

fn to_error<E: Debug>(error: E) -> Error {
  Error::Any(format!("{:?}", error))
}

enum EncodeType {
  Base32HexNoPad,
  Base32Hex,
}

type Result<T> = result::Result<T, Error>;

fn generate_hash(path: impl AsRef<Path>, encode_type: EncodeType) -> Result<String> {
  let mut file = fs::File::open(path).map_err(to_error)?;
  dbg!(file.metadata().unwrap());
  let mut hasher = Sha256::new();
  io::copy(&mut file, &mut hasher).map_err(to_error)?;
  let raw_hash = hasher.finalize();
  let hash = match encode_type {
    EncodeType::Base32HexNoPad => BASE32HEX_NOPAD.encode(&raw_hash).to_lowercase(),
    EncodeType::Base32Hex => BASE32HEX.encode(&raw_hash).to_lowercase(),
  };
  Ok(hash)
}

fn main() {
  generate_hash("./tests/fixtures/empty.txt", EncodeType::Base32Hex).unwrap();
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestCase {
    input: (&'static str, EncodeType),
    expected: &'static str,
  }

  fn test_generate_hash_each(case: TestCase) {
    let actual = generate_hash(case.input.0, case.input.1).unwrap();
    assert_eq!(actual, case.expected);
  }

  test_each!(
    test_generate_hash,
    test_generate_hash_each,
    "utf8" => TestCase{
      input: ("./tests/fixtures/empty-utf8.txt", EncodeType::Base32HexNoPad),
      expected: "seoc8gkovge196nruj49irtp4gjqsgf4cidp6j54imchmu2in1ag"
    },
    "utf16be" => TestCase{
      input: ("./tests/fixtures/empty-utf16be.txt", EncodeType::Base32HexNoPad),
      expected: "u6bmia0gqhbu55vst72maeo2b0fvj6iggkhn1sktfpfu8vct6vj0"
    },
    "utf16le" => TestCase{
      input: ("./tests/fixtures/empty-utf16le.txt", EncodeType::Base32HexNoPad),
      expected: "mfah1ro44teahpkosmpsnc7csea9tu955s6dp0sujrhkeg4q484g"
    },
    "1-nopad" => TestCase{
      input: ("./tests/fixtures/1.json", EncodeType::Base32HexNoPad),
      expected: "p8uhcetb0l9o30ji4oa0aq7jnrrulb0ofjlteq3os2r3t7i44db0"
    },
    "1" => TestCase{
      input: ("./tests/fixtures/1.json", EncodeType::Base32Hex),
      expected: "p8uhcetb0l9o30ji4oa0aq7jnrrulb0ofjlteq3os2r3t7i44db0===="
    },
    "npm" => TestCase{
      input: ("./tests/fixtures/npm/package-lock.json", EncodeType::Base32HexNoPad),
      expected: "n35ebkqk3qbla57hjkfrfrr6b6jng6s86hb3ocjrfug58q5r2j9g"
    },
    "yarn" => TestCase{
      input: ("./tests/fixtures/yarn/yarn.lock", EncodeType::Base32HexNoPad),
      expected: "epmbo75as3tpiqpf1sl9q9l71crb76veee1h0eklltctme3r32eg"
    },
    "pnpm" => TestCase{
      input: ("./tests/fixtures/pnpm/pnpm-lock.yaml", EncodeType::Base32HexNoPad),
      expected: "soecfp556svu7nriu4o2qlrpj216m549r3iifho84uk3oap9l55g"
    },
    "bun" => TestCase{
      input: ("./tests/fixtures/bun/bun.lockb", EncodeType::Base32HexNoPad),
      expected: "gk1jvsmhce20vlkhklrnd5002bplclmcl2tl3adb514ce1uah4o0"
    },
  );
}
