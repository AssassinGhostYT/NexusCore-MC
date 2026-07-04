use std::collections::HashMap;
use std::sync::OnceLock;
use crate::block::nbt::{parse_block_states_nbt, NbtTag, BlockState};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateProperties(pub Vec<(String, NbtTag)>);

impl StateProperties {
    pub fn from_slice(slice: &[(String, NbtTag)]) -> Self {
        let mut props = slice.to_vec();
        props.sort_by(|a, b| a.0.cmp(&b.0));
        Self(props)
    }

    pub fn from_map(map: &HashMap<String, NbtTag>) -> Self {
        let mut props: Vec<(String, NbtTag)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        props.sort_by(|a, b| a.0.cmp(&b.0));
        Self(props)
    }
}

pub struct Registry {
    states: Vec<BlockState>,
    lookup: HashMap<(String, StateProperties), u32>,
}

static REGISTRY: OnceLock<Registry> = OnceLock::new();

impl Registry {
    fn get() -> &'static Self {
        REGISTRY.get_or_init(|| {
            let data = include_bytes!("../../block_states.nbt");
            let states = parse_block_states_nbt(data);
            let mut lookup = HashMap::new();
            for (idx, state) in states.iter().enumerate() {
                let props = StateProperties::from_slice(&state.properties);
                lookup.insert((state.name.clone(), props), idx as u32);
            }
            Self { states, lookup }
        })
    }
}

/// Obtiene el runtime ID correspondiente a un nombre de bloque y sus propiedades (states).
pub fn get_runtime_id(name: &str, properties: &HashMap<String, NbtTag>) -> Option<u32> {
    let registry = Registry::get();
    let props = StateProperties::from_map(properties);
    // Asegurarse de que el nombre tenga el prefijo "minecraft:"
    let full_name = if name.contains(':') {
        name.to_string()
    } else {
        format!("minecraft:{}", name)
    };
    registry.lookup.get(&(full_name, props)).copied()
}

/// Obtiene la lista completa de BlockStates cargada desde block_states.nbt.
pub fn get_all_states() -> &'static [BlockState] {
    &Registry::get().states
}
