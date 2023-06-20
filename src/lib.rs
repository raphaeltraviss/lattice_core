use std::ffi::CString;
use std::os::raw::c_char;
use autosurgeon::{Reconcile, Hydrate, hydrate, reconcile};

#[no_mangle]
pub extern "C" fn lc_init() -> *mut c_char {
  let s = execute();
  let cs = CString::new(s).unwrap();
  cs.into_raw()
}

#[no_mangle]
pub extern "C" fn lc_free(s: *mut c_char) {
  unsafe {
    if s.is_null() { return }
    drop(CString::from_raw(s))
  };
}

#[derive(Debug, Clone, Reconcile, Hydrate, PartialEq)]
struct Contact {
  name: String,
  address: Address,
}

#[derive(Debug, Clone, Reconcile, Hydrate, PartialEq)]
struct Address {
  line_one: String,
  line_two: Option<String>,
  city: String,
  postcode: String,
}

fn execute() -> String {
  let mut contact = Contact {
  name: "Sherlock Holmes".to_string(),
  address: Address{
      line_one: "221B Baker St".to_string(),
      line_two: None,
      city: "London".to_string(),
      postcode: "NW1 6XE".to_string(),
  },
  };

  let mut doc = automerge::AutoCommit::new();
  reconcile(&mut doc, &contact).unwrap();

  let contact2: Contact = hydrate(&doc).unwrap();

  let mut doc2 = doc.fork().with_actor(automerge::ActorId::random());
  let mut contact2: Contact = hydrate(&doc2).unwrap();
  contact2.name = "Dangermouse".to_string();
  reconcile(&mut doc2, &contact2).unwrap();

  contact.address.line_one = "221C Baker St".to_string();
  reconcile(&mut doc, &contact).unwrap();

  doc.merge(&mut doc2).unwrap();

  let merged: Contact = hydrate(&doc).unwrap();

  merged.name
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
