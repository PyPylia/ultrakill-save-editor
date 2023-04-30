use super::{
    macros::{read_primitive, read_primitive_array, write_primitive, write_primitive_array},
    traits::{FieldMap, ParsableClass},
};
use crate::enums::{
    CustomizableWeaponType, EnemyType, Layer, Lockable, SecretLevel, UnlockableType,
    UnlockableWeaponVariant,
};
use indexmap::IndexMap;
use ms_nrbf::{Class, Field, Primitive, PrimitiveArray};
use std::collections::BTreeMap;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct GeneralData {
    pub money: String, //i32
    pub intro_seen: bool,
    pub tutorial_beat: bool,
    pub clash_mode_unlocked: bool,
    pub unlocked_weapons: BTreeMap<UnlockableWeaponVariant, bool>,
    pub secret_missions: BTreeMap<SecretLevel, Lockable>,
    pub limbo_switches: Vec<bool>,
    pub enemies_discovered: BTreeMap<EnemyType, Lockable>,
    pub unlockables_found: BTreeMap<UnlockableType, bool>,
    pub weapons_customizable: BTreeMap<CustomizableWeaponType, bool>,
    pub file_exists: bool,
}

impl Default for GeneralData {
    fn default() -> Self {
        Self {
            money: String::from("0"),
            intro_seen: false,
            tutorial_beat: false,
            clash_mode_unlocked: false,
            unlocked_weapons: BTreeMap::from_iter(
                UnlockableWeaponVariant::iter().map(|value| (value, false)),
            ),
            secret_missions: BTreeMap::from_iter(
                SecretLevel::iter().map(|value| (value, Lockable::Locked)),
            ),
            limbo_switches: vec![false; 4],
            enemies_discovered: BTreeMap::from_iter(
                EnemyType::iter().map(|value| (value, Lockable::Locked)),
            ),
            unlockables_found: BTreeMap::from_iter(
                UnlockableType::iter().map(|value| (value, false)),
            ),
            weapons_customizable: BTreeMap::from_iter(
                CustomizableWeaponType::iter().map(|value| (value, false)),
            ),
            file_exists: false,
        }
    }
}

const MONEY_FIELD: &str = "money";
const INTRO_SEEN_FIELD: &str = "introSeen";
const TUTORIAL_BEAT_FIELD: &str = "tutorialBeat";
const CLASH_MODE_UNLOCKED_FIELD: &str = "clashModeUnlocked";
const SECRET_MISSIONS_FIELD: &str = "secretMissions";
const LIMBO_SWITCHES_FIELD: &str = "limboSwitches";
const ENEMIES_DISCOVERED_FIELD: &str = "newEnemiesFound";
const UNLOCKABLES_FOUND_FIELD: &str = "unlockablesFound";

const REVOLVER_CUSTOMIZABLE_FIELD: &str = "revCustomizationUnlocked";
const SHOTGUN_CUSTOMIZABLE_FIELD: &str = "shoCustomizationUnlocked";
const NAILGUN_CUSTOMIZABLE_FIELD: &str = "naiCustomizationUnlocked";
const RAILGUN_CUSTOMIZABLE_FIELD: &str = "raiCustomizationUnlocked";
const ROCKET_LAUNCHER_CUSTOMIZABLE_FIELD: &str = "rockCustomizationUnlocked";

const BLUE_REVOLVER_UNLOCKED_FIELD: &str = "rev0";
const RED_REVOLVER_UNLOCKED_FIELD: &str = "rev1";
const GREEN_REVOLVER_UNLOCKED_FIELD: &str = "rev2";
const YELLOW_REVOLVER_UNLOCKED_FIELD: &str = "rev3";
const ALTERNATE_REVOLVER_UNLOCKED_FIELD: &str = "revalt";

const BLUE_SHOTGUN_UNLOCKED_FIELD: &str = "sho0";
const RED_SHOTGUN_UNLOCKED_FIELD: &str = "sho2";
const GREEN_SHOTGUN_UNLOCKED_FIELD: &str = "sho1";
const YELLOW_SHOTGUN_UNLOCKED_FIELD: &str = "sho3";

const BLUE_NAILGUN_UNLOCKED_FIELD: &str = "nai0";
const RED_NAILGUN_UNLOCKED_FIELD: &str = "nai2";
const GREEN_NAILGUN_UNLOCKED_FIELD: &str = "nai1";
const YELLOW_NAILGUN_UNLOCKED_FIELD: &str = "nai3";
const ALTERNATE_NAILGUN_UNLOCKED_FIELD: &str = "naialt";

const BLUE_RAILGUN_UNLOCKED_FIELD: &str = "rai0";
const RED_RAILGUN_UNLOCKED_FIELD: &str = "rai1";
const GREEN_RAILGUN_UNLOCKED_FIELD: &str = "rai2";
const YELLOW_RAILGUN_UNLOCKED_FIELD: &str = "rai3";

const BLUE_ROCKET_LAUNCHER_UNLOCKED_FIELD: &str = "rock0";
const RED_ROCKET_LAUNCHER_UNLOCKED_FIELD: &str = "rock2";
const GREEN_ROCKET_LAUNCHER_UNLOCKED_FIELD: &str = "rock1";
const YELLOW_ROCKET_LAUNCHER_UNLOCKED_FIELD: &str = "rock3";

const BLUE_BEAM_UNLOCKED_FIELD: &str = "beam0";
const RED_BEAM_UNLOCKED_FIELD: &str = "beam1";
const GREEN_BEAM_UNLOCKED_FIELD: &str = "beam2";
const YELLOW_BEAM_UNLOCKED_FIELD: &str = "beam3";

const RED_ARM_UNLOCKED_FIELD: &str = "arm1";
const GREEN_ARM_UNLOCKED_FIELD: &str = "arm2";
const YELLOW_ARM_UNLOCKED_FIELD: &str = "arm3";

const WEAPONTYPE_TO_FIELD_ARRAY: &[(CustomizableWeaponType, &str)] = &[
    (
        CustomizableWeaponType::Revolver,
        REVOLVER_CUSTOMIZABLE_FIELD,
    ),
    (
        CustomizableWeaponType::Shotgun,
        SHOTGUN_CUSTOMIZABLE_FIELD,
    ),
    (
        CustomizableWeaponType::Nailgun,
        NAILGUN_CUSTOMIZABLE_FIELD,
    ),
    (
        CustomizableWeaponType::Railgun,
        RAILGUN_CUSTOMIZABLE_FIELD,
    ),
    (
        CustomizableWeaponType::RocketLauncher,
        ROCKET_LAUNCHER_CUSTOMIZABLE_FIELD,
    ),
];

const UNLOCKABLEWEAPONVARIANT_TO_FIELD_ARRAY: &[(UnlockableWeaponVariant, &str)] = &[
    (
        UnlockableWeaponVariant::PiercerRevolver,
        BLUE_REVOLVER_UNLOCKED_FIELD,
    ),
    (
        UnlockableWeaponVariant::MarksmanRevolver,
        GREEN_REVOLVER_UNLOCKED_FIELD,
    ),
    (
        UnlockableWeaponVariant::SharpshooterRevolver,
        RED_REVOLVER_UNLOCKED_FIELD,
    ),
    (
        UnlockableWeaponVariant::AlternateRevolver,
        ALTERNATE_REVOLVER_UNLOCKED_FIELD,
    ),
    (
        UnlockableWeaponVariant::CoreEjectShotgun,
        BLUE_SHOTGUN_UNLOCKED_FIELD,
    ),
    (
        UnlockableWeaponVariant::PumpChargeShotgun,
        GREEN_SHOTGUN_UNLOCKED_FIELD,
    ),
    (
        UnlockableWeaponVariant::AttractorNailgun,
        BLUE_NAILGUN_UNLOCKED_FIELD,
    ),
    (
        UnlockableWeaponVariant::OverheatNailgun,
        GREEN_NAILGUN_UNLOCKED_FIELD,
    ),
    (
        UnlockableWeaponVariant::SawbladeLauncher,
        ALTERNATE_NAILGUN_UNLOCKED_FIELD,
    ),
    (
        UnlockableWeaponVariant::ElectricRailgun,
        BLUE_RAILGUN_UNLOCKED_FIELD,
    ),
    (
        UnlockableWeaponVariant::ScrewdriverRailgun,
        GREEN_RAILGUN_UNLOCKED_FIELD,
    ),
    (
        UnlockableWeaponVariant::MaliciousRailgun,
        RED_RAILGUN_UNLOCKED_FIELD,
    ),
    (
        UnlockableWeaponVariant::FreezeframeRocketLauncher,
        BLUE_ROCKET_LAUNCHER_UNLOCKED_FIELD,
    ),
    (
        UnlockableWeaponVariant::SRSCannonRocketLauncher,
        GREEN_ROCKET_LAUNCHER_UNLOCKED_FIELD,
    ),
    (
        UnlockableWeaponVariant::Knuckleblaster,
        RED_ARM_UNLOCKED_FIELD,
    ),
    (
        UnlockableWeaponVariant::Whiplash,
        GREEN_ARM_UNLOCKED_FIELD,
    ),
];

const ZEROED_FIELDS: &[&str] = &[
    YELLOW_REVOLVER_UNLOCKED_FIELD,
    RED_SHOTGUN_UNLOCKED_FIELD,
    YELLOW_SHOTGUN_UNLOCKED_FIELD,
    RED_NAILGUN_UNLOCKED_FIELD,
    YELLOW_NAILGUN_UNLOCKED_FIELD,
    YELLOW_RAILGUN_UNLOCKED_FIELD,
    RED_ROCKET_LAUNCHER_UNLOCKED_FIELD,
    YELLOW_ROCKET_LAUNCHER_UNLOCKED_FIELD,
    BLUE_BEAM_UNLOCKED_FIELD,
    RED_BEAM_UNLOCKED_FIELD,
    GREEN_BEAM_UNLOCKED_FIELD,
    YELLOW_BEAM_UNLOCKED_FIELD,
    YELLOW_ARM_UNLOCKED_FIELD,
];

fn read_boolean_map<K: Copy + Ord>(
    class: &Class,
    array: &[(K, &'static str)],
) -> Option<BTreeMap<K, bool>> {
    let mut mapped_array = vec![];

    for (key, value) in array {
        mapped_array.push((
            *key,
            *read_primitive!(class, *value, Boolean),
        ))
    }

    Some(BTreeMap::from_iter(mapped_array))
}

fn read_int32_bool_map<K: Copy + Ord>(
    class: &Class,
    array: &[(K, &'static str)],
) -> Option<BTreeMap<K, bool>> {
    let mut mapped_array = vec![];

    for (key, value) in array {
        mapped_array.push((
            *key,
            *read_primitive!(class, *value, Int32) != 0,
        ))
    }

    Some(BTreeMap::from_iter(mapped_array))
}

fn write_boolean_map<K: Copy + Ord>(
    fields: &mut FieldMap,
    map: &BTreeMap<K, bool>,
    array: &[(K, &'static str)],
) -> Option<()> {
    for (key, value) in array {
        write_primitive!(fields, *value, Boolean, *map.get(key)?);
    }

    Some(())
}

fn write_int32_bool_map<K: Copy + Ord>(
    fields: &mut FieldMap,
    map: &BTreeMap<K, bool>,
    array: &[(K, &'static str)],
) -> Option<()> {
    for (key, value) in array {
        write_primitive!(
            fields,
            *value,
            Int32,
            *map.get(key)? as i32
        )?;
    }

    Some(())
}

fn write_0_to_fields(fields: &mut FieldMap, array: &[&'static str]) -> Option<()> {
    for field in array {
        write_primitive!(fields, *field, Int32, 0)?;
    }

    Some(())
}

impl ParsableClass for GeneralData {
    const CLASS_NAME: &'static str = "GameProgressMoneyAndGear";
    const FILE_NAME: &'static str = "generalprogress.bepis";

    fn get_file_exists(&self) -> bool {
        self.file_exists
    }

    fn parse(class: &Class) -> Option<Self> {
        let mut secret_missions = BTreeMap::new();
        let mut enemies_discovered = BTreeMap::new();
        let mut unlockables_found = BTreeMap::new();

        for (i, value) in read_primitive_array!(class, SECRET_MISSIONS_FIELD, Int32)
            .iter()
            .enumerate()
        {
            secret_missions.insert(
                Layer::from_repr(i as u8)?.get_secret_level(),
                Lockable::from_repr(*value as u8)?,
            );
        }

        for (i, value) in read_primitive_array!(class, ENEMIES_DISCOVERED_FIELD, Int32)
            .iter()
            .enumerate()
        {
            enemies_discovered.insert(
                EnemyType::from_repr(i as u8)?,
                Lockable::from_repr(*value as u8)?,
            );
        }

        for (i, value) in read_primitive_array!(class, UNLOCKABLES_FOUND_FIELD, Boolean)
            .iter()
            .enumerate()
        {
            unlockables_found.insert(
                UnlockableType::from_repr(i as u8)?,
                *value,
            );
        }

        Some(Self {
            money: read_primitive!(class, MONEY_FIELD, Int32).to_string(),
            intro_seen: *read_primitive!(class, INTRO_SEEN_FIELD, Boolean),
            tutorial_beat: *read_primitive!(class, TUTORIAL_BEAT_FIELD, Boolean),
            clash_mode_unlocked: *read_primitive!(
                class,
                CLASH_MODE_UNLOCKED_FIELD,
                Boolean
            ),
            unlocked_weapons: read_int32_bool_map(
                class,
                UNLOCKABLEWEAPONVARIANT_TO_FIELD_ARRAY,
            )?,
            secret_missions,
            limbo_switches: read_primitive_array!(class, LIMBO_SWITCHES_FIELD, Boolean),
            enemies_discovered,
            unlockables_found,
            weapons_customizable: read_boolean_map(class, WEAPONTYPE_TO_FIELD_ARRAY)?,
            file_exists: true,
        })
    }

    fn unparse(&self) -> Option<FieldMap> {
        let mut fields = IndexMap::new();

        write_primitive!(
            fields,
            MONEY_FIELD,
            Int32,
            self.money.parse().ok()?
        );
        write_primitive!(
            fields,
            INTRO_SEEN_FIELD,
            Boolean,
            self.intro_seen
        );
        write_primitive!(
            fields,
            TUTORIAL_BEAT_FIELD,
            Boolean,
            self.tutorial_beat
        );
        write_primitive!(
            fields,
            CLASH_MODE_UNLOCKED_FIELD,
            Boolean,
            self.clash_mode_unlocked
        );
        write_primitive_array!(
            fields,
            SECRET_MISSIONS_FIELD,
            Int32,
            self.secret_missions
                .values()
                .map(|value| *value as i32)
                .collect::<Vec<i32>>()
        );
        write_primitive_array!(
            fields,
            LIMBO_SWITCHES_FIELD,
            Boolean,
            self.limbo_switches
        );
        write_primitive_array!(
            fields,
            ENEMIES_DISCOVERED_FIELD,
            Int32,
            self.enemies_discovered
                .values()
                .map(|value| *value as i32)
                .collect::<Vec<i32>>()
        );
        write_primitive_array!(
            fields,
            UNLOCKABLES_FOUND_FIELD,
            Boolean,
            self.unlockables_found
                .values()
                .map(|value| *value)
                .collect::<Vec<bool>>()
        );
        write_boolean_map(
            &mut fields,
            &self.weapons_customizable,
            WEAPONTYPE_TO_FIELD_ARRAY,
        );
        write_int32_bool_map(
            &mut fields,
            &self.unlocked_weapons,
            UNLOCKABLEWEAPONVARIANT_TO_FIELD_ARRAY,
        );
        write_0_to_fields(&mut fields, ZEROED_FIELDS)?;

        Some(fields)
    }
}
