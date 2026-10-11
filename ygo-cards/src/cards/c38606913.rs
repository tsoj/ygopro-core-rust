//! Cubic Causality
use super::prelude::*;

pub const ID: u32 = 38606913;

const COUNTER_CUBIC: u16 = 0x1038;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x1038], listed_series: &[SET_CUBIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // counters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetHintTiming(0,TIMING_BATTLE_START)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_BATTLE_START));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e2:SetOperation(s.damact)
    effect::set_operation(d, e2, Some(operation(damact)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CUBIC)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_CUBIC as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&cfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // if ct>0 then
    if ct > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        // local sg=g:Select(tp,1,ct,nil)
        let sg = group::select(d, &g, tp, 1, ct as u16, false, NO_EX).nil();
        // local ac=sg:GetFirst()
        let _ = group::get_first(d, &sg);
        // for ac in aux.Next(sg) do
        for ac in group::iter(d, &sg) {
            // ac:AddCounter(0x1038,1)
            card::add_counter(d, ac, COUNTER_CUBIC, 1, false);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
            // e1:SetCondition(s.condition)
            effect::set_condition(d, e1, Some(condition(cond)));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // ac:RegisterEffect(e1)
            card::register_effect(d, ac, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e2, EFFECT_DISABLE);
            // ac:RegisterEffect(e2)
            card::register_effect(d, ac, e2, false);
        }
    }
}
/// `s.condition(e)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetCounter(0x1038)>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, COUNTER_CUBIC) > 0
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CUBIC)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_CUBIC as u16])
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and s.tgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.damact(e,tp,eg,ep,ev,re,r,rp)`
fn damact(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
        // e1:SetCode(EVENT_LEAVE_FIELD_P)
        effect::set_code(d, e1, EVENT_LEAVE_FIELD_P);
        // e1:SetOperation(s.shchk)
        effect::set_operation(d, e1, Some(operation(shchk)));
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e2, aux::stringid(ID, 2));
        // e2:SetCategory(CATEGORY_DAMAGE)
        effect::set_category(d, e2, CATEGORY_DAMAGE);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_BATTLE_DESTROYING)
        effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
        // e2:SetCondition(s.shcon)
        effect::set_condition(d, e2, Some(condition(shcon)));
        // e2:SetOperation(s.damop)
        effect::set_operation(d, e2, Some(operation(damop)));
        // e2:SetLabelObject(tc)
        effect::set_label_object(d, e2, Value::Card(tc));
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_PHASE_END&~(RESET_LEAVE|RESET_TOGRAVE),0,1)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !(RESET_LEAVE | RESET_TOGRAVE)), 0, 1, 0, 0);
        // tc:CreateEffectRelation(e1)
        card::create_effect_relation(d, tc, e1);
    }
}
/// `s.shchk(e,tp,eg,ep,ev,re,r,rp)`
fn shchk(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    // if not tc or not tc:IsRelateToEffect(e) then return end
    let Some(tc) = effect::get_label_object(d, a.e).as_card() else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local bt=tc:GetBattleTarget()
    let bt = card::get_battle_target(d, tc);
    // for tc in aux.Next(eg) do
    let eg = a.eg.clone().nil();
    for x in group::iter(d, &eg) {
        // if bt==tc and tc:GetCounter(0x1038) > 0 then
        if bt == Some(x) && card::get_counter(d, x, COUNTER_CUBIC) > 0 {
            // tc:RegisterFlagEffect(id+2,RESET_CHAIN,0,1)
            card::register_flag_effect(d, x, ID + 2, RESET_CHAIN, 0, 1, 0, 0);
        }
    }
}
/// `s.shcon(e,tp,eg,ep,ev,re,r,rp)`
fn shcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    // local bt=tc:GetBattleTarget()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    let bt = card::get_battle_target(d, tc);
    // return tc and tc:GetFlagEffect(id)~=0 and bt and bt:GetFlagEffect(id+2)~=0
    if card::get_flag_effect(d, tc, ID) == 0 {
        return false;
    }
    match bt {
        Some(b) => card::get_flag_effect(d, b, ID + 2) != 0,
        None => false,
    }
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // local bt=tc:GetBattleTarget()
    let bt = card::get_battle_target(d, tc).nil();
    // local atk=bt:GetBaseAttack()
    let atk = card::get_base_attack(d, bt);
    // Duel.Damage(1-tp,atk,REASON_EFFECT)
    duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
}
