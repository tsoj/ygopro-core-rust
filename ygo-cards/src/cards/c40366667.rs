//! Dominus Impulse
use super::prelude::*;

pub const ID: u32 = 40366667;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate a card or effect that includes an effect that Special Summons a monster(s)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re:IsHasCategory(CATEGORY_SPECIAL_SUMMON) and Duel.IsChainDisablable(ev) end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If your opponent controls a card, you can activate this card from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    // e2:SetCondition(function(e) return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),0,LOCATION_ONFIELD)>0 end)
    effect::set_condition(d, e2, Some(condition(e2_condition)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(e,tp,eg,ep,ev,re,r,rp) return re:IsHasCategory(CATEGORY_SPECIAL_SUMMON) and Duel.IsChainDisablable(ev) end
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    effect::is_has_category(d, a.re.nil(), CATEGORY_SPECIAL_SUMMON) && duel::is_chain_disablable(d, a.ev as u8)
}
// s.distg(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local act_from_hand_chk=e:IsHasType(EFFECT_TYPE_ACTIVATE) and e:GetHandler():IsStatus(STATUS_ACT_FROM_HAND) and 1 or 0
    let act_from_hand_chk = if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && card::is_status(d, effect::get_handler(d, a.e).nil(), STATUS_ACT_FROM_HAND) { 1 } else { 0 };
    // e:SetLabel(act_from_hand_chk)
    effect::set_label(d, a.e, &[act_from_hand_chk]);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil());
    // if rc:IsRelateToEffect(re) and rc:IsDestructable()
    //     and Duel.IsExistingMatchingCard(Card.IsTrap,tp,LOCATION_GRAVE,0,1,nil) then
    if card::is_relate_to_effect(d, rc.nil(), a.re.nil())
        && card::is_destructable(d, rc.nil(), None)
        && duel::is_existing_matching_card(d, Some(&card::is_trap), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
    {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
    true
}
// s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re)
    //     and Duel.IsExistingMatchingCard(Card.IsTrap,tp,LOCATION_GRAVE,0,1,nil) then
    if duel::negate_effect(d, a.ev as u8) && card::is_relate_to_effect(d, effect::get_handler(d, a.re.nil()).nil(), a.re.nil()) && duel::is_existing_matching_card(d, Some(&card::is_trap), a.tp, LOCATION_GRAVE, 0, 1, NO_EX) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(eg,REASON_EFFECT)
        duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
    }
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) and e:GetLabel()==1 then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && effect::get_label(d, a.e) == 1 {
        // You cannot activate the effects of LIGHT, EARTH, and WIND monsters for the rest of this Duel
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetValue(function(_,re) return re:IsMonsterEffect() and re:GetHandler():IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_EARTH|ATTRIBUTE_WIND) end)
        effect::set_value_fn(d, e1, lua_fn(cannot_activate_value));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
// function(_,re) return re:IsMonsterEffect() and re:GetHandler():IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_EARTH|ATTRIBUTE_WIND) end
fn cannot_activate_value(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    effect::is_monster_effect(d, re) && card::is_attribute(d, effect::get_handler(d, re).nil(), ATTRIBUTE_LIGHT | ATTRIBUTE_EARTH | ATTRIBUTE_WIND)
}
// function(e) return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),0,LOCATION_ONFIELD)>0 end
fn e2_condition(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, effect::get_handler_player(d, a.e), 0, LOCATION_ONFIELD) > 0
}
