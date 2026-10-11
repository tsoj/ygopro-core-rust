//! Mikanko Water Arabesque
//! 御巫の水舞踏
use super::prelude::*;

pub const ID: u32 = 43527730;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MIKANKO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // --Prevent destruction by effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Special Summon 1 "Mikanko" monster, re-equip and return equip target to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsSetCard(SET_MIKANKO) and not c:IsOriginalCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and e:GetHandler():CheckEquipTarget(c)
    if !card::is_set_card(d, c, &[SET_MIKANKO as u16]) {
        return false;
    }
    if card::is_original_code(d, c, &[code]) {
        return false;
    }
    if !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(0xff)) {
        return false;
    }
    let h = effect::get_handler(d, e).nil();
    card::check_equip_target(d, h, c)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local eqc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let eqc = card::get_equip_target(d, h);
    // if chk==0 then return eqc and eqc:IsAbleToHand() and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp,eqc:GetOriginalCode()) end
    if a.chk == Some(0) {
        let Some(eqc) = eqc else { return false };
        if !card::is_able_to_hand(d, eqc, None) {
            return false;
        }
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        let code = card::get_original_code(d, eqc);
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp, code)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,eqc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, eqc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local eqc=c:GetEquipTarget()
    let eqc = card::get_equip_target(d, c);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp,eqc:GetOriginalCode())
    let code = card::get_original_code(d, eqc.nil());
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp, code)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local eq=g:GetFirst()
        let eq = group::get_first(d, &g).nil();
        // if Duel.Equip(tp,c,eq) and eqc:IsFaceup() and eqc:IsAbleToHand() then
        if duel::equip(d, tp, c, eq, true, false) {
            let eqc = eqc.nil();
            if card::is_faceup(d, eqc) && card::is_able_to_hand(d, eqc, None) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoHand(eqc,nil,REASON_EFFECT)
                duel::send_to_hand(d, eqc, None, REASON_EFFECT, None);
            }
        }
    }
}
