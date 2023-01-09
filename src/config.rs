use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;

use evdev::{InputEventKind, Key};
use serde::{de, Deserialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub device: PathBuf,
    pub vdevices: HashMap<DeviceName, DeviceDefinition>,
    #[serde(default)]
    pub default_mode: ModeName,
    pub modes: HashMap<ModeName, Mode>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct DeviceName(pub String);

impl<'de> de::Deserialize<'de> for DeviceName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Self)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ModeName(pub String);

impl<'de> de::Deserialize<'de> for ModeName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Self)
    }
}

impl Default for ModeName {
    fn default() -> Self {
        Self(String::from("default"))
    }
}

#[derive(Default, Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct DeviceDefinition {
    pub keys: BTreeSet<Key>,
}

#[derive(Default, Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct Mode {
    pub direct: HashMap<DeviceName, Mapping>,
    pub custom: HashMap<InputEventKind, CustomAction>,
}

pub type Mapping = HashMap<InputEventKind, InputEventKind>;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum CustomAction {
    ChangeMode(ModeName),
}
