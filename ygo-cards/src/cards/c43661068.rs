//! Star Light, Star Bright
use super::prelude::*;

pub const ID: u32 = 43661068;

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
fn filter(d: &mut Duel, c: CardId, atk: i32, def: i32) -> bool {
    // return c:IsFaceup() and c:GetLevel()>0 and (c:GetAttack()==atk or c:GetDefense()==def)
    card::is_faceup(d, c)
        && card::get_level(d, c) > 0
        && (card::get_attack(d, c) == atk || card::get_defense(d, c) == def)
}
fn tfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:GetLevel()>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,c,c:GetAttack(),c:GetDefense())
    if !(card::is_faceup(d, c) && card::get_level(d, c) > 0) {
        return false;
    }
    let atk = card::get_attack(d, c);
    let def = card::get_defense(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, atk, def)), tp, LOCATION_MZONE, 0, 1, c.into())
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && tfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.tfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    duel::select_target(d, a.tp, Some(&|d, c| tfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,0,tc,tc:GetAttack(),tc:GetDefense())
        let atk = card::get_attack(d, tc);
        let def = card::get_defense(d, tc);
        let g = duel::get_matching_group(d, Some(&|d, x| filter(d, x, atk, def)), a.tp, LOCATION_MZONE, 0, tc.into());
        // local lv=tc:GetLevel()
        let lv = card::get_level(d, tc);
        // local lc=g:GetFirst()
        let _ = group::get_first(d, &g);
        // for lc in aux.Next(g) do
        for lc in group::iter(d, &g) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_LEVEL_FINAL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL_FINAL);
            // e1:SetValue(lv)
            effect::set_value(d, e1, lv as i64);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // lc:RegisterEffect(e1)
            card::register_effect(d, lc, e1, false);
        }
    }
}
