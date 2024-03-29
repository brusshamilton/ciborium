// SPDX-License-Identifier: Apache-2.0

use serde::de::Deserializer;

use std::fmt;

use rstest::rstest;

struct TestVisitor;

impl TestVisitor {
    pub fn new() -> TestVisitor{
        TestVisitor{}
    }
}

impl<'de> serde::de::DeserializeSeed<'de> for TestVisitor {
  type Value = String;

  fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
  where
      D: serde::de::Deserializer<'de>,
  {
      deserializer.deserialize_any(self)
  }
}

impl<'de> serde::de::Visitor<'de> for TestVisitor {
  type Value = String;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("any valid CBOR")
  }

  fn visit_i32<E: serde::de::Error>(self, value: i32) -> Result<Self::Value, E> {
      Ok(value.to_string())
  }

  fn visit_i8<E: serde::de::Error>(self, value: i8) -> Result<Self::Value, E> {
      Ok(value.to_string())
  }

  fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Self::Value, E> {
      Ok(value.to_string())
  }

  fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
      Ok(value.to_string())
  }

  fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
      Ok(value.to_string())
  }

  fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Self::Value, E> {
      Ok(value.to_string())
  }

  fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
      Ok("\"".to_string()+value+"\"")
  }

  fn visit_borrowed_str<E: serde::de::Error>(self, value: &'de str) -> Result<Self::Value, E> {
      Ok("\"".to_string()+value+"\"")
  }

  fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Self::Value, E> {
      Ok("\"".to_string()+&value+"\"")
  }

  fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
      Ok("none".to_string())
  }

  fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
      Ok("unit".to_string())
  }

  fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
  where
      M: serde::de::MapAccess<'de>,
  {
      let mut result = "{".to_string();
      while let Some(key) = access.next_key::<Self::Value>()? {
        let mut element = String::new();
        element += &("\"".to_string() + &key + "\": ");
        let value = access.next_value_seed(TestVisitor{})?;
        element += &(value + ", ");
        result += &element;
      }
      result += "}";
      Ok(result)
  }

  fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
  where
      S: serde::de::SeqAccess<'de>,
  {
    let mut result = "[".to_string();
    while let Some(element) = access.next_element_seed(TestVisitor {})? {
      result += &(element + ", ");
    }
    result += "]";
    Ok(result)
  }
}

#[rstest(expected, bytes,
  case("[1, \"red\", ]", "820163726564"),
  case("{\"Foo\": 1, \"egg\": true, \"null\": none, }", "A363466F6F0163656767F5646E756C6CF6"),
)]
fn visitor_test<'de>(expected: &str, bytes: &str) {
    let bytes = hex::decode(bytes).unwrap();

    let visitor = TestVisitor::new();
    let mut deserializer = ciborium::de::Deserializer::from_reader(&bytes[..]);
    let got = deserializer.deserialize_any(visitor).unwrap();
    assert_eq!(expected, got);
}
