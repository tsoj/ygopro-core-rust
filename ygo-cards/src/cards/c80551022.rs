//! Mimighoul Slime
use super::prelude::*;

pub const ID: u32 = 80551022;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MIMIGHOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // FLIP: Apply these effects in sequence
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(main_phase)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card to your opponent's field in face-down Defense Position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.selfsptg)
    effect::set_target(d, e2, Some(cost_target(selfsptg)));
    // e2:SetOperation(s.selfspop)
    effect::set_operation(d, e2, Some(operation(selfspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function() return Duel.IsMainPhase() end`
fn main_phase(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - a.tp, LOCATION_DECK as i32);
    // if c:IsAbleToChangeControler() then
    if card::is_able_to_change_controler(d, c) {
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(c)), 1, a.tp, 0);
    }
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MIMIGHOUL) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MIMIGHOUL as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local p=1-tp
    let p = 1 - a.tp;
    // local break_chk=false
    let mut break_chk = false;
    // Your opponent can Special Summon 1 "Mimighoul" monster from their Deck to their field
    // local g=Duel.GetMatchingGroup(s.spfilter,p,LOCATION_DECK,0,nil,e,p)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, p)), p, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.GetLocationCount(p,LOCATION_MZONE,p)>0 and Duel.SelectYesNo(p,aux.Stringid(id,2)) then
    if !g.is_empty()
        && duel::get_location_count(d, p, LOCATION_MZONE as u8, Some(p), None, None).0 > 0
        && duel::select_yes_no(d, p, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(p,1,1,nil)
        let sg = group::select(d, &g, p, 1, 1, false, NO_EX).nil();
        // break_chk=#sg>0 and Duel.SpecialSummon(sg,0,p,p,false,false,POS_FACEUP)>0
        break_chk = !sg.is_empty() && duel::special_summon(d, sg, 0, p, p, false, false, POS_FACEUP as u8, None) > 0;
    }
    // Give control of this card to your opponent
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // if break_chk then Duel.BreakEffect() end
        if break_chk {
            duel::break_effect(d);
        }
        // Duel.GetControl(c,p)
        duel::get_control(d, c, p, 0, 0, None, None);
    }
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return (Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE,1-tp))
    // or (Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) end
    if a.chk == Some(0) {
        let b1 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), Some(1 - tp), None);
        if b1 {
            return true;
        }
        return duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > duel::get_field_group_count(d, tp, LOCATION_MZONE, 0)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local b1=Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE,1-tp)
    let b1 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
        && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), Some(1 - tp), None);
    // local b2=Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let b2 = duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > duel::get_field_group_count(d, tp, LOCATION_MZONE, 0)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    // local op=nil
    // if b1 and b2 then
    //     op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,3)}, {b2,aux.Stringid(id,4)})
    // else
    //     op=(b1 and 1) or (b2 and 2) or 1
    // end
    let op = if b1 && b2 {
        duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]).nil()
    } else if b1 {
        1
    } else if b2 {
        2
    } else {
        1
    };
    if op == 1 {
        // if Duel.SpecialSummon(c,0,tp,1-tp,false,false,POS_FACEDOWN_DEFENSE)==0 then return end
        if duel::special_summon(d, c, 0, tp, 1 - tp, false, false, POS_FACEDOWN_DEFENSE as u8, None) == 0 {
            return;
        }
        // Duel.ConfirmCards(tp,c)
        duel::confirm_cards(d, tp, c);
    } else if op == 2 {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
