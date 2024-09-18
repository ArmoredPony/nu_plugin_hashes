mod hasher;
mod hashers_generated;

use hasher::GenericHasher;
use nu_plugin::Plugin;

pub struct HashesPlugin;

impl Plugin for HashesPlugin {
  fn version(&self) -> String {
    env!("CARGO_PKG_VERSION").into()
  }

  fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
    vec![
      Box::new(GenericHasher::<ascon_hash::AsconHash>::default()),
      Box::new(GenericHasher::<ascon_hash::AsconAHash>::default()),
      Box::new(GenericHasher::<belt_hash::BeltHash>::default()),
      Box::new(GenericHasher::<blake2::Blake2s256>::default()),
      Box::new(GenericHasher::<blake2::Blake2b512>::default()),
      // TODO: try to add `blake2::Blake2bVar`
      Box::new(GenericHasher::<fsb::Fsb160>::default()),
      Box::new(GenericHasher::<fsb::Fsb224>::default()),
      Box::new(GenericHasher::<fsb::Fsb256>::default()),
      Box::new(GenericHasher::<fsb::Fsb384>::default()),
      Box::new(GenericHasher::<fsb::Fsb512>::default()),
      Box::new(GenericHasher::<gost94::Gost94CryptoPro>::default()),
      Box::new(GenericHasher::<gost94::Gost94UA>::default()),
      Box::new(GenericHasher::<gost94::Gost94s2015>::default()),
      Box::new(GenericHasher::<groestl::Groestl224>::default()),
      Box::new(GenericHasher::<groestl::Groestl256>::default()),
      Box::new(GenericHasher::<groestl::Groestl384>::default()),
      Box::new(GenericHasher::<groestl::Groestl512>::default()),
      Box::new(GenericHasher::<jh::Jh224>::default()),
      Box::new(GenericHasher::<jh::Jh256>::default()),
      Box::new(GenericHasher::<jh::Jh384>::default()),
      Box::new(GenericHasher::<jh::Jh512>::default()),
      Box::new(GenericHasher::<md2::Md2>::default()),
      Box::new(GenericHasher::<md4::Md4>::default()),
      // MD5 is skipped on purpose
      Box::new(GenericHasher::<ripemd::Ripemd128>::default()),
      Box::new(GenericHasher::<ripemd::Ripemd160>::default()),
      Box::new(GenericHasher::<ripemd::Ripemd256>::default()),
      Box::new(GenericHasher::<ripemd::Ripemd320>::default()),
      Box::new(GenericHasher::<sha1::Sha1>::default()),
      // TODO: try to add `sha1_checked::Sha1`
      Box::new(GenericHasher::<sha2::Sha224>::default()),
      // SHA256 is skipped on purpose
      Box::new(GenericHasher::<sha2::Sha384>::default()),
      Box::new(GenericHasher::<sha2::Sha512>::default()),
      Box::new(GenericHasher::<sha2::Sha512_224>::default()),
      Box::new(GenericHasher::<sha2::Sha512_256>::default()),
      Box::new(GenericHasher::<sha3::Sha3_224>::default()),
      Box::new(GenericHasher::<sha3::Sha3_256>::default()),
      Box::new(GenericHasher::<sha3::Sha3_384>::default()),
      Box::new(GenericHasher::<sha3::Sha3_512>::default()),
      Box::new(GenericHasher::<sha3::Keccak224>::default()),
      Box::new(GenericHasher::<sha3::Keccak256>::default()),
      Box::new(GenericHasher::<sha3::Keccak384>::default()),
      Box::new(GenericHasher::<sha3::Keccak512>::default()),
      Box::new(GenericHasher::<shabal::Shabal192>::default()),
      Box::new(GenericHasher::<shabal::Shabal224>::default()),
      Box::new(GenericHasher::<shabal::Shabal256>::default()),
      Box::new(GenericHasher::<shabal::Shabal384>::default()),
      Box::new(GenericHasher::<shabal::Shabal512>::default()),
      Box::new(GenericHasher::<skein::Skein256<skein::consts::U32>>::default()),
      Box::new(GenericHasher::<skein::Skein256<skein::consts::U64>>::default()),
      Box::new(
        GenericHasher::<skein::Skein256<skein::consts::U128>>::default(),
      ),
      Box::new(GenericHasher::<skein::Skein512<skein::consts::U32>>::default()),
      Box::new(GenericHasher::<skein::Skein512<skein::consts::U64>>::default()),
      Box::new(
        GenericHasher::<skein::Skein512<skein::consts::U128>>::default(),
      ),
      Box::new(
        GenericHasher::<skein::Skein1024<skein::consts::U32>>::default(),
      ),
      Box::new(
        GenericHasher::<skein::Skein1024<skein::consts::U64>>::default(),
      ),
      Box::new(
        GenericHasher::<skein::Skein1024<skein::consts::U128>>::default(),
      ),
      Box::new(GenericHasher::<sm3::Sm3>::default()),
      Box::new(GenericHasher::<streebog::Streebog256>::default()),
      Box::new(GenericHasher::<streebog::Streebog512>::default()),
      Box::new(GenericHasher::<tiger::Tiger>::default()),
      Box::new(GenericHasher::<tiger::Tiger2>::default()),
      Box::new(GenericHasher::<whirlpool::Whirlpool>::default()),
    ]
  }
}
