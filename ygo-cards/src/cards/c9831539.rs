//! Tannhauser Gate
use super::prelude::*;

pub const ID: u32 = 9831539;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c,tp)`
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLevelAbove(1) and c:IsAttackBelow(1000)
    //     and Duel.IsExistingTarget(s.filter2,tp,LOCATION_MZONE,0,1,c,c:GetRace())
    card::is_level_above(d, c, 1)
        && card::is_attack_below(d, c, 1000)
        && {
            let rac = card::get_race(d, c);
            duel::is_existing_target(d, Some(&|d, c2| filter2(d, c2, rac)), tp, LOCATION_MZONE, 0, 1, c.into())
        }
}
/// `s.filter2(c,rac)`
fn filter2(d: &mut Duel, c: CardId, rac: u64) -> bool {
    // return c:IsLevelAbove(1) and c:IsAttackBelow(1000) and c:IsRace(rac)
    card::is_level_above(d, c, 1) && card::is_attack_below(d, c, 1000) && card::is_race(d, c, rac)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter1(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g1 = duel::select_target(d, tp, Some(&|d, c| filter1(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter2,tp,LOCATION_MZONE,0,1,1,g1:GetFirst(),g1:GetFirst():GetRace())
    let first = group::get_first(d, &g1).nil();
    let rac = card::get_race(d, first);
    duel::select_target(d, tp, Some(&|d, c| filter2(d, c, rac)), tp, LOCATION_MZONE, 0, 1, 1, false, first.into());
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let g = g[0].as_group().cloned().nil();
    // local tc1=g:GetFirst()
    // local tc2=g:GetNext()
    let mut it = group::iter(d, &g).into_iter();
    let tc1 = it.next().nil();
    let tc2 = it.next();
    // if tc1:IsRelateToEffect(e) and tc1:IsFaceup() and tc2:IsRelateToEffect(e) and tc2:IsFaceup() then
    if card::is_relate_to_effect(d, tc1, a.e) && card::is_faceup(d, tc1) {
        let tc2 = tc2.nil();
        if card::is_relate_to_effect(d, tc2, a.e) && card::is_faceup(d, tc2) {
            // local lv=tc1:GetLevel()+tc2:GetLevel()
            let lv = card::get_level(d, tc1) + card::get_level(d, tc2);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_LEVEL_FINAL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL_FINAL);
            // e1:SetValue(lv)
            effect::set_value(d, e1, lv as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc1:RegisterEffect(e1)
            card::register_effect(d, tc1, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // tc2:RegisterEffect(e2)
            card::register_effect(d, tc2, e2, false);
        }
    }
}
