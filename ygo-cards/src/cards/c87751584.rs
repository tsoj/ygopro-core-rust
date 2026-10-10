//! Gatling Dragon
use super::prelude::*;

pub const ID: u32 = 87751584;

pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon Procedure
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(81480460), fusion::Mat::Code(25551951)]);
    // Toss 3 coins and destroy monsters on the field
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_COIN);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 3);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if g.is_empty() {
        return;
    }
    let mut ct = duel::count_heads(&duel::toss_coin(d, a.tp, 3));
    if ct == 0 {
        return;
    }
    if ct > g.len() {
        ct = g.len();
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let dg = group::select(d, &g, a.tp, ct as u16, ct as u16, false, NO_EX).nil();
    duel::hint_selection(d, dg.clone(), true);
    duel::destroy(d, dg, REASON_EFFECT, None, None);
}
