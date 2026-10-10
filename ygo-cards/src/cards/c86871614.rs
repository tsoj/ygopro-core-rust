//! Cloning
use super::prelude::*;

pub const ID: u32 = 86871614;
const TOKEN_CLONE: u32 = ID + 1; // "Clone Token"

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_CLONE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When your opponent Normal or Flip Summons a monster that has a Level: Target that face-up monster; Special Summon 1 "Clone Token", with the same original Type, Attribute, Level, ATK, and DEF as that target
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(|_d, a| a.ep == 1 - a.tp)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
}
fn token_data(d: &mut Duel, ec: CardId) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(card::get_base_attack(d, ec)),
        defense: Some(card::get_base_defense(d, ec)),
        level: Some(card::get_original_level(d, ec)),
        race: Some(card::get_original_race(d, ec)),
        attribute: Some(card::get_original_attribute(d, ec)),
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let ec = group::get_first(d, &eg);
    if a.chk == Some(0) {
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        if !card::is_faceup(d, ec.nil()) || !card::is_location(d, ec.nil(), LOCATION_MZONE) || card::get_original_level(d, ec.nil()) == 0 {
            return false;
        }
        let md = token_data(d, ec.nil());
        return duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_CLONE, md, None, None, 0);
    }
    card::create_effect_relation(d, ec.nil(), a.e);
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    let ec = group::get_first(d, &eg).nil();
    if !card::is_relate_to_effect(d, ec, a.e) || card::is_facedown(d, ec) {
        return;
    }
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let md = token_data(d, ec);
    if !duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_CLONE, md, None, None, 0) {
        return;
    }
    card::register_flag_effect(d, ec, ID, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 0, 0, 0, 0);
    let token = duel::create_token(d, a.tp, TOKEN_CLONE).nil();
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    let atk = card::get_base_attack(d, ec);
    effect::set_value(d, e1, atk as i64);
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
    card::register_effect(d, token, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_SET_BASE_DEFENSE);
    let def = card::get_base_defense(d, ec);
    effect::set_value(d, e2, def as i64);
    card::register_effect(d, token, e2, false);
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EFFECT_CHANGE_LEVEL);
    let lv = card::get_original_level(d, ec);
    effect::set_value(d, e3, lv as i64);
    card::register_effect(d, token, e3, false);
    let e4 = effect::clone(d, e1);
    effect::set_code(d, e4, EFFECT_CHANGE_RACE);
    let race = card::get_original_race(d, ec);
    effect::set_value(d, e4, race as i64);
    card::register_effect(d, token, e4, false);
    let e5 = effect::clone(d, e1);
    effect::set_code(d, e5, EFFECT_CHANGE_ATTRIBUTE);
    let attr = card::get_original_attribute(d, ec);
    effect::set_value(d, e5, attr as i64);
    card::register_effect(d, token, e5, false);
    duel::special_summon(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    // When that target is destroyed and sent to the GY, destroy this Token
    let de = effect::create_effect(d, c);
    effect::set_type(d, de, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, de, LOCATION_MZONE);
    effect::set_code(d, de, EVENT_TO_GRAVE);
    effect::set_condition(d, de, Some(condition(descon)));
    effect::set_operation(d, de, Some(operation(desop)));
    effect::set_reset(d, de, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
    card::register_effect(d, token, de, false);
}
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_reason(d, c, REASON_DESTROY) && card::get_flag_effect(d, c, ID) != 0
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &dfilter, 1, NO_EX)
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, c, REASON_EFFECT, None, None);
}
