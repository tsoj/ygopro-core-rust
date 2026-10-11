//! Zefraxa, Flame Beast of the Nekroz
use super::prelude::*;

pub const ID: u32 = 20773176;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NEKROZ, SET_ZEFRA],
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // --splimit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE, 0);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(s.splimit)
    effect::set_target(d, e2, Some(lua_fn(splimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --spsummon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e3:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_HAND | LOCATION_GRAVE);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(condition_fn)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(operation_fn)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // if c:IsSetCard({SET_NEKROZ,SET_ZEFRA}) then return false end
    let c = a.c(1);
    if card::is_set_card(d, c, &[SET_NEKROZ as u16, SET_ZEFRA as u16]) {
        return false;
    }
    // return (sumtype&SUMMON_TYPE_PENDULUM)==SUMMON_TYPE_PENDULUM
    let sumtype = a.int(3) as u64;
    (sumtype & SUMMON_TYPE_PENDULUM as u64) == SUMMON_TYPE_PENDULUM as u64
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReason(REASON_BATTLE|REASON_EFFECT)
    //     and c:IsSetCard({SET_NEKROZ,SET_ZEFRA}) and not c:IsCode(id)
    //     and c:IsPreviousControler(tp)
    //     and ((c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousPosition(POS_FACEUP))
    //     or c:IsPreviousLocation(LOCATION_PZONE))
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::is_set_card(d, c, &[SET_NEKROZ as u16, SET_ZEFRA as u16])
        && !card::is_code(d, c, &[ID])
        && card::is_previous_controler(d, c, tp)
        && ((card::is_previous_location(d, c, LOCATION_MZONE) && card::is_previous_position(d, c, POS_FACEUP as u32)) || card::is_previous_location(d, c, LOCATION_PZONE))
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.filter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter(d, c, a.tp), 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
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
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
