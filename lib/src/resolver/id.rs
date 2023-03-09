use crate::{get_frozen, get_netkans};

pub async fn resolve_mod_by_id(id: String) -> Option<String> {
    let live_mods = get_netkans().await;

    let live_mod = live_mods
        .iter()
        .cloned()
        .find(|v| v.identifier == Some(id.clone()));

    if let Some(mod_info) = live_mod {
        return mod_info.kref.clone();
    }

    let frozen_mods = get_frozen().await;
    
    let frozen_mod = frozen_mods
        .iter()
        .cloned()
        .find(|v| v.identifier == Some(id.clone()));

    if let Some(mod_info) = frozen_mod {
        return mod_info.kref.clone();
    }

    return None;
}
