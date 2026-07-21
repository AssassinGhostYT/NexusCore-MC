// Prioridades de evento — copiado directo de PMMP
// Menor número = ejecuta primero (LOWEST=5 corre primero, MONITOR=0 corre último)

pub const ALL_PRIORITIES: [EventPriority; 6] = [
    EventPriority::Lowest,
    EventPriority::Low,
    EventPriority::Normal,
    EventPriority::High,
    EventPriority::Highest,
    EventPriority::Monitor,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum EventPriority {
    Monitor = 0,
    Highest = 1,
    High = 2,
    Normal = 3,
    Low = 4,
    Lowest = 5,
}

impl EventPriority {
    pub fn from_str(name: &str) -> Option<Self> {
        match name.to_uppercase().as_str() {
            "MONITOR" => Some(Self::Monitor),
            "HIGHEST" => Some(Self::Highest),
            "HIGH" => Some(Self::High),
            "NORMAL" => Some(Self::Normal),
            "LOW" => Some(Self::Low),
            "LOWEST" => Some(Self::Lowest),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Monitor => "MONITOR",
            Self::Highest => "HIGHEST",
            Self::High => "HIGH",
            Self::Normal => "NORMAL",
            Self::Low => "LOW",
            Self::Lowest => "LOWEST",
        }
    }

    /// Valor numérico (menor = ejecuta primero)
    pub fn value(&self) -> u8 {
        *self as u8
    }
}

impl Default for EventPriority {
    fn default() -> Self {
        Self::Normal
    }
}
