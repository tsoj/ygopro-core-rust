//! Fusion Recovery
use super::prelude::*;

pub const ID: u32 = 18511384;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_POLYMERIZATION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Polymerization" and 1 monster in your GY that was used as material for a Fusion Summon to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_POLYMERIZATION) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_POLYMERIZATION]) && card::is_able_to_hand(d, c, None)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return (c:GetReason()&(REASON_FUSION|REASON_MATERIAL))==(REASON_FUSION|REASON_MATERIAL) and c:IsMonster() and c:IsAbleToHand()
    (card::get_reason(d, c) & (REASON_FUSION | REASON_MATERIAL)) == (REASON_FUSION | REASON_MATERIAL) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_GRAVE,0,1,nil) and Duel.IsExistingTarget(s.filter2,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter1), a.tp, LOCATION_GRAVE, 0, 1, NO_EX) && duel::is_existing_target(d, Some(&filter2), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_GRAVE,0,1,1,nil)
    let mut g1 = duel::select_target(d, a.tp, Some(&filter1), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_GRAVE,0,1,1,nil)
    let g2 = duel::select_target(d, a.tp, Some(&filter2), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g1,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g1), 2, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
    // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}
