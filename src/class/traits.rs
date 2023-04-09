use indexmap::IndexMap;
use ms_nrbf::{Class, Field, Stream};
use std::{
    collections::BTreeMap,
    fmt,
    fs::{remove_file, File},
    io,
    path::Path,
    str::FromStr,
};
use strum::IntoEnumIterator;

pub const LIBRARY_NAME: &str =
    "Assembly-CSharp, Version=0.0.0.0, Culture=neutral, PublicKeyToken=null";

pub type FieldMap = IndexMap<String, Field>;

pub trait IntoFileInfix {
    fn into_file_infix(&self) -> Box<dyn fmt::Display>;
}

pub(super) trait ParsableClass
where
    Self: Sized + Default,
{
    const CLASS_NAME: &'static str;
    const FILE_NAME: &'static str;

    fn parse(class: &Class) -> Option<Self>;
    fn unparse(&self) -> Option<FieldMap>;
    fn get_file_exists(&self) -> bool;
}
pub(super) trait ParsableClassKeyed<T: FromStr + IntoEnumIterator + Ord + IntoFileInfix>
where
    Self: Sized,
{
    const CLASS_NAME: &'static str;
    const FILE_PREFIX: &'static str;
    const FILE_SUFFIX: &'static str;

    fn create_new(variant: &T) -> Self;
    fn parse(class: &Class) -> Option<Self>;
    fn unparse(&self, variant: &T) -> Option<FieldMap>;
    fn get_file_exists(&self) -> bool;
}

pub(super) trait LoadableSavable {
    fn load<P: AsRef<Path>>(save_path: P) -> Self;
    fn save<P: AsRef<Path>>(&self, save_path: P) -> Result<(), io::Error>;
}

impl<T: ParsableClass> LoadableSavable for T {
    fn load<P: AsRef<Path>>(save_path: P) -> Self {
        let path = save_path.as_ref().join(Self::FILE_NAME);

        if let Ok(mut file) = File::open(path) {
            if let Ok(stream) = Stream::decode(&mut file) {
                if let Some(value) = Self::parse(&stream.root) {
                    return value;
                }
            }
        }

        Self::default()
    }

    fn save<P: AsRef<Path>>(&self, save_path: P) -> Result<(), io::Error> {
        let path = save_path.as_ref().join(Self::FILE_NAME);

        if self.get_file_exists() {
            if let Some(fields) = self.unparse() {
                let mut file = File::create(path)?;

                Stream {
                    root: Class {
                        library_name: LIBRARY_NAME.to_string(),
                        name: Self::CLASS_NAME.to_string(),
                        fields,
                    },
                }
                .encode(&mut file)?;
            }
        } else if path.exists() {
            remove_file(path)?;
        }

        Ok(())
    }
}

impl<V: FromStr + IntoEnumIterator + Ord + IntoFileInfix, T: ParsableClassKeyed<V>> LoadableSavable
    for BTreeMap<V, T>
{
    fn load<P: AsRef<Path>>(save_path: P) -> BTreeMap<V, T> {
        let mut map = BTreeMap::new();

        for variant in V::iter() {
            let path = save_path.as_ref().join(format!(
                "{}{}{}",
                T::FILE_PREFIX,
                variant.into_file_infix(),
                T::FILE_SUFFIX
            ));

            if let Ok(mut file) = File::open(path) {
                if let Ok(stream) = Stream::decode(&mut file) {
                    if let Some(value) = T::parse(&stream.root) {
                        map.insert(variant, value);
                        continue;
                    }
                }
            }

            let value = T::create_new(&variant);
            map.insert(variant, value);
        }

        map
    }

    fn save<P: AsRef<Path>>(&self, save_path: P) -> Result<(), io::Error> {
        for (key, value) in self {
            let path = save_path.as_ref().join(format!(
                "{}{}{}",
                T::FILE_PREFIX,
                key.into_file_infix(),
                T::FILE_SUFFIX
            ));

            if value.get_file_exists() {
                if let Some(fields) = value.unparse(&key) {
                    let mut file = File::create(path)?;

                    Stream {
                        root: Class {
                            library_name: LIBRARY_NAME.to_string(),
                            name: T::CLASS_NAME.to_string(),
                            fields,
                        },
                    }
                    .encode(&mut file)?;
                }
            } else if path.exists() {
                remove_file(path)?;
            }
        }

        Ok(())
    }
}
