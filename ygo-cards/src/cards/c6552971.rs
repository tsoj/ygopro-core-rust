//! A.I. Meet You
use super::prelude::*;

pub const ID: u32 = 6552971;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_IGNISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reveal 1 Cyberse monster and add 1 "@Ignister" monster to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_CYBERSE) and c:GetAttack()==2300 and not c:IsPublic()
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,c)
    card::is_race(d, c, RACE_CYBERSE)
        && card::get_attack(d, c) == 2300
        && !card::is_public(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// function s.filter(c,rc)
fn filter(d: &mut Duel, c: CardId, rc: CardId) -> bool {
    // return c:IsSetCard(SET_IGNISTER) and c:IsAttribute(rc:GetAttribute()) and c:IsAbleToHand()
    let att = card::get_attribute(d, rc);
    card::is_set_card(d, c, &[SET_IGNISTER as u16]) && card::is_attribute(d, c, att) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND|LOCATION_EXTRA,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, a.tp)), a.tp, LOCATION_HAND | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND|LOCATION_EXTRA,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, x| cfilter(d, x, a.tp)), a.tp, LOCATION_HAND | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let rc = group::get_first(d, &sel);
    // if rc then
    if let Some(rc) = rc {
        // Duel.ConfirmCards(1-tp,rc)
        duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(rc));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,rc)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| filter(d, x, rc)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, &g);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
        // e1:SetOperation(s.regop)
        effect::set_operation(d, e1, Some(operation(regop)));
        // e1:SetLabel(rc:GetCode())
        let code = card::get_code(d, rc)[0] as i64;
        effect::set_label(d, e1, &[code]);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
        // e2:SetCountLimit(1)
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        // e2:SetCondition(s.damcon)
        effect::set_condition(d, e2, Some(condition(damcon)));
        // e2:SetOperation(s.damop)
        effect::set_operation(d, e2, Some(operation(damop)));
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // e2:SetLabelObject(e1)
        effect::set_label_object(d, e2, Value::Effect(e1));
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    }
    // Can only activate the effects of Cyberse monsters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT+EFFECT_FLAG_OATH)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_OATH, 0);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetValue(s.aclimit)
    effect::set_value_fn(d, e3, lua_fn(aclimit));
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
}
// function s.aclimit(e,re,tp)
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.e(1)).nil();
    // return re:IsMonsterEffect() and not rc:IsRace(RACE_CYBERSE)
    effect::is_monster_effect(d, a.e(1)) && !card::is_race(d, rc, RACE_CYBERSE)
}
// function s.nmfilter(c,tp,code)
fn nmfilter(d: &mut Duel, c: CardId, tp: u8, code: i64) -> bool {
    // return c:IsCode(code) and c:IsSummonPlayer(tp)
    card::is_code(d, c, &[code as u32]) && card::is_summon_player(d, c, tp)
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabel()==0 then return end
    let label = effect::get_label(d, a.e);
    if label == 0 {
        return;
    }
    // if eg:IsExists(s.nmfilter,1,nil,tp,e:GetLabel()) then
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| nmfilter(d, c, a.tp, label), 1, NO_EX) {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
    }
}
// function s.damcon(e,tp,eg,ep,ev,re,r,rp)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetLabelObject():GetLabel()~=0
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::get_label(d, e1) != 0
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Damage(tp,2300,REASON_EFFECT)
    duel::damage(d, a.tp, 2300, REASON_EFFECT, false, None);
}
