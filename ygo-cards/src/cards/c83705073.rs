//! Valkyrie Erda
use super::prelude::*;

pub const ID: u32 = 83705073;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VALKYRIE as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // summon success
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(regcon)));
    effect::set_operation(d, e1, Some(operation(regop)));
    card::register_effect(d, c, e1, false);
    // remove
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e2, EFFECT_TO_GRAVE_REDIRECT);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, 0, 0xff);
    effect::set_value(d, e2, LOCATION_REMOVED as i64);
    effect::set_target(d, e2, Some(effect_card_fn(rmtg)));
    card::register_effect(d, c, e2, false);
}
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re and re:GetHandler():IsSetCard(SET_VALKYRIE)
    match a.re {
        Some(re) => {
            let h = effect::get_handler(d, re).nil();
            card::is_set_card(d, h, &[SET_VALKYRIE as u16])
        }
        None => false,
    }
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // atk down
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    effect::set_value(d, e1, -1000);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, false);
}
fn rmtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // return c:GetOwner()~=tp and Duel.IsPlayerCanRemove(tp,c)
    //     and c:IsReason(REASON_DESTROY) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    card::get_owner(d, c) != tp
        && duel::is_player_can_remove(d, tp, Some((c, Some(REASON_EFFECT))))
        && card::is_reason(d, c, REASON_DESTROY)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
