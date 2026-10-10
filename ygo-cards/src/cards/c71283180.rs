//! Tornado Bird
use super::prelude::*;

pub const ID: u32 = 71283180;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e)`
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToHand() and c:IsCanBeEffectTarget(e)
    card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsOnField() and s.filter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && filter(d, chkc, e);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil,e)
    let mut g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, e)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if #g<2 then
    if g.len() < 2 {
        // g:Clear()
        g = CardSet::new();
        // Duel.SetTargetCard(g)
        duel::set_target_card(d, g);
        // return
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local sg=g:Select(tp,2,2,nil)
    let sg = group::select(d, &g, tp, 2, 2, false, NO_EX).nil();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,sg,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(sg), 2, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    // if not tg then return end
    let tg = match duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned() {
        Some(tg) => tg,
        None => return,
    };
    // local dg=tg:Filter(Card.IsRelateToEffect,nil,e)
    let dg = group::filter(d, &tg, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #dg==2 then
    if dg.len() == 2 {
        // Duel.SendtoHand(dg,nil,REASON_EFFECT)
        duel::send_to_hand(d, dg, None, REASON_EFFECT, None);
    }
}
