//! Naturia Barkion
use super::prelude::*;

pub const ID: u32 = 2956282;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro summon
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_EARTH, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_EARTH, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    card::enable_revive_limit(d, c);
    // Negate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(discon)));
    effect::set_cost(d, e1, Some(cost_target(discost)));
    effect::set_target(d, e1, Some(cost_target(distg)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
        && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE)
        && effect::is_trap_effect(d, a.re.nil())
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, false, false)
}
/// `s.discost(e,tp,...,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 2, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 2, 2, false, NO_EX).nil();
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rh, re) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_faceup(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
