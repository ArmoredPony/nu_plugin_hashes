use std::io::Write;

use digest::DynDigest;

const TEST_TEXT: &str = "abcdefghijklmnopqrstuvwxyz";

struct GeneratedHasherImpl {
  dependency: &'static str,
  command: &'static str,
  hasher: Box<dyn DynDigest>,
}

fn main() {
  let hasher_impls: Vec<GeneratedHasherImpl> = vec![
    GeneratedHasherImpl {
      dependency: "ascon_hash::AsconHash",
      command: "ascon",
      hasher: Box::new(ascon_hash::AsconHash::default()),
    },
    GeneratedHasherImpl {
      dependency: "ascon_hash::AsconAHash",
      command: "ascon-a",
      hasher: Box::new(ascon_hash::AsconAHash::default()),
    },
    GeneratedHasherImpl {
      dependency: "belt_hash::BeltHash",
      command: "belt",
      hasher: Box::new(belt_hash::BeltHash::default()),
    },
    GeneratedHasherImpl {
      dependency: "blake2::Blake2s256",
      command: "blake2s-256",
      hasher: Box::new(blake2::Blake2s256::default()),
    },
    GeneratedHasherImpl {
      dependency: "blake2::Blake2b512",
      command: "blake2b-512",
      hasher: Box::new(blake2::Blake2b512::default()),
    },
    GeneratedHasherImpl {
      dependency: "fsb::Fsb160",
      command: "fsb160",
      hasher: Box::new(fsb::Fsb160::default()),
    },
    GeneratedHasherImpl {
      dependency: "fsb::Fsb224",
      command: "fsb224",
      hasher: Box::new(fsb::Fsb224::default()),
    },
    GeneratedHasherImpl {
      dependency: "fsb::Fsb256",
      command: "fsb256",
      hasher: Box::new(fsb::Fsb256::default()),
    },
    GeneratedHasherImpl {
      dependency: "fsb::Fsb384",
      command: "fsb384",
      hasher: Box::new(fsb::Fsb384::default()),
    },
    GeneratedHasherImpl {
      dependency: "fsb::Fsb512",
      command: "fsb512",
      hasher: Box::new(fsb::Fsb512::default()),
    },
    GeneratedHasherImpl {
      dependency: "gost94::Gost94CryptoPro",
      command: "gost94-crypto-pro",
      hasher: Box::new(gost94::Gost94CryptoPro::default()),
    },
    GeneratedHasherImpl {
      dependency: "gost94::Gost94UA",
      command: "gost94-ua",
      hasher: Box::new(gost94::Gost94UA::default()),
    },
    GeneratedHasherImpl {
      dependency: "gost94::Gost94s2015",
      command: "gost94-2015",
      hasher: Box::new(gost94::Gost94s2015::default()),
    },
    GeneratedHasherImpl {
      dependency: "groestl::Groestl224",
      command: "goestl224",
      hasher: Box::new(groestl::Groestl224::default()),
    },
    GeneratedHasherImpl {
      dependency: "groestl::Groestl256",
      command: "groestl256",
      hasher: Box::new(groestl::Groestl256::default()),
    },
    GeneratedHasherImpl {
      dependency: "groestl::Groestl384",
      command: "groestl384",
      hasher: Box::new(groestl::Groestl384::default()),
    },
    GeneratedHasherImpl {
      dependency: "groestl::Groestl512",
      command: "groestl512",
      hasher: Box::new(groestl::Groestl512::default()),
    },
    GeneratedHasherImpl {
      dependency: "jh::Jh224",
      command: "jh224",
      hasher: Box::new(jh::Jh224::default()),
    },
    GeneratedHasherImpl {
      dependency: "jh::Jh256",
      command: "jh256",
      hasher: Box::new(jh::Jh256::default()),
    },
    GeneratedHasherImpl {
      dependency: "jh::Jh384",
      command: "jh384",
      hasher: Box::new(jh::Jh384::default()),
    },
    GeneratedHasherImpl {
      dependency: "jh::Jh512",
      command: "jh512",
      hasher: Box::new(jh::Jh512::default()),
    },
    GeneratedHasherImpl {
      dependency: "md2::Md2",
      command: "md2",
      hasher: Box::new(md2::Md2::default()),
    },
    GeneratedHasherImpl {
      dependency: "md4::Md4",
      command: "md4",
      hasher: Box::new(md4::Md4::default()),
    },
    GeneratedHasherImpl {
      dependency: "ripemd::Ripemd128",
      command: "ripemd128",
      hasher: Box::new(ripemd::Ripemd128::default()),
    },
    GeneratedHasherImpl {
      dependency: "ripemd::Ripemd160",
      command: "ripemd160",
      hasher: Box::new(ripemd::Ripemd160::default()),
    },
    GeneratedHasherImpl {
      dependency: "ripemd::Ripemd256",
      command: "ripemd256",
      hasher: Box::new(ripemd::Ripemd256::default()),
    },
    GeneratedHasherImpl {
      dependency: "ripemd::Ripemd320",
      command: "ripemd320",
      hasher: Box::new(ripemd::Ripemd320::default()),
    },
    GeneratedHasherImpl {
      dependency: "sha1::Sha1",
      command: "sha1",
      hasher: Box::new(sha1::Sha1::default()),
    },
    GeneratedHasherImpl {
      dependency: "sha2::Sha224",
      command: "sha224",
      hasher: Box::new(sha2::Sha224::default()),
    },
    GeneratedHasherImpl {
      dependency: "sha2::Sha384",
      command: "sha384",
      hasher: Box::new(sha2::Sha384::default()),
    },
    GeneratedHasherImpl {
      dependency: "sha2::Sha512",
      command: "sha512",
      hasher: Box::new(sha2::Sha512::default()),
    },
    GeneratedHasherImpl {
      dependency: "sha2::Sha512_224",
      command: "sha512-224",
      hasher: Box::new(sha2::Sha512_224::default()),
    },
    GeneratedHasherImpl {
      dependency: "sha2::Sha512_256",
      command: "sha512-256",
      hasher: Box::new(sha2::Sha512_256::default()),
    },
    GeneratedHasherImpl {
      dependency: "sha3::Sha3_224",
      command: "sha3-224",
      hasher: Box::new(sha3::Sha3_224::default()),
    },
    GeneratedHasherImpl {
      dependency: "sha3::Sha3_256",
      command: "sha3-256",
      hasher: Box::new(sha3::Sha3_256::default()),
    },
    GeneratedHasherImpl {
      dependency: "sha3::Sha3_384",
      command: "sha3-384",
      hasher: Box::new(sha3::Sha3_384::default()),
    },
    GeneratedHasherImpl {
      dependency: "sha3::Sha3_512",
      command: "sha3-512",
      hasher: Box::new(sha3::Sha3_512::default()),
    },
    GeneratedHasherImpl {
      dependency: "sha3::Keccak224",
      command: "keccak224",
      hasher: Box::new(sha3::Keccak224::default()),
    },
    GeneratedHasherImpl {
      dependency: "sha3::Keccak256",
      command: "keccak256",
      hasher: Box::new(sha3::Keccak256::default()),
    },
    GeneratedHasherImpl {
      dependency: "sha3::Keccak384",
      command: "keccak384",
      hasher: Box::new(sha3::Keccak384::default()),
    },
    GeneratedHasherImpl {
      dependency: "sha3::Keccak512",
      command: "keccak512",
      hasher: Box::new(sha3::Keccak512::default()),
    },
    GeneratedHasherImpl {
      dependency: "shabal::Shabal192",
      command: "shabal192",
      hasher: Box::new(shabal::Shabal192::default()),
    },
    GeneratedHasherImpl {
      dependency: "shabal::Shabal224",
      command: "shabal224",
      hasher: Box::new(shabal::Shabal224::default()),
    },
    GeneratedHasherImpl {
      dependency: "shabal::Shabal256",
      command: "shabal256",
      hasher: Box::new(shabal::Shabal256::default()),
    },
    GeneratedHasherImpl {
      dependency: "shabal::Shabal384",
      command: "shabal384",
      hasher: Box::new(shabal::Shabal384::default()),
    },
    GeneratedHasherImpl {
      dependency: "shabal::Shabal512",
      command: "shabal512",
      hasher: Box::new(shabal::Shabal512::default()),
    },
    GeneratedHasherImpl {
      dependency: "skein::Skein256::<skein::consts::U32>",
      command: "skein256-32",
      hasher: Box::new(skein::Skein256::<skein::consts::U32>::default()),
    },
    GeneratedHasherImpl {
      dependency: "skein::Skein256::<skein::consts::U64>",
      command: "skein256-64",
      hasher: Box::new(skein::Skein256::<skein::consts::U64>::default()),
    },
    GeneratedHasherImpl {
      dependency: "skein::Skein256::<skein::consts::U128>",
      command: "skein256-128",
      hasher: Box::new(skein::Skein256::<skein::consts::U128>::default()),
    },
    GeneratedHasherImpl {
      dependency: "skein::Skein512::<skein::consts::U32>",
      command: "skein512-32",
      hasher: Box::new(skein::Skein512::<skein::consts::U32>::default()),
    },
    GeneratedHasherImpl {
      dependency: "skein::Skein512::<skein::consts::U64>",
      command: "skein512-64",
      hasher: Box::new(skein::Skein512::<skein::consts::U64>::default()),
    },
    GeneratedHasherImpl {
      dependency: "skein::Skein512::<skein::consts::U128>",
      command: "skein512-128",
      hasher: Box::new(skein::Skein512::<skein::consts::U128>::default()),
    },
    GeneratedHasherImpl {
      dependency: "skein::Skein1024::<skein::consts::U32>",
      command: "skein1024-32",
      hasher: Box::new(skein::Skein1024::<skein::consts::U32>::default()),
    },
    GeneratedHasherImpl {
      dependency: "skein::Skein1024::<skein::consts::U64>",
      command: "skein1024-64",
      hasher: Box::new(skein::Skein1024::<skein::consts::U64>::default()),
    },
    GeneratedHasherImpl {
      dependency: "skein::Skein1024::<skein::consts::U128>",
      command: "skein1024-128",
      hasher: Box::new(skein::Skein1024::<skein::consts::U128>::default()),
    },
    GeneratedHasherImpl {
      dependency: "sm3::Sm3",
      command: "sm3",
      hasher: Box::new(sm3::Sm3::default()),
    },
    GeneratedHasherImpl {
      dependency: "streebog::Streebog256",
      command: "streebog256",
      hasher: Box::new(streebog::Streebog256::default()),
    },
    GeneratedHasherImpl {
      dependency: "streebog::Streebog512",
      command: "streebog512",
      hasher: Box::new(streebog::Streebog512::default()),
    },
    GeneratedHasherImpl {
      dependency: "tiger::Tiger",
      command: "tiger",
      hasher: Box::new(tiger::Tiger::default()),
    },
    GeneratedHasherImpl {
      dependency: "tiger::Tiger2",
      command: "tiger2",
      hasher: Box::new(tiger::Tiger2::default()),
    },
    GeneratedHasherImpl {
      dependency: "whirlpool::Whirlpool",
      command: "whirlpool",
      hasher: Box::new(whirlpool::Whirlpool::default()),
    },
  ];

  let mut hashers_generated_file =
    std::fs::File::create("./src/hashers_generated.rs").unwrap();

  writeln!(
    hashers_generated_file,
    "use nu_protocol::{{Example, Span, Value}};\nuse crate::hasher::Hasher;"
  )
  .unwrap();

  for mut hasher_impl in hasher_impls {
    let dependency = hasher_impl.dependency;
    let command = hasher_impl.command;
    hasher_impl.hasher.update(TEST_TEXT.as_bytes());
    let hash = hasher_impl.hasher.finalize();

    writeln!(
      hashers_generated_file,
      "
impl Hasher for {dependency} {{
  fn name() -> &'static str {{
    \"{command}\"
  }}

  fn examples() -> Vec<Example<'static>> {{
    vec![
      Example {{
        description: \"Return the {command} hash of a string, hex-encoded\",
        example: \"'{TEST_TEXT}' | hash {command}\",
        result: Some(Value::string(
          \"{}\".to_owned(),
          Span::test_data(),
        )),
      }},
      Example {{
        description: \"Return the {command} hash of a string, as binary\",
        example: \"'{TEST_TEXT}' | hash {command} --binary\",
        result: Some(Value::binary(
          vec![{}],
          Span::test_data(),
        )),
      }},
      Example {{
        description: \"Return the {command} hash of a file's contents\",
        example: \"open ./nu_0_24_1_windows.zip | hash {command}\",
        result: None,
      }},
    ]
  }}
}}",
      hash
        .iter()
        .map(|b| format!("{b:02x?}"))
        .collect::<Vec<_>>()
        .join(""),
      hash
        .iter()
        .map(|b| format!("0x{b:02x?}"))
        .collect::<Vec<_>>()
        .join(",")
    )
    .unwrap();
  }
  hashers_generated_file.flush().unwrap();
}
