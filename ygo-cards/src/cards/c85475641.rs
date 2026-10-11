//! Infernity Avenger
use super::prelude::*;

pub const ID: u32 = 85475641;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsReason(REASON_BATTLE) and c:HasLevel()
    //     and c:IsControler(tp) and c:IsPreviousControler(tp) and c:GetCode()~=id
    card::is_location(d, c, LOCATION_GRAVE)
        && card::is_reason(d, c, REASON_BATTLE)
        && card::has_level(d, c)
        && card::is_controler(d, c, tp)
        && card::is_previous_controler(d, c, tp)
        && card::get_code(d, c)[0] != ID
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=eg:Filter(s.filter,nil,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, a.tp), NO_EX);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // e:SetLabel(tc:GetLevel())
        let lv = card::get_level(d, tc);
        effect::set_label(d, a.e, &[lv as i64]);
        // return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)==0
        duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) == 0
    } else {
        // else return false end
        false
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // if Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)~=0 or not c:IsRelateToEffect(e) then return end
    if duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) != 0 || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon_step(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
    // e1:SetValue(e:GetLabel())
    let lv = effect::get_label(d, a.e);
    effect::set_value(d, e1, lv);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
