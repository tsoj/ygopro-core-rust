//! Spell Card "Soul Exchange"
use super::prelude::*;

pub const ID: u32 = 15735108;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Tribute Summon 1 monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SUMMON + CATEGORY_SET);
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
    // Your opponent can apply this card's effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetOperation(s.opsumop)
    effect::set_operation(d, e2, Some(operation(opsumop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end` (of `e2`)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
/// `s.sumfilter(c,ec)`
fn sumfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // local e1=Effect.CreateEffect(ec)
    let e1 = effect::create_effect(d, ec);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e1:SetCode(EFFECT_ADD_EXTRA_TRIBUTE)
    effect::set_code(d, e1, EFFECT_ADD_EXTRA_TRIBUTE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(POS_FACEUP|POS_FACEDOWN)
    effect::set_value(d, e1, (POS_FACEUP | POS_FACEDOWN) as i64);
    // e1:SetReset(RESET_CHAIN)
    effect::set_reset(d, e1, RESET_CHAIN, 1);
    // c:RegisterEffect(e1,true)
    card::register_effect(d, c, e1, true);
    // local res=c:CanSummonOrSet(true,nil,1)
    let res = card::can_summon_or_set(d, c, true, None, 1, None);
    // e1:Reset()
    effect::reset(d, e1);
    // return res
    res
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.sumfilter,tp,LOCATION_HAND,0,1,nil,e:GetHandler()) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, x| sumfilter(d, x, c)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    activate_for(d, a.e, a.tp);
}
/// The body of `s.activate` for the player `tp` (`s.opsumop` calls it for the opponent)
fn activate_for(d: &mut Duel, e: EffectId, tp: u8) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.sumfilter,tp,LOCATION_HAND,0,1,1,nil,c):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, x| sumfilter(d, x, c)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Can Tribute monsters your opponent controls
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_ADD_EXTRA_TRIBUTE)
        effect::set_code(d, e1, EFFECT_ADD_EXTRA_TRIBUTE);
        // e1:SetTargetRange(0,LOCATION_MZONE)
        effect::set_target_range(d, e1, 0, LOCATION_MZONE);
        // e1:SetValue(POS_FACEUP|POS_FACEDOWN)
        effect::set_value(d, e1, (POS_FACEUP | POS_FACEDOWN) as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
        // Tribute Summon it
        // Duel.SummonOrSet(tp,tc,true,nil,1)
        duel::summon_or_set(d, tp, tc, true, None, 1, None);
        // Cannot be tributed this turn
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(3303)
        effect::set_description(d, e2, 3303);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
        // e2:SetCode(EFFECT_UNRELEASABLE_SUM)
        effect::set_code(d, e2, EFFECT_UNRELEASABLE_SUM);
        // e2:SetValue(1)
        effect::set_value(d, e2, 1);
        // e2:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD)|RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD) | RESET_PHASE | PHASE_END, 1);
        // tc:RegisterEffect(e2,true)
        card::register_effect(d, tc, e2, true);
        // local e3=e2:Clone()
        let e3 = effect::clone(d, e2);
        // e3:SetCode(EFFECT_UNRELEASABLE_NONSUM)
        effect::set_code(d, e3, EFFECT_UNRELEASABLE_NONSUM);
        // tc:RegisterEffect(e3,true)
        card::register_effect(d, tc, e3, true);
    }
}
/// `s.opsumop(e,tp,eg,ep,ev,re,r,rp)`
fn opsumop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.IsExistingMatchingCard(s.sumfilter,1-tp,LOCATION_HAND,0,1,nil,e:GetHandler())
    //     and Duel.SelectYesNo(1-tp,aux.Stringid(id,2)) then
    let c = effect::get_handler(d, e).nil();
    if duel::is_existing_matching_card(d, Some(&|d, x| sumfilter(d, x, c)), 1 - tp, LOCATION_HAND, 0, 1, NO_EX) && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 2)) {
        // s.activate(e,1-tp,eg,ep,ev,re,r,rp)
        activate_for(d, e, 1 - tp);
    }
}
