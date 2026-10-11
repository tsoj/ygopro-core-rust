//! Bujin Hirume
use super::prelude::*;

pub const ID: u32 = 9418365;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // discard
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e2, CATEGORY_HANDES);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.hdcon)
    effect::set_condition(d, e2, Some(condition(hdcon)));
    // e2:SetTarget(s.hdtg)
    effect::set_target(d, e2, Some(cost_target(hdtg)));
    // e2:SetOperation(s.hdop)
    effect::set_operation(d, e2, Some(operation(hdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spfilter(c,tp)
fn spfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_BUJIN) and c:IsMonster() and not c:IsCode(id) and c:IsAbleToRemoveAsCost()
    //     and aux.SpElimFilter(c,true)
    //     and (Duel.GetLocationCount(tp,LOCATION_MZONE)>0 or (c:IsLocation(LOCATION_MZONE) and c:GetSequence()<5))
    card::is_set_card(d, c, &[SET_BUJIN as u16])
        && card::is_monster(d, c)
        && !card::is_code(d, c, &[ID])
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
        && (duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            || (card::is_location(d, c, LOCATION_MZONE) && card::get_sequence(d, c) < 5))
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, cc| spfilter(d, cc, tp)), tp, LOCATION_GRAVE | LOCATION_MZONE, 0, NO_EX);
    // return aux.SelectUnselectGroup(g,e,tp,1,1,aux.ChkfMMZ(1),0)
    let chkf = aux::chkf_mmz(1);
    let rs: aux::Rescon = &chkf;
    matches!(
        aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(1), Some(rs), 0, tp, None, None, None, false),
        Err(true)
    )
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, cc| spfilter(d, cc, tp)), tp, LOCATION_GRAVE | LOCATION_MZONE, 0, NO_EX);
    // local rg=aux.SelectUnselectGroup(g,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE,nil,nil,true)
    let chkf = aux::chkf_mmz(1);
    let rs: aux::Rescon = &chkf;
    let rg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(1), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true)
        .unwrap_or_default();
    // if #rg>0 then
    if !rg.is_empty() {
        // e:SetLabelObject(rg)
        effect::set_label_object(d, e, Value::Group(rg));
        // return true
        return true;
    }
    // return false
    false
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp,c)
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local rg=e:GetLabelObject()
    // if not rg then return end
    if let Value::Group(rg) = effect::get_label_object(d, e) {
        // Duel.Remove(rg,POS_FACEUP,REASON_COST)
        duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    }
}
// function s.hdcon(e,tp,eg,ep,ev,re,r,rp)
fn hdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:GetSummonType()==SUMMON_TYPE_SPECIAL+1 and rp~=tp and c:IsReason(REASON_DESTROY)
    //     and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    card::get_summon_type(d, c) == SUMMON_TYPE_SPECIAL + 1
        && a.rp != a.tp
        && card::is_reason(d, c, REASON_DESTROY)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, a.tp)
}
// function s.hdtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn hdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)>0
    //     and Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) > 0
            && duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,PLAYER_ALL,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, PLAYER_ALL as u8, 1);
    true
}
// function s.hdop(e,tp,eg,ep,ev,re,r,rp)
fn hdop(d: &mut Duel, a: &EffArgs) {
    // if Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD)~=0 then
    if duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) != 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.DiscardHand(1-tp,nil,1,1,REASON_EFFECT|REASON_DISCARD)
        duel::discard_hand(d, 1 - a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
