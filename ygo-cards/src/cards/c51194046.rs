//! Qliphort Monolith
use super::prelude::*;

pub const ID: u32 = 51194046;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_QLI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // splimit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE, 0);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(s.splimit)
    effect::set_target(d, e2, Some(effect_card_fn(splimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // draw
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetRange(LOCATION_PZONE)
    effect::set_range(d, e3, LOCATION_PZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.drcon)
    effect::set_condition(d, e3, Some(condition(drcon)));
    // e3:SetTarget(s.drtg)
    effect::set_target(d, e3, Some(cost_target(drtg)));
    // e3:SetOperation(s.drop)
    effect::set_operation(d, e3, Some(operation(drop_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // s[0]=0
        set_s_var(d, ID, 0, Value::Int(0));
        // s[1]=0
        set_s_var(d, ID, 1, Value::Int(0));
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_SUMMON_SUCCESS)
        effect::set_code(d, ge1, EVENT_SUMMON_SUCCESS);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // local ge2=ge1:Clone()
        let ge2 = effect::clone(d, ge1);
        // ge2:SetCode(EVENT_MSET)
        effect::set_code(d, ge2, EVENT_MSET);
        // Duel.RegisterEffect(ge2,0)
        duel::register_effect(d, ge2, 0);
        // local ge3=Effect.CreateEffect(c)
        let ge3 = effect::create_effect(d, c);
        // ge3:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, ge3, EFFECT_TYPE_FIELD);
        // ge3:SetCode(EFFECT_MATERIAL_CHECK)
        effect::set_code(d, ge3, EFFECT_MATERIAL_CHECK);
        // ge3:SetTargetRange(LOCATION_HAND,LOCATION_HAND)
        effect::set_target_range(d, ge3, LOCATION_HAND, LOCATION_HAND);
        // ge3:SetValue(s.valcheck)
        effect::set_value_fn(d, ge3, effect_card_value(valcheck));
        // Duel.RegisterEffect(ge3,0)
        duel::register_effect(d, ge3, 0);
        // ge1:SetLabelObject(ge3)
        effect::set_label_object(d, ge1, Value::Effect(ge3));
        // ge2:SetLabelObject(ge3)
        effect::set_label_object(d, ge2, Value::Effect(ge3));
        // aux.AddValuesReset(function()
        //     s[0]=0
        //     s[1]=0
        // end)
        aux::add_values_reset(d, reset_values);
    });
}
// s.listed_series={SET_QLI}
// function s.splimit(e,c)
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_QLI)
    !card::is_set_card(d, c, &[SET_QLI as u16])
}
// function s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // if tc:IsTributeSummoned() then
    if card::is_tribute_summoned(d, tc) {
        // local p=tc:GetSummonPlayer()
        let p = card::get_summon_player(d, tc) as u64;
        // s[p]=s[p]+e:GetLabelObject():GetLabel()
        let ge3 = effect::get_label_object(d, a.e).as_effect().nil();
        let lbl = effect::get_label(d, ge3);
        let cur = s_var(d, ID, p).int();
        set_s_var(d, ID, p, Value::Int(cur + lbl));
    }
}
// function s.valcheck(e,c)
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local ct=c:GetMaterial():FilterCount(Card.IsSetCard,nil,SET_QLI)
    let mat = card::get_material(d, c);
    let ct = group::filter_count(d, &mat, &|d, x| card::is_set_card(d, x, &[SET_QLI as u16]), NO_EX);
    // e:SetLabel(ct)
    effect::set_label(d, e, &[ct as i64]);
    // (no return value: nil, read as 0 by the core)
    0
}
// function s.clearop ... is never referenced by the script's effects: omitted (see aux.AddValuesReset below)
/// `aux.AddValuesReset(function() s[0]=0 s[1]=0 end)`
fn reset_values(d: &mut Duel) {
    // s[0]=0
    set_s_var(d, ID, 0, Value::Int(0));
    // s[1]=0
    set_s_var(d, ID, 1, Value::Int(0));
}
// function s.drcon(e,tp,eg,ep,ev,re,r,rp)
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return s[tp]>0
    s_var(d, ID, a.tp as u64).int() > 0
}
// function s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,s[tp]) end
    if a.chk == Some(0) {
        let n = s_var(d, ID, tp as u64).int();
        return duel::is_player_can_draw(d, tp, n as u32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,s[tp])
    let n = s_var(d, ID, tp as u64).int();
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, n as i32);
    true
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.Draw(tp,s[tp],REASON_EFFECT)
    let n = s_var(d, ID, a.tp as u64).int();
    duel::draw(d, a.tp, n as u32, REASON_EFFECT);
}
