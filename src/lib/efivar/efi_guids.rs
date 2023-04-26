use crate::types::EfiGuid;
use crate::types::EfiGuidListEntry;
use std::collections::HashMap;
use std::str::FromStr;

pub struct EfiWellKnownGuids<'a> {
    guids_map: Option<HashMap<&'a str, EfiGuidListEntry>>,
}

pub static EFI_WELL_KNOWN_GUIDS: EfiWellKnownGuids = EfiWellKnownGuids { guids_map: None };

impl EfiWellKnownGuids<'_> {
    pub fn sort_by_guid(&self) -> Vec<&EfiGuidListEntry> {
        let mut sorted_entries = self.guids_map.values().collect::<Vec<_>>();
        sorted_entries.sort_unstable_by(|e1, e2| e1.guid.cmp(&e2.guid));
        return sorted_entries;
    }

    pub fn sort_by_name(&self) -> Vec<&EfiGuidListEntry> {
        let mut sorted_entries = self.guids_map.values().collect::<Vec<_>>();
        sorted_entries.sort_unstable_by(|e1, e2| e1.name.cmp(&e2.name));
        return sorted_entries;
    }
}
