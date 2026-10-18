use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Freeplay {
    #[serde(rename = "$type")]
    pub type_field: String,
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
    #[serde(default)]
    pub extra_shield: f64,
    pub has_taken_leak_damage_last_round: bool,
    pub claimed_freeplay_loot: bool,
    pub saved_maps_id: String,
    pub map_name: String,
    pub map_difficulty: String,
    pub mode_name: String,
    pub game_type: i64,
    pub freeplay_round_seed: i64,
    pub next_free_area_id: i64,
    pub loans: Vec<Value>,
    pub placed_towers: Vec<PlacedTower>,
    pub power_place_history: Vec<PowerPlaceHistory>,
    pub props_requiring_save_data: Vec<Value>,
    pub round_removeable_save_data: Vec<Value>,
    pub placed_projectiles: Vec<PlacedProjectile>,
    pub bloons: Vec<Value>,
    pub active_removeables_by_name: Vec<Value>,
    pub gizmo_save_data: Vec<Value>,
    pub map_event_save_data: MapEventSaveData,
    pub tower_discounts: Vec<Value>,
    pub players: Players,
    pub tower_type_max_placed_history: TowerTypeMaxPlacedHistory,
    pub tower_history: Vec<TowerHistory>,
    #[serde(rename = "towerMaxBaseIdAtATime")]
    pub tower_max_base_id_at_atime: TowerMaxBaseIdAtAtime,
    pub active_mutators: ActiveMutators,
    pub emergency_unlock_activated_at: EmergencyUnlockActivatedAt,
    pub golden_bloons_popped: i64,
    pub game_id: i64,
    pub attempt_id: i64,
    pub continues_used: i64,
    pub meta_data: MetaData3,
    pub cash_spent: f64,
    pub tile_id: i64,
    pub ts: Value,
    pub active_regen_removables_by_name: Vec<Value>,
    #[serde(rename = "playersWhoPlacedAHero")]
    pub players_who_placed_ahero: Vec<i64>,
    pub misc: String,
    pub active_knowledge: String,
    #[serde(default)]
    pub active_perks: String,
    pub active_relic_knowledge: Value,
    pub replay_recorder: Value,
    pub bloon_lives_lost_override: i64,
    pub rogue_map_save_data: Value,
    pub player_challenge_id: Value,
    pub player_challenge_type: Value,
    pub is_creation_mode: bool,
    #[serde(rename = "dailyChallengeEventID")]
    pub daily_challenge_event_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlacedTower {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub unique_id: String,
    pub damage_dealt: i64,
    pub cash_earned: i64,
    pub worth: f64,
    #[serde(rename = "IdLastSave")]
    pub id_last_save: i64,
    pub created_at: i64,
    pub target_type: TargetType,
    pub position: Position,
    pub rotation: f64,
    pub base_id: String,
    pub hero_id: String,
    pub hero_xp_in_current_level: f64,
    pub path_one_tier: i64,
    pub path_two_tier: i64,
    pub path_three_tier: i64,
    pub meta_data: MetaData,
    pub owner: i64,
    pub original_owner: i64,
    pub created_at_round_time: i64,
    pub parent_tower_id: i64,
    pub area_placed_on: i64,
    pub is_paragon: bool,
    pub geraldo_tower_id: Value,
    pub is_suspended: bool,
    pub is_selling_blocked: bool,
    pub is_selection_blocked: bool,
    pub tower_name: String,
    pub is_not_upgradeable: bool,
    pub lock_target_type_switching: bool,
    #[serde(rename = "path1NumBlockedTiers")]
    pub path1num_blocked_tiers: i64,
    #[serde(rename = "path2NumBlockedTiers")]
    pub path2num_blocked_tiers: i64,
    #[serde(rename = "path3NumBlockedTiers")]
    pub path3num_blocked_tiers: i64,
    pub power_id: Option<String>,
    pub custom_map_tower_index_id: i64,
    pub is_suspended_but_not_scaled: bool,
    pub frontier_id: i64,
    pub created_on_round: i64,
    pub lives_gained: f64,
    pub pops: i64,
    #[serde(default)]
    pub is_attacking_disabled: bool,
    #[serde(default)]
    pub are_abilities_disabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetType {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub id: String,
    pub is_actionable: bool,
    pub action_on_create: bool,
    #[serde(rename = "intID")]
    pub int_id: i64,
    pub continue_to_next_priority: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "Temple_Support")]
    pub temple_support: Option<String>,
    #[serde(rename = "SetCamoOn")]
    pub set_camo_on: Option<String>,
    #[serde(rename = "WeaponModel_Weapon_IDLastSave")]
    pub weapon_model_weapon_idlast_save: Option<String>,
    #[serde(rename = "ParagonInvestmentFromPops")]
    pub paragon_investment_from_pops: Option<String>,
    #[serde(rename = "ParagonInvestmentFromMoneySpent")]
    pub paragon_investment_from_money_spent: Option<String>,
    #[serde(rename = "ParagonInvestmentFromNonTier5")]
    pub paragon_investment_from_non_tier5: Option<String>,
    #[serde(rename = "ParagonInvestmentFromTier5")]
    pub paragon_investment_from_tier5: Option<String>,
    #[serde(rename = "ParagonInvestmentFromBonus")]
    pub paragon_investment_from_bonus: Option<String>,
    #[serde(rename = "ParagonInvestmentTotal")]
    pub paragon_investment_total: Option<String>,
    #[serde(rename = "ParagonActiveAt")]
    pub paragon_active_at: Option<String>,
    #[serde(rename = "Carpet Bomb")]
    pub carpet_bomb: Option<String>,
    #[serde(rename = "AirUnit Position")]
    pub air_unit_position: Option<String>,
    #[serde(rename = "AirUnit Rotation")]
    pub air_unit_rotation: Option<String>,
    #[serde(rename = "AirUnit TimeOnPath")]
    pub air_unit_time_on_path: Option<String>,
    #[serde(rename = "AirUnit GroundedAreaId")]
    pub air_unit_grounded_area_id: Option<String>,
    #[serde(rename = "CenterElipsePattern Position")]
    pub center_elipse_pattern_position: Option<String>,
    #[serde(rename = "AceParagonCarpetBombTargetPositions")]
    pub ace_paragon_carpet_bomb_target_positions: Option<String>,
    #[serde(rename = "TranceTargetPoint")]
    pub trance_target_point: Option<String>,
    #[serde(rename = "Temple_Primary")]
    pub temple_primary: Option<String>,
    #[serde(rename = "Temple_Magic")]
    pub temple_magic: Option<String>,
    #[serde(rename = "TrueTemple_Support")]
    pub true_temple_support: Option<String>,
    #[serde(rename = "TrueTemple_Magic")]
    pub true_temple_magic: Option<String>,
    #[serde(rename = "TrueTemple_Primary")]
    pub true_temple_primary: Option<String>,
    #[serde(rename = "TrueTemple_Military")]
    pub true_temple_military: Option<String>,
    #[serde(rename = "DarkTemple")]
    pub dark_temple: Option<String>,
    #[serde(rename = "WeaponModel_Right_IDLastSave")]
    pub weapon_model_right_idlast_save: Option<String>,
    #[serde(rename = "WeaponModel_Left_IDLastSave")]
    pub weapon_model_left_idlast_save: Option<String>,
    #[serde(rename = "WeaponModel_Right (1)_IDLastSave")]
    pub weapon_model_right_1_idlast_save: Option<String>,
    #[serde(rename = "WeaponModel_Left (1)_IDLastSave")]
    pub weapon_model_left_1_idlast_save: Option<String>,
    #[serde(rename = "WeaponModel_Right (2)_IDLastSave")]
    pub weapon_model_right_2_idlast_save: Option<String>,
    #[serde(rename = "WeaponModel_Left (2)_IDLastSave")]
    pub weapon_model_left_2_idlast_save: Option<String>,
    #[serde(rename = "Eruption")]
    pub eruption: Option<String>,
    #[serde(rename = "Meteor Impact")]
    pub meteor_impact: Option<String>,
    #[serde(rename = "WeaponModel_WeaponInverse_IDLastSave")]
    pub weapon_model_weapon_inverse_idlast_save: Option<String>,
    #[serde(rename = "PermanentBrewTowers")]
    pub permanent_brew_towers: Option<String>,
    #[serde(rename = "RemovePermaBrewTowers")]
    pub remove_perma_brew_towers: Option<String>,
    #[serde(rename = "Spike Para Radius")]
    pub spike_para_radius: Option<String>,
    #[serde(rename = "Controlled Burst")]
    pub controlled_burst: Option<String>,
    #[serde(rename = "Spikeageddon")]
    pub spikeageddon: Option<String>,
    #[serde(rename = "Find Path Segment")]
    pub find_path_segment: Option<String>,
    #[serde(rename = "Target Position")]
    pub target_position: Option<String>,
    #[serde(rename = "Drone Swarm")]
    pub drone_swarm: Option<String>,
    #[serde(rename = "UCAV")]
    pub ucav: Option<String>,
    #[serde(rename = "Round Based Attack Speed")]
    pub round_based_attack_speed: Option<String>,
    #[serde(rename = "SentryParagonSpawner")]
    pub sentry_paragon_spawner: Option<String>,
    #[serde(rename = "Paragon Overclock")]
    pub paragon_overclock: Option<String>,
    #[serde(rename = "CreateSequencedTypedTowerCurrentIndex currentIndex")]
    pub create_sequenced_typed_tower_current_index_current_index: Option<String>,
    #[serde(rename = "Overclock Linked Tower Id")]
    pub overclock_linked_tower_id: Option<String>,
    #[serde(rename = "Spirit Radius")]
    pub spirit_radius: Option<String>,
    #[serde(rename = "Jungle's Bounty")]
    pub jungle_s_bounty: Option<String>,
    #[serde(rename = "Primordial Wrath")]
    pub primordial_wrath: Option<String>,
    #[serde(rename = "PrimordialWrath_depletionTimer")]
    pub primordial_wrath_depletion_timer: Option<String>,
    #[serde(rename = "Bomb Blitz")]
    pub bomb_blitz: Option<String>,
    #[serde(rename = "Indomitable Surface-to-Air Ballistic Missile")]
    pub indomitable_surface_to_air_ballistic_missile: Option<String>,
    #[serde(rename = "Magnus Graveyard Mana State")]
    pub magnus_graveyard_mana_state: Option<String>,
    #[serde(rename = "Magnus Explosion Remaining Frames")]
    pub magnus_explosion_remaining_frames: Option<String>,
    #[serde(rename = "Magnus Draining Beam Toggled At")]
    pub magnus_draining_beam_toggled_at: Option<String>,
    #[serde(rename = "Magnus RBE Decay Rounds 0")]
    pub magnus_rbe_decay_rounds_0: Option<String>,
    #[serde(rename = "Magnus RBE Decay Rounds 1")]
    pub magnus_rbe_decay_rounds_1: Option<String>,
    #[serde(rename = "Magnus RBE Decay Rounds 2")]
    pub magnus_rbe_decay_rounds_2: Option<String>,
    #[serde(rename = "Magnus RBE Decay Rounds 3")]
    pub magnus_rbe_decay_rounds_3: Option<String>,
    #[serde(rename = "Magnus RBE Decay Rounds 4")]
    pub magnus_rbe_decay_rounds_4: Option<String>,
    #[serde(rename = "Magnus RBE Decay Rounds 5")]
    pub magnus_rbe_decay_rounds_5: Option<String>,
    #[serde(rename = "Magnus RBE Decay Rounds 6")]
    pub magnus_rbe_decay_rounds_6: Option<String>,
    #[serde(rename = "Magnus RBE Decay Rounds 7")]
    pub magnus_rbe_decay_rounds_7: Option<String>,
    #[serde(rename = "Magnus RBE Decay Rounds 8")]
    pub magnus_rbe_decay_rounds_8: Option<String>,
    #[serde(rename = "Magnus RBE Decay Rounds 9")]
    pub magnus_rbe_decay_rounds_9: Option<String>,
    #[serde(rename = "Magnus RBE Decay Rounds 10")]
    pub magnus_rbe_decay_rounds_10: Option<String>,
    #[serde(rename = "Arcane Metamorphosis")]
    pub arcane_metamorphosis: Option<String>,
    #[serde(rename = "Phoenix Rebirth")]
    pub phoenix_rebirth: Option<String>,
    #[serde(rename = "Phoenix Rebirth Reactivate At")]
    pub phoenix_rebirth_reactivate_at: Option<String>,
    #[serde(rename = "WeaponModel_SabotageWeapon_IDLastSave")]
    pub weapon_model_sabotage_weapon_idlast_save: Option<String>,
    #[serde(rename = "Eclipse of Fimbulwinter")]
    pub eclipse_of_fimbulwinter: Option<String>,
    #[serde(rename = "Super Maelstrom")]
    pub super_maelstrom: Option<String>,
    #[serde(rename = "Link")]
    pub link: Option<String>,
    #[serde(rename = "Unlink")]
    pub unlink: Option<String>,
    #[serde(rename = "Ambush Tech")]
    pub ambush_tech: Option<String>,
    #[serde(rename = "TakeAim2")]
    pub take_aim2: Option<String>,
    #[serde(rename = "Marked To Pop")]
    pub marked_to_pop: Option<String>,
    #[serde(rename = "SetBountyHunterToggleOn")]
    pub set_bounty_hunter_toggle_on: Option<String>,
    #[serde(rename = "WeaponModel_Weapon_BurstWeaponBehaviorCurrentCount")]
    pub weapon_model_weapon_burst_weapon_behavior_current_count: Option<String>,
    #[serde(rename = "TakeAim Linked Tower Id")]
    pub take_aim_linked_tower_id: Option<String>,
    #[serde(rename = "DesperadoMarkedToPopPhase")]
    pub desperado_marked_to_pop_phase: Option<String>,
    #[serde(rename = "DesperadoMarkedToPopExecutionStartedAt")]
    pub desperado_marked_to_pop_execution_started_at: Option<String>,
    #[serde(rename = "Glue Storm")]
    pub glue_storm: Option<String>,
    #[serde(rename = "Ultraboost")]
    pub ultraboost: Option<String>,
    #[serde(rename = "WeaponModel_Weapon_lastEmission")]
    pub weapon_model_weapon_last_emission: Option<String>,
    #[serde(rename = "Final Strike")]
    pub final_strike: Option<String>,
    #[serde(rename = "SubmergeTargetPreviousTargetPrio")]
    pub submerge_target_previous_target_prio: Option<String>,
    #[serde(rename = "WeaponModel_WeaponDamage_IDLastSave")]
    pub weapon_model_weapon_damage_idlast_save: Option<String>,
    #[serde(rename = "Homeland Defense")]
    pub homeland_defense: Option<String>,
    #[serde(rename = "Tech Bot Link Id39946983")]
    pub tech_bot_link_id39946983: Option<String>,
    #[serde(rename = "MultiHookBlockEmissionUntil0")]
    pub multi_hook_block_emission_until0: Option<String>,
    #[serde(rename = "MultiHookBlockEmissionUntil1")]
    pub multi_hook_block_emission_until1: Option<String>,
    #[serde(rename = "MultiHookBlockEmissionUntil2")]
    pub multi_hook_block_emission_until2: Option<String>,
    #[serde(rename = "MultiHookBlockEmissionUntil3")]
    pub multi_hook_block_emission_until3: Option<String>,
    #[serde(rename = "MultiHookBlockEmissionUntil4")]
    pub multi_hook_block_emission_until4: Option<String>,
    #[serde(rename = "MultiHookBlockEmissionUntil5")]
    pub multi_hook_block_emission_until5: Option<String>,
    #[serde(rename = "MultiHookBlockEmissionUntil6")]
    pub multi_hook_block_emission_until6: Option<String>,
    #[serde(rename = "MultiHookBlockEmissionUntil7")]
    pub multi_hook_block_emission_until7: Option<String>,
    #[serde(rename = "MultiHookBlockEmissionUntil8")]
    pub multi_hook_block_emission_until8: Option<String>,
    #[serde(rename = "MultiHookBlockEmissionUntil9")]
    pub multi_hook_block_emission_until9: Option<String>,
    #[serde(rename = "AddMakeShiftAreaIDAddMakeshiftAreaModel_TowerPlatformfirst")]
    pub add_make_shift_area_idadd_makeshift_area_model_tower_platformfirst: Option<String>,
    #[serde(rename = "AddMakeShiftAreaIDAddMakeshiftAreaModel_TowerPlatformsecond")]
    pub add_make_shift_area_idadd_makeshift_area_model_tower_platformsecond: Option<String>,
    #[serde(rename = "Large BAD Harpoon")]
    pub large_bad_harpoon: Option<String>,
    #[serde(rename = "WeaponModel_WeaponCannonballs_IDLastSave")]
    pub weapon_model_weapon_cannonballs_idlast_save: Option<String>,
    #[serde(rename = "WeaponModel_WeaponGrapeShot_IDLastSave")]
    pub weapon_model_weapon_grape_shot_idlast_save: Option<String>,
    #[serde(rename = "Tech Bot Link Id19109082")]
    pub tech_bot_link_id19109082: Option<String>,
    #[serde(rename = "Tech Bot Link Id19908627")]
    pub tech_bot_link_id19908627: Option<String>,
    #[serde(rename = "Tech Bot Link Id20004724")]
    pub tech_bot_link_id20004724: Option<String>,
    #[serde(rename = "Tech Bot Link Id19567818")]
    pub tech_bot_link_id19567818: Option<String>,
    #[serde(rename = "Tech Bot Link Id19815532")]
    pub tech_bot_link_id19815532: Option<String>,
    #[serde(rename = "WeaponModel_Weapon Secondary_IDLastSave")]
    pub weapon_model_weapon_secondary_idlast_save: Option<String>,
    #[serde(rename = "Overcharge")]
    pub overcharge: Option<String>,
    #[serde(rename = "WeaponModel_OrbitDamage_IDLastSave")]
    pub weapon_model_orbit_damage_idlast_save: Option<String>,
    #[serde(rename = "WeaponModel_MOABPress_IDLastSave")]
    pub weapon_model_moabpress_idlast_save: Option<String>,
    #[serde(rename = "Overcharge_currentStack")]
    pub overcharge_current_stack: Option<String>,
    #[serde(rename = "Overcharge_lastStackStartFrame")]
    pub overcharge_last_stack_start_frame: Option<String>,
    #[serde(rename = "Tech Bot Link Id59070606")]
    pub tech_bot_link_id59070606: Option<String>,
    #[serde(rename = "Marked To Pop2")]
    pub marked_to_pop2: Option<String>,
    #[serde(rename = "Tech Bot Link Id74383413")]
    pub tech_bot_link_id74383413: Option<String>,
    #[serde(rename = "Overclock")]
    pub overclock: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerPlaceHistory {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub power_id: String,
    pub owner: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlacedProjectile {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub pierce: f64,
    pub base_id: String,
    pub position: Position2,
    pub rotation: f64,
    pub meta_data: MetaData2,
    pub owner: i64,
    pub emitted_by_tower_id: i64,
    pub emitted_from: EmittedFrom,
    pub is_power_projectile: bool,
    pub power_id: Value,
    pub created_at: i64,
    #[serde(default)]
    pub weapon_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position2 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData2 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub created_at: Option<String>,
    pub rounds_left: Option<String>,
    #[serde(rename = "CurrentPositionExaustFraction")]
    pub current_position_exaust_fraction: Option<String>,
    pub save_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmittedFrom {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapEventSaveData {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "RemovingOre1", default)]
    pub removing_ore1: RemovingOre1,
    #[serde(rename = "Ore1", default)]
    pub ore1: Ore1,
    #[serde(rename = "RemovingOre2", default)]
    pub removing_ore2: RemovingOre2,
    #[serde(rename = "Ore2", default)]
    pub ore2: Ore2,
    #[serde(rename = "RemovingOre3", default)]
    pub removing_ore3: RemovingOre3,
    #[serde(rename = "Ore3", default)]
    pub ore3: Ore3,
    #[serde(rename = "RemovingOre4", default)]
    pub removing_ore4: RemovingOre4,
    #[serde(rename = "Ore4", default)]
    pub ore4: Ore4,
    #[serde(rename = "RemovingOre5", default)]
    pub removing_ore5: RemovingOre5,
    #[serde(rename = "Ore5", default)]
    pub ore5: Ore5,
    #[serde(rename = "RemovingOre6", default)]
    pub removing_ore6: RemovingOre6,
    #[serde(rename = "Ore6", default)]
    pub ore6: Ore6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemovingOre1 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub trigger_count: i64,
    pub trigger_save_data: TriggerSaveData,
    pub action_save_data: ActionSaveData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerSaveData {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "RoundRemoveableStartedTriggerModel1")]
    pub round_removeable_started_trigger_model1: RoundRemoveableStartedTriggerModel1,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundRemoveableStartedTriggerModel1 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub bool1: bool,
    pub bool2: bool,
    pub int1: i64,
    pub int2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionSaveData {
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ore1 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub trigger_count: i64,
    pub trigger_save_data: TriggerSaveData2,
    pub action_save_data: ActionSaveData2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerSaveData2 {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "RemoveableSoldTriggerModel1")]
    pub removeable_sold_trigger_model1: RemoveableSoldTriggerModel1,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveableSoldTriggerModel1 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub bool1: bool,
    pub bool2: bool,
    pub int1: i64,
    pub int2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionSaveData2 {
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemovingOre2 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub trigger_count: i64,
    pub trigger_save_data: TriggerSaveData3,
    pub action_save_data: ActionSaveData3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerSaveData3 {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "RoundRemoveableStartedTriggerModel1")]
    pub round_removeable_started_trigger_model1: RoundRemoveableStartedTriggerModel12,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundRemoveableStartedTriggerModel12 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub bool1: bool,
    pub bool2: bool,
    pub int1: i64,
    pub int2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionSaveData3 {
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ore2 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub trigger_count: i64,
    pub trigger_save_data: TriggerSaveData4,
    pub action_save_data: ActionSaveData4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerSaveData4 {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "RemoveableSoldTriggerModel1")]
    pub removeable_sold_trigger_model1: RemoveableSoldTriggerModel12,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveableSoldTriggerModel12 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub bool1: bool,
    pub bool2: bool,
    pub int1: i64,
    pub int2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionSaveData4 {
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemovingOre3 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub trigger_count: i64,
    pub trigger_save_data: TriggerSaveData5,
    pub action_save_data: ActionSaveData5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerSaveData5 {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "RoundRemoveableStartedTriggerModel1")]
    pub round_removeable_started_trigger_model1: RoundRemoveableStartedTriggerModel13,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundRemoveableStartedTriggerModel13 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub bool1: bool,
    pub bool2: bool,
    pub int1: i64,
    pub int2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionSaveData5 {
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ore3 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub trigger_count: i64,
    pub trigger_save_data: TriggerSaveData6,
    pub action_save_data: ActionSaveData6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerSaveData6 {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "RemoveableSoldTriggerModel1")]
    pub removeable_sold_trigger_model1: RemoveableSoldTriggerModel13,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveableSoldTriggerModel13 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub bool1: bool,
    pub bool2: bool,
    pub int1: i64,
    pub int2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionSaveData6 {
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemovingOre4 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub trigger_count: i64,
    pub trigger_save_data: TriggerSaveData7,
    pub action_save_data: ActionSaveData7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerSaveData7 {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "RoundRemoveableStartedTriggerModel1")]
    pub round_removeable_started_trigger_model1: RoundRemoveableStartedTriggerModel14,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundRemoveableStartedTriggerModel14 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub bool1: bool,
    pub bool2: bool,
    pub int1: i64,
    pub int2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionSaveData7 {
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ore4 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub trigger_count: i64,
    pub trigger_save_data: TriggerSaveData8,
    pub action_save_data: ActionSaveData8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerSaveData8 {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "RemoveableSoldTriggerModel1")]
    pub removeable_sold_trigger_model1: RemoveableSoldTriggerModel14,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveableSoldTriggerModel14 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub bool1: bool,
    pub bool2: bool,
    pub int1: i64,
    pub int2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionSaveData8 {
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemovingOre5 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub trigger_count: i64,
    pub trigger_save_data: TriggerSaveData9,
    pub action_save_data: ActionSaveData9,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerSaveData9 {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "RoundRemoveableStartedTriggerModel1")]
    pub round_removeable_started_trigger_model1: RoundRemoveableStartedTriggerModel15,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundRemoveableStartedTriggerModel15 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub bool1: bool,
    pub bool2: bool,
    pub int1: i64,
    pub int2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionSaveData9 {
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ore5 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub trigger_count: i64,
    pub trigger_save_data: TriggerSaveData10,
    pub action_save_data: ActionSaveData10,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerSaveData10 {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "RemoveableSoldTriggerModel1")]
    pub removeable_sold_trigger_model1: RemoveableSoldTriggerModel15,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveableSoldTriggerModel15 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub bool1: bool,
    pub bool2: bool,
    pub int1: i64,
    pub int2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionSaveData10 {
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemovingOre6 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub trigger_count: i64,
    pub trigger_save_data: TriggerSaveData11,
    pub action_save_data: ActionSaveData11,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerSaveData11 {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "RoundRemoveableStartedTriggerModel1")]
    pub round_removeable_started_trigger_model1: RoundRemoveableStartedTriggerModel16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundRemoveableStartedTriggerModel16 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub bool1: bool,
    pub bool2: bool,
    pub int1: i64,
    pub int2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionSaveData11 {
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ore6 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub trigger_count: i64,
    pub trigger_save_data: TriggerSaveData12,
    pub action_save_data: ActionSaveData12,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerSaveData12 {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "RemoveableSoldTriggerModel1")]
    pub removeable_sold_trigger_model1: RemoveableSoldTriggerModel16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveableSoldTriggerModel16 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub bool1: bool,
    pub bool2: bool,
    pub int1: i64,
    pub int2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionSaveData12 {
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Players {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "-1")]
    pub n1: N1,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N1 {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub index: i64,
    pub cash: f64,
    pub hero: String,
    pub free_towers: Vec<FreeTower>,
    pub tower_discounts: Vec<TowerDiscount>,
    pub geraldo_shop_items: Vec<GeraldoShopItem>,
    #[serde(rename = "lvl20GeraldoRestockApplied")]
    pub lvl20geraldo_restock_applied: bool,
    pub has_placed_hero: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreeTower {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "baseTowerID")]
    pub base_tower_id: String,
    pub charges: i64,
    pub mutually_exclusive_with: String,
    pub priority: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TowerDiscount {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub discount_name: String,
    pub charges: i64,
    pub towers: Vec<String>,
    pub multiplier: f64,
    pub subtraction: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeraldoShopItem {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub name: String,
    pub used: i64,
    pub remaining: i64,
    pub replenishing_from_round: i64,
    pub round_unlocked_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TowerTypeMaxPlacedHistory {
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TowerHistory {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub unique_id: String,
    pub base_id: String,
    pub tower_set: i64,
    pub tiers: Vec<i64>,
    pub cost: f64,
    pub destroyed: bool,
    pub tiers_modifier: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TowerMaxBaseIdAtAtime {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "SuperMonkey", default)]
    pub super_monkey: i64,
    #[serde(rename = "MonkeyAce", default)]
    pub monkey_ace: i64,
    #[serde(rename = "SpikeFactory", default)]
    pub spike_factory: i64,
    #[serde(rename = "Mermonkey", default)]
    pub mermonkey: i64,
    #[serde(rename = "NinjaMonkey", default)]
    pub ninja_monkey: i64,
    #[serde(rename = "Druid", default)]
    pub druid: i64,
    #[serde(rename = "TackShooter", default)]
    pub tack_shooter: i64,
    #[serde(rename = "BombShooter", default)]
    pub bomb_shooter: i64,
    #[serde(rename = "DartlingGunner", default)]
    pub dartling_gunner: i64,
    #[serde(rename = "Alchemist", default)]
    pub alchemist: i64,
    #[serde(rename = "HeliPilot", default)]
    pub heli_pilot: i64,
    #[serde(rename = "MonkeyBuccaneer", default)]
    pub monkey_buccaneer: i64,
    #[serde(rename = "BoomerangMonkey", default)]
    pub boomerang_monkey: i64,
    #[serde(rename = "EngineerMonkey", default)]
    pub engineer_monkey: i64,
    #[serde(rename = "WizardMonkey", default)]
    pub wizard_monkey: i64,
    #[serde(rename = "MonkeyVillage", default)]
    pub monkey_village: i64,
    #[serde(rename = "Etienne", default)]
    pub etienne: i64,
    #[serde(rename = "Desperado", default)]
    pub desperado: i64,
    #[serde(rename = "MonkeySub", default)]
    pub monkey_sub: i64,
    #[serde(rename = "IceMonkey", default)]
    pub ice_monkey: i64,
    #[serde(rename = "SniperMonkey", default)]
    pub sniper_monkey: i64,
    #[serde(rename = "GlueGunner", default)]
    pub glue_gunner: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveMutators {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "Overclock")]
    pub overclock: Vec<Overclock>,
    #[serde(rename = "TakeAim", default)]
    pub take_aim: Vec<TakeAim>,
    #[serde(rename = "Ultraboost")]
    pub ultraboost: Vec<Ultraboost>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Overclock {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub mutator_save_id: String,
    pub tower_id: i64,
    pub frames_left: i64,
    pub counter: i64,
    pub rounds_left: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakeAim {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub mutator_save_id: String,
    pub tower_id: i64,
    pub frames_left: i64,
    pub counter: i64,
    pub rounds_left: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ultraboost {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub mutator_save_id: String,
    pub tower_id: i64,
    pub frames_left: i64,
    pub counter: i64,
    pub rounds_left: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmergencyUnlockActivatedAt {
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData3 {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "SharedAbilityCooldowns")]
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
