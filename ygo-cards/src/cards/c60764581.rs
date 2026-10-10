//! Stray Lambs
use super::prelude::*;

pub const ID: u32 = 60764581;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[60764582], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_activity_count(d, a.tp, &[ACTIVITY_SUMMON])[0] == 0
            && duel::get_activity_count(d, a.tp, &[ACTIVITY_FLIPSUMMON])[0] == 0
            && duel::get_activity_count(d, a.tp, &[ACTIVITY_SPSUMMON])[0] == 0;
    }
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_label_object(d, e1, Value::Effect(a.e));
    effect::set_target(d, e1, Some(lua_fn(sumlimit)));
    duel::register_effect(d, e1, a.tp);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e2, 1, 0);
    duel::register_effect(d, e2, a.tp);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_CANNOT_FLIP_SUMMON);
    duel::register_effect(d, e3, a.tp);
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    true
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp,se)`: `return e:GetLabelObject()~=se`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    effect::get_label_object(d, a.e(0)).as_effect() != a.v(6).as_effect()
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_BEAST),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1
        && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0)
    {
        for i in 1..=2u32 {
            let token = duel::create_token(d, a.tp, ID + i).nil();
            duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
        duel::special_summon_complete(d);
    }
}
