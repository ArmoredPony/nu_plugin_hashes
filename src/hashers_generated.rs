use nu_protocol::{Example, Span, Value};
use crate::hasher::Hasher;

impl Hasher for ascon_hash::AsconHash {
  fn name() -> &'static str {
    "ascon"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the ascon hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash ascon",
        result: Some(Value::string(
          "c62368674e1b2301f19f46c50bb7f87a988a3e41205d68ab9d7882d2a15e917b".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the ascon hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash ascon --binary",
        result: Some(Value::binary(
          vec![0xc6,0x23,0x68,0x67,0x4e,0x1b,0x23,0x01,0xf1,0x9f,0x46,0xc5,0x0b,0xb7,0xf8,0x7a,0x98,0x8a,0x3e,0x41,0x20,0x5d,0x68,0xab,0x9d,0x78,0x82,0xd2,0xa1,0x5e,0x91,0x7b],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the ascon hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash ascon",
        result: None,
      },
    ]
  }
}

impl Hasher for ascon_hash::AsconAHash {
  fn name() -> &'static str {
    "ascon-a"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the ascon-a hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash ascon-a",
        result: Some(Value::string(
          "406b809260f361e12dcf0bf924bfe1ffd2f987fc18d90b94fc544ff80dc2946b".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the ascon-a hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash ascon-a --binary",
        result: Some(Value::binary(
          vec![0x40,0x6b,0x80,0x92,0x60,0xf3,0x61,0xe1,0x2d,0xcf,0x0b,0xf9,0x24,0xbf,0xe1,0xff,0xd2,0xf9,0x87,0xfc,0x18,0xd9,0x0b,0x94,0xfc,0x54,0x4f,0xf8,0x0d,0xc2,0x94,0x6b],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the ascon-a hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash ascon-a",
        result: None,
      },
    ]
  }
}

impl Hasher for belt_hash::BeltHash {
  fn name() -> &'static str {
    "belt"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the belt hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash belt",
        result: Some(Value::string(
          "e3228ad167accfe62fd5d834b0e59c5327aaf8e3d980f2da3680c8dc3ba5993e".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the belt hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash belt --binary",
        result: Some(Value::binary(
          vec![0xe3,0x22,0x8a,0xd1,0x67,0xac,0xcf,0xe6,0x2f,0xd5,0xd8,0x34,0xb0,0xe5,0x9c,0x53,0x27,0xaa,0xf8,0xe3,0xd9,0x80,0xf2,0xda,0x36,0x80,0xc8,0xdc,0x3b,0xa5,0x99,0x3e],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the belt hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash belt",
        result: None,
      },
    ]
  }
}

impl Hasher for blake2::Blake2s256 {
  fn name() -> &'static str {
    "blake2s-256"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the blake2s-256 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash blake2s-256",
        result: Some(Value::string(
          "bdf88eb1f86a0cdf0e840ba88fa118508369df186c7355b4b16cf79fa2710a12".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the blake2s-256 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash blake2s-256 --binary",
        result: Some(Value::binary(
          vec![0xbd,0xf8,0x8e,0xb1,0xf8,0x6a,0x0c,0xdf,0x0e,0x84,0x0b,0xa8,0x8f,0xa1,0x18,0x50,0x83,0x69,0xdf,0x18,0x6c,0x73,0x55,0xb4,0xb1,0x6c,0xf7,0x9f,0xa2,0x71,0x0a,0x12],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the blake2s-256 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash blake2s-256",
        result: None,
      },
    ]
  }
}

impl Hasher for blake2::Blake2b512 {
  fn name() -> &'static str {
    "blake2b-512"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the blake2b-512 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash blake2b-512",
        result: Some(Value::string(
          "c68ede143e416eb7b4aaae0d8e48e55dd529eafed10b1df1a61416953a2b0a5666c761e7d412e6709e31ffe221b7a7a73908cb95a4d120b8b090a87d1fbedb4c".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the blake2b-512 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash blake2b-512 --binary",
        result: Some(Value::binary(
          vec![0xc6,0x8e,0xde,0x14,0x3e,0x41,0x6e,0xb7,0xb4,0xaa,0xae,0x0d,0x8e,0x48,0xe5,0x5d,0xd5,0x29,0xea,0xfe,0xd1,0x0b,0x1d,0xf1,0xa6,0x14,0x16,0x95,0x3a,0x2b,0x0a,0x56,0x66,0xc7,0x61,0xe7,0xd4,0x12,0xe6,0x70,0x9e,0x31,0xff,0xe2,0x21,0xb7,0xa7,0xa7,0x39,0x08,0xcb,0x95,0xa4,0xd1,0x20,0xb8,0xb0,0x90,0xa8,0x7d,0x1f,0xbe,0xdb,0x4c],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the blake2b-512 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash blake2b-512",
        result: None,
      },
    ]
  }
}

impl Hasher for fsb::Fsb160 {
  fn name() -> &'static str {
    "fsb160"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the fsb160 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash fsb160",
        result: Some(Value::string(
          "32bb30a00beb0265917d45abbc4373df763e91e9".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the fsb160 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash fsb160 --binary",
        result: Some(Value::binary(
          vec![0x32,0xbb,0x30,0xa0,0x0b,0xeb,0x02,0x65,0x91,0x7d,0x45,0xab,0xbc,0x43,0x73,0xdf,0x76,0x3e,0x91,0xe9],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the fsb160 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash fsb160",
        result: None,
      },
    ]
  }
}

impl Hasher for fsb::Fsb224 {
  fn name() -> &'static str {
    "fsb224"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the fsb224 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash fsb224",
        result: Some(Value::string(
          "8e43a83b9596d8bd7744db24b9aacb5c1e532e3038c807e913393be3".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the fsb224 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash fsb224 --binary",
        result: Some(Value::binary(
          vec![0x8e,0x43,0xa8,0x3b,0x95,0x96,0xd8,0xbd,0x77,0x44,0xdb,0x24,0xb9,0xaa,0xcb,0x5c,0x1e,0x53,0x2e,0x30,0x38,0xc8,0x07,0xe9,0x13,0x39,0x3b,0xe3],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the fsb224 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash fsb224",
        result: None,
      },
    ]
  }
}

impl Hasher for fsb::Fsb256 {
  fn name() -> &'static str {
    "fsb256"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the fsb256 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash fsb256",
        result: Some(Value::string(
          "9bd737bac3e5270e1a402b57bed97382d34373df53cab1a6bec06ef695acae41".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the fsb256 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash fsb256 --binary",
        result: Some(Value::binary(
          vec![0x9b,0xd7,0x37,0xba,0xc3,0xe5,0x27,0x0e,0x1a,0x40,0x2b,0x57,0xbe,0xd9,0x73,0x82,0xd3,0x43,0x73,0xdf,0x53,0xca,0xb1,0xa6,0xbe,0xc0,0x6e,0xf6,0x95,0xac,0xae,0x41],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the fsb256 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash fsb256",
        result: None,
      },
    ]
  }
}

impl Hasher for fsb::Fsb384 {
  fn name() -> &'static str {
    "fsb384"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the fsb384 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash fsb384",
        result: Some(Value::string(
          "4b367de0fd7aef1fc9019c9e7974f59462615c34ae4fc0d679066e8c9f579c8fc8081865e849f5e1d26dc1d02947f0a1".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the fsb384 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash fsb384 --binary",
        result: Some(Value::binary(
          vec![0x4b,0x36,0x7d,0xe0,0xfd,0x7a,0xef,0x1f,0xc9,0x01,0x9c,0x9e,0x79,0x74,0xf5,0x94,0x62,0x61,0x5c,0x34,0xae,0x4f,0xc0,0xd6,0x79,0x06,0x6e,0x8c,0x9f,0x57,0x9c,0x8f,0xc8,0x08,0x18,0x65,0xe8,0x49,0xf5,0xe1,0xd2,0x6d,0xc1,0xd0,0x29,0x47,0xf0,0xa1],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the fsb384 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash fsb384",
        result: None,
      },
    ]
  }
}

impl Hasher for fsb::Fsb512 {
  fn name() -> &'static str {
    "fsb512"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the fsb512 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash fsb512",
        result: Some(Value::string(
          "a6781ed0149eda41f12a4dbeb357ffb86c446c5842cd769232cb880bed9284a5f0ebac49fbdb1537612b29b8d46386ccbc69cd0d0d44ce2c3dbccec95507cc9a".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the fsb512 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash fsb512 --binary",
        result: Some(Value::binary(
          vec![0xa6,0x78,0x1e,0xd0,0x14,0x9e,0xda,0x41,0xf1,0x2a,0x4d,0xbe,0xb3,0x57,0xff,0xb8,0x6c,0x44,0x6c,0x58,0x42,0xcd,0x76,0x92,0x32,0xcb,0x88,0x0b,0xed,0x92,0x84,0xa5,0xf0,0xeb,0xac,0x49,0xfb,0xdb,0x15,0x37,0x61,0x2b,0x29,0xb8,0xd4,0x63,0x86,0xcc,0xbc,0x69,0xcd,0x0d,0x0d,0x44,0xce,0x2c,0x3d,0xbc,0xce,0xc9,0x55,0x07,0xcc,0x9a],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the fsb512 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash fsb512",
        result: None,
      },
    ]
  }
}

impl Hasher for gost94::Gost94CryptoPro {
  fn name() -> &'static str {
    "gost94-crypto-pro"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the gost94-crypto-pro hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash gost94-crypto-pro",
        result: Some(Value::string(
          "8cda28cd45ce733e3d0837aed41bdf7fda7d83a6dfe211a0c695259a443250bd".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the gost94-crypto-pro hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash gost94-crypto-pro --binary",
        result: Some(Value::binary(
          vec![0x8c,0xda,0x28,0xcd,0x45,0xce,0x73,0x3e,0x3d,0x08,0x37,0xae,0xd4,0x1b,0xdf,0x7f,0xda,0x7d,0x83,0xa6,0xdf,0xe2,0x11,0xa0,0xc6,0x95,0x25,0x9a,0x44,0x32,0x50,0xbd],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the gost94-crypto-pro hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash gost94-crypto-pro",
        result: None,
      },
    ]
  }
}

impl Hasher for gost94::Gost94UA {
  fn name() -> &'static str {
    "gost94-ua"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the gost94-ua hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash gost94-ua",
        result: Some(Value::string(
          "f060d809dd1601ab7c094eec87af1408c1acec3e709fce08699f41df05dc903f".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the gost94-ua hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash gost94-ua --binary",
        result: Some(Value::binary(
          vec![0xf0,0x60,0xd8,0x09,0xdd,0x16,0x01,0xab,0x7c,0x09,0x4e,0xec,0x87,0xaf,0x14,0x08,0xc1,0xac,0xec,0x3e,0x70,0x9f,0xce,0x08,0x69,0x9f,0x41,0xdf,0x05,0xdc,0x90,0x3f],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the gost94-ua hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash gost94-ua",
        result: None,
      },
    ]
  }
}

impl Hasher for gost94::Gost94s2015 {
  fn name() -> &'static str {
    "gost94-2015"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the gost94-2015 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash gost94-2015",
        result: Some(Value::string(
          "5f9ddf7260bbd3581b24af252a90b82607af3acb5488baf087471b0fc9d6df32".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the gost94-2015 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash gost94-2015 --binary",
        result: Some(Value::binary(
          vec![0x5f,0x9d,0xdf,0x72,0x60,0xbb,0xd3,0x58,0x1b,0x24,0xaf,0x25,0x2a,0x90,0xb8,0x26,0x07,0xaf,0x3a,0xcb,0x54,0x88,0xba,0xf0,0x87,0x47,0x1b,0x0f,0xc9,0xd6,0xdf,0x32],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the gost94-2015 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash gost94-2015",
        result: None,
      },
    ]
  }
}

impl Hasher for groestl::Groestl224 {
  fn name() -> &'static str {
    "goestl224"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the goestl224 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash goestl224",
        result: Some(Value::string(
          "9ee8ca59e9ab4cba339ad91c7dffd33e6b694d8b1b83b1b502612b2d".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the goestl224 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash goestl224 --binary",
        result: Some(Value::binary(
          vec![0x9e,0xe8,0xca,0x59,0xe9,0xab,0x4c,0xba,0x33,0x9a,0xd9,0x1c,0x7d,0xff,0xd3,0x3e,0x6b,0x69,0x4d,0x8b,0x1b,0x83,0xb1,0xb5,0x02,0x61,0x2b,0x2d],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the goestl224 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash goestl224",
        result: None,
      },
    ]
  }
}

impl Hasher for groestl::Groestl256 {
  fn name() -> &'static str {
    "groestl256"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the groestl256 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash groestl256",
        result: Some(Value::string(
          "113f70bfdbca1fad4de646b3ef7331c55c0c9f727c31cab3871eb117a8cdabb2".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the groestl256 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash groestl256 --binary",
        result: Some(Value::binary(
          vec![0x11,0x3f,0x70,0xbf,0xdb,0xca,0x1f,0xad,0x4d,0xe6,0x46,0xb3,0xef,0x73,0x31,0xc5,0x5c,0x0c,0x9f,0x72,0x7c,0x31,0xca,0xb3,0x87,0x1e,0xb1,0x17,0xa8,0xcd,0xab,0xb2],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the groestl256 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash groestl256",
        result: None,
      },
    ]
  }
}

impl Hasher for groestl::Groestl384 {
  fn name() -> &'static str {
    "groestl384"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the groestl384 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash groestl384",
        result: Some(Value::string(
          "af3607759915be17cb74ccd97f6302776cd5c98b18623e74b70e2ba0022cfabd3a0f243d638c59ad673cc7d98d817c06".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the groestl384 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash groestl384 --binary",
        result: Some(Value::binary(
          vec![0xaf,0x36,0x07,0x75,0x99,0x15,0xbe,0x17,0xcb,0x74,0xcc,0xd9,0x7f,0x63,0x02,0x77,0x6c,0xd5,0xc9,0x8b,0x18,0x62,0x3e,0x74,0xb7,0x0e,0x2b,0xa0,0x02,0x2c,0xfa,0xbd,0x3a,0x0f,0x24,0x3d,0x63,0x8c,0x59,0xad,0x67,0x3c,0xc7,0xd9,0x8d,0x81,0x7c,0x06],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the groestl384 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash groestl384",
        result: None,
      },
    ]
  }
}

impl Hasher for groestl::Groestl512 {
  fn name() -> &'static str {
    "groestl512"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the groestl512 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash groestl512",
        result: Some(Value::string(
          "fe637169445eeacbd53763ef48cec130d7bd2a12425dd80a6410ef13d0cd5d2b98cc91b714f8d0ba637d6e872cae046c271f0e22f2a1eff46a2d2d5449ffec74".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the groestl512 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash groestl512 --binary",
        result: Some(Value::binary(
          vec![0xfe,0x63,0x71,0x69,0x44,0x5e,0xea,0xcb,0xd5,0x37,0x63,0xef,0x48,0xce,0xc1,0x30,0xd7,0xbd,0x2a,0x12,0x42,0x5d,0xd8,0x0a,0x64,0x10,0xef,0x13,0xd0,0xcd,0x5d,0x2b,0x98,0xcc,0x91,0xb7,0x14,0xf8,0xd0,0xba,0x63,0x7d,0x6e,0x87,0x2c,0xae,0x04,0x6c,0x27,0x1f,0x0e,0x22,0xf2,0xa1,0xef,0xf4,0x6a,0x2d,0x2d,0x54,0x49,0xff,0xec,0x74],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the groestl512 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash groestl512",
        result: None,
      },
    ]
  }
}

impl Hasher for jh::Jh224 {
  fn name() -> &'static str {
    "jh224"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the jh224 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash jh224",
        result: Some(Value::string(
          "8f4a448b971e639f7a05ae52d9c3ae25b5dbb4f348963462b4d6f394".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the jh224 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash jh224 --binary",
        result: Some(Value::binary(
          vec![0x8f,0x4a,0x44,0x8b,0x97,0x1e,0x63,0x9f,0x7a,0x05,0xae,0x52,0xd9,0xc3,0xae,0x25,0xb5,0xdb,0xb4,0xf3,0x48,0x96,0x34,0x62,0xb4,0xd6,0xf3,0x94],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the jh224 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash jh224",
        result: None,
      },
    ]
  }
}

impl Hasher for jh::Jh256 {
  fn name() -> &'static str {
    "jh256"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the jh256 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash jh256",
        result: Some(Value::string(
          "c392a84988b82fb5b745b7174e9f808b38a14dc00b34250775fa31dd58ab053d".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the jh256 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash jh256 --binary",
        result: Some(Value::binary(
          vec![0xc3,0x92,0xa8,0x49,0x88,0xb8,0x2f,0xb5,0xb7,0x45,0xb7,0x17,0x4e,0x9f,0x80,0x8b,0x38,0xa1,0x4d,0xc0,0x0b,0x34,0x25,0x07,0x75,0xfa,0x31,0xdd,0x58,0xab,0x05,0x3d],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the jh256 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash jh256",
        result: None,
      },
    ]
  }
}

impl Hasher for jh::Jh384 {
  fn name() -> &'static str {
    "jh384"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the jh384 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash jh384",
        result: Some(Value::string(
          "8aa48c3ee261534441d91ffd0b647638640ea5c7473dd6a823456e0d96cb0219528492862f7b684d47fcfd5c59c6df65".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the jh384 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash jh384 --binary",
        result: Some(Value::binary(
          vec![0x8a,0xa4,0x8c,0x3e,0xe2,0x61,0x53,0x44,0x41,0xd9,0x1f,0xfd,0x0b,0x64,0x76,0x38,0x64,0x0e,0xa5,0xc7,0x47,0x3d,0xd6,0xa8,0x23,0x45,0x6e,0x0d,0x96,0xcb,0x02,0x19,0x52,0x84,0x92,0x86,0x2f,0x7b,0x68,0x4d,0x47,0xfc,0xfd,0x5c,0x59,0xc6,0xdf,0x65],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the jh384 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash jh384",
        result: None,
      },
    ]
  }
}

impl Hasher for jh::Jh512 {
  fn name() -> &'static str {
    "jh512"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the jh512 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash jh512",
        result: Some(Value::string(
          "8735238cc6ac144c2639f5024cfb8706bed077094d4c5f9bde87275cc1eb68972b1cb7e2a01e80f26fcf0242a540a0e9ff515ed3dc54de308c624c134e9ebe3d".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the jh512 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash jh512 --binary",
        result: Some(Value::binary(
          vec![0x87,0x35,0x23,0x8c,0xc6,0xac,0x14,0x4c,0x26,0x39,0xf5,0x02,0x4c,0xfb,0x87,0x06,0xbe,0xd0,0x77,0x09,0x4d,0x4c,0x5f,0x9b,0xde,0x87,0x27,0x5c,0xc1,0xeb,0x68,0x97,0x2b,0x1c,0xb7,0xe2,0xa0,0x1e,0x80,0xf2,0x6f,0xcf,0x02,0x42,0xa5,0x40,0xa0,0xe9,0xff,0x51,0x5e,0xd3,0xdc,0x54,0xde,0x30,0x8c,0x62,0x4c,0x13,0x4e,0x9e,0xbe,0x3d],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the jh512 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash jh512",
        result: None,
      },
    ]
  }
}

impl Hasher for md2::Md2 {
  fn name() -> &'static str {
    "md2"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the md2 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash md2",
        result: Some(Value::string(
          "4e8ddff3650292ab5a4108c3aa47940b".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the md2 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash md2 --binary",
        result: Some(Value::binary(
          vec![0x4e,0x8d,0xdf,0xf3,0x65,0x02,0x92,0xab,0x5a,0x41,0x08,0xc3,0xaa,0x47,0x94,0x0b],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the md2 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash md2",
        result: None,
      },
    ]
  }
}

impl Hasher for md4::Md4 {
  fn name() -> &'static str {
    "md4"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the md4 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash md4",
        result: Some(Value::string(
          "d79e1c308aa5bbcdeea8ed63df412da9".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the md4 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash md4 --binary",
        result: Some(Value::binary(
          vec![0xd7,0x9e,0x1c,0x30,0x8a,0xa5,0xbb,0xcd,0xee,0xa8,0xed,0x63,0xdf,0x41,0x2d,0xa9],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the md4 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash md4",
        result: None,
      },
    ]
  }
}

impl Hasher for ripemd::Ripemd128 {
  fn name() -> &'static str {
    "ripemd128"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the ripemd128 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash ripemd128",
        result: Some(Value::string(
          "fd2aa607f71dc8f510714922b371834e".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the ripemd128 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash ripemd128 --binary",
        result: Some(Value::binary(
          vec![0xfd,0x2a,0xa6,0x07,0xf7,0x1d,0xc8,0xf5,0x10,0x71,0x49,0x22,0xb3,0x71,0x83,0x4e],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the ripemd128 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash ripemd128",
        result: None,
      },
    ]
  }
}

impl Hasher for ripemd::Ripemd160 {
  fn name() -> &'static str {
    "ripemd160"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the ripemd160 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash ripemd160",
        result: Some(Value::string(
          "f71c27109c692c1b56bbdceb5b9d2865b3708dbc".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the ripemd160 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash ripemd160 --binary",
        result: Some(Value::binary(
          vec![0xf7,0x1c,0x27,0x10,0x9c,0x69,0x2c,0x1b,0x56,0xbb,0xdc,0xeb,0x5b,0x9d,0x28,0x65,0xb3,0x70,0x8d,0xbc],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the ripemd160 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash ripemd160",
        result: None,
      },
    ]
  }
}

impl Hasher for ripemd::Ripemd256 {
  fn name() -> &'static str {
    "ripemd256"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the ripemd256 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash ripemd256",
        result: Some(Value::string(
          "649d3034751ea216776bf9a18acc81bc7896118a5197968782dd1fd97d8d5133".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the ripemd256 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash ripemd256 --binary",
        result: Some(Value::binary(
          vec![0x64,0x9d,0x30,0x34,0x75,0x1e,0xa2,0x16,0x77,0x6b,0xf9,0xa1,0x8a,0xcc,0x81,0xbc,0x78,0x96,0x11,0x8a,0x51,0x97,0x96,0x87,0x82,0xdd,0x1f,0xd9,0x7d,0x8d,0x51,0x33],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the ripemd256 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash ripemd256",
        result: None,
      },
    ]
  }
}

impl Hasher for ripemd::Ripemd320 {
  fn name() -> &'static str {
    "ripemd320"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the ripemd320 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash ripemd320",
        result: Some(Value::string(
          "cabdb1810b92470a2093aa6bce05952c28348cf43ff60841975166bb40ed234004b8824463e6b009".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the ripemd320 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash ripemd320 --binary",
        result: Some(Value::binary(
          vec![0xca,0xbd,0xb1,0x81,0x0b,0x92,0x47,0x0a,0x20,0x93,0xaa,0x6b,0xce,0x05,0x95,0x2c,0x28,0x34,0x8c,0xf4,0x3f,0xf6,0x08,0x41,0x97,0x51,0x66,0xbb,0x40,0xed,0x23,0x40,0x04,0xb8,0x82,0x44,0x63,0xe6,0xb0,0x09],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the ripemd320 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash ripemd320",
        result: None,
      },
    ]
  }
}

impl Hasher for sha1::Sha1 {
  fn name() -> &'static str {
    "sha1"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the sha1 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha1",
        result: Some(Value::string(
          "32d10c7b8cf96570ca04ce37f2a19d84240d3a89".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha1 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha1 --binary",
        result: Some(Value::binary(
          vec![0x32,0xd1,0x0c,0x7b,0x8c,0xf9,0x65,0x70,0xca,0x04,0xce,0x37,0xf2,0xa1,0x9d,0x84,0x24,0x0d,0x3a,0x89],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha1 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash sha1",
        result: None,
      },
    ]
  }
}

impl Hasher for sha2::Sha224 {
  fn name() -> &'static str {
    "sha224"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the sha224 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha224",
        result: Some(Value::string(
          "45a5f72c39c5cff2522eb3429799e49e5f44b356ef926bcf390dccc2".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha224 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha224 --binary",
        result: Some(Value::binary(
          vec![0x45,0xa5,0xf7,0x2c,0x39,0xc5,0xcf,0xf2,0x52,0x2e,0xb3,0x42,0x97,0x99,0xe4,0x9e,0x5f,0x44,0xb3,0x56,0xef,0x92,0x6b,0xcf,0x39,0x0d,0xcc,0xc2],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha224 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash sha224",
        result: None,
      },
    ]
  }
}

impl Hasher for sha2::Sha384 {
  fn name() -> &'static str {
    "sha384"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the sha384 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha384",
        result: Some(Value::string(
          "feb67349df3db6f5924815d6c3dc133f091809213731fe5c7b5f4999e463479ff2877f5f2936fa63bb43784b12f3ebb4".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha384 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha384 --binary",
        result: Some(Value::binary(
          vec![0xfe,0xb6,0x73,0x49,0xdf,0x3d,0xb6,0xf5,0x92,0x48,0x15,0xd6,0xc3,0xdc,0x13,0x3f,0x09,0x18,0x09,0x21,0x37,0x31,0xfe,0x5c,0x7b,0x5f,0x49,0x99,0xe4,0x63,0x47,0x9f,0xf2,0x87,0x7f,0x5f,0x29,0x36,0xfa,0x63,0xbb,0x43,0x78,0x4b,0x12,0xf3,0xeb,0xb4],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha384 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash sha384",
        result: None,
      },
    ]
  }
}

impl Hasher for sha2::Sha512 {
  fn name() -> &'static str {
    "sha512"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the sha512 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha512",
        result: Some(Value::string(
          "4dbff86cc2ca1bae1e16468a05cb9881c97f1753bce3619034898faa1aabe429955a1bf8ec483d7421fe3c1646613a59ed5441fb0f321389f77f48a879c7b1f1".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha512 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha512 --binary",
        result: Some(Value::binary(
          vec![0x4d,0xbf,0xf8,0x6c,0xc2,0xca,0x1b,0xae,0x1e,0x16,0x46,0x8a,0x05,0xcb,0x98,0x81,0xc9,0x7f,0x17,0x53,0xbc,0xe3,0x61,0x90,0x34,0x89,0x8f,0xaa,0x1a,0xab,0xe4,0x29,0x95,0x5a,0x1b,0xf8,0xec,0x48,0x3d,0x74,0x21,0xfe,0x3c,0x16,0x46,0x61,0x3a,0x59,0xed,0x54,0x41,0xfb,0x0f,0x32,0x13,0x89,0xf7,0x7f,0x48,0xa8,0x79,0xc7,0xb1,0xf1],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha512 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash sha512",
        result: None,
      },
    ]
  }
}

impl Hasher for sha2::Sha512_224 {
  fn name() -> &'static str {
    "sha512-224"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the sha512-224 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha512-224",
        result: Some(Value::string(
          "ff83148aa07ec30655c1b40aff86141c0215fe2a54f767d3f38743d8".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha512-224 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha512-224 --binary",
        result: Some(Value::binary(
          vec![0xff,0x83,0x14,0x8a,0xa0,0x7e,0xc3,0x06,0x55,0xc1,0xb4,0x0a,0xff,0x86,0x14,0x1c,0x02,0x15,0xfe,0x2a,0x54,0xf7,0x67,0xd3,0xf3,0x87,0x43,0xd8],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha512-224 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash sha512-224",
        result: None,
      },
    ]
  }
}

impl Hasher for sha2::Sha512_256 {
  fn name() -> &'static str {
    "sha512-256"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the sha512-256 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha512-256",
        result: Some(Value::string(
          "fc3189443f9c268f626aea08a756abe7b726b05f701cb08222312ccfd6710a26".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha512-256 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha512-256 --binary",
        result: Some(Value::binary(
          vec![0xfc,0x31,0x89,0x44,0x3f,0x9c,0x26,0x8f,0x62,0x6a,0xea,0x08,0xa7,0x56,0xab,0xe7,0xb7,0x26,0xb0,0x5f,0x70,0x1c,0xb0,0x82,0x22,0x31,0x2c,0xcf,0xd6,0x71,0x0a,0x26],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha512-256 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash sha512-256",
        result: None,
      },
    ]
  }
}

impl Hasher for sha3::Sha3_224 {
  fn name() -> &'static str {
    "sha3-224"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the sha3-224 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha3-224",
        result: Some(Value::string(
          "5cdeca81e123f87cad96b9cba999f16f6d41549608d4e0f4681b8239".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha3-224 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha3-224 --binary",
        result: Some(Value::binary(
          vec![0x5c,0xde,0xca,0x81,0xe1,0x23,0xf8,0x7c,0xad,0x96,0xb9,0xcb,0xa9,0x99,0xf1,0x6f,0x6d,0x41,0x54,0x96,0x08,0xd4,0xe0,0xf4,0x68,0x1b,0x82,0x39],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha3-224 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash sha3-224",
        result: None,
      },
    ]
  }
}

impl Hasher for sha3::Sha3_256 {
  fn name() -> &'static str {
    "sha3-256"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the sha3-256 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha3-256",
        result: Some(Value::string(
          "7cab2dc765e21b241dbc1c255ce620b29f527c6d5e7f5f843e56288f0d707521".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha3-256 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha3-256 --binary",
        result: Some(Value::binary(
          vec![0x7c,0xab,0x2d,0xc7,0x65,0xe2,0x1b,0x24,0x1d,0xbc,0x1c,0x25,0x5c,0xe6,0x20,0xb2,0x9f,0x52,0x7c,0x6d,0x5e,0x7f,0x5f,0x84,0x3e,0x56,0x28,0x8f,0x0d,0x70,0x75,0x21],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha3-256 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash sha3-256",
        result: None,
      },
    ]
  }
}

impl Hasher for sha3::Sha3_384 {
  fn name() -> &'static str {
    "sha3-384"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the sha3-384 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha3-384",
        result: Some(Value::string(
          "fed399d2217aaf4c717ad0c5102c15589e1c990cc2b9a5029056a7f7485888d6ab65db2370077a5cadb53fc9280d278f".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha3-384 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha3-384 --binary",
        result: Some(Value::binary(
          vec![0xfe,0xd3,0x99,0xd2,0x21,0x7a,0xaf,0x4c,0x71,0x7a,0xd0,0xc5,0x10,0x2c,0x15,0x58,0x9e,0x1c,0x99,0x0c,0xc2,0xb9,0xa5,0x02,0x90,0x56,0xa7,0xf7,0x48,0x58,0x88,0xd6,0xab,0x65,0xdb,0x23,0x70,0x07,0x7a,0x5c,0xad,0xb5,0x3f,0xc9,0x28,0x0d,0x27,0x8f],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha3-384 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash sha3-384",
        result: None,
      },
    ]
  }
}

impl Hasher for sha3::Sha3_512 {
  fn name() -> &'static str {
    "sha3-512"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the sha3-512 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha3-512",
        result: Some(Value::string(
          "af328d17fa28753a3c9f5cb72e376b90440b96f0289e5703b729324a975ab384eda565fc92aaded143669900d761861687acdc0a5ffa358bd0571aaad80aca68".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha3-512 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sha3-512 --binary",
        result: Some(Value::binary(
          vec![0xaf,0x32,0x8d,0x17,0xfa,0x28,0x75,0x3a,0x3c,0x9f,0x5c,0xb7,0x2e,0x37,0x6b,0x90,0x44,0x0b,0x96,0xf0,0x28,0x9e,0x57,0x03,0xb7,0x29,0x32,0x4a,0x97,0x5a,0xb3,0x84,0xed,0xa5,0x65,0xfc,0x92,0xaa,0xde,0xd1,0x43,0x66,0x99,0x00,0xd7,0x61,0x86,0x16,0x87,0xac,0xdc,0x0a,0x5f,0xfa,0x35,0x8b,0xd0,0x57,0x1a,0xaa,0xd8,0x0a,0xca,0x68],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sha3-512 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash sha3-512",
        result: None,
      },
    ]
  }
}

impl Hasher for sha3::Keccak224 {
  fn name() -> &'static str {
    "keccak224"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the keccak224 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash keccak224",
        result: Some(Value::string(
          "162bab64dc3ba594bd3b43fd8abec4aa03b36c2784cac53a58f9b076".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the keccak224 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash keccak224 --binary",
        result: Some(Value::binary(
          vec![0x16,0x2b,0xab,0x64,0xdc,0x3b,0xa5,0x94,0xbd,0x3b,0x43,0xfd,0x8a,0xbe,0xc4,0xaa,0x03,0xb3,0x6c,0x27,0x84,0xca,0xc5,0x3a,0x58,0xf9,0xb0,0x76],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the keccak224 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash keccak224",
        result: None,
      },
    ]
  }
}

impl Hasher for sha3::Keccak256 {
  fn name() -> &'static str {
    "keccak256"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the keccak256 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash keccak256",
        result: Some(Value::string(
          "9230175b13981da14d2f3334f321eb78fa0473133f6da3de896feb22fb258936".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the keccak256 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash keccak256 --binary",
        result: Some(Value::binary(
          vec![0x92,0x30,0x17,0x5b,0x13,0x98,0x1d,0xa1,0x4d,0x2f,0x33,0x34,0xf3,0x21,0xeb,0x78,0xfa,0x04,0x73,0x13,0x3f,0x6d,0xa3,0xde,0x89,0x6f,0xeb,0x22,0xfb,0x25,0x89,0x36],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the keccak256 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash keccak256",
        result: None,
      },
    ]
  }
}

impl Hasher for sha3::Keccak384 {
  fn name() -> &'static str {
    "keccak384"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the keccak384 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash keccak384",
        result: Some(Value::string(
          "c5a708ec2178d8c398461547435e482cee0d85de3d75ddbff54e6606a7e9f994f023a6033b2bf4c516a5f71fc7470d1a".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the keccak384 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash keccak384 --binary",
        result: Some(Value::binary(
          vec![0xc5,0xa7,0x08,0xec,0x21,0x78,0xd8,0xc3,0x98,0x46,0x15,0x47,0x43,0x5e,0x48,0x2c,0xee,0x0d,0x85,0xde,0x3d,0x75,0xdd,0xbf,0xf5,0x4e,0x66,0x06,0xa7,0xe9,0xf9,0x94,0xf0,0x23,0xa6,0x03,0x3b,0x2b,0xf4,0xc5,0x16,0xa5,0xf7,0x1f,0xc7,0x47,0x0d,0x1a],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the keccak384 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash keccak384",
        result: None,
      },
    ]
  }
}

impl Hasher for sha3::Keccak512 {
  fn name() -> &'static str {
    "keccak512"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the keccak512 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash keccak512",
        result: Some(Value::string(
          "e55bdca64dfe33f36ae3153c727833f9947d92958073f4dd02e38a82d8acb282b1ee1330a68252a54c6d3d27306508ca765acd45606caeaf51d6bdc459f551f1".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the keccak512 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash keccak512 --binary",
        result: Some(Value::binary(
          vec![0xe5,0x5b,0xdc,0xa6,0x4d,0xfe,0x33,0xf3,0x6a,0xe3,0x15,0x3c,0x72,0x78,0x33,0xf9,0x94,0x7d,0x92,0x95,0x80,0x73,0xf4,0xdd,0x02,0xe3,0x8a,0x82,0xd8,0xac,0xb2,0x82,0xb1,0xee,0x13,0x30,0xa6,0x82,0x52,0xa5,0x4c,0x6d,0x3d,0x27,0x30,0x65,0x08,0xca,0x76,0x5a,0xcd,0x45,0x60,0x6c,0xae,0xaf,0x51,0xd6,0xbd,0xc4,0x59,0xf5,0x51,0xf1],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the keccak512 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash keccak512",
        result: None,
      },
    ]
  }
}

impl Hasher for shabal::Shabal192 {
  fn name() -> &'static str {
    "shabal192"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the shabal192 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash shabal192",
        result: Some(Value::string(
          "f6316195b84bee598c8061536be254257bc76b055ce16e92".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the shabal192 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash shabal192 --binary",
        result: Some(Value::binary(
          vec![0xf6,0x31,0x61,0x95,0xb8,0x4b,0xee,0x59,0x8c,0x80,0x61,0x53,0x6b,0xe2,0x54,0x25,0x7b,0xc7,0x6b,0x05,0x5c,0xe1,0x6e,0x92],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the shabal192 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash shabal192",
        result: None,
      },
    ]
  }
}

impl Hasher for shabal::Shabal224 {
  fn name() -> &'static str {
    "shabal224"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the shabal224 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash shabal224",
        result: Some(Value::string(
          "bc954127c1da58a2a9b91af619c34007c4da4c51ffcde10018c3f551".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the shabal224 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash shabal224 --binary",
        result: Some(Value::binary(
          vec![0xbc,0x95,0x41,0x27,0xc1,0xda,0x58,0xa2,0xa9,0xb9,0x1a,0xf6,0x19,0xc3,0x40,0x07,0xc4,0xda,0x4c,0x51,0xff,0xcd,0xe1,0x00,0x18,0xc3,0xf5,0x51],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the shabal224 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash shabal224",
        result: None,
      },
    ]
  }
}

impl Hasher for shabal::Shabal256 {
  fn name() -> &'static str {
    "shabal256"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the shabal256 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash shabal256",
        result: Some(Value::string(
          "546e7c543bafa9d77cf7864ca0709c56331bbd122b376de1bb7520578623ddf3".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the shabal256 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash shabal256 --binary",
        result: Some(Value::binary(
          vec![0x54,0x6e,0x7c,0x54,0x3b,0xaf,0xa9,0xd7,0x7c,0xf7,0x86,0x4c,0xa0,0x70,0x9c,0x56,0x33,0x1b,0xbd,0x12,0x2b,0x37,0x6d,0xe1,0xbb,0x75,0x20,0x57,0x86,0x23,0xdd,0xf3],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the shabal256 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash shabal256",
        result: None,
      },
    ]
  }
}

impl Hasher for shabal::Shabal384 {
  fn name() -> &'static str {
    "shabal384"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the shabal384 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash shabal384",
        result: Some(Value::string(
          "33fdcea696fad519dd5d1a7cfa88b1e0e8c421506c76c2623a8d9a94b8c7e2cf84a70f4213b4b4a0f5ad857427b508c1".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the shabal384 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash shabal384 --binary",
        result: Some(Value::binary(
          vec![0x33,0xfd,0xce,0xa6,0x96,0xfa,0xd5,0x19,0xdd,0x5d,0x1a,0x7c,0xfa,0x88,0xb1,0xe0,0xe8,0xc4,0x21,0x50,0x6c,0x76,0xc2,0x62,0x3a,0x8d,0x9a,0x94,0xb8,0xc7,0xe2,0xcf,0x84,0xa7,0x0f,0x42,0x13,0xb4,0xb4,0xa0,0xf5,0xad,0x85,0x74,0x27,0xb5,0x08,0xc1],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the shabal384 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash shabal384",
        result: None,
      },
    ]
  }
}

impl Hasher for shabal::Shabal512 {
  fn name() -> &'static str {
    "shabal512"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the shabal512 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash shabal512",
        result: Some(Value::string(
          "1bed76035102b415939a3a216f7a942e9bcc805641dc55d5568a097cece7d509155f8789cbe76a846e227de8cab325618cca99cabff576b3851ae678bd3d0b95".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the shabal512 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash shabal512 --binary",
        result: Some(Value::binary(
          vec![0x1b,0xed,0x76,0x03,0x51,0x02,0xb4,0x15,0x93,0x9a,0x3a,0x21,0x6f,0x7a,0x94,0x2e,0x9b,0xcc,0x80,0x56,0x41,0xdc,0x55,0xd5,0x56,0x8a,0x09,0x7c,0xec,0xe7,0xd5,0x09,0x15,0x5f,0x87,0x89,0xcb,0xe7,0x6a,0x84,0x6e,0x22,0x7d,0xe8,0xca,0xb3,0x25,0x61,0x8c,0xca,0x99,0xca,0xbf,0xf5,0x76,0xb3,0x85,0x1a,0xe6,0x78,0xbd,0x3d,0x0b,0x95],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the shabal512 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash shabal512",
        result: None,
      },
    ]
  }
}

impl Hasher for skein::Skein256::<skein::consts::U32> {
  fn name() -> &'static str {
    "skein256-32"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the skein256-32 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein256-32",
        result: Some(Value::string(
          "46d8440685461b00e3ddb891b2ecc6855287d2bd8834a95fb1c1708b00ea5e82".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein256-32 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein256-32 --binary",
        result: Some(Value::binary(
          vec![0x46,0xd8,0x44,0x06,0x85,0x46,0x1b,0x00,0xe3,0xdd,0xb8,0x91,0xb2,0xec,0xc6,0x85,0x52,0x87,0xd2,0xbd,0x88,0x34,0xa9,0x5f,0xb1,0xc1,0x70,0x8b,0x00,0xea,0x5e,0x82],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein256-32 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash skein256-32",
        result: None,
      },
    ]
  }
}

impl Hasher for skein::Skein256::<skein::consts::U64> {
  fn name() -> &'static str {
    "skein256-64"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the skein256-64 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein256-64",
        result: Some(Value::string(
          "454982e8b1723f739da12eecf81c6e1e22648883a861cb31d08cb151150cf04e095a6fc33d839403256a41dca80cd783e58e98acda983691414a362eaee762b1".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein256-64 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein256-64 --binary",
        result: Some(Value::binary(
          vec![0x45,0x49,0x82,0xe8,0xb1,0x72,0x3f,0x73,0x9d,0xa1,0x2e,0xec,0xf8,0x1c,0x6e,0x1e,0x22,0x64,0x88,0x83,0xa8,0x61,0xcb,0x31,0xd0,0x8c,0xb1,0x51,0x15,0x0c,0xf0,0x4e,0x09,0x5a,0x6f,0xc3,0x3d,0x83,0x94,0x03,0x25,0x6a,0x41,0xdc,0xa8,0x0c,0xd7,0x83,0xe5,0x8e,0x98,0xac,0xda,0x98,0x36,0x91,0x41,0x4a,0x36,0x2e,0xae,0xe7,0x62,0xb1],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein256-64 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash skein256-64",
        result: None,
      },
    ]
  }
}

impl Hasher for skein::Skein256::<skein::consts::U128> {
  fn name() -> &'static str {
    "skein256-128"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the skein256-128 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein256-128",
        result: Some(Value::string(
          "507fbbccc891ac26568c4ed032fe629b6703e04957144ee7ab7b78897a38a122aa4682354d65df1e84c77a0d0f1066feb6e7caeedc5e637715f700efcbc8298119f8ef56833e8baaa4be4fb2132c28a2cf48f51ad09255ad765f5ffaa28229b77c555f0ae3ff9ac6a41bb52fb5bba2ac0cccdf4fd243e340264c54d7907e94f2".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein256-128 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein256-128 --binary",
        result: Some(Value::binary(
          vec![0x50,0x7f,0xbb,0xcc,0xc8,0x91,0xac,0x26,0x56,0x8c,0x4e,0xd0,0x32,0xfe,0x62,0x9b,0x67,0x03,0xe0,0x49,0x57,0x14,0x4e,0xe7,0xab,0x7b,0x78,0x89,0x7a,0x38,0xa1,0x22,0xaa,0x46,0x82,0x35,0x4d,0x65,0xdf,0x1e,0x84,0xc7,0x7a,0x0d,0x0f,0x10,0x66,0xfe,0xb6,0xe7,0xca,0xee,0xdc,0x5e,0x63,0x77,0x15,0xf7,0x00,0xef,0xcb,0xc8,0x29,0x81,0x19,0xf8,0xef,0x56,0x83,0x3e,0x8b,0xaa,0xa4,0xbe,0x4f,0xb2,0x13,0x2c,0x28,0xa2,0xcf,0x48,0xf5,0x1a,0xd0,0x92,0x55,0xad,0x76,0x5f,0x5f,0xfa,0xa2,0x82,0x29,0xb7,0x7c,0x55,0x5f,0x0a,0xe3,0xff,0x9a,0xc6,0xa4,0x1b,0xb5,0x2f,0xb5,0xbb,0xa2,0xac,0x0c,0xcc,0xdf,0x4f,0xd2,0x43,0xe3,0x40,0x26,0x4c,0x54,0xd7,0x90,0x7e,0x94,0xf2],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein256-128 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash skein256-128",
        result: None,
      },
    ]
  }
}

impl Hasher for skein::Skein512::<skein::consts::U32> {
  fn name() -> &'static str {
    "skein512-32"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the skein512-32 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein512-32",
        result: Some(Value::string(
          "3408e7ecad2f637906ef825f15c437fcbb0c716bc5328fe5b656caf1d10d3736".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein512-32 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein512-32 --binary",
        result: Some(Value::binary(
          vec![0x34,0x08,0xe7,0xec,0xad,0x2f,0x63,0x79,0x06,0xef,0x82,0x5f,0x15,0xc4,0x37,0xfc,0xbb,0x0c,0x71,0x6b,0xc5,0x32,0x8f,0xe5,0xb6,0x56,0xca,0xf1,0xd1,0x0d,0x37,0x36],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein512-32 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash skein512-32",
        result: None,
      },
    ]
  }
}

impl Hasher for skein::Skein512::<skein::consts::U64> {
  fn name() -> &'static str {
    "skein512-64"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the skein512-64 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein512-64",
        result: Some(Value::string(
          "23793ad900ef12f9165c8080da6fdfd2c8354a2929b8aadf83aa82a3c6470342f57cf8c035ec0d97429b626c4d94f28632c8f5134fd367dca5cf293d2ec13f8c".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein512-64 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein512-64 --binary",
        result: Some(Value::binary(
          vec![0x23,0x79,0x3a,0xd9,0x00,0xef,0x12,0xf9,0x16,0x5c,0x80,0x80,0xda,0x6f,0xdf,0xd2,0xc8,0x35,0x4a,0x29,0x29,0xb8,0xaa,0xdf,0x83,0xaa,0x82,0xa3,0xc6,0x47,0x03,0x42,0xf5,0x7c,0xf8,0xc0,0x35,0xec,0x0d,0x97,0x42,0x9b,0x62,0x6c,0x4d,0x94,0xf2,0x86,0x32,0xc8,0xf5,0x13,0x4f,0xd3,0x67,0xdc,0xa5,0xcf,0x29,0x3d,0x2e,0xc1,0x3f,0x8c],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein512-64 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash skein512-64",
        result: None,
      },
    ]
  }
}

impl Hasher for skein::Skein512::<skein::consts::U128> {
  fn name() -> &'static str {
    "skein512-128"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the skein512-128 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein512-128",
        result: Some(Value::string(
          "308e5f83e187665512522e2db2b65458effad62b1845a299683496490a304d1d061e4ad2ba83bd40b495b8d8775bf3412244979e0c32567052448883eddd3a55b3ffdad6f998677df5c266d7164ddaf2e278cd6d9afbfbbfbd3d662d1deb7be62eea35cc02a2936decffa1f187afb7205578ac95a77694cc00049f1b9738db2c".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein512-128 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein512-128 --binary",
        result: Some(Value::binary(
          vec![0x30,0x8e,0x5f,0x83,0xe1,0x87,0x66,0x55,0x12,0x52,0x2e,0x2d,0xb2,0xb6,0x54,0x58,0xef,0xfa,0xd6,0x2b,0x18,0x45,0xa2,0x99,0x68,0x34,0x96,0x49,0x0a,0x30,0x4d,0x1d,0x06,0x1e,0x4a,0xd2,0xba,0x83,0xbd,0x40,0xb4,0x95,0xb8,0xd8,0x77,0x5b,0xf3,0x41,0x22,0x44,0x97,0x9e,0x0c,0x32,0x56,0x70,0x52,0x44,0x88,0x83,0xed,0xdd,0x3a,0x55,0xb3,0xff,0xda,0xd6,0xf9,0x98,0x67,0x7d,0xf5,0xc2,0x66,0xd7,0x16,0x4d,0xda,0xf2,0xe2,0x78,0xcd,0x6d,0x9a,0xfb,0xfb,0xbf,0xbd,0x3d,0x66,0x2d,0x1d,0xeb,0x7b,0xe6,0x2e,0xea,0x35,0xcc,0x02,0xa2,0x93,0x6d,0xec,0xff,0xa1,0xf1,0x87,0xaf,0xb7,0x20,0x55,0x78,0xac,0x95,0xa7,0x76,0x94,0xcc,0x00,0x04,0x9f,0x1b,0x97,0x38,0xdb,0x2c],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein512-128 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash skein512-128",
        result: None,
      },
    ]
  }
}

impl Hasher for skein::Skein1024::<skein::consts::U32> {
  fn name() -> &'static str {
    "skein1024-32"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the skein1024-32 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein1024-32",
        result: Some(Value::string(
          "408f86cdeffaa688a33ba4f853716f075ed1b56b2fa600559cca151d52381af7".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein1024-32 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein1024-32 --binary",
        result: Some(Value::binary(
          vec![0x40,0x8f,0x86,0xcd,0xef,0xfa,0xa6,0x88,0xa3,0x3b,0xa4,0xf8,0x53,0x71,0x6f,0x07,0x5e,0xd1,0xb5,0x6b,0x2f,0xa6,0x00,0x55,0x9c,0xca,0x15,0x1d,0x52,0x38,0x1a,0xf7],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein1024-32 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash skein1024-32",
        result: None,
      },
    ]
  }
}

impl Hasher for skein::Skein1024::<skein::consts::U64> {
  fn name() -> &'static str {
    "skein1024-64"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the skein1024-64 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein1024-64",
        result: Some(Value::string(
          "2b77428bf83d4a747753145cde77be6b8a6c618e33946f9e8c2ed0a6f97797828cd3665947bb5a4f24ae1a97c094911d32d113e5b3ff01e74611d171c252baa0".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein1024-64 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein1024-64 --binary",
        result: Some(Value::binary(
          vec![0x2b,0x77,0x42,0x8b,0xf8,0x3d,0x4a,0x74,0x77,0x53,0x14,0x5c,0xde,0x77,0xbe,0x6b,0x8a,0x6c,0x61,0x8e,0x33,0x94,0x6f,0x9e,0x8c,0x2e,0xd0,0xa6,0xf9,0x77,0x97,0x82,0x8c,0xd3,0x66,0x59,0x47,0xbb,0x5a,0x4f,0x24,0xae,0x1a,0x97,0xc0,0x94,0x91,0x1d,0x32,0xd1,0x13,0xe5,0xb3,0xff,0x01,0xe7,0x46,0x11,0xd1,0x71,0xc2,0x52,0xba,0xa0],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein1024-64 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash skein1024-64",
        result: None,
      },
    ]
  }
}

impl Hasher for skein::Skein1024::<skein::consts::U128> {
  fn name() -> &'static str {
    "skein1024-128"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the skein1024-128 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein1024-128",
        result: Some(Value::string(
          "f23d95c2a25fbcd0e797cd058fec39d3c52d2b5afd7a9af1df934e63257d1d3dcf3246e7329c0f1104c1e51e3d22e300507b0c3b9f985bb1f645ef49835080536becf83788e17fed09c9982ba65c3cb7ffe6a5f745b911c506962adf226e435c42f6f6bc08d288f9c810e807e3216ef444f3db22744441deefa4900982a1371f".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein1024-128 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash skein1024-128 --binary",
        result: Some(Value::binary(
          vec![0xf2,0x3d,0x95,0xc2,0xa2,0x5f,0xbc,0xd0,0xe7,0x97,0xcd,0x05,0x8f,0xec,0x39,0xd3,0xc5,0x2d,0x2b,0x5a,0xfd,0x7a,0x9a,0xf1,0xdf,0x93,0x4e,0x63,0x25,0x7d,0x1d,0x3d,0xcf,0x32,0x46,0xe7,0x32,0x9c,0x0f,0x11,0x04,0xc1,0xe5,0x1e,0x3d,0x22,0xe3,0x00,0x50,0x7b,0x0c,0x3b,0x9f,0x98,0x5b,0xb1,0xf6,0x45,0xef,0x49,0x83,0x50,0x80,0x53,0x6b,0xec,0xf8,0x37,0x88,0xe1,0x7f,0xed,0x09,0xc9,0x98,0x2b,0xa6,0x5c,0x3c,0xb7,0xff,0xe6,0xa5,0xf7,0x45,0xb9,0x11,0xc5,0x06,0x96,0x2a,0xdf,0x22,0x6e,0x43,0x5c,0x42,0xf6,0xf6,0xbc,0x08,0xd2,0x88,0xf9,0xc8,0x10,0xe8,0x07,0xe3,0x21,0x6e,0xf4,0x44,0xf3,0xdb,0x22,0x74,0x44,0x41,0xde,0xef,0xa4,0x90,0x09,0x82,0xa1,0x37,0x1f],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the skein1024-128 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash skein1024-128",
        result: None,
      },
    ]
  }
}

impl Hasher for sm3::Sm3 {
  fn name() -> &'static str {
    "sm3"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the sm3 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sm3",
        result: Some(Value::string(
          "b80fe97a4da24afc277564f66a359ef440462ad28dcc6d63adb24d5c20a61595".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sm3 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash sm3 --binary",
        result: Some(Value::binary(
          vec![0xb8,0x0f,0xe9,0x7a,0x4d,0xa2,0x4a,0xfc,0x27,0x75,0x64,0xf6,0x6a,0x35,0x9e,0xf4,0x40,0x46,0x2a,0xd2,0x8d,0xcc,0x6d,0x63,0xad,0xb2,0x4d,0x5c,0x20,0xa6,0x15,0x95],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the sm3 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash sm3",
        result: None,
      },
    ]
  }
}

impl Hasher for streebog::Streebog256 {
  fn name() -> &'static str {
    "streebog256"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the streebog256 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash streebog256",
        result: Some(Value::string(
          "c9086ed61fb0a090aaf4438efd39f0d060cb3ec7e25343b5c4c350054bfd3e27".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the streebog256 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash streebog256 --binary",
        result: Some(Value::binary(
          vec![0xc9,0x08,0x6e,0xd6,0x1f,0xb0,0xa0,0x90,0xaa,0xf4,0x43,0x8e,0xfd,0x39,0xf0,0xd0,0x60,0xcb,0x3e,0xc7,0xe2,0x53,0x43,0xb5,0xc4,0xc3,0x50,0x05,0x4b,0xfd,0x3e,0x27],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the streebog256 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash streebog256",
        result: None,
      },
    ]
  }
}

impl Hasher for streebog::Streebog512 {
  fn name() -> &'static str {
    "streebog512"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the streebog512 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash streebog512",
        result: Some(Value::string(
          "ec7b127dcca6b0d741b10ed42062cc4487b4a93f96cfc7faf2e7f79778b1f44159089c91fb0910bec0eee7cdca524fcf291cf933fff406f4f3a03872f2341ff8".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the streebog512 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash streebog512 --binary",
        result: Some(Value::binary(
          vec![0xec,0x7b,0x12,0x7d,0xcc,0xa6,0xb0,0xd7,0x41,0xb1,0x0e,0xd4,0x20,0x62,0xcc,0x44,0x87,0xb4,0xa9,0x3f,0x96,0xcf,0xc7,0xfa,0xf2,0xe7,0xf7,0x97,0x78,0xb1,0xf4,0x41,0x59,0x08,0x9c,0x91,0xfb,0x09,0x10,0xbe,0xc0,0xee,0xe7,0xcd,0xca,0x52,0x4f,0xcf,0x29,0x1c,0xf9,0x33,0xff,0xf4,0x06,0xf4,0xf3,0xa0,0x38,0x72,0xf2,0x34,0x1f,0xf8],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the streebog512 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash streebog512",
        result: None,
      },
    ]
  }
}

impl Hasher for tiger::Tiger {
  fn name() -> &'static str {
    "tiger"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the tiger hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash tiger",
        result: Some(Value::string(
          "1714a472eee57d30040412bfcc55032a0b11602ff37beee9".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the tiger hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash tiger --binary",
        result: Some(Value::binary(
          vec![0x17,0x14,0xa4,0x72,0xee,0xe5,0x7d,0x30,0x04,0x04,0x12,0xbf,0xcc,0x55,0x03,0x2a,0x0b,0x11,0x60,0x2f,0xf3,0x7b,0xee,0xe9],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the tiger hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash tiger",
        result: None,
      },
    ]
  }
}

impl Hasher for tiger::Tiger2 {
  fn name() -> &'static str {
    "tiger2"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the tiger2 hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash tiger2",
        result: Some(Value::string(
          "f5b6b6a78c405c8547e91cd8624cb8be83fc804a474488fd".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the tiger2 hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash tiger2 --binary",
        result: Some(Value::binary(
          vec![0xf5,0xb6,0xb6,0xa7,0x8c,0x40,0x5c,0x85,0x47,0xe9,0x1c,0xd8,0x62,0x4c,0xb8,0xbe,0x83,0xfc,0x80,0x4a,0x47,0x44,0x88,0xfd],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the tiger2 hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash tiger2",
        result: None,
      },
    ]
  }
}

impl Hasher for whirlpool::Whirlpool {
  fn name() -> &'static str {
    "whirlpool"
  }

  fn examples() -> Vec<Example<'static>> {
    vec![
      Example {
        description: "Return the whirlpool hash of a string, hex-encoded",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash whirlpool",
        result: Some(Value::string(
          "f1d754662636ffe92c82ebb9212a484a8d38631ead4238f5442ee13b8054e41b08bf2a9251c30b6a0b8aae86177ab4a6f68f673e7207865d5d9819a3dba4eb3b".to_owned(),
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the whirlpool hash of a string, as binary",
        example: "'abcdefghijklmnopqrstuvwxyz' | hash whirlpool --binary",
        result: Some(Value::binary(
          vec![0xf1,0xd7,0x54,0x66,0x26,0x36,0xff,0xe9,0x2c,0x82,0xeb,0xb9,0x21,0x2a,0x48,0x4a,0x8d,0x38,0x63,0x1e,0xad,0x42,0x38,0xf5,0x44,0x2e,0xe1,0x3b,0x80,0x54,0xe4,0x1b,0x08,0xbf,0x2a,0x92,0x51,0xc3,0x0b,0x6a,0x0b,0x8a,0xae,0x86,0x17,0x7a,0xb4,0xa6,0xf6,0x8f,0x67,0x3e,0x72,0x07,0x86,0x5d,0x5d,0x98,0x19,0xa3,0xdb,0xa4,0xeb,0x3b],
          Span::test_data(),
        )),
      },
      Example {
        description: "Return the whirlpool hash of a file's contents",
        example: "open ./nu_0_24_1_windows.zip | hash whirlpool",
        result: None,
      },
    ]
  }
}
