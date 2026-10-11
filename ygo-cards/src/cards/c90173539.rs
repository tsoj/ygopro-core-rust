//! World Dino Wrestling
//! anime version scripted by Larry126, updates by Naim
use super::prelude::*;

pub const ID: u32 = 90173539;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DINOWRESTLER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // attack limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetCondition(s.atkcon1)
    effect::set_condition(d, e2, Some(lua_fn(atkcon1)));
    // e2:SetTarget(s.atktg1)
    effect::set_target(d, e2, Some(effect_card_fn(atktg1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetOperation(s.checkop)
    effect::set_operation(d, e3, Some(operation(checkop)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // atk up
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e4, EFFECT_UPDATE_ATTACK);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e4, LOCATION_MZONE, 0);
    // e4:SetTarget(s.atktg2)
    effect::set_target(d, e4, Some(effect_card_fn(atktg2)));
    // e4:SetCondition(s.atkcon2)
    effect::set_condition(d, e4, Some(lua_fn(atkcon2)));
    // e4:SetValue(200)
    effect::set_value(d, e4, 200);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // special summon
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e5, LOCATION_GRAVE);
    // e5:SetCountLimit(1,id)
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    // e5:SetCondition(s.spcon)
    effect::set_condition(d, e5, Some(condition(spcon)));
    // e5:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e5, Some(cost::self_banish()));
    // e5:SetTarget(s.sptg)
    effect::set_target(d, e5, Some(cost_target(sptg)));
    // e5:SetOperation(s.spop)
    effect::set_operation(d, e5, Some(operation(spop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.atkcon1(e)
fn atkcon1(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0 and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_DINOWRESTLER),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let e = a.e(0);
    let h = effect::get_handler(d, e).nil();
    card::get_flag_effect(d, h, ID) != 0
        && duel::is_existing_matching_card(
            d,
            Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_DINOWRESTLER as u16]))),
            effect::get_handler_player(d, e),
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        )
}
// function s.atktg1(e,c)
fn atktg1(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetFieldID()~=e:GetLabel()
    card::get_field_id(d, c) as i64 != effect::get_label(d, e)
}
// function s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():GetFlagEffect(id)~=0 then return end
    let h = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, h, ID) != 0 {
        return;
    }
    // local fid=eg:GetFirst():GetFieldID()
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    let fid = card::get_field_id(d, first);
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    // e:GetLabelObject():SetLabel(fid)
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, e2, &[fid as i64]);
}
// function s.atktg2(e,c)
fn atktg2(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_DINOWRESTLER) and Duel.GetTurnPlayer()==e:GetHandlerPlayer() and Duel.GetAttacker()==c
    card::is_set_card(d, c, &[SET_DINOWRESTLER as u16])
        && duel::get_turn_player(d) == effect::get_handler_player(d, e)
        && duel::get_attacker(d) == Some(c)
}
// function s.atkcon2(e)
fn atkcon2(d: &mut Duel, _a: Args) -> bool {
    // return Duel.IsPhase(PHASE_DAMAGE_CAL) and Duel.GetAttackTarget()~=nil
    duel::is_phase(d, PHASE_DAMAGE_CAL) && duel::get_attack_target(d).is_some()
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)
    duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DINOWRESTLER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_DINOWRESTLER as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    if a.chk == Some(0) {
        // and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
