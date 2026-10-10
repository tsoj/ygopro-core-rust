//! Spirit Reaper
use super::prelude::*;

pub const ID: u32 = 23205979;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // battle indestructable
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // handes
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_HANDES);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
    // self des
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_code(d, e3, EFFECT_SELF_DESTROY);
    effect::set_condition(d, e3, Some(condition(sdcon)));
    card::register_effect(d, c, e3, false);
    // be target
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_code(d, e4, EVENT_CHAIN_SOLVED);
    effect::set_operation(d, e4, Some(operation(desop1)));
    card::register_effect(d, c, e4, false);
    let e5 = effect::clone(d, e4);
    effect::set_code(d, e5, EVENT_BATTLED);
    effect::set_operation(d, e5, Some(operation(desop2)));
    card::register_effect(d, c, e5, false);
    let e6 = effect::clone(d, e4);
    effect::set_property(d, e6, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e6, EVENT_BECOME_TARGET);
    effect::set_operation(d, e6, Some(operation(register)));
    card::register_effect(d, c, e6, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp and Duel.GetAttackTarget()==nil
    a.ep != a.tp && duel::get_attack_target(d).is_none()
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_field_group(d, a.ep, LOCATION_HAND, 0);
    if g.is_empty() {
        return;
    }
    let sg = group::random_select(d, &g, 1 - a.tp, 1);
    duel::send_to_grave(d, sg, REASON_DISCARD | REASON_EFFECT, None, None);
}
fn sdcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_owner_target_count(d, c) > 0
}
fn register(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, c, ID + 1, RESET_CHAIN, 0, 1, a.ev as i64, 0);
}
fn desop1(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    // if not g or not g:IsContains(c) then return false end
    let Some(g) = g else { return };
    if !group::is_contains(&g, c) {
        return;
    }
    // for _,ch in ipairs({c:GetFlagEffectLabel(id+1)}) do
    for ch in card::get_flag_effect_label(d, c, ID + 1) {
        if ch == a.ev as i64 {
            let ph = duel::get_current_phase(d);
            if (ph & (PHASE_DAMAGE | PHASE_DAMAGE_CAL)) != 0 && !duel::is_damage_calculated(d) {
                card::register_flag_effect(d, c, ID, RESET_PHASE | PHASE_DAMAGE | RESETS_STANDARD, 0, 1, 0, 0);
            } else if card::is_has_effect(d, c, EFFECT_DISABLE, None).is_empty() && !card::is_disabled(d, c) {
                duel::destroy(d, c, REASON_EFFECT, None, None);
            }
            return;
        }
    }
}
fn desop2(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, c, ID) > 0 && card::is_has_effect(d, c, EFFECT_DISABLE, None).is_empty() && !card::is_disabled(d, c) {
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
