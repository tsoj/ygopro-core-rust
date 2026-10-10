//! Destruction of Destiny
use super::prelude::*;

pub const ID: u32 = 62980542;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // discard deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_DECKDES+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DECKDES + CATEGORY_DAMAGE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.distg(e,tp,...,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,3) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 3);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(3)
    duel::set_target_param(d, 3);
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,3)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 3);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsSpellTrap()
    card::is_location(d, c, LOCATION_GRAVE) && card::is_spell_trap(d, c)
}
/// `s.disop(e,tp,...)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local p,val=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.DiscardDeck(p,val,REASON_EFFECT)
    duel::discard_deck(d, info[0].int() as u8, info[1].int() as u16, REASON_EFFECT);
    // local g=Duel.GetOperatedGroup()
    let g = duel::get_operated_group(d);
    // local ct=g:FilterCount(s.filter,nil)
    let ct = group::filter_count(d, &g, &filter, NO_EX);
    // if ct>0 then
    if ct > 0 {
        // Duel.Damage(tp,ct*1000,REASON_EFFECT)
        duel::damage(d, a.tp, ct as i64 * 1000, REASON_EFFECT, false, None);
    }
}
