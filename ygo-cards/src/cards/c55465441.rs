//! Give and Take
use super::prelude::*;

pub const ID: u32 = 55465441;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
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
// s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:GetLevel()>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE,1-tp)
    card::get_level(d, c) > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), Some(1 - tp), None)
}
// s.filter2(c)
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetLevel()~=0
    card::is_faceup(d, c) && card::get_level(d, c) != 0
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter2,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter2), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g1=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g1 = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_MZONE,0,1,1,nil)
    let g2 = duel::select_target(d, a.tp, Some(&filter2), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g2:GetFirst())
    let tc = group::get_first(d, &g2);
    effect::set_label_object(d, a.e, Value::Card(tc.nil()));
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g1,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g1), 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc1=e:GetLabelObject()
    let tc1 = effect::get_label_object(d, a.e).as_card();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
    // local tc2=g:GetFirst()
    let items = group::iter(d, &g);
    let mut tc2 = items.first().copied();
    // if tc2==tc1 then tc2=g:GetNext() end
    if tc2 == tc1 {
        tc2 = items.get(1).copied();
    }
    let tc2 = tc2.nil();
    // if tc2:IsRelateToEffect(e) and Duel.SpecialSummon(tc2,0,tp,1-tp,false,false,POS_FACEUP_DEFENSE)~=0
    //     and tc1:IsFaceup() and tc1:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc2, a.e)
        && duel::special_summon(d, tc2, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) != 0
        && card::is_faceup(d, tc1.nil())
        && card::is_relate_to_effect(d, tc1.nil(), a.e)
    {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_LEVEL)
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        // e1:SetValue(tc2:GetLevel())
        let lv = card::get_level(d, tc2);
        effect::set_value(d, e1, lv as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc1:RegisterEffect(e1)
        card::register_effect(d, tc1.nil(), e1, false);
    }
}
