use crate::class::traits::IntoFileInfix;
use std::{num::ParseIntError, str::FromStr};
use strum::{Display, EnumIter, FromRepr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VariantParseError {
    #[error("failed to parse str to int")]
    ParseIntError(#[from] ParseIntError),
    #[error("invalid variant")]
    InvalidVariant,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, EnumIter, Display)]
pub enum WeaponType {
    Revolver,
    Shotgun,
    Nailgun,
    Railgun,
    #[strum(to_string = "Rocket Launcher")]
    RocketLauncher,
    Arm,
}

impl WeaponType {
    pub fn get_unlockable_variants(&self) -> &[UnlockableWeaponVariant] {
        match self {
            Self::Revolver => &[
                UnlockableWeaponVariant::PiercerRevolver,
                UnlockableWeaponVariant::MarksmanRevolver,
                UnlockableWeaponVariant::SharpshooterRevolver,
                UnlockableWeaponVariant::AlternateRevolver,
            ],
            Self::Shotgun => &[
                UnlockableWeaponVariant::CoreEjectShotgun,
                UnlockableWeaponVariant::PumpChargeShotgun,
            ],
            Self::Nailgun => &[
                UnlockableWeaponVariant::AttractorNailgun,
                UnlockableWeaponVariant::OverheatNailgun,
                UnlockableWeaponVariant::SawbladeLauncher,
            ],
            Self::Railgun => &[
                UnlockableWeaponVariant::ElectricRailgun,
                UnlockableWeaponVariant::MaliciousRailgun,
                UnlockableWeaponVariant::ScrewdriverRailgun,
            ],
            Self::RocketLauncher => &[
                UnlockableWeaponVariant::FreezeframeRocketLauncher,
                UnlockableWeaponVariant::SRSCannonRocketLauncher,
            ],
            Self::Arm => &[
                UnlockableWeaponVariant::Knuckleblaster,
                UnlockableWeaponVariant::Whiplash,
            ],
        }
    }

    pub fn get_customizable(&self) -> Option<CustomizableWeaponType> {
        Some(match self {
            Self::Revolver => CustomizableWeaponType::Revolver,
            Self::Shotgun => CustomizableWeaponType::Shotgun,
            Self::Nailgun => CustomizableWeaponType::Nailgun,
            Self::Railgun => CustomizableWeaponType::Railgun,
            Self::RocketLauncher => CustomizableWeaponType::RocketLauncher,
            Self::Arm => return None,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, EnumIter, Display)]
pub enum CustomizableWeaponType {
    Revolver,
    Shotgun,
    Nailgun,
    Railgun,
    #[strum(to_string = "Rocket Launcher")]
    RocketLauncher,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, EnumIter, Display)]
pub enum UnlockableWeaponVariant {
    #[strum(to_string = "Piercer Revolver")]
    PiercerRevolver,
    #[strum(to_string = "Marksman Revolver")]
    MarksmanRevolver,
    #[strum(to_string = "Sharpshooter Revolver")]
    SharpshooterRevolver,
    #[strum(to_string = "Alternate Revolver")]
    AlternateRevolver,
    #[strum(to_string = "Core Eject Shotgun")]
    CoreEjectShotgun,
    #[strum(to_string = "Pump Charge Shotgun")]
    PumpChargeShotgun,
    #[strum(to_string = "Attractor Nailgun")]
    AttractorNailgun,
    #[strum(to_string = "Overheat Nailgun")]
    OverheatNailgun,
    #[strum(to_string = "Sawblade Launcher")]
    SawbladeLauncher,
    #[strum(to_string = "Electric Railgun")]
    ElectricRailgun,
    #[strum(to_string = "Screwdriver Railgun")]
    ScrewdriverRailgun,
    #[strum(to_string = "Malicious Railgun")]
    MaliciousRailgun,
    #[strum(to_string = "Freezeframe Rocket Launcher")]
    FreezeframeRocketLauncher,
    #[strum(to_string = "S.R.S. Cannon Rocket Launcher")]
    SRSCannonRocketLauncher,
    Knuckleblaster,
    Whiplash,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, FromRepr, EnumIter, Display)]
#[repr(u8)]
pub enum UnlockableType {
    Florp,
    KITR,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, FromRepr, EnumIter, Display)]
#[repr(u8)]
pub enum EnemyType {
    Cerberus = 0,
    Drone = 1,
    #[strum(to_string = "Hideous Mass")]
    HideousMass = 2,
    Filth = 3,
    #[strum(to_string = "Malicious Face")]
    MaliciousFace = 4,
    Mindflayer = 5,
    Streetcleaner = 6,
    Swordsmachine = 7,
    V2 = 8,
    Virtue = 9,
    Wicked = 10,
    Minos = 11,
    Stalker = 12,
    Stray = 13,
    Schism = 14,
    Soldier = 15,
    #[strum(to_string = "Gabriel, Judge of Hell")]
    Gabriel = 16,
    #[strum(to_string = "Flesh Prison")]
    FleshPrison = 17,
    #[strum(to_string = "Minos Prime")]
    MinosPrime = 18,
    #[strum(to_string = "Sisyphean Insurrectionist")]
    SisypheanInsurrectionist = 19,
    Sentry = 20,
    Idol = 21,
    #[strum(to_string = "V2 (2nd)")]
    V2Second = 22,
    #[strum(to_string = "Cancerous Rodent")]
    CancerousRodent = 23,
    #[strum(to_string = "Very Cancerous Rodent")]
    VeryCancerousRodent = 24,
    #[strum(to_string = "Mysterious Druid Knight (& Owl)")]
    Mandalore = 25,
    Ferryman = 26,
    Leviathan = 27,
    #[strum(to_string = "Gabriel, Apostate of Hate")]
    GabrielSecond = 28,
    #[strum(to_string = "Sisyphus Prime")]
    SisyphusPrime = 29,
    #[strum(to_string = "Flesh Panopticon")]
    FleshPanopticon = 30,
    Mannequin = 31,
    Minotaur = 32,
    Gutterman = 33,
    Guttertank = 34,
    #[strum(to_string = "1000-THR \"Earthmover\"")]
    Centaur = 35,
    #[strum(to_string = "Big Johninator")]
    BigJohnator = 37,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, FromRepr, Display)]
#[repr(u8)]
pub enum Lockable {
    #[default]
    Locked = 0,
    Unlocked = 1,
    Completed = 2,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, FromRepr, Display)]
#[repr(i8)]
pub enum LevelRank {
    #[default]
    None = -1,
    D = 0,
    C = 1,
    B = 2,
    A = 3,
    S = 4,
    P = 12,
}

impl From<i32> for LevelRank {
    fn from(value: i32) -> Self {
        if value < 0 {
            Self::None
        } else {
            match value {
                1 => Self::C,
                2 => Self::B,
                3 => Self::A,
                4..=11 => Self::S,
                12 => Self::P,
                _ => Self::D,
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, FromRepr)]
#[repr(u8)]
pub enum SaveSlot {
    #[default]
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, EnumIter, FromRepr)]
#[repr(u8)]
pub enum Difficulty {
    Harmless = 0,
    Lenient = 1,
    #[default]
    Standard = 2,
    Violent = 3,
}

impl FromStr for Difficulty {
    type Err = VariantParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::from_repr(s.parse()?) {
            Some(value) => Ok(value),
            None => Err(VariantParseError::InvalidVariant),
        }
    }
}

impl IntoFileInfix for Difficulty {
    fn into_file_infix(&self) -> Box<dyn std::fmt::Display> {
        Box::new(*self as u8)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, EnumIter, Display)]
pub enum SecretLevel {
    #[strum(to_string = "0-S: SOMETHING WICKED")]
    SomethingWicked,
    #[strum(to_string = "1-S: THE WITLESS")]
    TheWitless,
    #[strum(to_string = "2-S: ALL-IMPERFECT LOVE SONG")]
    AllImperfectLoveSong,
    #[strum(to_string = "P-1: SOUL SURVIVOR")]
    SoulSurvivor,
    #[strum(to_string = "4-S: CLASH OF THE BRANDICOOT")]
    ClashOfTheBrandicoot,
    #[strum(to_string = "5-S: I ONLY SAY MORNING")]
    IOnlySayMorning,
    #[strum(to_string = "P-2: WAIT OF THE WORLD")]
    WaitOfTheWorld,
    UnknownSecret7,
    UnknownSecret8,
    UnknownPrime3,
}

impl SecretLevel {
    pub fn is_prime(&self) -> bool {
        match self {
            Self::SoulSurvivor | Self::WaitOfTheWorld | Self::UnknownPrime3 => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, FromRepr, EnumIter, Display)]
#[repr(u8)]
pub enum Act {
    #[strum(to_string = "PRELUDE")]
    Prelude,
    #[strum(to_string = "ACT I: INFINITE HYPERDEATH")]
    Act1,
    #[strum(to_string = "ACT II: IMPERFECT HATRED")]
    Act2,
    #[strum(to_string = "ACT III: GODFIST SUICIDE")]
    Act3,
}

impl Act {
    pub fn get_layers(&self) -> &[Layer] {
        match self {
            Self::Prelude => &[Layer::Overture],
            Self::Act1 => &[Layer::Limbo, Layer::Lust, Layer::Gluttony],
            Self::Act2 => &[Layer::Greed, Layer::Wrath, Layer::Heresy],
            Self::Act3 => &[Layer::Violence, Layer::Fraud, Layer::Treachery],
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, FromRepr, EnumIter, Display)]
#[repr(u8)]
pub enum Layer {
    #[strum(to_string = "OVERTURE: THE MOUTH OF HELL")]
    Overture = 0,
    #[strum(to_string = "LAYER 1: LIMBO")]
    Limbo = 1,
    #[strum(to_string = "LAYER 2: LUST")]
    Lust = 2,
    #[strum(to_string = "LAYER 3: GLUTTONY")]
    Gluttony = 3,
    #[strum(to_string = "LAYER 4: GREED")]
    Greed = 4,
    #[strum(to_string = "LAYER 5: WRATH")]
    Wrath = 5,
    #[strum(to_string = "LAYER 6: HERESY")]
    Heresy = 6,
    #[strum(to_string = "LAYER 7: VIOLENCE")]
    Violence = 7,
    #[strum(to_string = "LAYER 8: FRAUD")]
    Fraud = 8,
    #[strum(to_string = "LAYER 9: TREACHERY")]
    Treachery = 9,
}

impl Layer {
    pub fn get_levels(&self) -> &[Level] {
        match self {
            Self::Overture => &[
                Level::IntoTheFire,
                Level::TheMeatgrinder,
                Level::DoubleDown,
                Level::AOneMachineArmy,
                Level::Cerberus,
            ],
            Self::Limbo => &[
                Level::HeartOfTheSunrise,
                Level::TheBurningWorld,
                Level::HallsOfSacredRemains,
                Level::ClairDeLune,
            ],
            Self::Lust => &[
                Level::Bridgeburner,
                Level::DeathAt20000Volts,
                Level::SheerHeartAttack,
                Level::CourtOfTheCorpseKing,
            ],
            Self::Gluttony => &[
                Level::BellyOfTheBeast,
                Level::InTheFlesh,
                Level::SoulSurvivor,
            ],
            Self::Greed => &[
                Level::SlavesToPower,
                Level::GodDamnTheSun,
                Level::AShotInTheDark,
                Level::ClairDeSoleil,
            ],
            Self::Wrath => &[
                Level::InTheWakeOfPoseidon,
                Level::WavesOfTheStarlessSea,
                Level::ShipOfFools,
                Level::Leviathan,
            ],
            Self::Heresy => &[
                Level::CryForTheWeeper,
                Level::AestheticsOfHate,
                Level::WaitOfTheWorld,
            ],
            Self::Violence => &[
                Level::GardenOfForkingPaths,
                Level::LightUpTheNight,
                Level::NoSoundNoMemory,
                Level::LikeAntennasToHeaven,
            ],
            _ => &[],
        }
    }

    pub fn get_secret_level(&self) -> SecretLevel {
        match self {
            Self::Overture => SecretLevel::SomethingWicked,
            Self::Limbo => SecretLevel::TheWitless,
            Self::Lust => SecretLevel::AllImperfectLoveSong,
            Self::Gluttony => SecretLevel::SoulSurvivor,
            Self::Greed => SecretLevel::ClashOfTheBrandicoot,
            Self::Wrath => SecretLevel::IOnlySayMorning,
            Self::Heresy => SecretLevel::WaitOfTheWorld,
            Self::Violence => SecretLevel::UnknownSecret7,
            Self::Fraud => SecretLevel::UnknownSecret8,
            Self::Treachery => SecretLevel::UnknownPrime3,
        }
    }
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, FromRepr, EnumIter, Display, Default,
)]
#[repr(u16)]
pub enum Level {
    #[default]
    #[strum(to_string = "0-1: INTO THE FIRE")]
    IntoTheFire = 1,
    #[strum(to_string = "0-2: THE MEATGRINDER")]
    TheMeatgrinder = 2,
    #[strum(to_string = "0-3: DOUBLE DOWN")]
    DoubleDown = 3,
    #[strum(to_string = "0-4: A ONE-MACHINE ARMY")]
    AOneMachineArmy = 4,
    #[strum(to_string = "0-5: CERBERUS")]
    Cerberus = 5,
    #[strum(to_string = "1-1: HEART OF THE SUNRISE")]
    HeartOfTheSunrise = 6,
    #[strum(to_string = "1-2: THE BURNING WORLD")]
    TheBurningWorld = 7,
    #[strum(to_string = "1-3: HALLS OF SACRED REMAINS")]
    HallsOfSacredRemains = 8,
    #[strum(to_string = "1-4: CLAIR DE LUNE")]
    ClairDeLune = 9,
    #[strum(to_string = "2-1: BRIDGEBURNER")]
    Bridgeburner = 10,
    #[strum(to_string = "2-2: DEATH AT 20,000 VOLTS")]
    DeathAt20000Volts = 11,
    #[strum(to_string = "2-3: SHEER HEART ATTACK")]
    SheerHeartAttack = 12,
    #[strum(to_string = "2-4: COURT OF THE CORPSE KING")]
    CourtOfTheCorpseKing = 13,
    #[strum(to_string = "3-1: BELLY OF THE BEAST")]
    BellyOfTheBeast = 14,
    #[strum(to_string = "3-2: IN THE FLESH")]
    InTheFlesh = 15,
    #[strum(to_string = "4-1: SLAVES TO POWER")]
    SlavesToPower = 16,
    #[strum(to_string = "4-2: GOD DAMN THE SUN")]
    GodDamnTheSun = 17,
    #[strum(to_string = "4-3: A SHOT IN THE DARK")]
    AShotInTheDark = 18,
    #[strum(to_string = "4-4: CLAIR DE SOLEIL")]
    ClairDeSoleil = 19,
    #[strum(to_string = "5-1: IN THE WAKE OF POSEIDON")]
    InTheWakeOfPoseidon = 20,
    #[strum(to_string = "5-2: WAVES OF THE STARLESS SEA")]
    WavesOfTheStarlessSea = 21,
    #[strum(to_string = "5-3: SHIP OF FOOLS")]
    ShipOfFools = 22,
    #[strum(to_string = "5-4: LEVIATHAN")]
    Leviathan = 23,
    #[strum(to_string = "6-1: CRY FOR THE WEEPER")]
    CryForTheWeeper = 24,
    #[strum(to_string = "6-2: AESTHETICS OF HATE")]
    AestheticsOfHate = 25,
    #[strum(to_string = "7-1: GARDEN OF FORKING PATHS")]
    GardenOfForkingPaths = 26,
    #[strum(to_string = "7-2: LIGHT UP THE NIGHT")]
    LightUpTheNight = 27,
    #[strum(to_string = "7-3: NO SOUND, NO MEMORY")]
    NoSoundNoMemory = 28,
    #[strum(to_string = "7-4: ...LIKE ANTENNAS TO HEAVEN")]
    LikeAntennasToHeaven = 29,


    #[strum(to_string = "P-1: SOUL SURVIVOR")]
    SoulSurvivor = 666,
    #[strum(to_string = "P-2: WAIT OF THE WORLD")]
    WaitOfTheWorld = 667,
}

impl Level {
    pub fn is_prime(&self) -> bool {
        match self {
            Self::SoulSurvivor | Self::WaitOfTheWorld => true,
            _ => false,
        }
    }

    pub fn get_prime_index(&self) -> Option<u8> {
        match self {
            Self::SoulSurvivor => Some(0),
            Self::WaitOfTheWorld => Some(1),
            _ => None,
        }
    }

    pub fn get_secret_count(&self) -> u8 {
        match self {
            Self::IntoTheFire => 5,
            Self::TheMeatgrinder => 5,
            Self::DoubleDown => 3,
            Self::AOneMachineArmy => 3,
            Self::Cerberus => 0,
            Self::HeartOfTheSunrise => 5,
            Self::TheBurningWorld => 5,
            Self::HallsOfSacredRemains => 5,
            Self::ClairDeLune => 0,
            Self::Bridgeburner => 5,
            Self::DeathAt20000Volts => 5,
            Self::SheerHeartAttack => 5,
            Self::CourtOfTheCorpseKing => 0,
            Self::BellyOfTheBeast => 5,
            Self::InTheFlesh => 0,
            Self::SlavesToPower => 5,
            Self::GodDamnTheSun => 5,
            Self::AShotInTheDark => 5,
            Self::ClairDeSoleil => 0,
            Self::InTheWakeOfPoseidon => 5,
            Self::WavesOfTheStarlessSea => 5,
            Self::ShipOfFools => 5,
            Self::Leviathan => 0,
            Self::CryForTheWeeper => 5,
            Self::AestheticsOfHate => 0,
            Self::GardenOfForkingPaths => 5,
            Self::LightUpTheNight => 5,
            Self::NoSoundNoMemory => 5,
            Self::LikeAntennasToHeaven => 0,

            Self::SoulSurvivor => 0,
            Self::WaitOfTheWorld => 0,
        }
    }
}

impl FromStr for Level {
    type Err = VariantParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::from_repr(s.parse()?) {
            Some(value) => Ok(value),
            None => Err(VariantParseError::InvalidVariant),
        }
    }
}

impl IntoFileInfix for Level {
    fn into_file_infix(&self) -> Box<dyn std::fmt::Display> {
        Box::new(*self as u16)
    }
}
