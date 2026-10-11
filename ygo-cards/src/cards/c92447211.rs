//! Long-Tailed Black Horse
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 92447211;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --If this card is Normal or Special Summoned: You can reveal 1 Zombie monster in your hand and send 1 EARTH Zombie monster from your Deck to the GY; ...
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_TOGRAVE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1a, CATEGORY_TOGRAVE + CATEGORY_ATKCHANGE);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetCost(Cost.AND(Cost.Reveal(s.revealfilter,false,1,1,s.revealcostop),s.tgcost))
    let revealfilter_fn: cost::CostFilter = Arc::new(|d, c, _e, _tp| revealfilter(d, c));
    let revealcostop_fn: cost::CostOp = Arc::new(|d, e, tp, g| revealcostop(d, e, tp, g));
    let reveal = cost::reveal(Some(revealfilter_fn), false, Some(cost::Count::N(1)), Some(cost::Count::N(1)), Some(revealcostop_fn), None);
    effect::set_cost(d, e1a, Some(cost::and(vec![reveal, cost_target(tgcost)])));
    // e1a:SetTarget(s.tgtg)
    effect::set_target(d, e1a, Some(cost_target(tgtg)));
    // e1a:SetOperation(s.tgop)
    effect::set_operation(d, e1a, Some(operation(tgop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
}
// s.revealfilter(c)
fn revealfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsAbleToGrave()
    card::is_race(d, c, RACE_ZOMBIE) && card::is_able_to_grave(d, c, None)
}
// s.revealcostop(e,tp,g)
fn revealcostop(d: &mut Duel, e: EffectId, _tp: u8, g: &CardSet) {
    // local rc=g:GetFirst()
    let rc = group::get_first(d, g).nil();
    // e:SetLabelObject(rc)
    effect::set_label_object(d, e, Value::Card(rc));
    // rc:CreateEffectRelation(e)
    card::create_effect_relation(d, rc, e);
}
// s.costfilter(c)
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_ZOMBIE) and c:IsAbleToGraveAsCost()
    card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_race(d, c, RACE_ZOMBIE) && card::is_able_to_grave_as_cost(d, c)
}
// s.tgcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,e:GetLabelObject(),1,tp,0)
    let rc = effect::get_label_object(d, a.e).as_card();
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, rc.map(CardSet::from_card), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,e:GetHandler(),1,tp,500)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(h)), 1, a.tp, 500);
    true
}
// s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=e:GetLabelObject()
    let rc = effect::get_label_object(d, a.e).as_card().nil();
    // if rc:IsRelateToEffect(e) and Duel.SendtoGrave(rc,REASON_EFFECT) and rc:IsLocation(LOCATION_GRAVE)
    //     and c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, rc, a.e) {
        // a Lua number is always true: the count of Duel.SendtoGrave is not compared
        duel::send_to_grave(d, rc, REASON_EFFECT, None, None);
        if card::is_location(d, rc, LOCATION_GRAVE) && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
            // This card gains 500 ATK until the end of this turn
            // c:UpdateAttack(500,RESETS_STANDARD_DISABLE_PHASE_END)
            card::update_attack(d, c, 500, Some(RESETS_STANDARD_DISABLE_PHASE_END), None, None);
        }
    }
}
