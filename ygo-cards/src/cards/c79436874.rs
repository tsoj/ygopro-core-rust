//! R.B. VALCan Rocket
use super::prelude::*;

pub const ID: u32 = 79436874;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RB], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon procedure from hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    card::register_effect(d, c, e1, false);
    // Destroy up to 2 cards your opponent controls, also this card, and if you do, inflict 500 damage to your opponent for each card destroyed
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    if a.card(1).is_none() {
        return true;
    }
    let tp = effect::get_handler_player(d, a.e(0));
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, 0, NO_EX);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && (g.is_empty() || group::filter_count(d, &g, &|d, x| card::is_set_card(d, x, &[SET_RB as u16]), NO_EX) == g.len())
}
fn desconfilter(d: &mut Duel, c: CardId, ec: CardId, lg: &CardSet) -> bool {
    card::is_set_card(d, c, &[SET_RB as u16])
        && card::is_link_monster(d, c)
        && card::is_faceup(d, c)
        && (lg.contains(c) || card::get_linked_group(d, c).contains(ec))
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let lg = card::get_linked_group(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| desconfilter(d, x, c, &lg)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let mut g = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 2, false, NO_EX).nil();
    let c = effect::get_handler(d, a.e).nil();
    group::add_card(&mut g, &CardSet::from_card(c));
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, (g.len() * 500) as i32);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let mut tg = duel::get_target_cards(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        group::add_card(&mut tg, &CardSet::from_card(c));
    }
    if tg.is_empty() {
        return;
    }
    let dam = duel::destroy(d, &tg, REASON_EFFECT, None, None) as i64 * 500;
    if dam > 0 {
        duel::damage(d, 1 - a.tp, dam, REASON_EFFECT, false, None);
    }
}
