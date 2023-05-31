use super::{
    macros::{read_primitive, read_primitive_array, write_primitive, write_primitive_array},
    traits::{FieldMap, ParsableClassKeyed},
};
use crate::enums::{Difficulty, Level, Lockable};
use indexmap::IndexMap;
use ms_nrbf::{Class, Field, Primitive, PrimitiveArray};

#[derive(Debug)]
pub struct DifficultyData {
    pub current_level: Option<Level>,
    pub prime_levels: Vec<Lockable>,
    pub file_exists: bool,
}

const CURRENT_LEVEL_FIELD: &str = "levelNum";
const DIFFICULTY_FIELD: &str = "difficulty";
const PRIME_LEVELS_FIELD: &str = "primeLevels";

impl ParsableClassKeyed<Difficulty> for DifficultyData {
    const CLASS_NAME: &'static str = "GameProgressData";
    const FILE_PREFIX: &'static str = "difficulty";
    const FILE_SUFFIX: &'static str = "progress.bepis";

    fn get_file_exists(&self) -> bool {
        self.file_exists
    }

    fn create_new(_variant: &Difficulty) -> Self {
        Self {
            current_level: Some(Level::IntoTheFire),
            prime_levels: vec![Lockable::Locked; 3],
            file_exists: false,
        }
    }

    fn parse(class: &Class) -> Option<Self> {
        let mut prime_levels = vec![];

        for value in read_primitive_array!(class, PRIME_LEVELS_FIELD, Int32) {
            prime_levels.push(Lockable::from_repr(value as u8)?)
        }

        Some(Self {
            current_level: Level::from_repr(
                read_primitive!(class, CURRENT_LEVEL_FIELD, Int32) as u16,
            ),
            prime_levels,
            file_exists: true,
        })
    }

    fn unparse(&self, difficulty: &Difficulty) -> Option<FieldMap> {
        let mut fields = IndexMap::new();

        write_primitive!(
            fields,
            CURRENT_LEVEL_FIELD,
            Int32,
            match self.current_level {
                Some(current_level) => current_level as i32,
                None => 26,
            }
        );
        write_primitive!(
            fields,
            DIFFICULTY_FIELD,
            Int32,
            *difficulty as i32
        );
        write_primitive_array!(
            fields,
            PRIME_LEVELS_FIELD,
            Int32,
            self.prime_levels
                .iter()
                .map(|value| *value as i32)
                .collect::<Vec<i32>>()
        );

        Some(fields)
    }
}
