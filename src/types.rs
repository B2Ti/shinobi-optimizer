// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Freeplay;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Freeplay = serde_json::from_str(&json).unwrap();
// }

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Freeplay {
    #[serde(rename = "$type")]
    pub internal_type: String,
    pub version: i64,
    pub round: i64,
    pub synced_object_id: i64,
    pub time_step: i64,
    pub round_time_step: i64,
    pub match_won: bool,
    pub match_just_won: bool,
    pub game_version: String,
    pub health: f64,
    pub shield: f64,
    pub has_taken_leak_damage_last_round: bool,
    pub claimed_freeplay_loot: bool,
    pub saved_maps_id: String,
    pub map_name: String,
    pub map_difficulty: String,
    pub mode_name: String,
    pub game_type: i64,
    pub freeplay_round_seed: i64,
    pub next_free_area_id: i64,
    pub loans: Vec<Option<serde_json::Value>>,
    pub placed_towers: Vec<PlacedTower>,
    pub power_place_history: Vec<PowerPlaceHistory>,
    pub props_requiring_save_data: Vec<Option<serde_json::Value>>,
    pub round_removeable_save_data: Vec<Option<serde_json::Value>>,
    pub placed_projectiles: Vec<PlacedProjectile>,
    pub bloons: Vec<Option<serde_json::Value>>,
    pub active_removeables_by_name: Vec<Option<serde_json::Value>>,
    pub gizmo_save_data: Vec<Option<serde_json::Value>>,
    pub map_event_save_data: EmergencyUnlockActivatedAt,
    pub tower_discounts: Vec<Option<serde_json::Value>>,
    pub players: Players,
    pub tower_type_max_placed_history: EmergencyUnlockActivatedAt,
    pub tower_history: Vec<TowerHistory>,
    pub tower_max_base_id_at_a_time: TowerMaxBaseIdAtATime,
    pub active_mutators: ActiveMutators,
    pub emergency_unlock_activated_at: EmergencyUnlockActivatedAt,
    pub golden_bloons_popped: i64,
    pub game_id: i64,
    pub attempt_id: i64,
    pub continues_used: i64,
    pub meta_data: FreeplayMetaData,
    pub cash_spent: f64,
    pub tile_id: i64,
    pub ts: Option<serde_json::Value>,
    pub active_regen_removables_by_name: Vec<Option<serde_json::Value>>,
    pub players_who_placed_a_hero: Vec<i64>,
    pub misc: String,
    pub active_knowledge: String,
    pub active_relic_knowledge: Option<serde_json::Value>,
    pub replay_recorder: Option<serde_json::Value>,
    pub bloon_lives_lost_override: i64,
    pub rogue_map_save_data: Option<serde_json::Value>,
    pub player_challenge_id: Option<serde_json::Value>,
    pub player_challenge_type: Option<serde_json::Value>,
    pub is_creation_mode: bool,
    #[serde(rename = "dailyChallengeEventID")]
    pub daily_challenge_event_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActiveMutators {
    #[serde(rename = "$type")]
    pub internal_type: String,
    #[serde(flatten)]
    pub mutators: HashMap<String, Vec<Mutator>>,
    // pub overclock: Vec<Overclock>,
    // pub ultraboost: Vec<Overclock>,
    // pub take_aim: Vec<Overclock>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mutator {
    #[serde(rename = "$type")]
    pub internal_type: String,
    pub mutator_save_id: String,
    pub tower_id: i64,
    pub frames_left: i64,
    pub counter: i64,
    pub rounds_left: i64,
}

// #[derive(Serialize, Deserialize)]
// pub enum MutatorSaveId {
//     Overclock4,
//     Overclock5,
//     #[serde(rename = "OverclockParagon")]
//     OverclockParagon,
//     #[serde(rename = "TakeAim")]
//     TakeAim,
//     Ultraboost,
// }

#[derive(Serialize, Deserialize)]
pub struct EmergencyUnlockActivatedAt {
    #[serde(rename = "$type")]
    pub internal_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FreeplayMetaData {
    #[serde(rename = "$type")]
    pub internal_type: String,
    pub shared_ability_cooldowns: String,
    #[serde(rename = "CorvusMetaData_Attunement_isActive-1")]
    pub corvus_meta_data_attunement_is_active_1: String,
    #[serde(rename = "CorvusMetaData_Attunement_maxSpeed-1")]
    pub corvus_meta_data_attunement_max_speed_1: String,
    #[serde(rename = "CorvusMetaData_Attunement_maxDamage-1")]
    pub corvus_meta_data_attunement_max_damage_1: String,
    #[serde(rename = "CorvusMetaData_Mana_availableMana-1")]
    pub corvus_meta_data_mana_available_mana_1: String,
    #[serde(rename = "CorvusMetaData_Recovery_isRecovering-1")]
    pub corvus_meta_data_recovery_is_recovering_1: String,
    #[serde(rename = "CorvusMetaData_Recovery_recoveryFramesActive-1")]
    pub corvus_meta_data_recovery_recovery_frames_active_1: String,
    #[serde(rename = "CorvusMetaData_SoulBarrier_soulBarrierLivesAbsorbed-1")]
    pub corvus_meta_data_soul_barrier_soul_barrier_lives_absorbed_1: String,
    #[serde(rename = "CorvusMetaData_SoulBarrier_soulBarrierMaxLivesAbsorbed-1")]
    pub corvus_meta_data_soul_barrier_soul_barrier_max_lives_absorbed_1: String,
    #[serde(rename = "CorvusMetaData_Spells_activeContinuousSpells-1")]
    pub corvus_meta_data_spells_active_continuous_spells_1: String,
    #[serde(rename = "CorvusMetaData_Spells_activeInstantSpells-1")]
    pub corvus_meta_data_spells_active_instant_spells_1: String,
    #[serde(rename = "CorvusMetaData_Spells_instantSpellsOnCooldown-1")]
    pub corvus_meta_data_spells_instant_spells_on_cooldown_1: String,
    #[serde(rename = "CorvusMetaData_TimedEvents_spiritAbsorptionState-1")]
    pub corvus_meta_data_timed_events_spirit_absorption_state_1: String,
    #[serde(rename = "CorvusMetaData_TimedEvents_frameToAbsorbOn-1")]
    pub corvus_meta_data_timed_events_frame_to_absorb_on_1: String,
    #[serde(rename = "PowersInShop-RoadSpikes")]
    pub powers_in_shop_road_spikes: String,
    #[serde(rename = "PowersInShop-MoabMine")]
    pub powers_in_shop_moab_mine: String,
    #[serde(rename = "PowersInShop-GlueTrap")]
    pub powers_in_shop_glue_trap: String,
    #[serde(rename = "PowersInShop-CamoTrap")]
    pub powers_in_shop_camo_trap: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlacedProjectile {
    #[serde(rename = "$type")]
    pub internal_type: String,
    pub pierce: f64,
    pub base_id: String,
    pub position: EmittedFrom,
    pub rotation: f64,
    pub meta_data: PlacedProjectileMetaData,
    pub owner: i64,
    pub emitted_by_tower_id: i64,
    pub emitted_from: EmittedFrom,
    pub is_power_projectile: bool,
    pub power_id: Option<serde_json::Value>,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmittedFrom {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlacedProjectileMetaData {
    #[serde(rename = "$type")]
    pub internal_type: String,
    pub created_at: String,
    pub rounds_left: String,
    pub save_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlacedTower {
    #[serde(rename = "$type")]
    pub internal_type: String,
    pub unique_id: String,
    pub damage_dealt: i64,
    pub cash_earned: i64,
    pub worth: f64,
    #[serde(rename = "IdLastSave")]
    pub id_last_save: i64,
    pub created_at: i64,
    pub target_type: TargetType,
    pub position: EmittedFrom,
    pub rotation: f64,
    pub base_id: String,
    pub hero_id: String,
    pub hero_xp_in_current_level: i64,
    pub path_one_tier: i64,
    pub path_two_tier: i64,
    pub path_three_tier: i64,
    pub meta_data: PlacedTowerMetaData,
    pub owner: i64,
    pub original_owner: i64,
    pub created_at_round_time: i64,
    pub parent_tower_id: i64,
    pub area_placed_on: i64,
    pub is_paragon: bool,
    pub geraldo_tower_id: Option<serde_json::Value>,
    pub is_suspended: bool,
    pub is_selling_blocked: bool,
    pub is_selection_blocked: bool,
    pub tower_name: String,
    pub is_not_upgradeable: bool,
    pub lock_target_type_switching: bool,
    pub path1_num_blocked_tiers: i64,
    pub path2_num_blocked_tiers: i64,
    pub path3_num_blocked_tiers: i64,
    pub power_id: Option<serde_json::Value>,
    pub custom_map_tower_index_id: i64,
    pub is_suspended_but_not_scaled: bool,
    pub frontier_id: i64,
    pub created_on_round: i64,
    pub lives_gained: f64,
    pub pops: i64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PlacedTowerMetaData {
    #[serde(rename = "$type")]
    pub internal_type: String,
    pub paragon_investment_from_pops: Option<String>,
    pub paragon_investment_from_money_spent: Option<String>,
    pub paragon_investment_from_non_tier5: Option<String>,
    pub paragon_investment_from_tier5: Option<String>,
    pub paragon_investment_from_bonus: Option<String>,
    pub paragon_investment_total: Option<String>,
    pub paragon_active_at: Option<String>,
    pub multi_hook_block_emission_until0: Option<String>,
    pub multi_hook_block_emission_until1: Option<String>,
    pub multi_hook_block_emission_until2: Option<String>,
    pub multi_hook_block_emission_until3: Option<String>,
    pub multi_hook_block_emission_until4: Option<String>,
    pub multi_hook_block_emission_until5: Option<String>,
    pub multi_hook_block_emission_until6: Option<String>,
    pub multi_hook_block_emission_until7: Option<String>,
    pub multi_hook_block_emission_until8: Option<String>,
    pub multi_hook_block_emission_until9: Option<String>,
    #[serde(rename = "AddMakeShiftAreaIDAddMakeshiftAreaModel_TowerPlatformfirst")]
    pub add_make_shift_area_id_add_makeshift_area_model_tower_platformfirst: Option<String>,
    #[serde(rename = "AddMakeShiftAreaIDAddMakeshiftAreaModel_TowerPlatformsecond")]
    pub add_make_shift_area_id_add_makeshift_area_model_tower_platformsecond: Option<String>,
    #[serde(rename = "Large BAD Harpoon")]
    pub large_bad_harpoon: Option<String>,
    #[serde(rename = "Temple_Magic")]
    pub temple_magic: Option<String>,
    #[serde(rename = "Temple_Military")]
    pub temple_military: Option<String>,
    #[serde(rename = "Temple_Primary")]
    pub temple_primary: Option<String>,
    #[serde(rename = "TrueTemple_Magic")]
    pub true_temple_magic: Option<String>,
    #[serde(rename = "TrueTemple_Military")]
    pub true_temple_military: Option<String>,
    #[serde(rename = "TrueTemple_Primary")]
    pub true_temple_primary: Option<String>,
    #[serde(rename = "TrueTemple_Support")]
    pub true_temple_support: Option<String>,
    pub dark_temple: Option<String>,
    pub set_camo_on: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargetType {
    #[serde(rename = "$type")]
    pub internal_type: String,
    pub id: String,
    pub is_actionable: bool,
    pub action_on_create: bool,
    #[serde(rename = "intID")]
    pub int_id: i64,
    pub continue_to_next_priority: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Players {
    #[serde(rename = "$type")]
    pub internal_type: String,
    #[serde(flatten)]
    pub players: HashMap<String, Player>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    #[serde(rename = "$type")]
    pub internal_type: String,
    pub index: i64,
    pub cash: f64,
    pub hero: String,
    pub free_towers: Vec<FreeTower>,
    pub tower_discounts: Vec<TowerDiscount>,
    pub geraldo_shop_items: Vec<GeraldoShopItem>,
    pub lvl20_geraldo_restock_applied: bool,
    pub has_placed_hero: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreeTower {
    #[serde(rename = "$type")]
    pub internal_type: String,
    #[serde(rename = "baseTowerID")]
    pub base_tower_id: String,
    pub charges: i64,
    pub mutually_exclusive_with: String,
    pub priority: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeraldoShopItem {
    #[serde(rename = "$type")]
    pub internal_type: String,
    pub name: String,
    pub used: i64,
    pub remaining: i64,
    pub replenishing_from_round: i64,
    pub round_unlocked_at: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TowerDiscount {
    #[serde(rename = "$type")]
    pub internal_type: String,
    pub discount_name: String,
    pub charges: i64,
    pub towers: Vec<String>,
    pub multiplier: f64,
    pub subtraction: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerPlaceHistory {
    #[serde(rename = "$type")]
    pub internal_type: String,
    pub power_id: String,
    pub owner: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TowerHistory {
    #[serde(rename = "$type")]
    pub internal_type: String,
    pub unique_id: String,
    pub base_id: String,
    pub tower_set: i64,
    pub tiers: Vec<i64>,
    pub cost: f64,
    pub destroyed: bool,
    pub tiers_modifier: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TowerMaxBaseIdAtATime {
    #[serde(rename = "$type")]
    pub internal_type: String,
    pub alchemist: i64,
    pub banana_farm: i64,
    pub beast_handler: i64,
    pub bomb_shooter: i64,
    pub boomerang_monkey: i64,
    pub dart_monkey: i64,
    pub dartling_gunner: i64,
    pub desperado: i64,
    pub druid: i64,
    pub engineer_monkey: i64,
    pub ezili: i64,
    pub glue_gunner: i64,
    pub heli_pilot: i64,
    pub ice_monkey: i64,
    pub mermonkey: i64,
    pub monkey_ace: i64,
    pub monkey_buccaneer: i64,
    pub monkey_sub: i64,
    pub monkey_village: i64,
    pub ninja_monkey: i64,
    pub sniper_monkey: i64,
    pub spike_factory: i64,
    pub super_monkey: i64,
    pub tack_shooter: i64,
    pub wizard_monkey: i64,
}
