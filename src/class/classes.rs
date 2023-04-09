use super::{
    cybergrind::CybergrindData, difficulty::DifficultyData, general::GeneralData, level::LevelData,
    traits::LoadableSavable,
};
use crate::enums::{Difficulty, Level};
use std::{collections::BTreeMap, io, path::Path};

type LevelMap = BTreeMap<Level, LevelData>;
type DifficultyMap = BTreeMap<Difficulty, DifficultyData>;

#[derive(Debug)]
pub struct Classes {
    pub levels: LevelMap,
    pub cybergrind: CybergrindData,
    pub difficulty: DifficultyMap,
    pub general: GeneralData,
}

impl Classes {
    pub fn load<P: AsRef<Path>>(save_path: P) -> Option<Self> {
        Some(Self {
            levels: LevelMap::load(&save_path),
            cybergrind: CybergrindData::load(&save_path),
            difficulty: DifficultyMap::load(&save_path),
            general: GeneralData::load(&save_path),
        })
    }

    pub fn save<P: AsRef<Path>>(&self, save_path: P) -> Result<(), io::Error> {
        self.levels.save(&save_path)?;
        self.cybergrind.save(&save_path)?;
        self.difficulty.save(&save_path)?;
        self.general.save(&save_path)
    }
}
