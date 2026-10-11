//! Crystal Master
use super::prelude::*;

pub const ID: u32 = 87475570;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CRYSTAL_BEAST, SET_ULTIMATE_CRYSTAL, SET_CRYSTAL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    pendulum::add_procedure(d, c, None, None);
    // cannot be target
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_range(d, e2, LOCATION_PZONE);
    effect::set_target_range(d, e2, LOCATION_ONFIELD, 0);
    effect::set_target(d, e2, Some(effect_card_fn(tgtg)));
    effect::set_value_fn(d, e2, aux::tgoval());
    card::register_effect(d, c, e2, false);
    // tohand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_TOHAND | CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_cost(d, e3, Some(cost::self_tribute()));
    effect::set_target(d, e3, Some(cost_target(target)));
    effect::set_operation(d, e3, Some(operation(op)));
    card::register_effect(d, c, e3, false);
}
// s.tgtg(e,c)
fn tgtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_CRYSTAL_BEAST) or (c:IsLocation(LOCATION_MZONE) and c:IsSetCard(SET_ULTIMATE_CRYSTAL))
    card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])
        || (card::is_location(d, c, LOCATION_MZONE) && card::is_set_card(d, c, &[SET_ULTIMATE_CRYSTAL as u16]))
}
// s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return (((c:IsSetCard(SET_CRYSTAL_BEAST) or c:IsSetCard(SET_ULTIMATE_CRYSTAL)) and c:IsMonster())
    //     or (c:IsSetCard(SET_CRYSTAL) and c:IsSpellTrap())) and c:IsAbleToHand()
    (((card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]) || card::is_set_card(d, c, &[SET_ULTIMATE_CRYSTAL as u16]))
        && card::is_monster(d, c))
        || (card::is_set_card(d, c, &[SET_CRYSTAL as u16]) && card::is_spell_trap(d, c)))
        && card::is_able_to_hand(d, c, None)
}
// s.target
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// s.operation
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
