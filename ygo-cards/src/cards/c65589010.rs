//! Dogmatika Nation
use super::prelude::*;

pub const ID: u32 = 65589010;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DOGMATIKA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_DOGMATIKA))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_DOGMATIKA as u16]))));
    // e2:SetValue(s.tgval)
    effect::set_value_fn(d, e2, lua_fn(tgval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLED)
    effect::set_code(d, e3, EVENT_BATTLED);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Send to the graveyard
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e4, CATEGORY_TOGRAVE);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetCondition(s.gycon)
    effect::set_condition(d, e4, Some(condition(gycon)));
    // e4:SetTarget(s.gytg)
    effect::set_target(d, e4, Some(cost_target(gytg)));
    // e4:SetOperation(s.gyop)
    effect::set_operation(d, e4, Some(operation(gyop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.tgval(e,re,rp)
fn tgval(d: &mut Duel, a: Args) -> bool {
    // return re:IsMonsterEffect() and re:GetHandler():IsSpecialSummoned() and re:GetHandler():IsSummonLocation(LOCATION_EXTRA)
    let re = a.e(1);
    effect::is_monster_effect(d, re) && {
        let h = effect::get_handler(d, re).nil();
        card::is_special_summoned(d, h) && card::is_summon_location(d, h, LOCATION_EXTRA)
    }
}
// function s.check(c,tp)
fn check(d: &mut Duel, c: Option<CardId>, tp: u8) -> bool {
    // return c and c:IsControler(tp) and c:IsSetCard(SET_DOGMATIKA)
    match c {
        Some(c) => card::is_controler(d, c, tp) && card::is_set_card(d, c, &[SET_DOGMATIKA as u16]),
        None => false,
    }
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetAttackTarget()~=nil and (s.check(Duel.GetAttacker(),tp) or s.check(Duel.GetAttackTarget(),tp)) end
    if a.chk == Some(0) {
        return duel::get_attack_target(d).is_some()
            && (check(d, duel::get_attacker(d), tp) || check(d, duel::get_attack_target(d), tp));
    }
    // if s.check(Duel.GetAttacker(),tp) then
    if check(d, duel::get_attacker(d), tp) {
        // Duel.SetTargetCard(Duel.GetAttackTarget())
        let at = duel::get_attack_target(d).nil();
        duel::set_target_card(d, at);
    } else {
        // Duel.SetTargetCard(Duel.GetAttacker())
        let att = duel::get_attacker(d).nil();
        duel::set_target_card(d, att);
    }
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
// function s.gycon(e,tp,eg,ep,ev,re,r,rp)
fn gycon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and c:IsReason(REASON_DESTROY)
    //   and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousPosition(POS_FACEUP)
    a.rp == 1 - tp
        && card::is_reason(d, c, REASON_DESTROY)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_position(d, c, POS_FACEUP as u32)
}
// function s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_EXTRA,0) * Duel.GetFieldGroupCount(tp,0,LOCATION_EXTRA) > 0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, LOCATION_EXTRA, 0) * duel::get_field_group_count(d, tp, 0, LOCATION_EXTRA) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,2,PLAYER_ALL,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 2, PLAYER_ALL as u8, LOCATION_EXTRA as i32);
    true
}
// function s.gyop(e,tp,eg,ep,ev,re,r,rp)
fn gyop(d: &mut Duel, _a: &EffArgs) {
    // local sg=Group.CreateGroup()
    let mut sg = CardSet::new();
    // for p=0,1 do
    for p in 0..2u8 {
        // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(p,Card.IsAbleToGrave,p,LOCATION_EXTRA,0,1,1,nil)
        let g = duel::select_matching_card(d, p, Some(&|d, c| card::is_able_to_grave(d, c, None)), p, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // sg:AddCard(g:GetFirst())
            let first = group::get_first(d, &g).nil();
            group::add_card(&mut sg, &CardSet::from_card(first));
        }
    }
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoGrave(sg,REASON_EFFECT)
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    }
}
