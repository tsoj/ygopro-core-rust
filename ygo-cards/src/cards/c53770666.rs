//! Armament Reincarnation
use super::prelude::*;

pub const ID: u32 = 53770666;
const TOKEN_ARMAMENT_REINCARNATION: u32 = ID + 1;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID + 1], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon "Armament Reincarnation Tokens" (Warrior/LIGHT/Level 1/ATK 500/DEF 500) up to the number of Equip Spells, and Traps with an effect that equip themselves to a monster, in your GY, then you can destroy as many cards in your Spell & Trap Zone as possible, including this card, then Set as many Equip Spells, and Traps with an effect that equip themselves to a monster, as possible from your GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN + CATEGORY_DESTROY + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(500),
        defense: Some(500),
        level: Some(1),
        race: Some(RACE_WARRIOR),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
fn can_summon_token(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(d, tp, TOKEN_ARMAMENT_REINCARNATION, token_data(), None, None, 0)
}
fn equip_or_trap(d: &mut Duel, c: CardId) -> bool {
    card::is_equip_spell(d, c) || card::is_equip_trap(d, c)
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_equip_spell(d, c) || card::is_equip_trap(d, c)) && card::is_s_setable(d, c, true, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && can_summon_token(d, a.tp)
            && duel::is_existing_matching_card(d, Some(&equip_or_trap), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    let c = effect::get_handler(d, a.e).nil();
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    let max_ct = duel::get_matching_group_count(d, Some(&equip_or_trap), a.tp, LOCATION_GRAVE, 0, NO_EX) as i32;
    if ft > 0 && max_ct > 0 && can_summon_token(d, a.tp) {
        ft = ft.min(max_ct);
        if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        if ft > 1 {
            ft = duel::announce_number_range(d, a.tp, Some(1), Some(ft as u32), &[]).nil().0 as i32;
        }
        for _ in 1..=ft {
            let token = duel::create_token(d, a.tp, TOKEN_ARMAMENT_REINCARNATION).nil();
            duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
        if duel::special_summon_complete(d) > 0
            && card::is_relate_to_effect(d, c, a.e)
            && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, c| setfilter(d, c))), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
        {
            let g = duel::get_field_group(d, a.tp, LOCATION_STZONE, 0);
            duel::break_effect(d);
            if duel::destroy(d, g, REASON_EFFECT, None, None) > 0 && duel::get_operated_group(d).contains(c) {
                let st_zones = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, Some(0), None, None).0;
                if st_zones > 0 {
                    let equip_g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| setfilter(d, c))), a.tp, LOCATION_GRAVE, 0, NO_EX);
                    let sg = if equip_g.len() as i32 > st_zones {
                        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
                        group::select(d, &equip_g, a.tp, st_zones as u16, st_zones as u16, false, NO_EX).nil()
                    } else {
                        equip_g
                    };
                    if !sg.is_empty() {
                        duel::break_effect(d);
                        if duel::s_set(d, a.tp, sg, None, true) > 0 {
                            let mut set_traps = duel::get_operated_group(d);
                            group::match_(d, &mut set_traps, &|d, c| card::is_trap(d, c), NO_EX);
                            for sc in group::iter(d, &set_traps) {
                                // These Traps can be activated this turn
                                let e1 = effect::create_effect(d, c);
                                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_SET_AVAILABLE, 0);
                                effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
                                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                                card::register_effect(d, sc, e1, false);
                            }
                        }
                    }
                }
            }
        }
    }
    // For the rest of this turn, you cannot Special Summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 2));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e2, 1, 0);
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, a.tp);
}
