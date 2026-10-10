//! Ryu Senshi
use super::prelude::*;

pub const ID: u32 = 49868263;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, false, false, vec![fusion::Mat::Code(75953262), fusion::Mat::Code(67957315)]);
    // negate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(discon)));
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    effect::set_target(d, e1, Some(cost_target(distg)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);
    // disable effect
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_operation(d, e2, Some(operation(disop2)));
    card::register_effect(d, c, e2, false);
    // disable
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_DISABLE);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, LOCATION_SZONE, LOCATION_SZONE);
    effect::set_target(d, e3, Some(effect_card_fn(distg2)));
    card::register_effect(d, c, e3, false);
    // self destroy
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_code(d, e4, EFFECT_SELF_DESTROY);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_target_range(d, e4, LOCATION_SZONE, LOCATION_SZONE);
    effect::set_target(d, e4, Some(effect_card_fn(distg2)));
    card::register_effect(d, c, e4, false);
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    let re = a.re.nil();
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::get_active_type(d, re) == TYPE_TRAP && duel::is_chain_disablable(d, a.ev as u8)
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    duel::negate_effect(d, a.ev as u8);
}
fn disop2(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    if effect::is_spell_effect(d, re) && effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
        if let Some(g) = g {
            let c = effect::get_handler(d, a.e).nil();
            if group::is_contains(&g, c) && duel::negate_effect(d, a.ev as u8) {
                let rc = effect::get_handler(d, re).nil();
                if card::is_relate_to_effect(d, rc, re) {
                    duel::destroy(d, rc, REASON_EFFECT, None, None);
                }
            }
        }
    }
}
fn distg2(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetCardTargetCount()>0 and c:IsSpell() and c:GetCardTarget():IsContains(e:GetHandler())
    card::get_card_target_count(d, c) > 0 && card::is_spell(d, c) && {
        let t = card::get_card_target(d, c);
        group::is_contains(&t, effect::get_handler(d, e).nil())
    }
}
