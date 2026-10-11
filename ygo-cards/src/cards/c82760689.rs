//! Lucky Cloud
use super::prelude::*;

pub const ID: u32 = 82760689;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CLOUDIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // s[0]=false
        // s[1]=Group.CreateGroup()
        // s[1]:KeepAlive()
        reset_values(d);
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
        // local ge2=Effect.CreateEffect(c)
        let ge2 = effect::create_effect(d, c);
        // ge2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
        effect::set_code(d, ge2, EVENT_FLIP_SUMMON_SUCCESS);
        // ge2:SetOperation(s.checkop)
        effect::set_operation(d, ge2, Some(operation(checkop)));
        // Duel.RegisterEffect(ge2,0)
        duel::register_effect(d, ge2, 0);
        // local ge3=Effect.CreateEffect(c)
        let ge3 = effect::create_effect(d, c);
        // ge3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge3:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, ge3, EVENT_SPSUMMON_SUCCESS);
        // ge3:SetOperation(s.checkop)
        effect::set_operation(d, ge3, Some(operation(checkop)));
        // Duel.RegisterEffect(ge3,0)
        duel::register_effect(d, ge3, 0);
        // aux.AddValuesReset(function() s[0]=false s[1]:Clear() end)
        aux::add_values_reset(d, reset_values);
    });
}
/// `s[1]` as a group (the script variable holds a group)
fn s1(d: &Duel) -> CardSet {
    match s_var(d, ID, 1) {
        Value::Group(g) => g,
        _ => CardSet::new(),
    }
}
/// `s[0]` as a flag
fn s0(d: &Duel) -> bool {
    matches!(s_var(d, ID, 0), Value::Bool(true))
}
/// `s[0]=false; s[1]=Group.CreateGroup()` (also the values reset)
fn reset_values(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Bool(false));
    set_s_var(d, ID, 1, Value::Group(CardSet::new()));
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let _ = group::get_first(d, &eg);
    // for tc in aux.Next(eg) do
    for tc in group::iter(d, &eg) {
        // if not s[0] and tc:IsFaceup() and tc:IsSetCard(SET_CLOUDIAN) then
        if !s0(d) && card::is_faceup(d, tc) && card::is_set_card(d, tc, &[SET_CLOUDIAN as u16]) {
            let mut g = s1(d);
            // if s[1]:IsContains(tc) then s[0]=true
            if group::is_contains(&g, tc) {
                set_s_var(d, ID, 0, Value::Bool(true));
            // else s[1]:AddCard(tc) end
            } else {
                group::add_card(&mut g, &CardSet::from_card(tc));
                set_s_var(d, ID, 1, Value::Group(g));
            }
        }
    }
}
/// `s.filter(c,g)`
fn filter(d: &mut Duel, c: CardId, g: &CardSet) -> bool {
    // return g:IsExists(Card.IsCode,1,c,c:GetCode())
    let codes = card::get_code(d, c);
    group::is_exists(d, g, &|d, x| card::is_code(d, x, &codes), 1, c.into())
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return s[0] or s[1]:IsExists(s.filter,1,nil,s[1])
    if s0(d) {
        return true;
    }
    let g = s1(d);
    group::is_exists(d, &g, &|d, c| filter(d, c, &g), 1, NO_EX)
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // Duel.Draw(tp,2,REASON_EFFECT)
    duel::draw(d, a.tp, 2, REASON_EFFECT);
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.drcon)
    effect::set_condition(d, e1, Some(condition(drcon)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
