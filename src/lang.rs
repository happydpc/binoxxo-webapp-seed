use elsa::FrozenMap;
use fluent_bundle::{FluentBundle, FluentResource};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref LANG_RESOURCES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("de-DE", include_str!("lang/de-DE"));
        m.insert("en-US", include_str!("lang/en-US"));
        m
    };
}

pub struct ResourceManager {
    resources: FrozenMap<String, Box<FluentResource>>,
}

impl std::fmt::Debug for ResourceManager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ResourceManager {{}}")
    }
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            resources: FrozenMap::new(),
        }
    }

    fn get_resource(&self, locale: &str) -> &FluentResource {
        if let Some(res) = self.resources.get(locale) {
            res
        } else {
            let raw_resource = LANG_RESOURCES.get(locale).unwrap();
            let res = match FluentResource::try_new(raw_resource.to_string()) {
                Ok(res) => res,
                Err((res, _err)) => res,
            };
            self.resources.insert(locale.to_string(), Box::new(res))
        }
    }

    pub fn get_bundle(&self, locale: &str) -> FluentBundle {
        let mut bundle = FluentBundle::new(&[locale]);
        let res = self.get_resource(locale);
        bundle.add_resource(res).unwrap();
        bundle
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trans_en() {
        let rmgr = ResourceManager::new();
        let b = rmgr.get_bundle("en-US");

        assert_eq!("Easy", b.format("difficulty-Easy", None).unwrap().0);
    }

    #[test]
    fn trans_de() {
        let rmgr = ResourceManager::new();
        let b = rmgr.get_bundle("de-DE");

        assert_eq!("Leicht", b.format("difficulty-Easy", None).unwrap().0);
    }
}