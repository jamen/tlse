use crate::P_MAIN_GAME_COMPONENT;

use tlse_sys::{CGameScriptInterface, CScriptThing};

unsafe fn game_script_interface() -> *mut CGameScriptInterface {
    (*(*(*P_MAIN_GAME_COMPONENT)).p_world.0)
        .p_game_script_interface
        .0
}

macro_rules! impl_simple_game_script_fns {
    (

        $(
            $(#[$fn_attr:meta])*
            $v:vis fn $fn_name:ident ($($param_name:ident : $param_type:ty),* $(,)?) $(-> $ret_type:ty)?
        );*
        $(;)?
    ) => {

        $(
            $(#[$fn_attr])*
            $v fn $fn_name ($($param_name : $param_type),*) $(-> $ret_type)? {
                unsafe {
                    let s = crate::script::game_script_interface();
                    ((*(*s).vmt).$fn_name)(s, $($param_name.into()),*)
                }
            }
        )*
    }
}

use std::os::raw::{c_float, c_long, c_ulong};

impl_simple_game_script_fns! {
    pub fn end_letter_box();
    pub fn validate();
    pub fn set_debug_camera_type(unnamed_1: c_long);
    pub fn deactivate_boast_ui();
    pub fn is_xbox() -> bool;
    pub fn new_script_frame();
    pub fn msg_is_boast_made(unnamed_1: c_long) -> bool ;
    pub fn remove_boast_message() ;
    pub fn msg_on_leaving_experience_spending_screen() -> bool ;
    pub fn msg_is_answered_yes_or_no() -> c_long ;
    pub fn msg_is_game_info_clicked_past() -> bool ;
    pub fn msg_is_tutorial_click_past() -> bool ;
    pub fn msg_is_action_mode_button_pressed() -> bool ;
    pub fn msg_is_cut_scene_skipped() -> bool ;
    pub fn msg_on_hero_used_guild_seal() -> bool ;
    pub fn msg_on_game_saved_manually() -> bool ;
    pub fn msg_on_hero_fired_ranged_weapon() -> bool ;
    pub fn msg_on_chest_opening_cancelled() -> bool ;
    pub fn dont_populate_next_loaded_region() ;
    pub fn is_hero_allowed_henchmen_in_current_region() -> bool ;
    pub fn post_add_scripted_entities() ;
    pub fn is_player_holding_unsheathe_ranged_weapon_button() -> bool ;
    pub fn is_player_holding_lock_target_button() -> bool ;
    pub fn is_player_holding_fire_ranged_weapon_button() -> bool ;
    pub fn is_player_holding_first_person_targeting_button() -> bool ;
    pub fn is_hero_in_projectile_weapon_mode() -> bool ;
    pub fn register_timer() -> c_long ;
    pub fn deregister_timer(unnamed_1: c_long) ;
    pub fn set_timer(unnamed_1: c_long, unnamed_2: c_long) ;
    pub fn get_timer(unnamed_1: c_long) -> c_long ;
    pub fn set_creature_creation_delay_frames(unnamed_1: c_long) ;
    pub fn reset_creature_creation_delay_frames() ;
    pub fn is_thing_with_this_uid_alive(unnamed_1: c_ulong) -> bool ;
    pub fn clear_all_rumbles() ;
    pub fn is_safe_to_display_game_info() -> bool ;
    pub fn is_tutorial_system_enabled() -> bool ;
    pub fn give_hero_gold(unnamed_1: c_long) ;
    pub fn get_hero_gold() -> c_long ;
    pub fn give_hero_experience(unnamed_1: c_long) ;
    pub fn set_hero_able_to_gain_experience(unnamed_1: bool) ;
    pub fn sheathe_hero_weapons() ;
    pub fn set_hero_will_as_usable(unnamed_1: bool) ;
    pub fn set_hero_weapons_as_usable(unnamed_1: bool) ;
    pub fn set_weapon_out_crime_enabled(unnamed_1: bool) ;
    pub fn set_guards_ignore_crimes(unnamed_1: bool) ;
    pub fn remove_all_hero_weapons() ;
    pub fn confiscate_all_hero_items() ;
    pub fn confiscate_all_hero_weapons() ;
    pub fn return_all_confiscated_items_to_hero() ;
    pub fn is_player_creature_blocking() -> bool ;
    pub fn get_player_creature_combat_multiplier() -> c_long ;
    pub fn get_player_creature_combat_multiplier_running_num_hits() -> c_long ;
    pub fn reset_player_creature_combat_multiplier() ;
    pub fn is_player_creature_flourish_enabled() -> bool ;
    pub fn reset_player_creature_only_target() ;
    pub fn respawn_hero(unnamed_1: bool) ;
    pub fn give_hero_morality(unnamed_1: c_float) ;
    pub fn get_hero_morality() -> c_float ;
    pub fn change_hero_morality_due_to_theft() ;
    pub fn change_hero_morality_due_to_picklock() ;
    pub fn give_hero_renown_points(unnamed_1: c_long) ;
    pub fn get_hero_renown_level() -> c_long ;
    pub fn is_hero_renown_level_full() -> bool ;
    pub fn increase_hero_renown_level() ;
    pub fn get_hero_strength_level() -> c_long ;
    pub fn get_hero_skill_level() -> c_long ;
    pub fn get_hero_will_level() -> c_long ;
    pub fn set_hero_age(unnamed_1: c_float) ;
    pub fn get_hero_age() -> c_float ;
    pub fn set_hero_as_teenager(unnamed_1: bool) ;
    pub fn set_hero_as_apprentice(unnamed_1: bool) ;
    pub fn get_distance_hero_can_be_heard_from() -> c_float ;
    pub fn get_hero_rough_experience_level() -> c_long ;
    pub fn get_hero_experience_available_to_spend() -> c_long ;
    pub fn get_hero_fatness() -> c_float ;
    pub fn get_hero_scariness() -> c_float ;
    pub fn get_hero_attractiveness() -> c_float ;
    pub fn get_hero_will_energy_level() -> c_float ;
    pub fn set_hero_will_energy_level(unnamed_1: c_float) ;
    pub fn set_hero_will_energy_as_able_to_refill(unnamed_1: bool) ;
    pub fn is_hero_hand_lamp_lit() -> bool ;
    pub fn set_hero_hand_lamp_as_lit(unnamed_1: bool) ;
    pub fn is_hero_naked() -> bool ;
    pub fn remove_hero_clothing() ;
    pub fn remove_hero_hairstyle() ;
    pub fn is_player_wielding_weapon() -> bool ;
    pub fn apply_hero_penalty_for_death() ;
    pub fn get_hero_has_married() -> bool ;
    pub fn get_hero_has_current_marriage() -> bool ;
    pub fn get_hero_has_divorced_marriage() -> bool ;
    pub fn get_hero_has_children() -> bool ;
    pub fn get_hero_has_murdered_wife() -> bool ;
    pub fn is_hero_child() -> bool ;
    pub fn cancel_hero_teleport_effects() ;
    pub fn get_number_of_times_hero_has_had_sex() -> c_long ;
    pub fn set_number_of_times_hero_has_had_sex(unnamed_1: c_long) ;
    pub fn set_hero_as_having_had_sex(unnamed_1: bool) ;
    pub fn set_hero_as_having_had_gay_sex(unnamed_1: bool) ;
    pub fn set_hero_guide_to_show_quest_cards_when_spoken_to(unnamed_1: bool) ;
    pub fn set_teleporting_as_active(unnamed_1: bool) ;
    pub fn is_teleporting_active() -> bool ;
    pub fn set_region_text_display_as_active(unnamed_1: bool) ;
    pub fn set_hero_sleeping_as_enabled(unnamed_1: bool) ;
    pub fn is_hero_sleeping_enabled() -> bool ;
    pub fn set_experience_spending_as_enabled(unnamed_1: bool) ;
    pub fn set_morality_changing_as_enabled(unnamed_1: bool) ;
    pub fn set_summoner_death_explosion_affects_hero(unnamed_1: bool) ;
    pub fn is_deed_witnessed(unnamed_1: c_long) -> bool ;
    pub fn teleport_all_followers_to_hero_position() ;
    pub fn hero_go_fishing(unnamed_1: bool) ;
    pub fn get_hero_fishing_level() -> c_long ;
    pub fn hero_go_digging() ;
    pub fn hero_stop_digging() ;
    pub fn hero_play_oracle_minigame() ;
    pub fn is_hero_playing_oracle_minigame() -> bool ;
    pub fn has_hero_won_oracle_minigame() -> bool ;
    pub fn hero_play_fireheart_minigame() ;
    pub fn hero_quit_fireheart_minigame() ;
    pub fn has_hero_force_quit_fireheart_minigame() -> bool ;
    pub fn get_hero_health() -> c_float ;
    pub fn get_hero_health_max() -> c_float ;
    pub fn get_hero_health_percentage() -> c_float ;
    pub fn get_hero_will_energy() -> c_long ;
    pub fn get_hero_will_energy_max() -> c_long ;
    pub fn get_hero_will_energy_percentage() -> c_long ;
    pub fn change_hero_health_by(unnamed_1: c_float, unnamed_2: bool, unnamed_3: bool) ;
    pub fn tell_hero_quest_objective_completed(unnamed_1: c_ulong) ;
    pub fn tell_hero_quest_objective_failed(unnamed_1: c_ulong) ;
    pub fn is_hero_on_quest() -> bool ;
    pub fn set_guild_master_messages(unnamed_1: bool) ;
    pub fn prepare_quests_when_final_quest_is_completed() ;
    pub fn remove_all_available_quest_cards_from_guild() ;
    pub fn fail_all_active_quests() ;
    pub fn kick_off_death_screen() ;
    pub fn reset_death_recovery_marker_name_to_default() ;
    pub fn is_to_fail_quest_on_death() -> bool ;
    pub fn set_whether_to_fail_quest_on_death(unnamed_1: bool) ;
    pub fn reset_whether_to_fail_quest_on_death_to_default() ;
    pub fn display_quest_info(unnamed_1: bool) ;
    pub fn update_quest_info_timer(unnamed_1: c_long, unnamed_2: c_float);
    pub fn update_quest_info_tick(unnamed_1: c_long, unnamed_2: bool);
    pub fn remove_quest_info_element(unnamed_1: c_long);
    pub fn remove_all_quest_info_elements();
    pub fn display_time(unnamed_1: bool);
    pub fn display_money_bag(unnamed_1: bool);
    pub fn update_mini_game_info_bar(unnamed_1: c_float);
    pub fn mini_map_remove_all_markers();
    pub fn mini_map_set_as_enabled(unnamed_1: bool) ;
    pub fn set_hud_enabled(unnamed_1: bool) ;
    pub fn remove_conversation(unnamed_1: c_long, unnamed_2: bool) ;
    pub fn is_conversation_active(unnamed_1: c_long) -> bool ;
    pub fn fix_movie_sequence_camera(unnamed_1: bool) ;
    pub fn fade_screen_out_until_next_call_to_fade_screen_in(unnamed_1: c_float, unnamed_2: c_float) ;
    pub fn fade_screen_in() ;
    pub fn is_screen_fading_out() -> bool ;
    pub fn pause(unnamed_1: c_float) ;
    pub fn end_cut_fade() ;
    pub fn pause_all_non_scripted_entities(unnamed_1: bool) ;
    pub fn pause_all_entities(unnamed_1: bool) ;
    pub fn set_allow_screen_fading_on_next_region_change(unnamed_1: bool) ;
    pub fn set_allow_screen_fading_if_already_faded(unnamed_1: bool) ;
    pub fn set_to_keep_hero_abilities_during_cutscenes(unnamed_1: bool) ;
    pub fn set_to_display_tutorials_during_cutscenes(unnamed_1: bool) ;
    pub fn set_cutscene_mode(unnamed_1: bool, unnamed_2: bool) ;
    pub fn is_in_cutscene() -> bool ;
    pub fn set_cutscene_skippable(unnamed_1: bool) ;
    pub fn set_cutscene_skippable_while_paused(unnamed_1: bool) ;
    pub fn preload_new_scene(unnamed_1: c_float) ;
    pub fn start_progress_display() ;
    pub fn stop_progress_display() ;
    pub fn set_screen_messages_enabled(unnamed_1: bool) ;
    pub fn is_hero_controlled_by_player() -> bool ;
    pub fn is_in_movie_sequence() -> bool ;
    pub fn cancel_abilities_for_cutscene() ;
    pub fn resume_abilities_for_cutscene() ;
    pub fn set_environmental_effects_always_update(unnamed_1: bool) ;
    pub fn set_dead_creatures_and_experience_orbs_and_drop_bags_as_hidden(unnamed_1: bool) ;
    pub fn camera_set_camera_preload_flag(unnamed_1: bool) ;
    pub fn camera_default() ;
    pub fn camera_reset_to_view_behind_hero(unnamed_1: c_float) ;
    pub fn is_camera_in_scripted_mode() -> bool ;
    pub fn camera_cancel_screen_effect() ;
    pub fn get_game_angle_xy() -> c_float ;
    pub fn camera_shake(unnamed_1: c_float, unnamed_2: c_float) ;
    pub fn set_time_of_day(unnamed_1: c_float) ;
    pub fn get_time_of_day() -> c_long ;
    pub fn fast_forward_time_to(unnamed_1: c_float, unnamed_2: c_float) ;
    pub fn is_time_of_day_between(unnamed_1: c_long, unnamed_2: c_long) -> bool ;
    pub fn get_day_of_week() -> c_long ;
    pub fn get_day_count() -> c_long ;
    pub fn get_world_frame() -> c_long ;
    pub fn get_constant_fps() -> c_long ;
    pub fn reset_to_default_theme(unnamed_1: c_float) ;
    pub fn reset_to_default_theme_all_internals(unnamed_1: c_float) ;
    pub fn reset_to_default_theme_externals(unnamed_1: c_float) ;
    pub fn set_all_sounds_as_muted(unnamed_1: bool) ;
    pub fn is_radial_blur_fade_active() -> bool ;
    pub fn cancel_radial_blur_fade() ;
    pub fn enable_decals(unnamed_1: bool) ;
    pub fn is_sound_playing(unnamed_1: c_ulong) -> bool ;
    pub fn stop_sound(unnamed_1: c_ulong) ;
    pub fn enable_sounds(unnamed_1: bool) ;
    pub fn stop_override_music(unnamed_1: bool) ;
    pub fn enable_danger_music(unnamed_1: bool) ;
    pub fn is_danger_music_enabled() -> bool ;
    pub fn start_countdown_timer(unnamed_1: c_float) ;
    pub fn get_countdown_timer() -> c_float ;
    pub fn auto_save_check_point() ;
    pub fn auto_save_quest_start() ;
    pub fn auto_save() ;
    pub fn set_saving_as_enabled(unnamed_1: bool) ;
    pub fn is_saving_enabled() -> bool ;
    pub fn reset_to_front_end() ;
    pub fn get_guild_seal_recall_angle_xy() -> c_float ;
    pub fn update_online_score_archery(unnamed_1: c_float) ;
    pub fn update_online_score_chicken_kick(unnamed_1: c_float) ;
    pub fn update_online_score_chapel_or_temple(unnamed_1: c_float) ;
    pub fn update_online_score_fishing_compo(unnamed_1: c_float) ;
    pub fn update_score_fishing_competition(unnamed_1: c_float) ;
    pub fn get_best_time_pairs() -> c_float ;
    pub fn get_best_time_sorting() -> c_float ;
    pub fn get_best_score_blackjack() -> c_long ;
    pub fn get_best_score_coin_golf_oak_vale() -> c_long ;
    pub fn get_best_score_coin_golf_snow_spire() -> c_long ;
    pub fn get_best_score_shove_ha_penny() -> c_long ;
    pub fn get_best_time_guess_the_addition() -> c_float ;
    pub fn is_hero_in_tavern_game() -> bool ;
    pub fn get_num_houses_owned() -> c_long ;
    pub fn start_sneaking() ;
    pub fn force_ships_visible() ;
    pub fn set_player_using_melee_dummies(unnamed_1: bool) ;
    pub fn get_player_using_melee_dummies() -> bool ;
    pub fn set_player_using_ranged_dummies(unnamed_1: bool) ;
    pub fn get_player_using_ranged_dummies() -> bool ;
    pub fn set_player_using_will_dummies(unnamed_1: bool) ;
    pub fn get_player_using_will_dummies() -> bool ;
    pub fn set_cheap_head_looking(unnamed_1: bool) ;
    pub fn get_cheap_head_looking() -> bool ;
    pub fn set_quit_tavern_game(unnamed_1: bool) ;
    pub fn get_quit_tavern_game() -> bool ;
    pub fn set_prize_tavern_table(unnamed_1: bool) ;
    pub fn get_prize_tavern_table() -> bool ;
    pub fn set_betting_active(unnamed_1: bool) ;
    pub fn get_betting_active() -> bool ;
    pub fn set_betting_accept(unnamed_1: bool) ;
    pub fn get_betting_accept() -> bool ;
    pub fn set_betting_amount(unnamed_1: c_long) ;
    pub fn get_betting_amount() -> c_long ;
    pub fn set_count_bet_money_down(unnamed_1: bool) ;
    pub fn get_count_bet_money_down() -> bool ;
    pub fn set_spot_the_addition_beaten(unnamed_1: bool) ;
    pub fn get_spot_the_addition_beaten() -> bool ;
    pub fn set_global_targeting_distance_offset(unnamed_1: c_float) ;
    pub fn get_global_targeting_distance_offset() -> c_float ;
    pub fn set_trading_price_mult(unnamed_1: c_float) ;
    pub fn get_trading_price_mult() -> c_float ;
    pub fn set_boasting_enabled(unnamed_1: bool) ;
    pub fn get_boasting_enabled() -> bool ;
    pub fn clear_active_gossip_categories() ;
    pub fn clear_is_gossip_for_player() ;

    // pub fn cancel_using_ability(_: EHeroAbility) ;
    // pub fn set_ability_availability(_: EHeroAbility, _: bool) ;
    // pub fn error(unnamed_1: String, unnamed_2: String, unnamed_3: c_ulong);
    // pub fn trace_message(unnamed_1: String);
    // pub fn start_scripting_entity(
    //     unnamed_1: *const CScriptThing,
    //     unnamed_2: *mut CScriptGameResourceObjectScriptedThingBase,
    //     unnamed_3: EScriptAIPriority,
    // ) -> bool;
    // pub fn is_entity_under_scripted_control(unnamed_1: *const CScriptThing) -> bool;
    // pub fn is_active_thread_terminating(unnamed_2: *const CScriptThing);
    // pub fn is_level_loaded(unnamed_1: String) -> bool;
    // pub fn is_region_loaded(unnamed_1: String) -> bool;
    // pub fn is_region_loaded_and_preloaded(unnamed_1: String) -> bool;
    // pub fn is_region_def_loaded(unnamed_1: String) -> bool;
    // pub fn get_region_name() -> String;
    // pub fn msg_is_level_loaded(unnamed_1: String) -> bool;
    // pub fn msg_is_level_unloaded(unnamed_1: String) -> bool;
    // pub fn msg_on_level_loaded(unnamed_1: *mut CxxList<CCharString>) -> bool ;
    // pub fn msg_on_level_unloaded(_: *mut CxxList<CCharString>) -> bool ;
    // pub fn msg_is_region_loaded(unnamed_1: String) -> bool;
    // pub fn msg_is_region_unloaded(unnamed_1: String) -> bool ;
    // pub fn msg_on_region_loaded(_: &mut String) -> bool ;
    // pub fn msg_on_region_unloaded(unnamed_1: String) -> bool ;
    // pub fn msg_on_region_preunloaded(_: &mut String) -> bool ;
    // pub fn msg_on_quest_completed(_: &mut String) -> bool ;
    // pub fn msg_on_any_quest_completed(_: &mut String) -> bool ;
    // pub fn msg_on_quest_failed(param_1: String) -> bool ;
    // pub fn msg_on_quest_completed_before_screen_shown(_: String) -> bool ;
    // pub fn msg_on_quest_failed_before_screen_shown(_: String) -> bool ;
    // pub fn msg_on_quest_accepted(_: &mut String) -> bool ;
    // pub fn msg_on_feat_accepted(_: *mut c_long) -> bool ;
    // pub fn msg_on_boast_made(_: &mut String) -> bool ;
    // pub fn msg_on_boasts_made(_: *mut CxxVector<CCharString>) -> bool ;
    // pub fn is_quest_start_screen_active(_: &mut String) -> bool ;
    // pub fn msg_on_leaving_quest_start_screen(_: &mut String) -> bool ;
    // pub fn msg_on_expression_performed(_: &mut String) -> bool ;
    // pub fn remove_all_cut_scene_skipped_messages(_: &mut String) ;
    // pub fn msg_on_hero_hair_type_changed(
    //     _: EClothingCoversArea,
    //     _: &mut String,
    // ) -> bool;
    // pub fn msg_on_hero_used_teleporter(_: &mut String) -> bool ;
    // pub fn msg_on_hero_slept(_: *mut bool) -> bool ;
    // pub fn msg_on_hero_cast_spell(_: *mut EHeroAbility) -> bool ;
    // pub fn msg_on_hero_picked_pocket(_: *mut CScriptThing) -> bool ;
    // pub fn msg_on_hero_picked_lock(_: *mut CScriptThing) -> bool ;
    // pub fn msg_on_fishing_game_finished(_: *mut CScriptThing) -> bool ;
    // pub fn msg_on_tavern_game_finished(_: *const CScriptThing) -> bool ;
    // pub fn msg_on_hero_rewarded_with_items_from(_: &mut String) -> bool ;
    // pub fn get_wandering_population_script_def_name_in_current_region(_: &mut String) ;
    // pub fn get_wandering_population_script_def_name_in_region(_: &mut String) ;
    // pub fn is_hero_allowed_henchmen_in_region(_: String) -> bool ;
    // pub fn get_hero() -> *mut CScriptThing ;
    // pub fn get_hero_targeted_thing() -> *mut CScriptThing ;
    // pub fn get_thing_with_script_name(_: String) -> CScriptThing ;
    // pub fn get_thing_with_script_name_2(_: String, _: String) -> CScriptThing ;
    // pub fn get_random_thing_with_script_name(_: String) -> CScriptThing ;
    // pub fn get_all_things_with_script_name(
    //     _: String,
    //     _: *mut CxxVector<CScriptThing>,
    // ) -> c_long;
    // pub fn get_all_creatures_in_area_with_script_name(
    //     _: String,
    //     _: *const C3DVector,
    //     _: c_float,
    //     _: *const CxxVector<CScriptThing>,
    // ) -> c_long;
    // pub fn get_nearest_with_script_name(
    //     _: *const CScriptThing,
    //     _: String,
    // ) -> CScriptThing;
    // pub fn get_furthest_with_script_name(
    //     _: *const CScriptThing,
    //     _: String,
    // ) -> CScriptThing;
    // pub fn get_all_things_with_def_name(
    //     _: String,
    //     _: *mut CxxVector<CScriptThing>,
    // ) -> c_long;
    // pub fn get_all_things_with_def_name_by_distance_from(
    //     _: *const CScriptThing,
    //     _: String,
    //     _: *mut CxxVector<CScriptThing>,
    // ) -> c_long;
    // pub fn get_nearest_with_def_name(
    //     _: *const CScriptThing,
    //     _: String,
    // ) -> CScriptThing;
    // pub fn get_furthest_with_def_name(
    //     _: *const CScriptThing,
    //     _: String,
    // ) -> CScriptThing;
    // pub fn get_thing_with_uid(unnamed_1: c_ulong) -> CScriptThing ;
    // pub fn get_all_creatures_excluding_hero(_: *mut CxxVector<CScriptThing>) -> c_long ;
    // pub fn get_all_things_in_level(
    //     _: String,
    //     _: *mut CxxVector<CScriptThing>,
    // ) -> c_long;
    // pub fn create_creature(
    //     _: String,
    //     _: *const C3DVector,
    //     _: String,
    //     _: bool,
    // ) -> CScriptThing;
    // pub fn create_creature_nearby(
    //     _: String,
    //     _: *const C3DVector,
    //     _: c_float,
    //     _: String,
    //     _: bool,
    // ) -> CScriptThing;
    // pub fn create_creature_on_entity(
    //     _: String,
    //     _: *const CScriptThing,
    //     _: String,
    // ) -> CScriptThing;
    // pub fn turn_creature_into(_: *const CScriptThing, _: String) -> CScriptThing ;
    // pub fn create_object(
    //     _: String,
    //     _: *const C3DVector,
    //     _: c_float,
    //     _: String,
    // ) -> CScriptThing;
    // pub fn create_object_2(
    //     _: String,
    //     _: *const C3DVector,
    //     _: String,
    // ) -> CScriptThing;
    // pub fn create_object_on_entity(
    //     _: String,
    //     _: *const CScriptThing,
    //     _: String,
    // ) -> CScriptThing;
    // pub fn create_effect(
    //     _: String,
    //     _: *const C3DVector,
    //     _: String,
    //     _: c_float,
    //     _: bool,
    //     _: bool,
    // ) -> CScriptThing;
    // pub fn create_effect_2(
    //     _: String,
    //     _: *const CScriptThing,
    //     _: String,
    //     _: String,
    //     _: bool,
    //     _: bool,
    // ) -> CScriptThing;
    // pub fn create_light(
    //     _: *const C3DVector,
    //     _: *const CRGBColour,
    //     _: String,
    //     _: c_float,
    //     _: c_float,
    //     _: bool,
    // ) -> CScriptThing;
    // pub fn create_experience_orb(_: *const C3DVector, _: c_long) -> CScriptThing ;
    // pub fn create_explosion(
    //     _: String,
    //     _: *const C3DVector,
    //     _: *const C3DVector,
    //     _: CCharString,
    // ) -> CScriptThing;
    // pub fn create_physical_barrier(
    //     _: c_float,
    //     _: *const C3DVector,
    //     _: *const C3DVector,
    //     _: CCharString,
    // ) -> CScriptThing;
    // pub fn create_rumble(
    //     _: *const C3DVector,
    //     _: c_float,
    //     _: c_float,
    //     _: CCharString,
    // ) -> CScriptThing;
    // pub fn remove_thing(_: *const CScriptThing, _: bool, _: bool) ;
    // pub fn show_on_screen_message(
    //     _: *const C2DVector,
    //     _: String,
    //     _: *const CRGBColour,
    //     _: String,
    // );
    // pub fn show_on_screen_message_2(
    //     _: String,
    //     _: String,
    //     _: String,
    //     _: c_float,
    // );
    // pub fn show_on_screen_message_3(_: String, _: c_float) ;
    // pub fn add_screen_message(_: String, _: ETextGroupSelectionMethod) ;
    // pub fn add_screen_title_message(_: String, _: c_float, _: bool) ;
    // pub fn give_hero_yes_no_question(
    //     _: String,
    //     _: String,
    //     _: String,
    //     _: String,
    //     _: bool,
    // );
    // pub fn display_game_info(_: String) ;
    // pub fn display_game_info_text(_: *const CWideString) ;
    // pub fn display_tutorial(_: ETutorialCategory) -> bool ;
    // pub fn give_hero_weapon(_: String, _: bool) ;
    // pub fn give_hero_object(_: String, _: c_long, _: bool) ;
    // pub fn set_weapon_as_heros_active_weapon(_: String) ;
    // pub fn give_hero_item(_: *const CScriptThing) ;
    // pub fn give_hero_items_from_container(_: *const CScriptThing, _: bool) -> bool ;
    // pub fn take_object_from_hero(_: String) ;
    // pub fn is_reported_or_unreported_crime_known(_: *const CScriptThing) -> bool ;
    // pub fn confiscate_items_of_type_from_hero(_: String) ;
    // pub fn give_hero_tutorial(_: ETutorialCategory) -> bool ;
    // pub fn make_hero_carry_item_in_hand(
    //     _: *const CScriptThing,
    //     _: bool,
    //     _: bool,
    // );
    // pub fn make_hero_carry_item_in_hand_2(_: String) ;
    // pub fn add_tattoo_to_hero(_: String) ;
    // pub fn give_hero_ability(_: EHeroAbility, _: bool) ;
    // pub fn is_player_z_targeting_thing(_: *const CScriptThing) -> bool ;
    // pub fn is_player_creature_ready_to_fire_projectile_weapon(_: *mut c_float) -> bool ;
    // pub fn set_player_creature_only_target(_: *const CScriptThing) ;
    // pub fn get_hero_morality_category() -> EMorality ;
    // pub fn get_hero_stat_level(_: EHeroTrainableStatType) -> c_long ;
    // pub fn get_hero_stat_max(_: EHeroTrainableStatType) -> c_long ;
    // pub fn get_number_of_items_of_type_in_inventory(_: String) -> c_long ;
    // pub fn is_object_in_things_possession(_: String, _: *const CScriptThing) -> bool ;
    // pub fn is_wearing_clothing_item(_: *const CScriptThing, _: String) -> bool ;
    // pub fn set_hero_as_wearing(_: String) ;
    // pub fn change_hero_hairstyle(_: String) ;
    // pub fn is_wearing_hairstyle(_: *mut CScriptThing, _: String) -> bool ;
    // pub fn is_player_carrying_item_of_type(_: String) -> bool ;
    // pub fn is_entity_wielding_weapon(_: *const CScriptThing) -> bool ;
    // pub fn is_entity_wielding_melee_weapon(_: *const CScriptThing) -> bool ;
    // pub fn is_entity_wielding_ranged_weapon(_: *const CScriptThing) -> bool ;
    // pub fn get_previously_wielded_melee_weapon_name(_: &mut String) -> bool ;
    // pub fn get_previously_wielded_ranged_weapon_name(_: &mut String) -> bool ;
    // pub fn is_entity_able_to_attack(_: *const CScriptThing) -> bool ;
    // pub fn entity_get_thing_in_primary_slot(_: *const CScriptThing) -> CScriptThing ;
    // pub fn give_hero_title(_: String) ;
    // pub fn get_hero_title() -> EHeroTitle ;
    // pub fn entity_set_as_marryable(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_as_able_to_region_follow_when_married(
    //     _: *const CScriptThing,
    //     _: bool,
    // );
    // pub fn entity_force_marriage_to_hero(_: *const CScriptThing, _: bool) ;
    // pub fn is_entity_married_to_hero(_: *const CScriptThing) -> bool ;
    // pub fn is_entity_marriable(_: *const CScriptThing) -> bool ;
    // pub fn give_thing_hero_reward_item(
    //     _: *const CScriptThing,
    //     _: String,
    //     _: String,
    // );
    // pub fn give_thing_item_in_hand(_: *const CScriptThing, _: String, _: bool);
    // pub fn give_thing_item_in_slot(
    //     _: *const CScriptThing,
    //     _: String,
    //     _: String,
    // );
    // pub fn give_hero_expression(_: String, _: c_long, _: bool) ;
    // pub fn hero_has_expression(_: String) -> bool ;
    // pub fn is_hero_performing_expression(_: String) -> bool ;
    // pub fn is_d_pad_button_held_for_expression(_: String) -> bool ;
    // pub fn entity_follow_thing(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    //     _: c_float,
    //     _: bool,
    // );
    // pub fn entity_stop_following(_: *const CScriptThing) ;
    // pub fn get_following_entity_list(
    //     _: *const CScriptThing,
    //     _: *mut CxxVector<CScriptThing>,
    // ) -> bool;
    // pub fn get_perceiving_hero_entity_list(_: *mut CxxVector<CScriptThing>) -> bool ;
    // pub fn get_hero_summoned_creatures_list(_: *mut CxxVector<CScriptThing>) -> bool ;
    // pub fn is_entity_following_hero(_: *const CScriptThing) -> bool ;
    // pub fn entity_set_as_allowed_to_follow_hero(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_as_allowed_to_change_region_following_state(
    //     _: *const CScriptThing,
    //     _: bool,
    // );
    // pub fn entity_set_as_responding_to_follow_and_wait_expressions(
    //     _: *const CScriptThing,
    //     _: bool,
    // );
    // pub fn entity_set_as_mirroring_hero_enemy_relations_while_following(
    //     _: *const CScriptThing,
    //     _: bool,
    // );
    // pub fn entity_teleport_to_hero_position(_: *const CScriptThing) ;
    // pub fn send_entity_event(
    //     _: NEntityEvents::EEventType,
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    //     _: *mut CThing,
    // );
    // pub fn get_water_height_at_position(_: *const C3DVector) -> c_float ;
    // pub fn is_fishing_spot_enabled(_: *const CScriptThing) -> bool ;
    // pub fn disable_fishing_spot(_: *const CScriptThing) ;
    // pub fn update_fish_weight(_: *const CScriptThing) ;
    // pub fn get_best_fish_weight(_: String) -> c_float ;
    // pub fn set_thing_as_killed(_: *const CScriptThing) ;
    // pub fn get_health(_: *const CScriptThing) -> c_float ;
    // pub fn modify_thing_health(_: *const CScriptThing, _: c_float, _: bool) ;
    // pub fn entity_set_max_health(_: *const CScriptThing, _: c_float, _: bool) ;
    // pub fn give_hero_new_quest_objective(_: String, _: c_ulong) ;
    // pub fn add_quest_region(_: String, _: String) ;
    // pub fn set_quest_world_map_offset(_: String, _: *const C2DCoordI) ;
    // pub fn hero_receive_message_from_guild_master(
    //     _: String,
    //     _: String,
    //     _: bool,
    //     _: bool,
    // );
    // pub fn activate_quest(_: String) ;
    // pub fn activate_multiple_quests(_: Vec<String>) ;
    // pub fn activate_quest_without_loading_resources(_: String) ;
    // pub fn activate_multiple_quests_without_loading_resources(_: Vec<String>) ;
    // pub fn deactivate_quest(_: String, _: c_ulong) ;
    // pub fn deactivate_quest_later(_: String, _: c_ulong) ;
    // pub fn prepare_quests_when_final_quest_is_activated(_: String) ;
    // pub fn is_quest_active(_: String) -> bool ;
    // pub fn is_quest_registered(_: String) -> bool ;
    // pub fn is_quest_completed(_: String) -> bool ;
    // pub fn is_quest_failed(_: String) -> bool ;
    // pub fn set_quest_as_completed(
    //     _: String,
    //     _: bool,
    //     _: bool,
    //     _: bool,
    // );
    // pub fn set_quest_as_failed(
    //     _: String,
    //     _: bool,
    //     _: *const CWideString,
    //     _: bool,
    // );
    // pub fn set_quest_as_persistent(_: String, _: bool) ;
    // pub fn get_exclusive_quest_script_name() -> String ;
    // pub fn add_quest_card(_: String, _: String, _: bool, _: bool) ;
    // pub fn remove_quest_card_from_guild(_: String) ;
    // pub fn remove_quest_card_from_hero(_: String) ;
    // pub fn give_hero_quest_card_directly(_: String, _: String, _: bool) ;
    // pub fn set_quest_card_objective(
    //     _: String,
    //     _: String,
    //     _: String,
    //     _: String,
    // );
    // pub fn set_quest_card_gold_reward(_: String, _: c_long) ;
    // pub fn set_quest_card_renown_reward(_: String, _: c_long) ;
    // pub fn get_all_active_quest_info(
    //     _: *mut CxxVector<CCharString>,
    //     _: *mut CxxVector<CCharString>,
    // );
    // pub fn add_feat_card(unnamed_1: c_long, _: String, _: String) ;
    // pub fn add_boast(
    //     _: String,
    //     _: c_long,
    //     _: c_long,
    //     _: c_long,
    //     _: bool,
    //     _: String,
    //     _: c_long,
    // );
    // pub fn remove_boast(unnamed_1: c_long, _: String) ;
    // pub fn set_boast_as_failed(unnamed_1: c_long, _: String) ;
    // pub fn set_boast_as_completed(unnamed_1: c_long, _: String) ;
    // pub fn is_boast_taken(unnamed_1: c_long, _: String) -> bool ;
    // pub fn add_log_book_entry(
    //     _: *const CWideString,
    //     _: *const CWideString,
    //     _: *const CWideString,
    //     _: ECategory,
    // );
    // pub fn kick_off_quest_start_screen(_: String, _: bool, _: bool) ;
    // pub fn kick_off_credits_screen(_: &mut String) ;
    // pub fn set_preferred_quick_access_item(_: String, _: c_long, _: c_long) ;
    // pub fn get_death_recovery_marker_name() -> CCharString ;
    // pub fn set_death_recovery_marker_name(_: String) ;
    // pub fn get_most_recent_valid_used_target() -> CScriptThing ;
    // pub fn get_most_recent_valid_used_target_name() -> CCharString ;
    // pub fn set_quest_info_name(_: *const c_char) ;
    // pub fn set_quest_info_text(_: *const c_char) ;
    // pub fn add_quest_info_bar(
    //     _: c_float,
    //     _: c_float,
    //     _: *const CRGBColour,
    //     _: *const CRGBColour,
    //     _: String,
    //     _: String,
    //     _: c_float,
    // ) -> c_long;

    // pub fn add_quest_info_bar_health(
    //     _: *const CScriptThing,
    //     _: *const CRGBColour,
    //     _: String,
    //     _: c_float,
    // ) -> c_long;
    // pub fn add_quest_info_timer(
    //     _: *const CTimer,
    //     _: String,
    //     _: c_float,
    // ) -> c_long;
    // pub fn add_quest_info_counter(_: String, _: c_long, _: c_float) -> c_long ;
    // pub fn add_quest_info_counter_2(
    //     _: *const CCounter,
    //     _: String,
    //     _: c_float,
    // ) -> c_long;
    // pub fn add_quest_info_counter_list(
    //     _: String,
    //     _: c_long,
    //     _: c_float,
    // ) -> c_long;
    // pub fn add_quest_info_tick(_: EGameAction, _: bool, _: c_float) -> c_long ;
    // pub fn add_quest_info_tick_2(_: String, _: bool, _: c_float) -> c_long ;
    // pub fn update_quest_info_bar(
    //     _: c_long,
    //     _: c_float,
    //     _: c_float,
    //     _: c_float,
    // );
    // pub fn change_quest_info_bar_colour(
    //     _: c_long,
    //     _: *const CRGBColour,
    //     _: *const CRGBColour,
    // );
    // pub fn update_quest_info_counter(unnamed_1: c_long, unnamed_2: c_long, unnamed_3: c_long);
    // pub fn update_quest_info_counter_list(unnamed_1: c_long, unnamed_2: c_long, unnamed_3: c_long);
    // pub fn display_mini_game_info(_: bool, _: EMiniGameType) ;
    // pub fn is_entity_pick_pocketable(_: *const CScriptThing) -> bool ;
    // pub fn is_entity_pick_lockable(_: *const CScriptThing) -> bool ;
    // pub fn is_entity_stealable(_: *const CScriptThing) -> bool ;
    // pub fn entity_set_as_pick_pocketed(_: *const CScriptThing) ;
    // pub fn entity_set_as_pick_locked(_: *const CScriptThing) ;
    // pub fn entity_set_as_stolen(_: *const CScriptThing) ;
    // pub fn mini_map_add_marker(_: *const CScriptThing, _: String) ;
    // pub fn mini_map_set_marker_graphic(_: *const CScriptThing, _: String) ;
    // pub fn mini_map_remove_marker(_: *const CScriptThing) ;
    // pub fn mini_map_allow_route_between_regions(_: String, _: String, _: bool);
    // pub fn entity_set_as_hidden_on_mini_map(_: *const CScriptThing, _: bool) ;
    // pub fn text_entry_exists(_: String) -> bool ;
    // pub fn get_valid_text_entry_name_with_attitude(
    //     _: *const CScriptThing,
    //     _: String,
    // ) -> CCharString;
    // pub fn set_thing_has_information(
    //     _: *const CScriptThing,
    //     _: bool,
    //     _: bool,
    //     _: bool,
    // );
    // pub fn clear_thing_has_information(_: *const CScriptThing) ;
    // pub fn entity_set_will_be_using_narrator(_: *const CScriptThing, _: String) ;
    // pub fn entity_reset_as_pure_ai_narrator(_: *const CScriptThing) ;
    // pub fn add_new_conversation(
    //     _: *const CScriptThing,
    //     _: bool,
    //     _: bool,
    // ) -> c_long;
    // pub fn add_person_to_conversation(unnamed_1: c_long, _: *const CScriptThing) ;
    // pub fn add_line_to_conversation(
    //     _: c_long,
    //     _: String,
    //     _: bool,
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // );
    // pub fn play_avi_movie(_: String) ;
    // pub fn start_movie_sequence(_: String, _: *mut CScriptGameResourceObjectMovieBase);
    // pub fn fade_screen_out(unnamed_1: c_float, _: c_float, _: CRGBColour) -> bool ;
    // pub fn set_cutscene_action_mode(_: bool, _: String) ;
    // pub fn remove_dead_creature(_: String) ;
    // pub fn camera_circle_around_thing(
    //     _: *const CScriptThing,
    //     _: *const C3DVector,
    //     _: c_float,
    // );
    // pub fn camera_circle_around_pos(
    //     _: *const C3DVector,
    //     _: *const C3DVector,
    //     _: c_float,
    // );
    // pub fn camera_move_to_pos_and_look_at_pos(
    //     _: *const C3DVector,
    //     _: *const C3DVector,
    //     _: c_float,
    // );
    // pub fn camera_move_to_pos_and_look_at_thing(
    //     _: *const C3DVector,
    //     _: *const CScriptThing,
    //     _: c_float,
    // );
    // pub fn camera_move_between_looking_at(
    //     _: *const C3DVector,
    //     _: *const C3DVector,
    //     _: *const CScriptThing,
    //     _: c_float,
    //     _: c_float,
    // );
    // pub fn camera_move_between_looking_at_2(
    //     _: *const C3DVector,
    //     _: *const C3DVector,
    //     _: *const C3DVector,
    //     _: c_float,
    //     _: c_float,
    // );
    // pub fn camera_move_between_look_from_and_look_to(
    //     _: *const C3DVector,
    //     _: *const C3DVector,
    //     _: *const C3DVector,
    //     _: *const C3DVector,
    //     _: c_float,
    // );
    // pub fn camera_use_camera_point(
    //     _: *const CScriptThing,
    //     _: *const C3DVector,
    //     _: *const CRightHandedSet,
    //     _: c_float,
    //     _: c_long,
    //     _: c_long,
    // );
    // pub fn camera_use_camera_point_2(
    //     _: String,
    //     _: *const C3DVector,
    //     _: *const CRightHandedSet,
    //     _: c_float,
    //     _: c_long,
    //     _: c_long,
    // );
    // pub fn camera_use_camera_point_3(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    //     _: c_float,
    //     _: c_long,
    //     _: c_long,
    // );
    // pub fn camera_use_camera_point_4(
    //     _: String,
    //     _: *const CScriptThing,
    //     _: c_float,
    //     _: c_long,
    //     _: c_long,
    // );
    // pub fn camera_do_conversation(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    //     _: ECameraOp,
    //     _: bool,
    // );
    // pub fn camera_use_screen_effect(unnamed_1: c_float, unnamed_2: c_float, unnamed_3: c_float) ;
    // pub fn is_camera_pos_on_screen(_: *const C3DVector) -> bool ;
    // pub fn camera_earthquake_intensity_at_pos(
    //     _: *const C3DVector,
    //     _: c_float,
    //     _: c_float,
    // );
    // pub fn open_door(_: *const CScriptThing) ;
    // pub fn close_door(_: *const CScriptThing) ;
    // pub fn open_house_doors(_: *const CScriptThing) ;
    // pub fn close_house_doors(_: *const CScriptThing) ;
    // pub fn jam_door(_: *const CScriptThing) ;
    // pub fn set_door_trigger_type(_: *const CScriptThing, _: EDoorTriggerType) ;
    // pub fn override_automatic_house_locking(_: *const CScriptThing, _: bool) ;
    // pub fn set_house_owned_by_player(_: *const CScriptThing, _: bool, _: bool);
    // pub fn set_buyable_house_as_scripted(_: *const CScriptThing, _: bool) ;
    // pub fn is_chest_open(_: *const CScriptThing) -> bool ;
    // pub fn open_chest(_: *const CScriptThing, _: bool) -> bool ;
    // pub fn close_chest(_: *const CScriptThing) ;
    // pub fn get_number_of_keys_needed_to_unlock_chest(
    //     _: *const CScriptThing,
    //     _: &mut String,
    // ) -> c_long;
    // pub fn display_locked_chest_message(_: *const CScriptThing) ;
    // pub fn set_trophy_as_mountable(_: *const CScriptThing, _: bool) ;
    // pub fn set_village_limbo(_: *const CScriptThing, _: bool) ;
    // pub fn set_creature_not_reload(_: *const CScriptThing) ;
    // pub fn is_sleeping_time(_: *const CScriptThing) -> bool ;
    // pub fn enable_guards(_: *const CScriptThing, _: bool) ;
    // pub fn enable_villager_def_types(
    //     _: *const CScriptThing,
    //     _: bool,
    //     _: String,
    // );
    // pub fn try_to_respawn_def_named(
    //     _: *const CScriptThing,
    //     _: String,
    //     _: *const C3DVector,
    // ) -> CScriptThing;
    // pub fn clear_hero_enemy_of_guards(_: *const CScriptThing) ;
    // pub fn set_thing_as_usable(_: *const CScriptThing, _: bool) ;
    // pub fn set_thing_home_building(_: *const CScriptThing, _: *const CScriptThing) ;
    // pub fn set_village_attitude(_: *const CScriptThing, _: EScriptVillageAttitude) ;
    // pub fn add_crime_committed(
    //     _: *const CScriptThing,
    //     _: ECrime,
    //     _: bool,
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    //     _: EOpinionPostDeedType,
    // );
    // pub fn give_thing_best_enemy_target(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // );
    // pub fn clear_thing_best_enemy_target(_: *const CScriptThing) ;
    // pub fn entity_set_in_limbo(_: *const CScriptThing, _: bool, _: bool) ;
    // pub fn is_entity_in_limbo(_: *const CScriptThing) -> bool ;
    // pub fn entity_get_shot_strike_pos(
    //     _: *const CScriptThing,
    //     _: *mut C3DVector,
    // ) -> bool;
    // pub fn entity_set_negate_all_hits(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_evade_all_hits(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_able_to_be_engaged_in_combat(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_always_block_attacks_from_thing(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    //     _: bool,
    // );
    // pub fn entity_set_attack_thing_immediately(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    //     _: bool,
    //     _: bool,
    // );
    // pub fn entity_set_combat_type(_: *const CScriptThing, _: String) ;
    // pub fn entity_reset_combat_type_to_default(_: *const CScriptThing) ;
    // pub fn entity_set_max_number_of_attackers(_: *const CScriptThing, _: c_long) ;
    // pub fn entity_clear_max_number_of_attackers(_: *const CScriptThing) ;
    // pub fn entity_attach_to_script(_: *const CScriptThing, _: String) ;
    // pub fn entity_set_combat_ability(_: *const CScriptThing, _: c_float) ;
    // pub fn entity_set_ranged_target(_: *const CScriptThing, _: *const CScriptThing) ;
    // pub fn entity_clear_ranged_target(_: *const CScriptThing) ;
    // pub fn entity_set_targetable(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_targeting_type(_: *const CScriptThing, _: ETargetingType) ;
    // pub fn entity_set_targeting_valid_target_without_los(
    //     _: *const CScriptThing,
    //     _: bool,
    // );
    // pub fn entity_teleport_to_thing(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    //     _: bool,
    // );
    // pub fn entity_teleport_to_position(
    //     _: *const CScriptThing,
    //     _: *const C3DVector,
    //     _: c_float,
    //     _: bool,
    //     _: bool,
    // );
    // pub fn entity_set_facing_angle(
    //     _: *const CScriptThing,
    //     _: c_float,
    //     _: bool,
    // );
    // pub fn entity_set_facing_angle_towards_thing(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    //     _: bool,
    // );
    // pub fn entity_set_perception_variables(
    //     _: *const CScriptThing,
    //     _: c_float,
    //     _: c_float,
    //     _: c_float,
    // );
    // pub fn set_thing_persistent(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_thing_as_wanting_money(_: *const CScriptThing, _: c_long) ;
    // pub fn entity_set_appearance_morph_seed(_: *const CScriptThing, _: c_long) ;
    // pub fn set_entity_as_region_following(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    //     _: bool,
    // );
    // pub fn set_entity_as_following_hero_through_teleporters(
    //     _: *const CScriptThing,
    //     _: bool,
    // );
    // pub fn entity_set_appearance_seed(_: *const CScriptThing, _: c_ulong) ;
    // pub fn entity_get_appearance_seed(_: *const CScriptThing, _: *mut c_ulong) ;
    // pub fn entity_set_as_for_sale(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_stock_item_price(_: *const CScriptThing, _: c_long) ;
    // pub fn entity_get_stock_item_price(_: *const CScriptThing) -> c_long ;
    // pub fn entity_play_object_animation(
    //     _: *const CScriptThing,
    //     _: String,
    //     _: bool,
    // );
    // pub fn entity_set_max_running_speed(_: *const CScriptThing, _: c_float) ;
    // pub fn entity_set_max_walking_speed(_: *const CScriptThing, _: c_float) ;
    // pub fn entity_reset_max_running_speed(_: *const CScriptThing) ;
    // pub fn entity_reset_max_walking_speed(_: *const CScriptThing) ;
    // pub fn entity_attach_to_village(_: *const CScriptThing, _: *const CScriptThing) ;
    // pub fn entity_set_as_sitting_on_floor(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_as_scared(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_as_drunk(_: *mut CScriptThing, _: bool) ;
    // pub fn entity_set_as_having_bound_hands(_: *mut CScriptThing, _: bool) ;
    // pub fn entity_set_as_remove_all_movement_blocking_modes(_: *mut CScriptThing) ;
    // pub fn entity_force_to_look_at_thing(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // );
    // pub fn entity_force_to_look_at_camera(_: *const CScriptThing, _: bool) ;
    // pub fn entity_force_to_look_at_nothing(_: *const CScriptThing) ;
    // pub fn entity_reset_force_to_look_at(_: *const CScriptThing) ;
    // pub fn entity_set_shot_accuracy_percentage(_: *const CScriptThing, _: c_long) ;
    // pub fn entity_get_standing_on_thing(_: *const CScriptThing) -> CScriptThing ;
    // pub fn entity_get_standing_inside_building(_: *const CScriptThing) -> CScriptThing ;
    // pub fn entity_drop_generic_box(_: *const CScriptThing) ;
    // pub fn entity_sheathe_weapons(_: *const CScriptThing, _: bool) ;
    // pub fn entity_unsheathe_weapons(_: *const CScriptThing, _: bool) ;
    // pub fn entity_unsheathe_melee_weapon(_: *const CScriptThing, _: bool) ;
    // pub fn entity_unsheathe_ranged_weapon(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_alpha(_: *const CScriptThing, _: c_float, _: bool) ;
    // pub fn entity_set_as_drawable(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_cutscene_behaviour(
    //     _: *const CScriptThing,
    //     _: ECutsceneBehaviour,
    // );
    // pub fn entity_get_sex(_: *const CScriptThing) -> ESex ;
    // pub fn entity_set_as_able_to_walk_through_solid_objects(
    //     _: *const CScriptThing,
    //     _: bool,
    // );
    // pub fn entity_set_as_respond_to_hit(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_as_damageable(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_as_killable(_: *const CScriptThing, _: bool, _: bool) ;
    // pub fn entity_set_as_locked(_: *const CScriptThing, _: bool) ;
    // pub fn entity_decapitate(_: *const CScriptThing) ;
    // pub fn entity_give_gold(_: *const CScriptThing, _: c_long) ;
    // pub fn entity_set_allow_boss_phase_changes(_: *const CScriptThing, _: bool) ;
    // pub fn entity_get_boss_phase(_: *const CScriptThing) -> c_long ;
    // pub fn entity_set_boss_phase(_: *const CScriptThing, _: c_long) ;
    // pub fn entity_reset_creature_mode(_: *const CScriptThing) ;
    // pub fn entity_set_as_receiving_events(_: *const CScriptThing, _: bool) -> bool ;
    // pub fn entity_set_as_to_add_to_combo_multiplier_when_hit(
    //     _: *const CScriptThing,
    //     _: bool,
    // );
    // pub fn entity_set_as_to_add_to_stat_changes_when_hit(
    //     _: *const CScriptThing,
    //     _: bool,
    // );
    // pub fn entity_leave_combat_stance(_: *const CScriptThing) ;
    // pub fn entity_set_as_use_movement_in_actions(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_as_displaying_emote_icon(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_as_collidable_to_things(_: *const CScriptThing, _: bool) ;
    // pub fn entity_enable_gravity(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_light_as_on(_: *const CScriptThing, _: bool) ;
    // pub fn entity_fade_out(_: *const CScriptThing, _: c_float) ;
    // pub fn entity_fade_in(_: *const CScriptThing, _: c_float) ;
    // pub fn entity_begin_loading_animation(_: *const CScriptThing, _: String) ;
    // pub fn entity_begin_loading_basic_animations(_: *const CScriptThing) ;
    // pub fn entity_cast_force_push(_: *const CScriptThing, _: bool) -> bool ;
    // pub fn entity_cast_lightning_at_target(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // );
    // pub fn begin_loading_mesh(_: String) ;
    // pub fn entity_will_teleport_to_area(
    //     _: *const CScriptThing,
    //     _: C3DVector,
    //     _: c_float,
    //     _: c_float,
    // ) -> bool;
    // pub fn entity_start_screamer_super_attack_thing(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // );
    // pub fn entity_end_screamer_super_attack_thing(_: *const CScriptThing) ;
    // pub fn set_light_colour(_: *const CScriptThing, _: *const CRGBColour) ;
    // pub fn creature_generator_set_family(_: *const CScriptThing, _: String) ;
    // pub fn creature_generator_trigger(_: *const CScriptThing) ;
    // pub fn creature_generator_set_always_create_creatures_on_trigger(
    //     _: *const CScriptThing,
    //     _: bool,
    // );
    // pub fn creature_generator_is_depleted(_: *const CScriptThing) -> bool ;
    // pub fn creature_generator_is_destroyed(_: *const CScriptThing) -> bool ;
    // pub fn creature_generator_set_generated_creature_script_name(
    //     _: *const CScriptThing,
    //     _: String,
    // );
    // pub fn creature_generator_set_num_triggers(_: *const CScriptThing, _: c_long) ;
    // pub fn creature_generator_get_num_generated_creatures(_: *const CScriptThing) -> c_long ;
    // pub fn creature_generator_are_all_creatures_alive(_: *const CScriptThing) -> bool ;
    // pub fn creature_generator_add_triggerer(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // );
    // pub fn creature_generator_remove_triggerer(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // );
    // pub fn set_creature_generator_enabled(_: *const CScriptThing, _: bool) ;
    // pub fn set_creature_generators_enabled(_: String, _: bool) ;
    // pub fn set_creature_generators_enabled_during_script(_: String, _: bool) ;
    // pub fn set_creature_generators_creature_group_as_enabled(
    //     _: ECreatureGroup,
    //     _: bool,
    // );
    // pub fn is_creature_generation_enabled_for_region(_: String) -> bool ;
    // pub fn is_creature_flying(_: *const CScriptThing) -> bool ;
    // pub fn set_teleporter_as_active(_: *const CScriptThing, _: bool) ;
    // pub fn is_teleporter_active(_: *const CScriptThing) -> bool ;
    // pub fn set_region_exit_as_active(_: *const CScriptThing, _: bool) ;
    // pub fn set_region_entrance_as_active(_: *const CScriptThing, _: bool) ;
    // pub fn get_nearest_enabled_digging_spot(_: *const CScriptThing) -> CScriptThing ;
    // pub fn is_digging_spot_enabled(_: *const CScriptThing) -> bool ;
    // pub fn is_digging_spot_hidden(_: *const CScriptThing) -> bool ;
    // pub fn set_digging_spot_as_hidden(_: *const CScriptThing, _: bool) ;
    // pub fn check_for_camera_message(_: String) -> bool ;
    // pub fn wait_for_camera_message(_: String) ;
    // pub fn set_thing_as_conscious(_: *const CScriptThing, _: bool, _: String) ;
    // pub fn set_fire_to_thing(_: *const CScriptThing) ;
    // pub fn extinguish_fires_on_thing(_: *const CScriptThing) ;
    // pub fn is_thing_on_fire(_: *const CScriptThing) -> bool ;
    // pub fn add_item_to_container(_: *const CScriptThing, _: String) ;
    // pub fn remove_item_from_container(_: *const CScriptThing, _: String) ;
    // pub fn entity_set_death_container_as_enabled(_: *const CScriptThing, _: bool) ;
    // pub fn get_item_def_names_from_container(
    //     _: *const CScriptThing,
    //     _: *mut CxxVector<CCharString>,
    // );
    // pub fn set_creature_brain(_: *const CScriptThing, _: String) ;
    // pub fn entity_set_stategroup_enabled(
    //     _: *const CScriptThing,
    //     _: String,
    //     _: bool,
    // );
    // pub fn entity_set_all_stategroups_enabled(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_combat_enabled(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_sleep_enabled(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_opinion_reactions_enabled(_: *const CScriptThing, _: bool) ;
    // pub fn entity_set_deed_reactions_enabled(_: *const CScriptThing, _: bool) ;
    // pub fn debug_get_all_text_entries_for_targeted_thing(_: *mut CxxSet<c_ulong>) ;
    // pub fn entity_set_thing_as_enemy_of_thing(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // );
    // pub fn entity_unset_thing_as_enemy_of_thing(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // );
    // pub fn entity_set_thing_as_ally_of_thing(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // );
    // pub fn entity_unset_thing_as_ally_of_thing(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // );
    // pub fn entity_set_in_faction(_: *const CScriptThing, _: String) ;
    // pub fn set_faction_as_allied_to_faction(_: String, _: String) ;
    // pub fn set_faction_as_neutral_to_faction(_: String, _: String) ;
    // pub fn set_faction_as_enemy_to_faction(_: String, _: String) ;
    // pub fn are_entities_enemies(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // ) -> bool;
    // pub fn get_next_in_opinion_attitude_graph(_: EOpinionAttitudeType) -> EOpinionAttitudeType;
    // pub fn get_opinion_attitude_as_string(
    //     _: EOpinionAttitudeType,
    //     _: &mut String,
    // );
    // pub fn entity_get_opinion_attitude_to_player(
    //     _: *const CScriptThing,
    // ) -> EOpinionAttitudeType;
    // pub fn entity_get_opinion_attitude_to_player_as_string(
    //     _: *const CScriptThing,
    //     _: &mut String,
    // );
    // pub fn entity_get_opinion_of_player(
    //     _: *const CScriptThing,
    //     _: EOpinion,
    // ) -> c_float;
    // pub fn entity_set_opinion_reaction_mask(_: *const CScriptThing, _: String) ;
    // pub fn entity_set_opinion_reaction_mask_2(_: *const CScriptThing, _: c_long) ;
    // pub fn entity_set_opinion_deed_mask(_: *const CScriptThing, _: String) ;
    // pub fn entity_set_opinion_deed_mask_2(_: *const CScriptThing, _: c_long) ;
    // pub fn entity_set_opinion_deed_type_enabled(
    //     _: *const CScriptThing,
    //     _: EOpinionDeedType,
    //     _: bool,
    // );
    // pub fn entity_set_opinion_attitude_enabled(
    //     _: *const CScriptThing,
    //     _: EOpinionAttitudeType,
    //     _: bool,
    // );
    // pub fn entity_set_opinion_reaction_enabled(
    //     _: *const CScriptThing,
    //     _: EOpinionReactionType,
    //     _: bool,
    // );
    // pub fn entity_set_personality_override(_: *const CScriptThing, _: c_long) ;
    // pub fn entity_set_personality_override_2(_: *const CScriptThing, _: String) ;
    // pub fn entity_clear_personality_override(_: *const CScriptThing) ;
    // pub fn entity_set_as_opinion_source(_: *const CScriptThing, _: c_long) ;
    // pub fn entity_set_as_opinion_source_2(_: *const CScriptThing, _: String) ;
    // pub fn entity_unset_as_opinion_source(_: *const CScriptThing, _: bool) ;
    // pub fn opinion_source_set_as_exclusive(_: *const CScriptThing, _: bool) ;
    // pub fn opinion_source_set_as_attention_grabbing(_: *const CScriptThing, _: bool) ;
    // pub fn entity_post_opinion_deed_to_all(
    //     _: *const CScriptThing,
    //     _: EOpinionDeedType,
    // );
    // pub fn entity_post_opinion_deed_to_recipient(
    //     _: *const CScriptThing,
    //     _: EOpinionDeedType,
    //     _: *const CScriptThing,
    // );
    // pub fn entity_post_opinion_deed_to_recipient_village(
    //     _: *const CScriptThing,
    //     _: EOpinionDeedType,
    //     _: *const CScriptThing,
    // );
    // pub fn entity_post_opinion_deed_keep_searching_for_witnesses(
    //     _: *const CScriptThing,
    //     _: EOpinionDeedType,
    //     _: *const CScriptThing,
    // ) -> c_long;
    // pub fn remove_opinion_deed_still_searching_for_witnesses(
    //     _: *const CScriptThing,
    //     _: c_long,
    // );
    // pub fn can_thing_be_seen_by_other_thing(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // ) -> bool;
    // pub fn can_thing_be_nearly_seen_by_other_thing(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // ) -> bool;
    // pub fn can_thing_be_smelled_by_other_thing(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // ) -> bool;
    // pub fn can_thing_be_heard_by_other_thing(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // ) -> bool;
    // pub fn is_thing_aware_of_other_thing_in_any_way(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // ) -> bool;
    // pub fn entity_set_as_aware_of_thing(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    // );
    // pub fn entity_set_sound_radius(_: *const CScriptThing, _: c_float) ;
    // pub fn entity_set_smell_radius(_: *const CScriptThing, _: c_float) ;
    // pub fn entity_set_sight_radius(_: *const CScriptThing, _: c_float) ;
    // pub fn entity_set_extended_sight_radius(_: *const CScriptThing, _: c_float) ;
    // pub fn entity_set_give_up_chase_radius(_: *const CScriptThing, _: c_float) ;
    // pub fn entity_get_hearing_radius(_: *const CScriptThing) -> c_float ;
    // pub fn manually_trigger_trap(_: *const CScriptThing) -> bool ;
    // pub fn manually_reset_trap(_: *const CScriptThing) -> bool ;
    // pub fn set_time_as_stopped(_: bool, _: *mut c_long) ;
    // pub fn get_active_quest_name() -> CCharString ;
    // pub fn transition_to_theme(_: String, _: c_float) ;
    // pub fn transition_to_theme_all_internals(_: String, _: c_float) ;
    // pub fn transition_to_theme_externals(_: String, _: c_float) ;
    // pub fn set_environment_theme_weight_all_channels(_: String, _: c_float) ;
    // pub fn set_environment_theme_weight_all_internals(_: String, _: c_float) ;
    // pub fn set_environment_theme_weight_externals(_: String, _: c_float) ;
    // pub fn set_sound_themes_as_enabled_for_region(_: String, _: bool) ;
    // pub fn radial_blur_fade_to(
    //     _: c_float,
    //     _: c_float,
    //     _: c_float,
    //     _: c_float,
    //     _: c_float,
    //     _: c_float,
    //     _: c_float,
    // ) -> *mut c_void
    // ;
    // pub fn radial_blur_fade_to_2(
    //     _: c_float,
    //     _: C3DVector,
    //     _: c_float,
    //     _: C3DVector,
    //     _: c_float,
    //     _: c_float,
    //     _: c_float,
    // ) -> *mut c_void;
    // pub fn radial_blur_fade_out(unnamed_1: c_float, _: *mut c_void) ;
    // pub fn radial_blur_set_center_world_pos(_: *mut c_void, _: *const C3DVector) ;
    // pub fn displacement_monochrome_effect_colour_fade_to(
    //     _: c_float,
    //     _: *const CRGBFloatColour,
    // ) -> c_void;
    // pub fn displacement_monochrome_effect_colour_fade_out(unnamed_1: c_float, _: *mut c_void) ;
    // pub fn screen_filter_fade_to(
    //     _: c_float,
    //     _: c_float,
    //     _: c_float,
    //     _: c_float,
    //     _: c_float,
    //     _: *const CRGBFloatColour,
    //     _: *const CxxVector<CScreenFilterSThingByPass>,
    // ) -> c_void;
    // pub fn screen_filter_fade_out(unnamed_1: c_float, _: *mut c_void) ;
    // pub fn set_thing_and_carried_items_not_affected_by_screen_filter(
    //     _: *mut CScriptThing,
    //     _: *mut c_void,
    // );
    // pub fn un_set_thing_and_carried_items_not_affected_by_screen_filter(_: *mut CScriptThing) ;
    // pub fn is_gift_romantic(_: String) -> bool ;
    // pub fn is_gift_friendly(_: String) -> bool ;
    // pub fn is_gift_offensive(_: String) -> bool ;
    // pub fn is_thing_a_bed(_: *const CScriptThing) -> bool ;
    // pub fn is_thing_a_chest(_: *const CScriptThing) -> bool ;
    // pub fn is_thing_a_door(_: *const CScriptThing) -> bool ;
    // pub fn is_thing_smashable(_: *const CScriptThing) -> bool ;
    // pub fn is_thing_searchable(_: *const CScriptThing) -> bool ;
    // pub fn apply_script_brush(_: String) ;
    // pub fn play_criteria_sound_on_thing(_: *const CScriptThing, _: String) -> c_ulong ;
    // pub fn play_sound_on_thing(_: *const CScriptThing, _: String) -> c_ulong ;
    // pub fn play_sound_at_pos(_: *const C3DVector, _: String) -> c_ulong ;
    // pub fn play_2d_sound(_: String) -> c_ulong ;
    // pub fn override_music(_: EMusicSetType, _: bool, _: bool) ;
    // pub fn cache_music_set(_: EMusicSetType) ;
    // pub fn set_save_game_marker_pos(_: *const C3DVector) ;
    // pub fn set_guild_seal_recall_location(_: *const C3DVector, _: c_float) ;
    // pub fn get_guild_seal_recall_pos() -> C3DVector ;
    // pub fn set_readable_object_text(_: *const CScriptThing, _: *const CWideString) ;
    // pub fn set_readable_object_text_tag(_: *const CScriptThing, _: String) ;
    // pub fn get_formatted_string(
    //     _: String,
    //     _: *const CxxVector<CWideString>,
    // ) -> CWideString;
    // pub fn get_text_string(_: String) -> CWideString ;
    // pub fn add_rumour_category(_: String) ;
    // pub fn add_new_rumour_to_category(_: String, _: String) ;
    // pub fn remove_rumour_category(_: String) ;
    // pub fn set_category_activity(_: String, _: bool) ;
    // pub fn add_gossip_village(_: String, _: String) ;
    // pub fn add_gossip_faction_to_category(_: String, _: String) ;
    // pub fn set_is_gossip_for_player(_: CCharString, _: bool) ;
    // pub fn set_is_gossip_for_player_2(_: String, _: bool) ;
    // pub fn get_steal_duration(_: *const CScriptThing) -> c_long ;
    // pub fn set_useable_by_hero(_: *const CScriptThing, _: bool) ;
    // pub fn set_owned_by_hero(_: *const CScriptThing, _: bool) ;
    // pub fn set_tavern_table_available_for_use(_: *const CScriptThing, _: bool) ;
    // pub fn set_is_thing_turncoatable(_: *const CScriptThing, _: bool) ;
    // pub fn set_is_thing_force_pushable(_: *const CScriptThing, _: bool) ;
    // pub fn set_is_thing_lightningable(_: *const CScriptThing, _: bool) ;
    // pub fn set_is_thing_epic_spellable(_: *const CScriptThing, _: bool) ;
    // pub fn is_thing_turncoated(_: *const CScriptThing) -> bool ;
    // pub fn add_creature_scripted_mode(_: *const CScriptThing, _: String) ;
    // pub fn remove_creature_scripted_mode(_: *const CScriptThing) ;
    // pub fn get_sleeping_position_and_orientation_from_bed(
    //     _: *const CScriptThing,
    //     _: *const CScriptThing,
    //     _: *mut C3DVector,
    //     _: *mut C3DVector,
    // ) -> bool;
    // pub fn set_bed_availability(_: *const CScriptThing, _: bool) ;
    // pub fn repopulate_village(_: *const CScriptThing) ;
    // pub fn smash_all_windows_within_radius_of_point(_: *const C3DVector, _: c_float) ;
    // pub fn set_residency(_: *const CScriptThing, _: bool) -> CScriptThing ;
    // pub fn set_thanking_phrase(_: CScriptThing, _: c_ulong) ;
    // pub fn get_thanking_phrase(_: CScriptThing) -> c_ulong ;
    // pub fn reset_thanking_phrase(_: CScriptThing) ;
    // pub fn set_ignoring_phrase(_: CScriptThing, _: c_ulong) ;
    // pub fn get_ignoring_phrase(_: CScriptThing) -> c_ulong ;
    // pub fn reset_ignoring_phrase(_: CScriptThing) ;
    // pub fn set_wander_centre_point(_: CScriptThing, _: C3DVector) ;
    // pub fn get_wander_centre_point(_: CScriptThing) -> C3DVector ;
    // pub fn reset_wander_centre_point(_: CScriptThing) ;
    // pub fn set_wander_min_distance(_: CScriptThing, _: c_float) ;
    // pub fn get_wander_min_distance(_: CScriptThing) -> c_float ;
    // pub fn reset_wander_min_distance(_: CScriptThing) ;
    // pub fn set_wander_max_distance(_: CScriptThing, _: c_float) ;
    // pub fn get_wander_max_distance(_: CScriptThing) -> c_float ;
    // pub fn reset_wander_max_distance(_: CScriptThing) ;
    // pub fn set_gossip_counter(_: CScriptThing, _: c_long) ;
    // pub fn get_gossip_counter(_: CScriptThing) -> c_long ;
    // pub fn reset_gossip_counter(_: CScriptThing) ;
    // pub fn set_max_gossip_phrase(_: CScriptThing, _: c_long) ;
    // pub fn get_max_gossip_phrase(_: CScriptThing) -> c_long ;
    // pub fn reset_max_gossip_phrase(_: CScriptThing) ;
    // pub fn set_warning_phrase(_: CScriptThing, _: c_ulong) ;
    // pub fn get_warning_phrase(_: CScriptThing) -> c_ulong ;
    // pub fn reset_warning_phrase(_: CScriptThing) ;
    // pub fn set_beer_request_phrase(_: CScriptThing, _: c_ulong) ;
    // pub fn get_beer_request_phrase(_: CScriptThing) -> c_ulong ;
    // pub fn reset_beer_request_phrase(_: CScriptThing) ;
    // pub fn set_scripting_state_group(_: CScriptThing, _: EScriptingStateGroups) ;
    // pub fn get_scripting_state_group(_: CScriptThing) -> EScriptingStateGroups ;
    // pub fn reset_scripting_state_group(_: CScriptThing) ;
    // pub fn set_max_hero_reaction_distance(_: CScriptThing, _: c_float) ;
    // pub fn get_max_hero_reaction_distance(_: CScriptThing) -> c_float ;
    // pub fn reset_max_hero_reaction_distance(_: CScriptThing) ;
    // pub fn set_action_frequency(_: CScriptThing, _: c_long) ;
    // pub fn get_action_frequency(_: CScriptThing) -> c_long ;
    // pub fn reset_action_frequency(_: CScriptThing) ;
    // pub fn set_action_frequency_variation(_: CScriptThing, _: c_float) ;
    // pub fn get_action_frequency_variation(_: CScriptThing) -> c_float ;
    // pub fn reset_action_frequency_variation(_: CScriptThing) ;
    // pub fn set_action(_: CScriptThing, _: CCharString) ;
    // pub fn get_action(_: CScriptThing) -> CCharString ;
    // pub fn reset_action(_: CScriptThing) ;
    // pub fn set_face_hero_for_action(_: CScriptThing, _: bool) ;
    // pub fn get_face_hero_for_action(_: CScriptThing) -> bool ;
    // pub fn reset_face_hero_for_action(_: CScriptThing) ;
    // pub fn set_target_name(_: CScriptThing, _: CCharString) ;
    // pub fn get_target_name(_: CScriptThing) -> CCharString ;
    // pub fn reset_target_name(_: CScriptThing) ;
    // pub fn set_follow_distance(_: CScriptThing, _: c_float) ;
    // pub fn get_follow_distance(_: CScriptThing) -> c_float ;
    // pub fn reset_follow_distance(_: CScriptThing) ;
    // pub fn set_attack_hero_on_sight(_: CScriptThing, _: bool) ;
    // pub fn get_attack_hero_on_sight(_: CScriptThing) -> bool ;
    // pub fn reset_attack_hero_on_sight(_: CScriptThing) ;
    // pub fn set_time_to_spend_harassing_hero(_: CScriptThing, _: c_long) ;
    // pub fn get_time_to_spend_harassing_hero(_: CScriptThing) -> c_long ;
    // pub fn reset_time_to_spend_harassing_hero(_: CScriptThing) ;
    // pub fn set_combat_nearby_enemy_fleeing_break_off_range(
    //     _: CScriptThing,
    //     _: c_float,
    // );
    // pub fn get_combat_nearby_enemy_fleeing_break_off_range(_: CScriptThing) -> c_float ;
    // pub fn reset_combat_nearby_enemy_fleeing_break_off_range(_: CScriptThing) ;
    // pub fn set_combat_nearby_break_off_range(_: CScriptThing, _: c_float) ;
    // pub fn get_combat_nearby_break_off_range(_: CScriptThing) -> c_float ;
    // pub fn reset_combat_nearby_break_off_range(_: CScriptThing) ;
    // pub fn set_steal_stealable_items(_: CScriptThing, _: bool) ;
    // pub fn get_steal_stealable_items(_: CScriptThing) -> bool ;
    // pub fn reset_steal_stealable_items(_: CScriptThing) ;
    // pub fn set_recover_stealable_items(_: CScriptThing, _: bool) ;
    // pub fn get_recover_stealable_items(_: CScriptThing) -> bool ;
    // pub fn reset_recover_stealable_items(_: CScriptThing) ;
    // pub fn set_take_stealable_item_to_random_destination(_: CScriptThing, _: bool) ;
    // pub fn get_take_stealable_item_to_random_destination(_: CScriptThing) -> bool ;
    // pub fn reset_take_stealable_item_to_random_destination(_: CScriptThing) ;
    // pub fn set_kill_self_and_stealable_item_after_reaching_destination(
    //     _: CScriptThing,
    //     _: bool,
    // );
    // pub fn get_kill_self_and_stealable_item_after_reaching_destination(
    //     _: CScriptThing,
    // ) -> bool;
    // pub fn reset_kill_self_and_stealable_item_after_reaching_destination(_: CScriptThing) ;
    // pub fn set_allowed_to_follow(_: CScriptThing, _: bool) ;
    // pub fn get_allowed_to_follow(_: CScriptThing) -> bool ;
    // pub fn reset_allowed_to_follow(_: CScriptThing) ;
    // pub fn set_table_name(_: CScriptThing, _: CCharString) ;
    // pub fn get_table_name(_: CScriptThing) -> CCharString ;
    // pub fn reset_table_name(_: CScriptThing) ;
    // pub fn set_seat_name(_: CScriptThing, _: CCharString) ;
    // pub fn get_seat_name(_: CScriptThing) -> CCharString ;
    // pub fn reset_seat_name(_: CScriptThing) ;
    // pub fn set_disable_head_looking(_: CScriptThing, _: bool) ;
    // pub fn get_disable_head_looking(_: CScriptThing) -> bool ;
    // pub fn reset_disable_head_looking(_: CScriptThing) ;
    // pub fn set_is_pushable_by_hero(_: CScriptThing, _: bool) ;
    // pub fn get_is_pushable_by_hero(_: CScriptThing) -> bool ;
    // pub fn reset_is_pushable_by_hero(_: CScriptThing) ;
    // pub fn set_look_for_finite_time(_: CScriptThing, _: bool) ;
    // pub fn get_look_for_finite_time(_: CScriptThing) -> bool ;
    // pub fn reset_look_for_finite_time(_: CScriptThing) ;
    // pub fn set_avoid_region_exits(_: CScriptThing, _: bool) ;
    // pub fn get_avoid_region_exits(_: CScriptThing) -> bool ;
    // pub fn reset_avoid_region_exits(_: CScriptThing) ;
    // pub fn set_targeting_distance_offset(_: CScriptThing, _: c_float) ;
    // pub fn get_targeting_distance_offset(_: CScriptThing) -> c_float ;
    // pub fn reset_targeting_distance_offset(_: CScriptThing) ;
    // pub fn set_active_gossip_categories(_: CCharString, _: bool) ;
    // pub fn get_active_gossip_categories() -> *const CxxMap<CCharString, _: bool> ;
    // pub fn get_active_gossip_categories_2(_: CCharString) -> *mut bool ;
    // pub fn get_active_gossip_categories_size() -> i32 ;
    // pub fn get_is_gossip_for_player() -> *const CxxMap<CCharString, _: bool> ;
    // pub fn get_is_gossip_for_player_2(_: CCharString) -> *mut bool ;
    // pub fn get_is_gossip_for_player_size() -> i32 ;
    // pub fn set_gossip(_: CCharString, _: CCharString, _: c_long) ;
    // pub fn get_gossip(_: CCharString) -> *const CxxVector<CCharString> ;
    // pub fn get_gossip_2(_: CCharString, _: c_long) -> CCharString ;
    // pub fn get_gossip_size(_: CCharString) -> i32 ;
    // pub fn clear_gossip(_: CCharString) ;
    // pub fn remove_gossip(_: CCharString) ;
    // pub fn add_gossip(_: CCharString) ;
    // pub fn add_gossip_2(_: CCharString, _: CCharString) ;
    // pub fn set_gossip_villages(_: CCharString, _: CCharString, _: c_long) ;
    // pub fn get_gossip_villages(_: CCharString) -> *const CxxVector<CCharString> ;
    // pub fn get_gossip_villages_2(_: CCharString, _: c_long) -> CCharString ;
    // pub fn get_gossip_villages_size(_: CCharString) -> i32 ;
    // pub fn clear_gossip_villages(_: CCharString) ;
    // pub fn remove_gossip_villages(_: CCharString) ;
    // pub fn add_gossip_villages(_: CCharString) ;
    // pub fn add_gossip_villages_2(_: CCharString, _: CCharString) ;
    // pub fn set_gossip_factions(_: CCharString, _: CCharString, _: c_long) ;
    // pub fn get_gossip_factions(_: CCharString) -> *const CxxVector<CCharString> ;
    // pub fn get_gossip_factions_2(_: CCharString, _: c_long) -> CCharString ;
    // pub fn get_gossip_factions_size(_: CCharString) -> i32 ;
    // pub fn clear_gossip_factions(_: CCharString) ;
    // pub fn remove_gossip_factions(_: CCharString) ;
    // pub fn add_gossip_factions(_: CCharString) ;
    // pub fn add_gossip_factions_2(_: CCharString, _: CCharString) ;
}

pub unsafe fn lua_fable_lib(lua: &mut hlua::Lua<'_>) {
    lua.set("end_letter_box", hlua::function0(&end_letter_box));
    lua.set("validate", hlua::function0(&validate));
    lua.set(
        "set_debug_camera_type",
        hlua::function1(&set_debug_camera_type),
    );
    lua.set("deactivate_boast_ui", hlua::function0(&deactivate_boast_ui));
    lua.set("is_xbox", hlua::function0(&is_xbox));
    lua.set("new_script_frame", hlua::function0(&new_script_frame));
    lua.set("msg_is_boast_made", hlua::function1(&msg_is_boast_made));
    lua.set(
        "remove_boast_message",
        hlua::function0(remove_boast_message),
    );
    lua.set(
        "msg_on_leaving_experience_spending_screen",
        hlua::function0(msg_on_leaving_experience_spending_screen),
    );
    lua.set(
        "msg_is_answered_yes_or_no",
        hlua::function0(msg_is_answered_yes_or_no),
    );
    lua.set(
        "msg_is_game_info_clicked_past",
        hlua::function0(msg_is_game_info_clicked_past),
    );
    lua.set(
        "msg_is_tutorial_click_past",
        hlua::function0(msg_is_tutorial_click_past),
    );
    lua.set(
        "msg_is_action_mode_button_pressed",
        hlua::function0(msg_is_action_mode_button_pressed),
    );
    lua.set(
        "msg_is_cut_scene_skipped",
        hlua::function0(msg_is_cut_scene_skipped),
    );
    lua.set(
        "msg_on_hero_used_guild_seal",
        hlua::function0(msg_on_hero_used_guild_seal),
    );
    lua.set(
        "msg_on_game_saved_manually",
        hlua::function0(msg_on_game_saved_manually),
    );
    lua.set(
        "msg_on_hero_fired_ranged_weapon",
        hlua::function0(msg_on_hero_fired_ranged_weapon),
    );
    lua.set(
        "msg_on_chest_opening_cancelled",
        hlua::function0(msg_on_chest_opening_cancelled),
    );
    lua.set(
        "dont_populate_next_loaded_region",
        hlua::function0(dont_populate_next_loaded_region),
    );
    lua.set(
        "is_hero_allowed_henchmen_in_current_region",
        hlua::function0(is_hero_allowed_henchmen_in_current_region),
    );
    lua.set(
        "post_add_scripted_entities",
        hlua::function0(post_add_scripted_entities),
    );
    lua.set(
        "is_player_holding_unsheathe_ranged_weapon_button",
        hlua::function0(is_player_holding_unsheathe_ranged_weapon_button),
    );
    lua.set(
        "is_player_holding_lock_target_button",
        hlua::function0(is_player_holding_lock_target_button),
    );
    lua.set(
        "is_player_holding_fire_ranged_weapon_button",
        hlua::function0(is_player_holding_fire_ranged_weapon_button),
    );
    lua.set(
        "is_player_holding_first_person_targeting_button",
        hlua::function0(is_player_holding_first_person_targeting_button),
    );
    lua.set(
        "is_hero_in_projectile_weapon_mode",
        hlua::function0(is_hero_in_projectile_weapon_mode),
    );
    lua.set("register_timer", hlua::function0(register_timer));
    lua.set("deregister_timer", hlua::function1(deregister_timer));
    lua.set("set_timer", hlua::function2(set_timer));
    lua.set("get_timer", hlua::function1(get_timer));
    lua.set(
        "set_creature_creation_delay_frames",
        hlua::function1(set_creature_creation_delay_frames),
    );
    lua.set(
        "reset_creature_creation_delay_frames",
        hlua::function0(reset_creature_creation_delay_frames),
    );
    lua.set(
        "is_thing_with_this_uid_alive",
        hlua::function1(is_thing_with_this_uid_alive),
    );
    lua.set("clear_all_rumbles", hlua::function0(clear_all_rumbles));
    lua.set(
        "is_safe_to_display_game_info",
        hlua::function0(is_safe_to_display_game_info),
    );
    lua.set(
        "is_tutorial_system_enabled",
        hlua::function0(is_tutorial_system_enabled),
    );
    lua.set("give_hero_gold", hlua::function1(give_hero_gold));
    lua.set("get_hero_gold", hlua::function0(get_hero_gold));
    lua.set(
        "give_hero_experience",
        hlua::function1(give_hero_experience),
    );
    lua.set(
        "set_hero_able_to_gain_experience",
        hlua::function1(set_hero_able_to_gain_experience),
    );
    lua.set(
        "sheathe_hero_weapons",
        hlua::function0(sheathe_hero_weapons),
    );
    lua.set(
        "set_hero_will_as_usable",
        hlua::function1(set_hero_will_as_usable),
    );
    lua.set(
        "set_hero_weapons_as_usable",
        hlua::function1(set_hero_weapons_as_usable),
    );
    lua.set(
        "set_weapon_out_crime_enabled",
        hlua::function1(set_weapon_out_crime_enabled),
    );
    lua.set(
        "set_guards_ignore_crimes",
        hlua::function1(set_guards_ignore_crimes),
    );
    lua.set(
        "remove_all_hero_weapons",
        hlua::function0(remove_all_hero_weapons),
    );
    lua.set(
        "confiscate_all_hero_items",
        hlua::function0(confiscate_all_hero_items),
    );
    lua.set(
        "confiscate_all_hero_weapons",
        hlua::function0(confiscate_all_hero_weapons),
    );
    lua.set(
        "return_all_confiscated_items_to_hero",
        hlua::function0(return_all_confiscated_items_to_hero),
    );
    lua.set(
        "is_player_creature_blocking",
        hlua::function0(is_player_creature_blocking),
    );
    lua.set(
        "get_player_creature_combat_multiplier",
        hlua::function0(get_player_creature_combat_multiplier),
    );
    lua.set(
        "get_player_creature_combat_multiplier_running_num_hits",
        hlua::function0(get_player_creature_combat_multiplier_running_num_hits),
    );
    lua.set(
        "reset_player_creature_combat_multiplier",
        hlua::function0(reset_player_creature_combat_multiplier),
    );
    lua.set(
        "is_player_creature_flourish_enabled",
        hlua::function0(is_player_creature_flourish_enabled),
    );
    lua.set(
        "reset_player_creature_only_target",
        hlua::function0(reset_player_creature_only_target),
    );
    lua.set("respawn_hero", hlua::function1(respawn_hero));
    lua.set("give_hero_morality", hlua::function1(give_hero_morality));
    lua.set("get_hero_morality", hlua::function0(get_hero_morality));
    lua.set(
        "change_hero_morality_due_to_theft",
        hlua::function0(change_hero_morality_due_to_theft),
    );
    lua.set(
        "change_hero_morality_due_to_picklock",
        hlua::function0(change_hero_morality_due_to_picklock),
    );
    lua.set(
        "give_hero_renown_points",
        hlua::function1(give_hero_renown_points),
    );
    lua.set(
        "get_hero_renown_level",
        hlua::function0(get_hero_renown_level),
    );
    lua.set(
        "is_hero_renown_level_full",
        hlua::function0(is_hero_renown_level_full),
    );
    lua.set(
        "increase_hero_renown_level",
        hlua::function0(increase_hero_renown_level),
    );
    lua.set(
        "get_hero_strength_level",
        hlua::function0(get_hero_strength_level),
    );
    lua.set(
        "get_hero_skill_level",
        hlua::function0(get_hero_skill_level),
    );
    lua.set("get_hero_will_level", hlua::function0(get_hero_will_level));
    lua.set("set_hero_age", hlua::function1(set_hero_age));
    lua.set("get_hero_age", hlua::function0(get_hero_age));
    lua.set(
        "set_hero_as_teenager",
        hlua::function1(set_hero_as_teenager),
    );
    lua.set(
        "set_hero_as_apprentice",
        hlua::function1(set_hero_as_apprentice),
    );
    lua.set(
        "get_distance_hero_can_be_heard_from",
        hlua::function0(get_distance_hero_can_be_heard_from),
    );
    lua.set(
        "get_hero_rough_experience_level",
        hlua::function0(get_hero_rough_experience_level),
    );
    lua.set(
        "get_hero_experience_available_to_spend",
        hlua::function0(get_hero_experience_available_to_spend),
    );
    lua.set("get_hero_fatness", hlua::function0(get_hero_fatness));
    lua.set("get_hero_scariness", hlua::function0(get_hero_scariness));
    lua.set(
        "get_hero_attractiveness",
        hlua::function0(get_hero_attractiveness),
    );
    lua.set(
        "get_hero_will_energy_level",
        hlua::function0(get_hero_will_energy_level),
    );
    lua.set(
        "set_hero_will_energy_level",
        hlua::function1(set_hero_will_energy_level),
    );
    lua.set(
        "set_hero_will_energy_as_able_to_refill",
        hlua::function1(set_hero_will_energy_as_able_to_refill),
    );
    lua.set(
        "is_hero_hand_lamp_lit",
        hlua::function0(is_hero_hand_lamp_lit),
    );
    lua.set(
        "set_hero_hand_lamp_as_lit",
        hlua::function1(set_hero_hand_lamp_as_lit),
    );
    lua.set("is_hero_naked", hlua::function0(is_hero_naked));
    lua.set(
        "remove_hero_clothing",
        hlua::function0(remove_hero_clothing),
    );
    lua.set(
        "remove_hero_hairstyle",
        hlua::function0(remove_hero_hairstyle),
    );
    lua.set(
        "is_player_wielding_weapon",
        hlua::function0(is_player_wielding_weapon),
    );
    lua.set(
        "apply_hero_penalty_for_death",
        hlua::function0(apply_hero_penalty_for_death),
    );
    lua.set(
        "get_hero_has_married",
        hlua::function0(get_hero_has_married),
    );
    lua.set(
        "get_hero_has_current_marriage",
        hlua::function0(get_hero_has_current_marriage),
    );
    lua.set(
        "get_hero_has_divorced_marriage",
        hlua::function0(get_hero_has_divorced_marriage),
    );
    lua.set(
        "get_hero_has_children",
        hlua::function0(get_hero_has_children),
    );
    lua.set(
        "get_hero_has_murdered_wife",
        hlua::function0(get_hero_has_murdered_wife),
    );
    lua.set("is_hero_child", hlua::function0(is_hero_child));
    lua.set(
        "cancel_hero_teleport_effects",
        hlua::function0(cancel_hero_teleport_effects),
    );
    lua.set(
        "get_number_of_times_hero_has_had_sex",
        hlua::function0(get_number_of_times_hero_has_had_sex),
    );
    lua.set(
        "set_number_of_times_hero_has_had_sex",
        hlua::function1(set_number_of_times_hero_has_had_sex),
    );
    lua.set(
        "set_hero_as_having_had_sex",
        hlua::function1(set_hero_as_having_had_sex),
    );
    lua.set(
        "set_hero_as_having_had_gay_sex",
        hlua::function1(set_hero_as_having_had_gay_sex),
    );
    lua.set(
        "set_hero_guide_to_show_quest_cards_when_spoken_to",
        hlua::function1(set_hero_guide_to_show_quest_cards_when_spoken_to),
    );
    lua.set(
        "set_teleporting_as_active",
        hlua::function1(set_teleporting_as_active),
    );
    lua.set(
        "is_teleporting_active",
        hlua::function0(is_teleporting_active),
    );
    lua.set(
        "set_region_text_display_as_active",
        hlua::function1(set_region_text_display_as_active),
    );
    lua.set(
        "set_hero_sleeping_as_enabled",
        hlua::function1(set_hero_sleeping_as_enabled),
    );
    lua.set(
        "is_hero_sleeping_enabled",
        hlua::function0(is_hero_sleeping_enabled),
    );
    lua.set(
        "set_experience_spending_as_enabled",
        hlua::function1(set_experience_spending_as_enabled),
    );
    lua.set(
        "set_morality_changing_as_enabled",
        hlua::function1(set_morality_changing_as_enabled),
    );
    lua.set(
        "set_summoner_death_explosion_affects_hero",
        hlua::function1(set_summoner_death_explosion_affects_hero),
    );
    lua.set("is_deed_witnessed", hlua::function1(is_deed_witnessed));
    lua.set(
        "teleport_all_followers_to_hero_position",
        hlua::function0(teleport_all_followers_to_hero_position),
    );
    lua.set("hero_go_fishing", hlua::function1(hero_go_fishing));
    lua.set(
        "get_hero_fishing_level",
        hlua::function0(get_hero_fishing_level),
    );
    lua.set("hero_go_digging", hlua::function0(hero_go_digging));
    lua.set("hero_stop_digging", hlua::function0(hero_stop_digging));
    lua.set(
        "hero_play_oracle_minigame",
        hlua::function0(hero_play_oracle_minigame),
    );
    lua.set(
        "is_hero_playing_oracle_minigame",
        hlua::function0(is_hero_playing_oracle_minigame),
    );
    lua.set(
        "has_hero_won_oracle_minigame",
        hlua::function0(has_hero_won_oracle_minigame),
    );
    lua.set(
        "hero_play_fireheart_minigame",
        hlua::function0(hero_play_fireheart_minigame),
    );
    lua.set(
        "hero_quit_fireheart_minigame",
        hlua::function0(hero_quit_fireheart_minigame),
    );
    lua.set(
        "has_hero_force_quit_fireheart_minigame",
        hlua::function0(has_hero_force_quit_fireheart_minigame),
    );
    lua.set("get_hero_health", hlua::function0(get_hero_health));
    lua.set("get_hero_health_max", hlua::function0(get_hero_health_max));
    lua.set(
        "get_hero_health_percentage",
        hlua::function0(get_hero_health_percentage),
    );
    lua.set(
        "get_hero_will_energy",
        hlua::function0(get_hero_will_energy),
    );
    lua.set(
        "get_hero_will_energy_max",
        hlua::function0(get_hero_will_energy_max),
    );
    lua.set(
        "get_hero_will_energy_percentage",
        hlua::function0(get_hero_will_energy_percentage),
    );
    lua.set(
        "change_hero_health_by",
        hlua::function3(change_hero_health_by),
    );
    lua.set(
        "tell_hero_quest_objective_completed",
        hlua::function1(tell_hero_quest_objective_completed),
    );
    lua.set(
        "tell_hero_quest_objective_failed",
        hlua::function1(tell_hero_quest_objective_failed),
    );
    lua.set("is_hero_on_quest", hlua::function0(is_hero_on_quest));
    lua.set(
        "set_guild_master_messages",
        hlua::function1(set_guild_master_messages),
    );
    lua.set(
        "prepare_quests_when_final_quest_is_completed",
        hlua::function0(prepare_quests_when_final_quest_is_completed),
    );
    lua.set(
        "remove_all_available_quest_cards_from_guild",
        hlua::function0(remove_all_available_quest_cards_from_guild),
    );
    lua.set(
        "fail_all_active_quests",
        hlua::function0(fail_all_active_quests),
    );
    lua.set(
        "kick_off_death_screen",
        hlua::function0(kick_off_death_screen),
    );
    lua.set(
        "reset_death_recovery_marker_name_to_default",
        hlua::function0(reset_death_recovery_marker_name_to_default),
    );
    lua.set(
        "is_to_fail_quest_on_death",
        hlua::function0(is_to_fail_quest_on_death),
    );
    lua.set(
        "set_whether_to_fail_quest_on_death",
        hlua::function1(set_whether_to_fail_quest_on_death),
    );
    lua.set(
        "reset_whether_to_fail_quest_on_death_to_default",
        hlua::function0(reset_whether_to_fail_quest_on_death_to_default),
    );
    lua.set("display_quest_info", hlua::function1(display_quest_info));
    lua.set(
        "update_quest_info_timer",
        hlua::function2(update_quest_info_timer),
    );
    lua.set(
        "update_quest_info_tick",
        hlua::function2(update_quest_info_tick),
    );
    lua.set(
        "remove_quest_info_element",
        hlua::function1(remove_quest_info_element),
    );
    lua.set(
        "remove_all_quest_info_elements",
        hlua::function0(remove_all_quest_info_elements),
    );
    lua.set("display_time", hlua::function1(display_time));
    lua.set("display_money_bag", hlua::function1(display_money_bag));
    lua.set(
        "update_mini_game_info_bar",
        hlua::function1(update_mini_game_info_bar),
    );
    lua.set(
        "mini_map_remove_all_markers",
        hlua::function0(mini_map_remove_all_markers),
    );
    lua.set(
        "mini_map_set_as_enabled",
        hlua::function1(mini_map_set_as_enabled),
    );
    lua.set("set_hud_enabled", hlua::function1(set_hud_enabled));
    lua.set("remove_conversation", hlua::function2(remove_conversation));
    lua.set(
        "is_conversation_active",
        hlua::function1(is_conversation_active),
    );
    lua.set(
        "fix_movie_sequence_camera",
        hlua::function1(fix_movie_sequence_camera),
    );
    lua.set(
        "fade_screen_out_until_next_call_to_fade_screen_in",
        hlua::function2(fade_screen_out_until_next_call_to_fade_screen_in),
    );
    lua.set("fade_screen_in", hlua::function0(fade_screen_in));
    lua.set(
        "is_screen_fading_out",
        hlua::function0(is_screen_fading_out),
    );
    lua.set("pause", hlua::function1(pause));
    lua.set("end_cut_fade", hlua::function0(end_cut_fade));
    lua.set(
        "pause_all_non_scripted_entities",
        hlua::function1(pause_all_non_scripted_entities),
    );
    lua.set("pause_all_entities", hlua::function1(pause_all_entities));
    lua.set(
        "set_allow_screen_fading_on_next_region_change",
        hlua::function1(set_allow_screen_fading_on_next_region_change),
    );
    lua.set(
        "set_allow_screen_fading_if_already_faded",
        hlua::function1(set_allow_screen_fading_if_already_faded),
    );
    lua.set(
        "set_to_keep_hero_abilities_during_cutscenes",
        hlua::function1(set_to_keep_hero_abilities_during_cutscenes),
    );
    lua.set(
        "set_to_display_tutorials_during_cutscenes",
        hlua::function1(set_to_display_tutorials_during_cutscenes),
    );
    lua.set("set_cutscene_mode", hlua::function2(set_cutscene_mode));
    lua.set("is_in_cutscene", hlua::function0(is_in_cutscene));
    lua.set(
        "set_cutscene_skippable",
        hlua::function1(set_cutscene_skippable),
    );
    lua.set(
        "set_cutscene_skippable_while_paused",
        hlua::function1(set_cutscene_skippable_while_paused),
    );
    lua.set("preload_new_scene", hlua::function1(preload_new_scene));
    lua.set(
        "start_progress_display",
        hlua::function0(start_progress_display),
    );
    lua.set(
        "stop_progress_display",
        hlua::function0(stop_progress_display),
    );
    lua.set(
        "set_screen_messages_enabled",
        hlua::function1(set_screen_messages_enabled),
    );
    lua.set(
        "is_hero_controlled_by_player",
        hlua::function0(is_hero_controlled_by_player),
    );
    lua.set(
        "is_in_movie_sequence",
        hlua::function0(is_in_movie_sequence),
    );
    lua.set(
        "cancel_abilities_for_cutscene",
        hlua::function0(cancel_abilities_for_cutscene),
    );
    lua.set(
        "resume_abilities_for_cutscene",
        hlua::function0(resume_abilities_for_cutscene),
    );
    lua.set(
        "set_environmental_effects_always_update",
        hlua::function1(set_environmental_effects_always_update),
    );
    lua.set(
        "set_dead_creatures_and_experience_orbs_and_drop_bags_as_hidden",
        hlua::function1(set_dead_creatures_and_experience_orbs_and_drop_bags_as_hidden),
    );
    lua.set(
        "camera_set_camera_preload_flag",
        hlua::function1(camera_set_camera_preload_flag),
    );
    lua.set("camera_default", hlua::function0(camera_default));
    lua.set(
        "camera_reset_to_view_behind_hero",
        hlua::function1(camera_reset_to_view_behind_hero),
    );
    lua.set(
        "is_camera_in_scripted_mode",
        hlua::function0(is_camera_in_scripted_mode),
    );
    lua.set(
        "camera_cancel_screen_effect",
        hlua::function0(camera_cancel_screen_effect),
    );
    lua.set("get_game_angle_xy", hlua::function0(get_game_angle_xy));
    lua.set("camera_shake", hlua::function2(camera_shake));
    lua.set("set_time_of_day", hlua::function1(set_time_of_day));
    lua.set("get_time_of_day", hlua::function0(get_time_of_day));
    lua.set(
        "fast_forward_time_to",
        hlua::function2(fast_forward_time_to),
    );
    lua.set(
        "is_time_of_day_between",
        hlua::function2(is_time_of_day_between),
    );
    lua.set("get_day_of_week", hlua::function0(get_day_of_week));
    lua.set("get_day_count", hlua::function0(get_day_count));
    lua.set("get_world_frame", hlua::function0(get_world_frame));
    lua.set("get_constant_fps", hlua::function0(get_constant_fps));
    lua.set(
        "reset_to_default_theme",
        hlua::function1(reset_to_default_theme),
    );
    lua.set(
        "reset_to_default_theme_all_internals",
        hlua::function1(reset_to_default_theme_all_internals),
    );
    lua.set(
        "reset_to_default_theme_externals",
        hlua::function1(reset_to_default_theme_externals),
    );
    lua.set(
        "set_all_sounds_as_muted",
        hlua::function1(set_all_sounds_as_muted),
    );
    lua.set(
        "is_radial_blur_fade_active",
        hlua::function0(is_radial_blur_fade_active),
    );
    lua.set(
        "cancel_radial_blur_fade",
        hlua::function0(cancel_radial_blur_fade),
    );
    lua.set("enable_decals", hlua::function1(enable_decals));
    lua.set("is_sound_playing", hlua::function1(is_sound_playing));
    lua.set("stop_sound", hlua::function1(stop_sound));
    lua.set("enable_sounds", hlua::function1(enable_sounds));
    lua.set("stop_override_music", hlua::function1(stop_override_music));
    lua.set("enable_danger_music", hlua::function1(enable_danger_music));
    lua.set(
        "is_danger_music_enabled",
        hlua::function0(is_danger_music_enabled),
    );
    lua.set(
        "start_countdown_timer",
        hlua::function1(start_countdown_timer),
    );
    lua.set("get_countdown_timer", hlua::function0(get_countdown_timer));
    lua.set(
        "auto_save_check_point",
        hlua::function0(auto_save_check_point),
    );
    lua.set(
        "auto_save_quest_start",
        hlua::function0(auto_save_quest_start),
    );
    lua.set("auto_save", hlua::function0(auto_save));
    lua.set(
        "set_saving_as_enabled",
        hlua::function1(set_saving_as_enabled),
    );
    lua.set("is_saving_enabled", hlua::function0(is_saving_enabled));
    lua.set("reset_to_front_end", hlua::function0(reset_to_front_end));
    lua.set(
        "get_guild_seal_recall_angle_xy",
        hlua::function0(get_guild_seal_recall_angle_xy),
    );
    lua.set(
        "update_online_score_archery",
        hlua::function1(update_online_score_archery),
    );
    lua.set(
        "update_online_score_chicken_kick",
        hlua::function1(update_online_score_chicken_kick),
    );
    lua.set(
        "update_online_score_chapel_or_temple",
        hlua::function1(update_online_score_chapel_or_temple),
    );
    lua.set(
        "update_online_score_fishing_compo",
        hlua::function1(update_online_score_fishing_compo),
    );
    lua.set(
        "update_score_fishing_competition",
        hlua::function1(update_score_fishing_competition),
    );
    lua.set("get_best_time_pairs", hlua::function0(get_best_time_pairs));
    lua.set(
        "get_best_time_sorting",
        hlua::function0(get_best_time_sorting),
    );
    lua.set(
        "get_best_score_blackjack",
        hlua::function0(get_best_score_blackjack),
    );
    lua.set(
        "get_best_score_coin_golf_oak_vale",
        hlua::function0(get_best_score_coin_golf_oak_vale),
    );
    lua.set(
        "get_best_score_coin_golf_snow_spire",
        hlua::function0(get_best_score_coin_golf_snow_spire),
    );
    lua.set(
        "get_best_score_shove_ha_penny",
        hlua::function0(get_best_score_shove_ha_penny),
    );
    lua.set(
        "get_best_time_guess_the_addition",
        hlua::function0(get_best_time_guess_the_addition),
    );
    lua.set(
        "is_hero_in_tavern_game",
        hlua::function0(is_hero_in_tavern_game),
    );
    lua.set(
        "get_num_houses_owned",
        hlua::function0(get_num_houses_owned),
    );
    lua.set("start_sneaking", hlua::function0(start_sneaking));
    lua.set("force_ships_visible", hlua::function0(force_ships_visible));
    lua.set(
        "set_player_using_melee_dummies",
        hlua::function1(set_player_using_melee_dummies),
    );
    lua.set(
        "get_player_using_melee_dummies",
        hlua::function0(get_player_using_melee_dummies),
    );
    lua.set(
        "set_player_using_ranged_dummies",
        hlua::function1(set_player_using_ranged_dummies),
    );
    lua.set(
        "get_player_using_ranged_dummies",
        hlua::function0(get_player_using_ranged_dummies),
    );
    lua.set(
        "set_player_using_will_dummies",
        hlua::function1(set_player_using_will_dummies),
    );
    lua.set(
        "get_player_using_will_dummies",
        hlua::function0(get_player_using_will_dummies),
    );
    lua.set(
        "set_cheap_head_looking",
        hlua::function1(set_cheap_head_looking),
    );
    lua.set(
        "get_cheap_head_looking",
        hlua::function0(get_cheap_head_looking),
    );
    lua.set(
        "set_quit_tavern_game",
        hlua::function1(set_quit_tavern_game),
    );
    lua.set(
        "get_quit_tavern_game",
        hlua::function0(get_quit_tavern_game),
    );
    lua.set(
        "set_prize_tavern_table",
        hlua::function1(set_prize_tavern_table),
    );
    lua.set(
        "get_prize_tavern_table",
        hlua::function0(get_prize_tavern_table),
    );
    lua.set("set_betting_active", hlua::function1(set_betting_active));
    lua.set("get_betting_active", hlua::function0(get_betting_active));
    lua.set("set_betting_accept", hlua::function1(set_betting_accept));
    lua.set("get_betting_accept", hlua::function0(get_betting_accept));
    lua.set("set_betting_amount", hlua::function1(set_betting_amount));
    lua.set("get_betting_amount", hlua::function0(get_betting_amount));
    lua.set(
        "set_count_bet_money_down",
        hlua::function1(set_count_bet_money_down),
    );
    lua.set(
        "get_count_bet_money_down",
        hlua::function0(get_count_bet_money_down),
    );
    lua.set(
        "set_spot_the_addition_beaten",
        hlua::function1(set_spot_the_addition_beaten),
    );
    lua.set(
        "get_spot_the_addition_beaten",
        hlua::function0(get_spot_the_addition_beaten),
    );
    lua.set(
        "set_global_targeting_distance_offset",
        hlua::function1(set_global_targeting_distance_offset),
    );
    lua.set(
        "get_global_targeting_distance_offset",
        hlua::function0(get_global_targeting_distance_offset),
    );
    lua.set(
        "set_trading_price_mult",
        hlua::function1(set_trading_price_mult),
    );
    lua.set(
        "get_trading_price_mult",
        hlua::function0(get_trading_price_mult),
    );
    lua.set(
        "set_boasting_enabled",
        hlua::function1(set_boasting_enabled),
    );
    lua.set(
        "get_boasting_enabled",
        hlua::function0(get_boasting_enabled),
    );
    lua.set(
        "clear_active_gossip_categories",
        hlua::function0(clear_active_gossip_categories),
    );
    lua.set(
        "clear_is_gossip_for_player",
        hlua::function0(clear_is_gossip_for_player),
    );
}
