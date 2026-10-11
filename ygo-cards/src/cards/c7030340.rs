//! Factory of 100 Machines
use super::prelude::*;

pub const ID: u32 = 7030340;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter1(c)
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MORPHTRONIC) and c:IsMonster() and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::is_set_card(d, c, &[SET_MORPHTRONIC as u16])
        && card::is_monster(d, c)
        && card::is_able_to_remove(d, c, None, None, None)
        && aux::sp_elim_filter(d, c, true, false)
}
// function s.filter2(c,sg)
fn filter2(d: &mut Duel, c: CardId, sg: Option<&CardSet>) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_MACHINE) and not sg:IsContains(c)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE) && !group::is_contains(sg.nil(), c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter2(chkc) end
    if let Some(chkc) = a.chkc() {
        // s.filter2(chkc) is called without sg: sg:IsContains errors like Lua does
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter2(d, chkc, None);
    }
    // local rg=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&filter1), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return #rg>0 and Duel.IsExistingTarget(s.filter2,tp,LOCATION_MZONE,0,1,nil,rg) end
    if a.chk == Some(0) {
        return rg.len() > 0 && duel::is_existing_target(d, Some(&|d, c| filter2(d, c, Some(&rg))), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter2,tp,LOCATION_MZONE,0,1,1,nil,rg)
    let _ = duel::select_target(d, tp, Some(&|d, c| filter2(d, c, Some(&rg))), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rg,#rg,0,0)
    let n = rg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(rg), n, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&filter1), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local ct=Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    let ct = duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::get_first_target(d).first().copied();
    // if ct>0 and tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if ct > 0 && card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(#g*200)
            effect::set_value(d, e1, g.len() as i64 * 200);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
