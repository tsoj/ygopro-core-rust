//! Audhumla, Progenitor of the Frozen Expanse
use super::prelude::*;

pub const ID: u32 = 101402101;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If 4 or more cards are on the field: You can Special Summon this card from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, a| duel::get_field_group_count(d, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD) >= 4)));
    effect::set_target(d, e1, Some(cost_target(sptg1)));
    effect::set_operation(d, e1, Some(operation(spop1)));
    card::register_effect(d, c, e1, false);
    // You can send up to 2 Spells/Traps you control to the GY; draw that many cards, then you can Special Summon 1 WATER monster from your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DRAW + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost_target(drcost)));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop_op)));
    card::register_effect(d, c, e2, false);
    // When your opponent activates a card or effect (Quick Effect): You can change this card to face-down Defense Position, also for the rest of this turn, all face-down cards you control cannot be destroyed by your opponent's card effects, and your opponent cannot target them with card effects
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_POSITION + CATEGORY_SET);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_CHAINING);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    effect::set_condition(d, e3, Some(condition(|_d, a| a.ep == 1 - a.tp)));
    effect::set_target(d, e3, Some(cost_target(postg)));
    effect::set_operation(d, e3, Some(operation(posop)));
    card::register_effect(d, c, e3, false);
}
// e1's target: function(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg1(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, c) = (a.tp, effect::get_handler(d, a.e).nil());
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
// e1's operation
fn spop1(d: &mut Duel, a: &EffArgs) {
    let (c, tp) = (effect::get_handler(d, a.e).nil(), a.tp);
    // if c:IsRelateToEffect(e) then Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP) end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// s.drcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let f = |d: &mut Duel, c: CardId| card::is_spell_trap(d, c) && card::is_able_to_grave_as_cost(d, c);
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.AND(Card.IsSpellTrap,Card.IsAbleToGraveAsCost),tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&f), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // local max_count=Duel.IsPlayerCanDraw(tp,2) and 2 or 1
    let max_count = if duel::is_player_can_draw(d, tp, 2) { 2 } else { 1 };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.AND(Card.IsSpellTrap,Card.IsAbleToGraveAsCost),tp,LOCATION_ONFIELD,0,1,max_count,nil)
    let g = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_ONFIELD, 0, 1, max_count, false, NO_EX).nil();
    // e:GetChainData().draw_count=#g
    chain::set_effect_data(d, e, None, chain::field::CARD, Value::Int(g.len() as i64));
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    // local draw_count=e:GetChainData().draw_count
    let draw_count = chain::effect_data(d, a.e, None, chain::field::CARD).int();
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,draw_count)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, draw_count as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
// s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local draw_count=e:GetChainData().draw_count
    let draw_count = chain::effect_data(d, e, None, chain::field::CARD).int();
    // if Duel.Draw(tp,draw_count,REASON_EFFECT)==draw_count and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) then
    if duel::draw(d, tp, draw_count as u32, REASON_EFFECT) as i64 == draw_count
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX)
    {
        // if not Duel.SelectYesNo(tp,aux.Stringid(id,3)) then return Duel.ShuffleHand(tp) end
        if !duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
            duel::shuffle_hand(d, tp);
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
// s.postg(e,tp,eg,ep,ev,re,r,rp,chk)
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanTurnSet() end
    if a.chk == Some(0) {
        return card::is_can_turn_set(d, c);
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,c,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(c)), 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
// s.posop(e,tp,eg,ep,ev,re,r,rp)
fn posop(d: &mut Duel, a: &EffArgs) {
    let (c, tp) = (effect::get_handler(d, a.e).nil(), a.tp);
    // if c:IsRelateToEffect(e) then Duel.ChangePosition(c,POS_FACEDOWN_DEFENSE) end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::change_position(d, CardSet::from_card(c), POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,4))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 4)), 0, None);
    // Also for the rest of this turn, all face-down cards you control cannot be destroyed by your opponent's card effects, and your opponent cannot target them with card effects
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    // e1a:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e1a, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e1a:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1a, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1a:SetTargetRange(LOCATION_ONFIELD,0)
    effect::set_target_range(d, e1a, LOCATION_ONFIELD, 0);
    // e1a:SetTarget(function(e,c) return c:IsFacedown() end)
    effect::set_target(d, e1a, Some(effect_card_fn(|d, _e, c| card::is_facedown(d, c))));
    // e1a:SetValue(aux.indoval)
    effect::set_value_fn(d, e1a, aux::indoval());
    // e1a:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1a, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1a,tp)
    duel::register_effect(d, e1a, tp);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1b, EFFECT_FLAG_SET_AVAILABLE | EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1b:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1b, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1b:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1b, aux::tgoval());
    // Duel.RegisterEffect(e1b,tp)
    duel::register_effect(d, e1b, tp);
}
