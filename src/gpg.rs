extern crate gpgme;

use std::str::Utf8Error;
use std::vec::Vec;
use std::clone::Clone;
use gpgme::Protocol;

trait KeyListProvider {
    type K: KeyProvider;

    fn new() -> Self;
    fn keys(&mut self) -> Result<Vec<Self::K>, &'static str>;
    fn import(&mut self, &str) -> Result<Self::K, &'static str>;
}

trait KeyProvider {
    fn id(&self) -> Result<&str, Option<Utf8Error>>;
    //fn sign(&mut self) -> Result<(), &'static str>;
    //fn remove(&mut self) -> Result<(), &'static str>;
}

struct GPGKeyList {
    context: gpgme::Context,
}

impl KeyListProvider for GPGKeyList {
    type K = gpgme::Key;

    fn new() -> Self {
        GPGKeyList { context: gpgme::Context::from_protocol(Protocol::OpenPgp).unwrap() }
    }

    fn keys(&mut self) -> Result<Vec<gpgme::Key>, &'static str> {
        self.context
            .keys()
            .map(|keys| keys.filter_map(Result::ok).collect())
            .or_else(|_| Err("Could not get any keys from GPGME"))
    }

    fn import(&mut self, filepath: &str) -> Result<gpgme::Key, &'static str> {
        gpgme::Data::load(filepath)
            .and_then(|d| self.context.import(&mut d))
            .and_then(|r| match r.imports().next() {
                Some(k) => k.fingerprint(),
                None => _
            })
            .and_then(|f| self.context.find_key(f))
            .or_else(|_| Err(format!("Could not import key in {}", filepath)))
    }
}

struct MockKeyList {
    keys: Vec<MockKey>,
}

impl KeyListProvider for MockKeyList {
    type K = MockKey;

    fn new() -> Self {
        MockKeyList { keys: Vec::new() }
    }

    fn keys(&mut self) -> Result<Vec<MockKey>, &'static str> {
        Ok(self.keys.clone())
    }

    fn import(&mut self, filepath: &str) -> Result<MockKey, &'static str> {
        let key = MockKey;
        self.keys.push(key);
        Ok(key)
    }
}

impl KeyProvider for gpgme::Key {
    fn id(&self) -> Result<&str, Option<Utf8Error>> {
        Self::id(&self)
    }
}

struct MockKey;

impl Clone for MockKey {
    fn clone(&self) -> Self {
        MockKey
    }
}

impl KeyProvider for MockKey {
    fn id(&self) -> Result<&str, Option<Utf8Error>> {
        Ok("hej")
    }
}

pub fn get_keys() -> Result<Vec<gpgme::Key>, &'static str> {
    let mut provider = GPGKeyList::new();
    provider.keys()
}
