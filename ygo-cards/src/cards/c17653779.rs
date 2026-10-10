//! Fairy's Hand Mirror
use super::prelude::*;

pub const ID: u32 = 17653779;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(tgcon)));
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    effect::set_operation(d, e1, Some(operation(tgop)));
    card::register_effect(d, c, e1, false);
}
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    if a.rp == a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) || !effect::is_spell_effect(d, re) || !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let Some(g) = info.first().and_then(|v| v.as_group().cloned()) else {
        return false;
    };
    if g.len() != 1 {
        return false;
    }
    let tc = group::get_first(d, &g).nil();
    effect::set_label_object(d, a.e, Value::Card(tc));
    card::is_location(d, tc, LOCATION_MZONE)
}
/// `s.filter(c,ct)`
fn filter(d: &mut Duel, c: CardId, ct: u32) -> bool {
    duel::check_chain_target(d, ct as u8, c)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let ev = a.ev;
    let lo = effect::get_label_object(d, a.e).as_card();
    if let Some(chkc) = a.chkc() {
        return Some(chkc) != lo && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc, ev);
    }
    let ex = lo.map_or(NO_EX, Exclude::Card);
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, ev)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, ex);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    duel::select_target(d, tp, Some(&|d, c| filter(d, c, ev)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, ex);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
    let first = group::get_first(d, &g).nil();
    if card::is_relate_to_effect(d, first, a.e) {
        duel::change_target_card(d, a.ev as u8, &g);
    }
}
