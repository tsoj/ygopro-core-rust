//! Chaos Core
use super::prelude::*;

pub const ID: u32 = 54040484;

// s.listed_names={69890967,6007213,32491822}
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[69890967, 6007213, 32491822], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x202)
    card::enable_counter_permit(d, c, 0x202, None, None);
    // --Add counter when targeted
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_COUNTER + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_BECOME_TARGET)
    effect::set_code(d, e1, EVENT_BECOME_TARGET);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.ctcon)
    effect::set_condition(d, e1, Some(condition(ctcon)));
    // e1:SetTarget(s.cttg)
    effect::set_target(d, e1, Some(cost_target(cttg)));
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e2, EVENT_BE_BATTLE_TARGET);
    // e2:SetCondition(s.ctcon)
    effect::set_condition(d, e2, Some(condition(ctcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Destruction replacement
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetTarget(s.reptg)
    effect::set_target(d, e3, Some(cost_target(reptg)));
    // e3:SetOperation(s.repop)
    effect::set_operation(d, e3, Some(operation(repop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.ctcon(e,tp,eg,ep,ev,re,r,rp)
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsContains(e:GetHandler())
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    group::is_contains(&eg, h)
}
// function s.ctfilter(c)
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(69890967,6007213,32491822) and c:IsAbleToGrave()
    card::is_code(d, c, &[69890967, 6007213, 32491822]) && card::is_able_to_grave(d, c, None)
}
// function s.ctcheck(sg,e,tp)
fn ctcheck(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetCode)==#sg and e:GetHandler():IsCanAddCounter(0x202,#sg)
    let n = sg.len();
    let h = effect::get_handler(d, e.nil()).nil();
    let ok = group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == n
        && card::is_can_add_counter(d, h, 0x202, n as u16, false, 0);
    (ok, false)
}
// function s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.ctfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil)
    //   and e:GetHandler():IsCanAddCounter(0x202,1) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&ctfilter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
            && card::is_can_add_counter(d, h, 0x202, 1, false, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,0,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 0, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,0,0,0x202)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 0, 0, 0x202);
    true
}
// function s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(s.ctfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&ctfilter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,3,s.ctcheck,1,tp,HINTMSG_TOGRAVE)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(3), Some(&ctcheck), 1, a.tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // if #sg>0 and Duel.SendtoGrave(sg,REASON_EFFECT)~=0 then
    if !sg.is_empty() && duel::send_to_grave(d, &sg, REASON_EFFECT, None, None) != 0 {
        // local ct=sg:FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)
        let ct = group::filter_count(d, &sg, &|d, x| card::is_location(d, x, LOCATION_GRAVE), NO_EX);
        // if ct>0 and c:IsRelateToEffect(e) and c:IsFaceup() then
        if ct > 0 && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
            // local oc=#(Duel.GetOperatedGroup())
            let oc = duel::get_operated_group(d).len();
            // c:AddCounter(0x202,oc)
            card::add_counter(d, c, 0x202, oc as u16, false);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
            effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
            // e1:SetTargetRange(1,0)
            effect::set_target_range(d, e1, 1, 0);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, a.tp);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT+EFFECT_FLAG_OATH)
            effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_OATH, 0);
            // e2:SetDescription(aux.Stringid(id,2))
            effect::set_description(d, e2, aux::stringid(ID, 2));
            // e2:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
            // e2:SetTargetRange(1,0)
            effect::set_target_range(d, e2, 1, 0);
            // Duel.RegisterEffect(e2,tp)
            duel::register_effect(d, e2, a.tp);
        }
    }
}
// function s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE) and c:IsCanRemoveCounter(tp,0x202,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE) && card::is_can_remove_counter(d, c, a.tp, 0x202, 1, REASON_EFFECT);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
// function s.repop(e,tp,eg,ep,ev,re,r,rp)
fn repop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RemoveCounter(tp,0x202,1,REASON_EFFECT)
    let c = effect::get_handler(d, a.e).nil();
    card::remove_counter(d, c, a.tp, 0x202, 1, REASON_EFFECT);
}
