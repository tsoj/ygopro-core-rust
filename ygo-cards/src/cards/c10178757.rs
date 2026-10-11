//! R-Genex Oracle
use super::prelude::*;

pub const ID: u32 = 10178757;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GENEX], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e1, EVENT_TO_HAND);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // synchro limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CANNOT_BE_SYNCHRO_MATERIAL)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetValue(s.synlimit)
    effect::set_value_fn(d, e2, effect_card_fn(synlimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.synlimit(e,c)`
fn synlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // if not c then return false end
    // return not c:IsSetCard(SET_GENEX)
    !card::is_set_card(d, c, &[SET_GENEX as u16])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_EFFECT)>0 and re:GetHandler():IsSetCard(SET_GENEX)
    //     and e:GetHandler():GetPreviousLocation()==LOCATION_DECK and e:GetHandler():IsPreviousControler(tp)
    if (a.r & REASON_EFFECT) == 0 {
        return false;
    }
    let re_h = effect::get_handler(d, a.re.nil()).nil();
    if !card::is_set_card(d, re_h, &[SET_GENEX as u16]) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    card::get_previous_location(d, h) == LOCATION_DECK && card::is_previous_controler(d, h, a.tp)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
