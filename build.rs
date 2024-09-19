use digest::DynDigest;

const TEST_TEXT: &str = "abcdefghijklmnopqrstuvwxyz";

struct GeneratedHasherImplMeta {
  crate_name: &'static str,
  hasher_type_name: &'static str,
  hasher_command: &'static str,
  hasher: Box<dyn DynDigest>,
}

#[cfg(not(any(
  feature = "ascon-hash",
  feature = "belt-hash",
  feature = "blake2",
  feature = "fsb",
  feature = "gost94",
  feature = "groestl",
  feature = "jh",
  feature = "md2",
  feature = "md4",
  feature = "ripemd",
  feature = "sha1",
  feature = "sha2",
  feature = "sha3",
  feature = "shabal",
  feature = "skein",
  feature = "sm3",
  feature = "streebog",
  feature = "tiger",
  feature = "whirlpool",
)))]
fn main() {
  compile_error!("enable at least one feature to compile this plugin");
}

#[cfg(any(
  feature = "ascon-hash",
  feature = "belt-hash",
  feature = "blake2",
  feature = "fsb",
  feature = "gost94",
  feature = "groestl",
  feature = "jh",
  feature = "md2",
  feature = "md4",
  feature = "ripemd",
  feature = "sha1",
  feature = "sha2",
  feature = "sha3",
  feature = "shabal",
  feature = "skein",
  feature = "sm3",
  feature = "streebog",
  feature = "tiger",
  feature = "whirlpool",
))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
  use std::io::Write;

  // MD5 and SHA256 are skipped on purpose
  let hasher_impls: Vec<GeneratedHasherImplMeta> = vec![
    #[cfg(feature = "ascon-hash")]
    GeneratedHasherImplMeta {
      crate_name: "ascon_hash",
      hasher_type_name: "AsconHash",
      hasher_command: "ascon",
      hasher: Box::new(ascon_hash::AsconHash::default()),
    },
    #[cfg(feature = "ascon-hash")]
    GeneratedHasherImplMeta {
      crate_name: "ascon_hash",
      hasher_type_name: "AsconAHash",
      hasher_command: "ascon-a",
      hasher: Box::new(ascon_hash::AsconAHash::default()),
    },
    #[cfg(feature = "belt-hash")]
    GeneratedHasherImplMeta {
      crate_name: "belt_hash",
      hasher_type_name: "BeltHash",
      hasher_command: "belt",
      hasher: Box::new(belt_hash::BeltHash::default()),
    },
    #[cfg(feature = "blake2")]
    GeneratedHasherImplMeta {
      crate_name: "blake2",
      hasher_type_name: "Blake2s256",
      hasher_command: "blake2s-256",
      hasher: Box::new(blake2::Blake2s256::default()),
    },
    #[cfg(feature = "blake2")]
    GeneratedHasherImplMeta {
      crate_name: "blake2",
      hasher_type_name: "Blake2b512",
      hasher_command: "blake2b-512",
      hasher: Box::new(blake2::Blake2b512::default()),
    },
    #[cfg(feature = "fsb")]
    GeneratedHasherImplMeta {
      crate_name: "fsb",
      hasher_type_name: "Fsb160",
      hasher_command: "fsb160",
      hasher: Box::new(fsb::Fsb160::default()),
    },
    #[cfg(feature = "fsb")]
    GeneratedHasherImplMeta {
      crate_name: "fsb",
      hasher_type_name: "Fsb224",
      hasher_command: "fsb224",
      hasher: Box::new(fsb::Fsb224::default()),
    },
    #[cfg(feature = "fsb")]
    GeneratedHasherImplMeta {
      crate_name: "fsb",
      hasher_type_name: "Fsb256",
      hasher_command: "fsb256",
      hasher: Box::new(fsb::Fsb256::default()),
    },
    #[cfg(feature = "fsb")]
    GeneratedHasherImplMeta {
      crate_name: "fsb",
      hasher_type_name: "Fsb384",
      hasher_command: "fsb384",
      hasher: Box::new(fsb::Fsb384::default()),
    },
    #[cfg(feature = "fsb")]
    GeneratedHasherImplMeta {
      crate_name: "fsb",
      hasher_type_name: "Fsb512",
      hasher_command: "fsb512",
      hasher: Box::new(fsb::Fsb512::default()),
    },
    #[cfg(feature = "gost94")]
    GeneratedHasherImplMeta {
      crate_name: "gost94",
      hasher_type_name: "Gost94CryptoPro",
      hasher_command: "gost94-crypto-pro",
      hasher: Box::new(gost94::Gost94CryptoPro::default()),
    },
    #[cfg(feature = "gost94")]
    GeneratedHasherImplMeta {
      crate_name: "gost94",
      hasher_type_name: "Gost94UA",
      hasher_command: "gost94-ua",
      hasher: Box::new(gost94::Gost94UA::default()),
    },
    #[cfg(feature = "gost94")]
    GeneratedHasherImplMeta {
      crate_name: "gost94",
      hasher_type_name: "Gost94s2015",
      hasher_command: "gost94-2015",
      hasher: Box::new(gost94::Gost94s2015::default()),
    },
    #[cfg(feature = "groestl")]
    GeneratedHasherImplMeta {
      crate_name: "groestl",
      hasher_type_name: "Groestl224",
      hasher_command: "goestl224",
      hasher: Box::new(groestl::Groestl224::default()),
    },
    #[cfg(feature = "groestl")]
    GeneratedHasherImplMeta {
      crate_name: "groestl",
      hasher_type_name: "Groestl256",
      hasher_command: "groestl256",
      hasher: Box::new(groestl::Groestl256::default()),
    },
    #[cfg(feature = "groestl")]
    GeneratedHasherImplMeta {
      crate_name: "groestl",
      hasher_type_name: "Groestl384",
      hasher_command: "groestl384",
      hasher: Box::new(groestl::Groestl384::default()),
    },
    #[cfg(feature = "groestl")]
    GeneratedHasherImplMeta {
      crate_name: "groestl",
      hasher_type_name: "Groestl512",
      hasher_command: "groestl512",
      hasher: Box::new(groestl::Groestl512::default()),
    },
    #[cfg(feature = "jh")]
    GeneratedHasherImplMeta {
      crate_name: "jh",
      hasher_type_name: "Jh224",
      hasher_command: "jh224",
      hasher: Box::new(jh::Jh224::default()),
    },
    #[cfg(feature = "jh")]
    GeneratedHasherImplMeta {
      crate_name: "jh",
      hasher_type_name: "Jh256",
      hasher_command: "jh256",
      hasher: Box::new(jh::Jh256::default()),
    },
    #[cfg(feature = "jh")]
    GeneratedHasherImplMeta {
      crate_name: "jh",
      hasher_type_name: "Jh384",
      hasher_command: "jh384",
      hasher: Box::new(jh::Jh384::default()),
    },
    #[cfg(feature = "jh")]
    GeneratedHasherImplMeta {
      crate_name: "jh",
      hasher_type_name: "Jh512",
      hasher_command: "jh512",
      hasher: Box::new(jh::Jh512::default()),
    },
    #[cfg(feature = "md2")]
    GeneratedHasherImplMeta {
      crate_name: "md2",
      hasher_type_name: "Md2",
      hasher_command: "md2",
      hasher: Box::new(md2::Md2::default()),
    },
    #[cfg(feature = "md4")]
    GeneratedHasherImplMeta {
      crate_name: "md4",
      hasher_type_name: "Md4",
      hasher_command: "md4",
      hasher: Box::new(md4::Md4::default()),
    },
    #[cfg(feature = "ripemd")]
    GeneratedHasherImplMeta {
      crate_name: "ripemd",
      hasher_type_name: "Ripemd128",
      hasher_command: "ripemd128",
      hasher: Box::new(ripemd::Ripemd128::default()),
    },
    #[cfg(feature = "ripemd")]
    GeneratedHasherImplMeta {
      crate_name: "ripemd",
      hasher_type_name: "Ripemd160",
      hasher_command: "ripemd160",
      hasher: Box::new(ripemd::Ripemd160::default()),
    },
    #[cfg(feature = "ripemd")]
    GeneratedHasherImplMeta {
      crate_name: "ripemd",
      hasher_type_name: "Ripemd256",
      hasher_command: "ripemd256",
      hasher: Box::new(ripemd::Ripemd256::default()),
    },
    #[cfg(feature = "ripemd")]
    GeneratedHasherImplMeta {
      crate_name: "ripemd",
      hasher_type_name: "Ripemd320",
      hasher_command: "ripemd320",
      hasher: Box::new(ripemd::Ripemd320::default()),
    },
    #[cfg(feature = "sha1")]
    GeneratedHasherImplMeta {
      crate_name: "sha1",
      hasher_type_name: "Sha1",
      hasher_command: "sha1",
      hasher: Box::new(sha1::Sha1::default()),
    },
    #[cfg(feature = "sha2")]
    GeneratedHasherImplMeta {
      crate_name: "sha2",
      hasher_type_name: "Sha224",
      hasher_command: "sha224",
      hasher: Box::new(sha2::Sha224::default()),
    },
    #[cfg(feature = "sha2")]
    GeneratedHasherImplMeta {
      crate_name: "sha2",
      hasher_type_name: "Sha384",
      hasher_command: "sha384",
      hasher: Box::new(sha2::Sha384::default()),
    },
    #[cfg(feature = "sha2")]
    GeneratedHasherImplMeta {
      crate_name: "sha2",
      hasher_type_name: "Sha512",
      hasher_command: "sha512",
      hasher: Box::new(sha2::Sha512::default()),
    },
    #[cfg(feature = "sha2")]
    GeneratedHasherImplMeta {
      crate_name: "sha2",
      hasher_type_name: "Sha512_224",
      hasher_command: "sha512-224",
      hasher: Box::new(sha2::Sha512_224::default()),
    },
    #[cfg(feature = "sha2")]
    GeneratedHasherImplMeta {
      crate_name: "sha2",
      hasher_type_name: "Sha512_256",
      hasher_command: "sha512-256",
      hasher: Box::new(sha2::Sha512_256::default()),
    },
    #[cfg(feature = "sha3")]
    GeneratedHasherImplMeta {
      crate_name: "sha3",
      hasher_type_name: "Sha3_224",
      hasher_command: "sha3-224",
      hasher: Box::new(sha3::Sha3_224::default()),
    },
    #[cfg(feature = "sha3")]
    GeneratedHasherImplMeta {
      crate_name: "sha3",
      hasher_type_name: "Sha3_256",
      hasher_command: "sha3-256",
      hasher: Box::new(sha3::Sha3_256::default()),
    },
    #[cfg(feature = "sha3")]
    GeneratedHasherImplMeta {
      crate_name: "sha3",
      hasher_type_name: "Sha3_384",
      hasher_command: "sha3-384",
      hasher: Box::new(sha3::Sha3_384::default()),
    },
    #[cfg(feature = "sha3")]
    GeneratedHasherImplMeta {
      crate_name: "sha3",
      hasher_type_name: "Sha3_512",
      hasher_command: "sha3-512",
      hasher: Box::new(sha3::Sha3_512::default()),
    },
    #[cfg(feature = "sha3")]
    GeneratedHasherImplMeta {
      crate_name: "sha3",
      hasher_type_name: "Keccak224",
      hasher_command: "keccak224",
      hasher: Box::new(sha3::Keccak224::default()),
    },
    #[cfg(feature = "sha3")]
    GeneratedHasherImplMeta {
      crate_name: "sha3",
      hasher_type_name: "Keccak256",
      hasher_command: "keccak256",
      hasher: Box::new(sha3::Keccak256::default()),
    },
    #[cfg(feature = "sha3")]
    GeneratedHasherImplMeta {
      crate_name: "sha3",
      hasher_type_name: "Keccak384",
      hasher_command: "keccak384",
      hasher: Box::new(sha3::Keccak384::default()),
    },
    #[cfg(feature = "sha3")]
    GeneratedHasherImplMeta {
      crate_name: "sha3",
      hasher_type_name: "Keccak512",
      hasher_command: "keccak512",
      hasher: Box::new(sha3::Keccak512::default()),
    },
    #[cfg(feature = "shabal")]
    GeneratedHasherImplMeta {
      crate_name: "shabal",
      hasher_type_name: "Shabal192",
      hasher_command: "shabal192",
      hasher: Box::new(shabal::Shabal192::default()),
    },
    #[cfg(feature = "shabal")]
    GeneratedHasherImplMeta {
      crate_name: "shabal",
      hasher_type_name: "Shabal224",
      hasher_command: "shabal224",
      hasher: Box::new(shabal::Shabal224::default()),
    },
    #[cfg(feature = "shabal")]
    GeneratedHasherImplMeta {
      crate_name: "shabal",
      hasher_type_name: "Shabal256",
      hasher_command: "shabal256",
      hasher: Box::new(shabal::Shabal256::default()),
    },
    #[cfg(feature = "shabal")]
    GeneratedHasherImplMeta {
      crate_name: "shabal",
      hasher_type_name: "Shabal384",
      hasher_command: "shabal384",
      hasher: Box::new(shabal::Shabal384::default()),
    },
    #[cfg(feature = "shabal")]
    GeneratedHasherImplMeta {
      crate_name: "shabal",
      hasher_type_name: "Shabal512",
      hasher_command: "shabal512",
      hasher: Box::new(shabal::Shabal512::default()),
    },
    #[cfg(feature = "skein")]
    GeneratedHasherImplMeta {
      crate_name: "skein",
      hasher_type_name: "Skein256::<skein::consts::U32>",
      hasher_command: "skein256-32",
      hasher: Box::new(skein::Skein256::<skein::consts::U32>::default()),
    },
    #[cfg(feature = "skein")]
    GeneratedHasherImplMeta {
      crate_name: "skein",
      hasher_type_name: "Skein256::<skein::consts::U64>",
      hasher_command: "skein256-64",
      hasher: Box::new(skein::Skein256::<skein::consts::U64>::default()),
    },
    #[cfg(feature = "skein")]
    GeneratedHasherImplMeta {
      crate_name: "skein",
      hasher_type_name: "Skein256::<skein::consts::U128>",
      hasher_command: "skein256-128",
      hasher: Box::new(skein::Skein256::<skein::consts::U128>::default()),
    },
    #[cfg(feature = "skein")]
    GeneratedHasherImplMeta {
      crate_name: "skein",
      hasher_type_name: "Skein512::<skein::consts::U32>",
      hasher_command: "skein512-32",
      hasher: Box::new(skein::Skein512::<skein::consts::U32>::default()),
    },
    #[cfg(feature = "skein")]
    GeneratedHasherImplMeta {
      crate_name: "skein",
      hasher_type_name: "Skein512::<skein::consts::U64>",
      hasher_command: "skein512-64",
      hasher: Box::new(skein::Skein512::<skein::consts::U64>::default()),
    },
    #[cfg(feature = "skein")]
    GeneratedHasherImplMeta {
      crate_name: "skein",
      hasher_type_name: "Skein512::<skein::consts::U128>",
      hasher_command: "skein512-128",
      hasher: Box::new(skein::Skein512::<skein::consts::U128>::default()),
    },
    #[cfg(feature = "skein")]
    GeneratedHasherImplMeta {
      crate_name: "skein",
      hasher_type_name: "Skein1024::<skein::consts::U32>",
      hasher_command: "skein1024-32",
      hasher: Box::new(skein::Skein1024::<skein::consts::U32>::default()),
    },
    #[cfg(feature = "skein")]
    GeneratedHasherImplMeta {
      crate_name: "skein",
      hasher_type_name: "Skein1024::<skein::consts::U64>",
      hasher_command: "skein1024-64",
      hasher: Box::new(skein::Skein1024::<skein::consts::U64>::default()),
    },
    #[cfg(feature = "skein")]
    GeneratedHasherImplMeta {
      crate_name: "skein",
      hasher_type_name: "Skein1024::<skein::consts::U128>",
      hasher_command: "skein1024-128",
      hasher: Box::new(skein::Skein1024::<skein::consts::U128>::default()),
    },
    #[cfg(feature = "sm3")]
    GeneratedHasherImplMeta {
      crate_name: "sm3",
      hasher_type_name: "Sm3",
      hasher_command: "sm3",
      hasher: Box::new(sm3::Sm3::default()),
    },
    #[cfg(feature = "streebog")]
    GeneratedHasherImplMeta {
      crate_name: "streebog",
      hasher_type_name: "Streebog256",
      hasher_command: "streebog256",
      hasher: Box::new(streebog::Streebog256::default()),
    },
    #[cfg(feature = "streebog")]
    GeneratedHasherImplMeta {
      crate_name: "streebog",
      hasher_type_name: "Streebog512",
      hasher_command: "streebog512",
      hasher: Box::new(streebog::Streebog512::default()),
    },
    #[cfg(feature = "tiger")]
    GeneratedHasherImplMeta {
      crate_name: "tiger",
      hasher_type_name: "Tiger",
      hasher_command: "tiger",
      hasher: Box::new(tiger::Tiger::default()),
    },
    #[cfg(feature = "tiger")]
    GeneratedHasherImplMeta {
      crate_name: "tiger",
      hasher_type_name: "Tiger2",
      hasher_command: "tiger2",
      hasher: Box::new(tiger::Tiger2::default()),
    },
    #[cfg(feature = "whirlpool")]
    GeneratedHasherImplMeta {
      crate_name: "whirlpool",
      hasher_type_name: "Whirlpool",
      hasher_command: "whirlpool",
      hasher: Box::new(whirlpool::Whirlpool::default()),
    },
  ];

  let mut hashers_generated_file =
    std::fs::File::create("./src/hashers_generated.rs").unwrap();
  let mut commands_generated_file =
    std::fs::File::create("./src/commands_generated.rs").unwrap();

  write!(
    hashers_generated_file,
    "#![allow(unused_imports)]

use nu_protocol::{{Example, Span, Value, ShellError}};
use nu_plugin::PluginCommand;
use crate::hasher::{{Hasher, GenericHasher}};
use crate::HashesPlugin;
"
  )?;

  write!(
    commands_generated_file,
    "use nu_plugin::PluginCommand;
use crate::{{HashesPlugin, hasher::GenericHasher}};

pub fn commands() -> Vec<Box<dyn PluginCommand<Plugin = HashesPlugin>>> {{
  vec![
"
  )?;

  for mut hasher_impl_meta in hasher_impls {
    let feature_name =
      hasher_impl_meta.crate_name.replace("-", "_").to_uppercase();
    if std::env::var(format!("CARGO_FEATURE_{feature_name}")).is_ok() {
      hashers_generated_file
        .write_all(build_impl_str(&mut hasher_impl_meta).as_bytes())?;
      hashers_generated_file
        .write_all(build_test_str(&hasher_impl_meta).as_bytes())?;

      let crate_name = hasher_impl_meta.crate_name;
      let hasher_type_name = hasher_impl_meta.hasher_type_name;
      writeln!(
        commands_generated_file,
        "    Box::new(GenericHasher::<{crate_name}::{hasher_type_name}>::default()),"
      )?;
    }
  }

  write!(
    commands_generated_file,
    "  ]
}}"
  )?;

  hashers_generated_file.flush()?;
  commands_generated_file.flush()?;

  Ok(())
}

#[cfg(any(
  feature = "ascon-hash",
  feature = "belt-hash",
  feature = "blake2",
  feature = "fsb",
  feature = "gost94",
  feature = "groestl",
  feature = "jh",
  feature = "md2",
  feature = "md4",
  feature = "ripemd",
  feature = "sha1",
  feature = "sha2",
  feature = "sha3",
  feature = "shabal",
  feature = "skein",
  feature = "sm3",
  feature = "streebog",
  feature = "tiger",
  feature = "whirlpool",
))]
fn build_impl_str(meta: &mut GeneratedHasherImplMeta) -> String {
  let crate_name = meta.crate_name;
  let hasher_type_name = meta.hasher_type_name;
  let command = meta.hasher_command;
  let mut hasher = meta.hasher.clone();
  hasher.update(TEST_TEXT.as_bytes());
  let hash = hasher.clone().finalize();
  format!(
    "
impl Hasher for {crate_name}::{hasher_type_name} {{
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
}}
",
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
}

#[cfg(any(
  feature = "ascon-hash",
  feature = "belt-hash",
  feature = "blake2",
  feature = "fsb",
  feature = "gost94",
  feature = "groestl",
  feature = "jh",
  feature = "md2",
  feature = "md4",
  feature = "ripemd",
  feature = "sha1",
  feature = "sha2",
  feature = "sha3",
  feature = "shabal",
  feature = "skein",
  feature = "sm3",
  feature = "streebog",
  feature = "tiger",
  feature = "whirlpool",
))]
fn build_test_str(meta: &GeneratedHasherImplMeta) -> String {
  let crate_name = meta.crate_name;
  let hasher_type_name = meta.hasher_type_name;
  let command = meta.hasher_command;
  let test_name = command.replace("-", "_");
  format!(
    "
#[test]
fn test_{test_name}_examples() -> Result<(), ShellError> {{
  nu_plugin_test_support::PluginTest::new
    (
      \"hash {command}\",
      HashesPlugin.into()
    )?
    .test_examples(&GenericHasher::<{crate_name}::{hasher_type_name}>::default().examples())
}}
  "
  )
}
