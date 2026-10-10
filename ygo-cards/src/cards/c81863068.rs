//! Hiro's Shadow Scout
use super::prelude::*;

pub const ID: u32 = 81863068;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(3)
    duel::set_target_param(d, 3);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,1-tp,3)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, 1 - a.tp, 3);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dn = info[1].int() as u32;
    // if Duel.Draw(p,d,REASON_EFFECT)==0 then return end
    if duel::draw(d, p, dn, REASON_EFFECT) == 0 {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // local g=Duel.GetOperatedGroup()
    let g = duel::get_operated_group(d);
    // Duel.ConfirmCards(1-p,g)
    duel::confirm_cards(d, 1 - p, &g);
    // local dg=g:Filter(Card.IsSpell,nil)
    let dg = group::filter(d, &g, &card::is_spell, NO_EX);
    // Duel.SendtoGrave(dg,REASON_EFFECT|REASON_DISCARD)
    duel::send_to_grave(d, dg, REASON_EFFECT | REASON_DISCARD, None, None);
    // Duel.ShuffleHand(p)
    duel::shuffle_hand(d, p);
}
