//! Falchion β
//! (ファルシオンβ)
use super::prelude::*;

pub const ID: u32 = 86170989;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsAttackBelow(1200) and c:IsAbleToGrave()
    card::is_race(d, c, RACE_MACHINE) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_attack_below(d, c, 1200) && card::is_able_to_grave(d, c, None)
}
/// `s.filter2(c,e,tp)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsAttackBelow(1200)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_MACHINE) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_attack_below(d, c, 1200) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter2(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter2(d, chkc, e, tp);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local op=0
    let mut op = 0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
    // local t1=Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_DECK,0,1,nil)
    let t1 = duel::is_existing_matching_card(d, Some(&filter1), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local t2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter2,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    let t2 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, c| filter2(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // if t1 and t2 then
    if t1 && t2 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))+1
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]) + 1;
    // elseif t1 then
    } else if t1 {
        // Duel.SelectOption(tp,aux.Stringid(id,1))
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1)]);
        // op=1
        op = 1;
    // elseif t2 then
    } else if t2 {
        // Duel.SelectOption(tp,aux.Stringid(id,2))
        duel::select_option(d, tp, true, &[aux::stringid(ID, 2)]);
        // op=2
        op = 2;
    }
    // e:SetLabel(op)
    effect::set_label(d, e, &[op as i64]);
    // if op==2 then
    if op == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_target(d, tp, Some(&|d, c| filter2(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
    // elseif op==1 then
    } else if op == 1 {
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
        // e:SetProperty(0)
        effect::set_property(d, e, 0, 0);
        // e:SetCategory(CATEGORY_TOGRAVE)
        effect::set_category(d, e, CATEGORY_TOGRAVE);
    } else {
        // e:SetProperty(0)
        effect::set_property(d, e, 0, 0);
        // e:SetCategory(0)
        effect::set_category(d, e, 0);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if e:GetLabel()==2 then
    let label = effect::get_label(d, e);
    if label == 2 {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d);
        // if tc and tc:IsRelateToEffect(e) then
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, e) {
                // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    // elseif e:GetLabel()==1 then
    } else if label == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter1,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&filter1), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoGrave(g,REASON_EFFECT)
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    }
}
