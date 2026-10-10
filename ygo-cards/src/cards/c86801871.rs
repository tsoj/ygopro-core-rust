//! Cobra Jar
use super::prelude::*;

pub const ID: u32 = 86801871;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[86801872], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(1200),
        defense: Some(1200),
        level: Some(3),
        race: Some(RACE_REPTILE),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0)
    {
        let token = duel::create_token(d, tp, ID + 1);
        if duel::special_summon_step(d, token.nil(), 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            let c = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
            effect::set_label_object(d, e1, Value::Card(token.nil()));
            effect::set_condition(d, e1, Some(condition(damcon)));
            effect::set_operation(d, e1, Some(operation(damop)));
            duel::register_effect(d, e1, tp);
        }
        duel::special_summon_complete(d);
    }
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tok = effect::get_label_object(d, a.e).as_card().nil();
    if a.eg.clone().nil().contains(tok) {
        return true;
    }
    if !card::is_location(d, tok, LOCATION_MZONE) {
        effect::reset(d, a.e);
    }
    false
}
fn damop(d: &mut Duel, a: &EffArgs) {
    duel::damage(d, 1 - a.tp, 500, REASON_EFFECT, false, None);
}
