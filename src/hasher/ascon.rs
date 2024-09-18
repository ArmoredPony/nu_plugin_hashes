use ascon_hash::AsconHash;
use nu_protocol::Example;

use super::{GenericHasher, Hasher};

pub type HasherAscon = GenericHasher<AsconHash>;

impl Hasher for AsconHash {
  fn name() -> &'static str {
    todo!()
  }

  fn examples() -> Vec<Example<'static>> {
    todo!()
  }
}
