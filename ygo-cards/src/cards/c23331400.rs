//! Link Streamer
use super::prelude::*;

pub const ID: u32 = 23331400;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[23331401], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // token
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON | CATEGORY_TOKEN);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(tkcon)));
    effect::set_target(d, e1, Some(cost_target(tktg)));
    effect::set_operation(d, e1, Some(operation(tkop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_CYBERSE) && card::is_controler(d, c, tp)
}
fn tkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, x| cfilter(d, x, tp), 1, NO_EX) && !group::is_contains(&eg, c)
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_CYBERSE),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), Some(POS_FACEUP as u8), Some(tp), 0);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
fn tkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), Some(POS_FACEUP as u8), Some(tp), 0)
    {
        let token = duel::create_token(d, tp, ID + 1).nil();
        duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
