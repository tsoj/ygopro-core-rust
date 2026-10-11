//! Blackwing - Hillen the Tengu-wind
use super::prelude::*;

pub const ID: u32 = 3072808;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLACKWING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_DUEL as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and ev>=2000 and Duel.GetAttackTarget()==nil
    a.ep == a.tp && a.ev >= 2000 && duel::get_attack_target(d).is_none()
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(3) and c:IsSetCard(SET_BLACKWING) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 3)
        && card::is_set_card(d, c, &[SET_BLACKWING as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,e:GetHandler(),e,tp) end
    if a.chk == Some(0) {
        if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
            return false;
        }
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 1 {
            return false;
        }
        let h = effect::get_handler(d, a.e).nil();
        if !card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None) {
            return false;
        }
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,e:GetHandler(),e,tp)
    let h = effect::get_handler(d, a.e).nil();
    let mut g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, h.into()).nil();
    // g:AddCard(e:GetHandler())
    group::add_card(&mut g, &CardSet::from_card(h));
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 2, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // if not c:IsRelateToEffect(e) or not c:IsCanBeSpecialSummoned(e,0,tp,false,false) then return end
    if !card::is_relate_to_effect(d, c, a.e) || !card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None) {
        return;
    }
    // if not tc:IsRelateToEffect(e) or not tc:IsCanBeSpecialSummoned(e,0,tp,false,false) then return end
    let tc = tc.nil();
    if !card::is_relate_to_effect(d, tc, a.e) || !card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, None, None, None) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_DISABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // tc:RegisterEffect(e3)
    card::register_effect(d, tc, e3, false);
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // tc:RegisterEffect(e4)
    card::register_effect(d, tc, e4, false);
    // Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon_step(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
