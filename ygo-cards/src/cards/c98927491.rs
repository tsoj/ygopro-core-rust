//! Ambulance Rescueroid
use super::prelude::*;

pub const ID: u32 = 98927491;

pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_ROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,24311595,36378213)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(24311595), fusion::Mat::Code(36378213)]);
    // spsummon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.fuslimit)
    effect::set_value_fn(d, e1, aux::fuslimit());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // spsummon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.activate)
    effect::set_operation(d, e2, Some(operation(activate)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsReason(REASON_BATTLE) and c:IsPreviousControler(tp)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_location(d, c, LOCATION_GRAVE)
        && card::is_reason(d, c, REASON_BATTLE)
        && card::is_previous_controler(d, c, tp)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=eg:Filter(s.filter,nil,e,tp)
        let eg = a.eg.clone().nil();
        let g = group::filter(d, &eg, &|d, c| filter(d, c, e, tp), NO_EX);
        // e:SetLabelObject(g:GetFirst())
        let first = group::get_first(d, &g);
        effect::set_label_object(d, e, first.map_or(Value::Nil, Value::Card));
        // return #g~=0
        return !g.is_empty();
    }
    // Duel.SetTargetCard(e:GetLabelObject())
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetLabelObject(),1,0,0)
    let lo = effect::get_label_object(d, e).as_card();
    if let Some(tc) = lo {
        duel::set_target_card(d, CardSet::from_card(tc));
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, lo.map(CardSet::from_card), 1, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
