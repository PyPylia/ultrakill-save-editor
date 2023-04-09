use super::{
    macros::{read_primitive, read_primitive_array, write_primitive, write_primitive_array},
    traits::{FieldMap, ParsableClassKeyed},
};
use crate::enums::{Level, LevelRank};
use indexmap::IndexMap;
use ms_nrbf::{Class, Field, Primitive, PrimitiveArray};

#[derive(Debug)]
pub struct LevelData {
    pub ranks: Vec<LevelRank>,
    pub secrets_found: Vec<bool>,
    pub challenge: bool,
    pub major_assists: Vec<bool>,
    pub file_exists: bool,
}

const RANKS_FIELD: &str = "ranks";
const SECRETS_AMOUNT_FIELD: &str = "secretsAmount";
const SECRETS_FOUND_FIELD: &str = "secretsFound";
const CHALLENGE_FIELD: &str = "challenge";
const LEVEL_NUMBER_FIELD: &str = "levelNumber";
const MAJOR_ASSISTS_FIELD: &str = "majorAssists";

impl ParsableClassKeyed<Level> for LevelData {
    const CLASS_NAME: &'static str = "RankData";
    const FILE_PREFIX: &'static str = "lvl";
    const FILE_SUFFIX: &'static str = "progress.bepis";

    fn get_file_exists(&self) -> bool {
        self.file_exists
    }

    fn create_new(level: &Level) -> Self {
        Self {
            ranks: vec![LevelRank::None; 6],
            secrets_found: vec![false; level.get_secret_count() as usize],
            challenge: false,
            major_assists: vec![false; 6],
            file_exists: false,
        }
    }

    fn parse(class: &Class) -> Option<Self> {
        Some(Self {
            ranks: read_primitive_array!(class, RANKS_FIELD, Int32)
                .iter()
                .map(|value| (*value).into())
                .collect(),
            secrets_found: read_primitive_array!(class, SECRETS_FOUND_FIELD, Boolean),
            challenge: *read_primitive!(class, CHALLENGE_FIELD, Boolean),
            major_assists: read_primitive_array!(class, MAJOR_ASSISTS_FIELD, Boolean),
            file_exists: true,
        })
    }

    fn unparse(&self, level: &Level) -> Option<FieldMap> {
        let mut fields = IndexMap::new();

        write_primitive_array!(
            fields,
            RANKS_FIELD,
            Int32,
            self.ranks
                .iter()
                .map(|value| *value as i32)
                .collect::<Vec<i32>>()
        );
        write_primitive!(
            fields,
            SECRETS_AMOUNT_FIELD,
            Int32,
            self.secrets_found.len() as i32
        );
        write_primitive_array!(
            fields,
            SECRETS_FOUND_FIELD,
            Boolean,
            self.secrets_found
        );
        write_primitive!(
            fields,
            CHALLENGE_FIELD,
            Boolean,
            self.challenge
        );
        write_primitive!(
            fields,
            LEVEL_NUMBER_FIELD,
            Int32,
            *level as i32
        );
        write_primitive_array!(
            fields,
            MAJOR_ASSISTS_FIELD,
            Boolean,
            self.major_assists
        );

        Some(fields)
    }
}
