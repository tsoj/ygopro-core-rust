//! Hyper Psychic Riser
use super::prelude::*;

pub const ID: u32 = 99115354;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // limit attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(effect_card_fn(atktg)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot activate
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetValue(s.aclimit)
    effect::set_value_fn(d, e2, lua_fn(aclimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // special summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.thcon)
    effect::set_condition(d, e3, Some(condition(thcon)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.atktg(e,c)
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetAttack()<e:GetHandler():GetAttack()
    let h = effect::get_handler(d, e).nil();
    card::get_attack(d, c) < card::get_attack(d, h)
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let (e, re) = (a.e(0), a.e(1));
    // local loc=re:GetActivateLocation()
    let loc = effect::get_activate_location(d, re);
    // return loc==LOCATION_MZONE and re:IsMonsterEffect() and re:GetHandler():GetAttack()>e:GetHandler():GetAttack()
    if loc != LOCATION_MZONE || !effect::is_monster_effect(d, re) {
        return false;
    }
    let rh = effect::get_handler(d, re).nil();
    let h = effect::get_handler(d, e).nil();
    card::get_attack(d, rh) > card::get_attack(d, h)
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and c:IsReason(REASON_DESTROY)
    a.rp == 1 - a.tp && card::is_reason(d, c, REASON_DESTROY)
}
// function s.thfilter(c,e)
fn thfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsMonster() and c:IsCanBeEffectTarget(e) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_can_be_effect_target(d, c, Some(e)) && card::is_able_to_hand(d, c, None)
}
// function s.thcheck(sg,e,tp,mg)
fn thcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetAttribute)==1
    //     and sg:GetClassCount(Card.GetRace)==1
    //     and sg:IsExists(Card.IsType,1,nil,TYPE_TUNER)
    //     and sg:IsExists(aux.NOT(Card.IsType),1,nil,TYPE_TUNER)
    let res = group::get_class_count(d, sg, &|d, c| card::get_attribute(d, c) as i64) == 1
        && group::get_class_count(d, sg, &|d, c| card::get_race(d, c) as i64) == 1
        && group::is_exists(d, sg, &|d, c| card::is_type(d, c, TYPE_TUNER), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| !card::is_type(d, c, TYPE_TUNER), 1, NO_EX);
    (res, false)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let tp = a.tp;
    let e = a.e;
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_GRAVE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| thfilter(d, c, e)), tp, LOCATION_GRAVE, 0, NO_EX);
    let rs: aux::Rescon = &thcheck;
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,s.thcheck,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true));
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.thcheck,1,tp,HINTMSG_ATOHAND)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,sg,#sg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(sg.clone()), sg.len() as u8, 0, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
    }
}
