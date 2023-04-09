use super::{
    macros::{read_primitive_array, write_primitive, write_primitive_array},
    traits::{FieldMap, ParsableClass},
};
use indexmap::IndexMap;
use ms_nrbf::{Class, Field, Primitive, PrimitiveArray};

#[derive(Debug)]
pub struct CybergrindData {
    pub waves: Vec<String>, //f32
    pub kills: Vec<String>, //i32
    pub style: Vec<String>, //i32
    pub times: Vec<String>, //f32
    pub file_exists: bool,
}

impl Default for CybergrindData {
    fn default() -> Self {
        Self {
            waves: vec!["0.0".to_string(); 6],
            kills: vec!["0".to_string(); 6],
            style: vec!["0".to_string(); 6],
            times: vec!["0.0".to_string(); 6],
            file_exists: false,
        }
    }
}

macro_rules! parse_string_array {
    ($array:expr) => {{
        let mut new_array = vec![];

        for value in $array {
            new_array.push(match value.is_empty() {
                true => <_>::default(),
                false => value.parse().ok()?,
            })
        }

        new_array
    }};
}

const WAVE_INT_FIELD: &str = "wave";
const WAVES_FIELD: &str = "preciseWavesByDifficulty";
const KILLS_FIELD: &str = "kills";
const STYLE_FIELD: &str = "style";
const TIMES_FIELD: &str = "time";

impl ParsableClass for CybergrindData {
    const CLASS_NAME: &'static str = "CyberRankData";
    const FILE_NAME: &'static str = "cybergrindhighscore.bepis";

    fn get_file_exists(&self) -> bool {
        self.file_exists
    }

    fn parse(class: &Class) -> Option<Self> {
        Some(Self {
            waves: read_primitive_array!(class, WAVES_FIELD, Single)
                .iter()
                .map(|value| value.to_string())
                .collect(),
            kills: read_primitive_array!(class, KILLS_FIELD, Int32)
                .iter()
                .map(|value| value.to_string())
                .collect(),
            style: read_primitive_array!(class, STYLE_FIELD, Int32)
                .iter()
                .map(|value| value.to_string())
                .collect(),
            times: read_primitive_array!(class, TIMES_FIELD, Single)
                .iter()
                .map(|value| value.to_string())
                .collect(),
            file_exists: true,
        })
    }

    fn unparse(&self) -> Option<FieldMap> {
        let mut fields = IndexMap::new();

        write_primitive!(fields, WAVE_INT_FIELD, Int32, 0);
        write_primitive_array!(
            fields,
            WAVES_FIELD,
            Single,
            parse_string_array!(&self.waves)
        );
        write_primitive_array!(
            fields,
            KILLS_FIELD,
            Int32,
            parse_string_array!(&self.kills)
        );
        write_primitive_array!(
            fields,
            STYLE_FIELD,
            Int32,
            parse_string_array!(&self.style)
        );
        write_primitive_array!(
            fields,
            TIMES_FIELD,
            Single,
            parse_string_array!(&self.times)
        );

        Some(fields)
    }
}
