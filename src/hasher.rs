//! Contains a generic trait for hashers that implement `Digest`.
//! This implementation is stolen with minimal changes from *generic_digest.rs*
//! source code file of nushell v0.98.0 which can be found at
//! https://github.com/nushell/nushell/blob/0.98.0/crates/nu-command/src/hash/generic_digest.rs
//! The *hash* crate is private, so I had no choice.

use std::{io::Write, marker::PhantomData, ops::Not};

use digest::{Digest, Output};
use nu_cmd_base::input_handler::{operate, CmdArgument};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{
  ast::CellPath,
  Category,
  Example,
  IntoPipelineData,
  LabeledError,
  PipelineData,
  ShellError,
  Signature,
  Span,
  SyntaxShape,
  Type,
  Value,
};

use crate::HashesPlugin;

pub trait Hasher: Digest + Clone {
  fn name() -> &'static str;
  fn examples() -> Vec<Example<'static>>;
}

#[derive(Clone)]
pub struct GenericHasher<H: Hasher> {
  name: String,
  description: String,
  _hasher: PhantomData<H>,
}

impl<H: Hasher> Default for GenericHasher<H> {
  fn default() -> Self {
    Self {
      name: format!("hash {}", H::name()),
      description: format!(
        "Hash a value using the {} hash algorithm.",
        H::name()
      ),
      _hasher: PhantomData,
    }
  }
}

struct Arguments {
  cell_paths: Option<Vec<CellPath>>,
  binary: bool,
}

impl CmdArgument for Arguments {
  fn take_cell_paths(&mut self) -> Option<Vec<CellPath>> {
    self.cell_paths.take()
  }
}

impl<H> PluginCommand for GenericHasher<H>
where
  H: Hasher + Write + Send + Sync + 'static,
  Output<H>: core::fmt::LowerHex,
{
  type Plugin = HashesPlugin;

  fn name(&self) -> &str {
    &self.name
  }

  fn signature(&self) -> Signature {
    Signature::build(self.name())
      .category(Category::Hash)
      .input_output_types(vec![
        (Type::Binary, Type::Any),
        (Type::String, Type::Any),
        (Type::table(), Type::table()),
        (Type::record(), Type::record()),
      ])
      .allow_variants_without_examples(true)
      .switch(
        "binary",
        "Output binary instead of hexadecimal representation",
        Some('b'),
      )
      .rest(
        "rest",
        SyntaxShape::CellPath,
        format!("Optionally {} hash data by cell path.", H::name()),
      )
  }

  fn description(&self) -> &str {
    &self.description
  }

  fn examples(&self) -> Vec<Example> {
    H::examples()
  }

  fn run(
    &self,
    _plugin: &HashesPlugin,
    engine: &EngineInterface,
    call: &EvaluatedCall,
    input: PipelineData,
  ) -> Result<PipelineData, LabeledError> {
    let head = call.head;
    let binary = call.has_flag("binary")?;
    let cell_paths: Vec<CellPath> = call.rest(0)?;
    let cell_paths = cell_paths.is_empty().not().then_some(cell_paths);

    if let PipelineData::ByteStream(stream, ..) = input {
      let mut hasher = H::new();
      stream.write_to(&mut hasher)?;
      let digest = hasher.finalize();
      if binary {
        Ok(Value::binary(digest.to_vec(), head).into_pipeline_data())
      } else {
        Ok(Value::string(format!("{digest:x}"), head).into_pipeline_data())
      }
    } else {
      operate(
        action::<H>,
        Arguments { binary, cell_paths },
        input,
        head,
        engine.signals(),
      )
      .map_err(Into::into)
    }
  }
}

fn action<H>(input: &Value, args: &Arguments, _span: Span) -> Value
where
  H: Hasher,
  Output<H>: core::fmt::LowerHex,
{
  let span = input.span();
  let (bytes, span) = match input {
    Value::String { val, .. } => (val.as_bytes(), span),
    Value::Binary { val, .. } => (val.as_slice(), span),
    // Propagate existing errors
    Value::Error { .. } => return input.clone(),
    other => {
      let span = input.span();

      return Value::error(
        ShellError::OnlySupportsThisInputType {
          exp_input_type: "string or binary".into(),
          wrong_type: other.get_type().to_string(),
          dst_span: span,
          src_span: other.span(),
        },
        span,
      );
    }
  };

  let digest = H::digest(bytes);

  if args.binary {
    Value::binary(digest.to_vec(), span)
  } else {
    Value::string(format!("{digest:x}"), span)
  }
}
