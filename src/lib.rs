use cirru_edn::{Edn, EdnListView, EdnMapView, EdnRecordView, EdnTupleView};
use cirru_parser::Cirru;
use json::JsonValue;
use std::{collections::HashMap, sync::Arc};

#[no_mangle]
pub fn abi_version() -> String {
  String::from("0.0.8")
}

#[no_mangle]
pub fn json_stringify(args: Vec<Edn>) -> Result<Edn, String> {
  if args.len() == 1 || args.len() == 2 {
    let pretty = if args.len() == 2 {
      match args[1] {
        Edn::Bool(b) => b,
        Edn::Nil => false,
        _ => return Err(format!("json-stringify expected 2nd arg to be bool, got {:?}", args)),
      }
    } else {
      false
    };
    let edn = &args[0];
    let json = edn_to_json(edn)?;
    if pretty {
      Ok(Edn::str(json.pretty(2)))
    } else {
      Ok(Edn::str(json.dump()))
    }
  } else {
    Err(format!("json-stringify expected 1 arg, got {:?}", args))
  }
}

#[no_mangle]
pub fn json_parse(args: Vec<Edn>) -> Result<Edn, String> {
  if args.len() == 1 {
    if let Edn::Str(content) = &args[0] {
      let json = json::parse(content).unwrap();
      Ok(json_to_edn(json))
    } else {
      Err(format!("json-parse expected string, got {:?}", args))
    }
  } else {
    Err(format!("json-parse expected 1 arg, got {:?}", args))
  }
}

/// convert json to edn
fn json_to_edn(json: JsonValue) -> Edn {
  match json {
    JsonValue::Null => Edn::Nil,
    JsonValue::Short(s) => Edn::Str(Arc::from(s.to_string())),
    JsonValue::String(s) => Edn::str(s),
    JsonValue::Number(n) => Edn::Number(n.into()),
    JsonValue::Boolean(b) => Edn::Bool(b),
    JsonValue::Array(arr) => {
      let mut vec = Vec::new();
      for item in arr {
        vec.push(json_to_edn(item));
      }
      Edn::List(EdnListView(vec))
    }
    JsonValue::Object(obj) => {
      let mut map = HashMap::new();
      for (k, v) in obj.iter() {
        map.insert(k.into(), json_to_edn(v.clone()));
      }
      Edn::Map(EdnMapView::from(map))
    }
  }
}

// convert edn to json
fn edn_to_json(edn: &Edn) -> Result<JsonValue, String> {
  match edn {
    Edn::Nil => Ok(JsonValue::Null),
    Edn::Str(s) => Ok(JsonValue::String(s.to_string())),
    Edn::Number(n) => Ok(JsonValue::Number((*n).into())),
    Edn::Bool(b) => Ok(JsonValue::Boolean(*b)),
    Edn::List(list) => {
      let mut arr = Vec::new();
      for item in list {
        arr.push(edn_to_json(item)?);
      }
      Ok(JsonValue::Array(arr))
    }
    Edn::Map(map) => {
      let mut obj = json::object::Object::new();
      for (k, v) in &map.0 {
        if let Edn::Str(k) = k {
          obj.insert(k, edn_to_json(v)?);
        } else if let Edn::Tag(k) = k {
          obj.insert(&k.arc_str(), edn_to_json(v)?);
        } else {
          return Err(format!("json-parse expected string, got {:?}", k));
        }
      }
      Ok(JsonValue::Object(obj))
    }
    Edn::Symbol(s) => Ok(JsonValue::String(s.to_string())),
    Edn::Tag(s) => Ok(JsonValue::String(s.to_string())),
    Edn::Set(xs) => {
      let mut arr = Vec::new();
      for x in &xs.0 {
        arr.push(edn_to_json(x)?);
      }
      Ok(JsonValue::Array(arr))
    }
    Edn::Tuple(EdnTupleView { tag, extra }) => {
      let mut arr = vec![edn_to_json(&tag.to_owned())?];
      for item in extra {
        arr.push(edn_to_json(&item.to_owned())?);
      }
      Ok(JsonValue::Array(arr))
    }
    Edn::Quote(x) => cirru_to_json(x),
    Edn::Buffer(buf) => Ok(JsonValue::String(format!("0x{}", hex::encode(buf)))),
    Edn::Record(EdnRecordView { tag: _r, pairs: entries }) => {
      let mut obj = json::object::Object::new();
      for (k, v) in entries {
        obj.insert(&k.arc_str(), edn_to_json(v)?);
      }
      Ok(JsonValue::Object(obj))
    }
    Edn::AnyRef(_r) => Err("any-ref is a reference of unknown".to_owned()),
  }
}

/// convert cirru into json
fn cirru_to_json(code: &Cirru) -> Result<JsonValue, String> {
  match code {
    Cirru::Leaf(s) => Ok(JsonValue::String(s.to_string())),
    Cirru::List(list) => {
      let mut arr = Vec::new();
      for item in list {
        arr.push(cirru_to_json(item)?);
      }
      Ok(JsonValue::Array(arr))
    }
  }
}
