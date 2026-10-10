//! Dominus Purge
use super::prelude::*;

pub const ID: u32 = 97045737;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate an effect that includes an effect to add a card(s) from the Deck to the hand
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
    // e1:SetCondition(s.discond)
    effect::set_condition(d, e1, Some(condition(discond)));
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can be activated from the hand if your opponent controls a card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    // e2:SetCondition(function(e) return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),0,LOCATION_ONFIELD)>0 end)
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),0,LOCATION_ONFIELD)>0 end`
fn e2_cond(d: &mut Duel, a: Args) -> bool {
    let p = effect::get_handler_player(d, a.e(0));
    duel::get_field_group_count(d, p, 0, LOCATION_ONFIELD) > 0
}
/// `s.discond(e,tp,eg,ep,ev,re,r,rp)`
fn discond(d: &mut Duel, a: &EffArgs) -> bool {
    // if not Duel.IsChainDisablable(ev) then return false end
    if !duel::is_chain_disablable(d, a.ev as u8) {
        return false;
    }
    let re = a.re.nil();
    // if re:IsHasCategory(CATEGORY_SEARCH) or re:IsHasCategory(CATEGORY_DRAW) then return true end
    if effect::is_has_category(d, re, CATEGORY_SEARCH) || effect::is_has_category(d, re, CATEGORY_DRAW) {
        return true;
    }
    // local ex1,g1,gc1,dp1,loc1=Duel.GetOperationInfo(ev,CATEGORY_TOHAND)
    let info1 = duel::get_operation_info(d, a.ev, CATEGORY_TOHAND);
    // local ex2,g2,gc2,dp2,loc2=Duel.GetPossibleOperationInfo(ev,CATEGORY_TOHAND)
    let info2 = duel::get_possible_operation_info(d, a.ev, CATEGORY_TOHAND);
    // local g=Group.CreateGroup()
    let mut g = CardSet::new();
    // if g1 then g:Merge(g1) end
    if let Some(g1) = info1.as_ref().and_then(|o| o.op_cards.clone()) {
        group::add_card(&mut g, &g1);
    }
    // if g2 then g:Merge(g2) end
    if let Some(g2) = info2.as_ref().and_then(|o| o.op_cards.clone()) {
        group::add_card(&mut g, &g2);
    }
    // return (((loc1 or 0)|(loc2 or 0))&LOCATION_DECK)>0 or (#g>0 and g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK))
    let loc1 = info1.as_ref().map_or(0, |o| o.op_param as i64);
    let loc2 = info2.as_ref().map_or(0, |o| o.op_param as i64);
    if ((loc1 | loc2) & LOCATION_DECK as i64) > 0 {
        return true;
    }
    !g.is_empty() && group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local act_from_hand_chk=e:IsHasType(EFFECT_TYPE_ACTIVATE) and e:GetHandler():IsStatus(STATUS_ACT_FROM_HAND) and 1 or 0
    let act_from_hand_chk = if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && {
        let h = effect::get_handler(d, a.e).nil();
        card::is_status(d, h, STATUS_ACT_FROM_HAND)
    } {
        1
    } else {
        0
    };
    // e:SetLabel(act_from_hand_chk)
    effect::set_label(d, a.e, &[act_from_hand_chk]);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if rc:IsRelateToEffect(re) and rc:IsDestructable() and Duel.IsExistingMatchingCard(Card.IsTrap,tp,LOCATION_GRAVE,0,1,nil) then
    if card::is_relate_to_effect(d, rc, re)
        && card::is_destructable(d, rc, None)
        && duel::is_existing_matching_card(d, Some(&card::is_trap), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
    {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re)
    //   and Duel.IsExistingMatchingCard(Card.IsTrap,tp,LOCATION_GRAVE,0,1,nil) then
    if duel::negate_effect(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) && duel::is_existing_matching_card(d, Some(&card::is_trap), a.tp, LOCATION_GRAVE, 0, 1, NO_EX) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) and e:GetLabel()==1 then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && effect::get_label(d, a.e) == 1 {
        // You cannot activate DARK/WATER/FIRE monster effects for the rest of this Duel if this card is activated from the hand
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetValue(function(_,re) return re:IsMonsterEffect() and re:GetHandler():IsAttribute(ATTRIBUTE_DARK|ATTRIBUTE_WATER|ATTRIBUTE_FIRE) end)
        effect::set_value_fn(d, e1, lua_fn(e1_value));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
/// `function(_,re) return re:IsMonsterEffect() and re:GetHandler():IsAttribute(ATTRIBUTE_DARK|ATTRIBUTE_WATER|ATTRIBUTE_FIRE) end`
fn e1_value(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_attribute(d, h, ATTRIBUTE_DARK | ATTRIBUTE_WATER | ATTRIBUTE_FIRE)
}
