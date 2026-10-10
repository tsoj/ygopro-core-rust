//! Masked HERO Dark Law
use super::prelude::*;

pub const ID: u32 = 58481572;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Must be Special Summoned by "Mask Change"
    // c:AddCannotBeSpecialSummoned()
    card::add_cannot_be_special_summoned(d, c);
    // Any card sent to your opponent's GY is banished instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_RANGE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_TO_GRAVE_REDIRECT)
    effect::set_code(d, e1, EFFECT_TO_GRAVE_REDIRECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_ALL,LOCATION_ALL)
    effect::set_target_range(d, e1, LOCATION_ALL, LOCATION_ALL);
    // e1:SetTarget(function(e,c) local tp=e:GetHandlerPlayer() return not c:IsOwner(tp) and Duel.IsPlayerCanRemove(tp,c) end)
    effect::set_target(d, e1, Some(effect_card_fn(redirect_target)));
    // e1:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e1, LOCATION_REMOVED as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish 1 random card from your opponent's hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e2, EVENT_TO_HAND);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.handrmcon)
    effect::set_condition(d, e2, Some(condition(handrmcon)));
    // e2:SetTarget(s.handrmtg)
    effect::set_target(d, e2, Some(cost_target(handrmtg)));
    // e2:SetOperation(s.handrmop)
    effect::set_operation(d, e2, Some(operation(handrmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,c) local tp=e:GetHandlerPlayer() return not c:IsOwner(tp) and Duel.IsPlayerCanRemove(tp,c) end`
fn redirect_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let tp = effect::get_handler_player(d, e);
    !card::is_owner(d, c, tp) && duel::is_player_can_remove(d, tp, Some((c, None)))
}
fn handrmconfilter(d: &mut Duel, c: CardId, opp: u8) -> bool {
    // return c:IsPreviousControler(opp) and c:IsPreviousLocation(LOCATION_DECK) and c:IsControler(opp)
    card::is_previous_controler(d, c, opp) && card::is_previous_location(d, c, LOCATION_DECK) && card::is_controler(d, c, opp)
}
fn handrmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsDrawPhase() and eg:IsExists(s.handrmconfilter,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    let opp = 1 - a.tp;
    !duel::is_draw_phase(d, None) && group::is_exists(d, &eg, &|d, c| handrmconfilter(d, c, opp), 1, NO_EX)
}
fn handrmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_HAND,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_HAND, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_HAND as i32);
    true
}
fn handrmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_HAND,nil):RandomSelect(tp,1)
    let all = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_HAND, NO_EX);
    let g = group::random_select(d, &all, tp, 1);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
