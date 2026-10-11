//! Jungle Dweller
//! 密林に潜む者
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 73490417;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableUnsummonable()
    card::enable_unsummonable(d, c);
    // c:AddMustBeSpecialSummonedByCardEffect()
    card::add_must_be_special_summoned_by_card_effect(d, c);
    // Cannot be targeted by opponent's card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function() return Duel.GetFieldGroupCount(0,LOCATION_FZONE,LOCATION_FZONE)==2 end)
    effect::set_condition(d, e1, Some(lua_fn(|d, _a| duel::get_field_group_count(d, 0, LOCATION_FZONE, LOCATION_FZONE) == 2)));
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Shuffle up to 2 Field Spells to the Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TODECK+CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_TODECK + CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp and e:GetHandler():IsPreviousControler(tp) end)
    effect::set_condition(d, e3, Some(condition(|d, a| {
        if a.rp != 1 - a.tp {
            return false;
        }
        let h = effect::get_handler(d, a.e).nil();
        card::is_previous_controler(d, h, a.tp)
    })));
    // e3:SetTarget(s.tdtg)
    effect::set_target(d, e3, Some(cost_target(tdtg)));
    // e3:SetOperation(s.tdop)
    effect::set_operation(d, e3, Some(operation(tdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.spcostfilter(c,tp)
fn spcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK) and c:IsAbleToRemoveAsCost()
    //     and Duel.GetMZoneCount(tp,c)>0 and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK)
        && card::is_able_to_remove_as_cost(d, c, None)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && aux::sp_elim_filter(d, c, true, false)
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spcostfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spcostfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFieldSpell() and c:IsFaceup() and c:IsAbleToDeck()
    card::is_field_spell(d, c) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tdfilter), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,2,nil)
    let nv = aux::necro_valley_filter(|d, c| tdfilter(d, c));
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 2, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.HintSelection(g)
    duel::hint_selection(d, &g, true);
    // if Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==0 then return end
    if duel::send_to_deck(d, &g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 0 {
        return;
    }
    // local ct=g:FilterCount(Card.IsLocation,nil,LOCATION_DECK)
    let ct = group::filter_count(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local dg=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let dg = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD);
    // if #dg==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
    if dg.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local sg=dg:Select(tp,1,ct,nil)
    let sg = group::select(d, &dg, tp, 1, ct as u16, false, NO_EX).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.HintSelection(sg)
        duel::hint_selection(d, &sg, true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(sg,REASON_EFFECT)
        duel::destroy(d, &sg, REASON_EFFECT, None, None);
    }
}
