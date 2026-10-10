//! Dark Blade the Dragon Knight
use super::prelude::*;

pub const ID: u32 = 86805855;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(11321183), fusion::Mat::Code(47415292)]);
    // remove
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e1, Some(condition(rmcon)));
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    effect::set_operation(d, e1, Some(operation(rmop)));
    card::register_effect(d, c, e1, false);
}
fn rmcon(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep != a.tp
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemove() and aux.SpElimFilter(c)
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, false, false)
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, 3, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
