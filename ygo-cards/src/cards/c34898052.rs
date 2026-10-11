//! Dragoroar
use super::prelude::*;

pub const ID: u32 = 34898052;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE as u32));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rfilter(c,att)`
fn rfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsAttribute(att) and c:IsMonster() and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, att) && card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.filter(c,g,sg)`
fn filter(d: &mut Duel, c: CardId, g: &CardSet, sg: &CardSet) -> bool {
    // if #sg==0 then
    if sg.is_empty() {
        // if not c:IsAttribute(ATTRIBUTE_EARTH) then return false end
        if !card::is_attribute(d, c, ATTRIBUTE_EARTH) {
            return false;
        }
    } else if sg.len() == 1 {
        // elseif #sg==1 then if not c:IsAttribute(ATTRIBUTE_WATER) then return false end
        if !card::is_attribute(d, c, ATTRIBUTE_WATER) {
            return false;
        }
    } else if sg.len() == 2 {
        // elseif #sg==2 then if not c:IsAttribute(ATTRIBUTE_FIRE) then return false end
        if !card::is_attribute(d, c, ATTRIBUTE_FIRE) {
            return false;
        }
    } else if sg.len() == 3 {
        // elseif #sg==3 then if not c:IsAttribute(ATTRIBUTE_WIND) then return false end
        if !card::is_attribute(d, c, ATTRIBUTE_WIND) {
            return false;
        }
    }
    // sg:AddCard(c)
    let mut sg = sg.clone();
    group::add_card(&mut sg, &CardSet::from_card(c));
    // local res
    // if #sg<4 then res=g:IsExists(s.filter,1,nil,g,sg)
    // else res=Duel.IsExistingMatchingCard(Card.IsAbleToDeck,0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,sg) end
    let res = if sg.len() < 4 {
        group::is_exists(d, g, &|d, x| filter(d, x, g, &sg), 1, NO_EX)
    } else {
        duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_deck(d, x)), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, (&sg).into())
    };
    // sg:RemoveCard(c) (the copy is dropped)
    // return res
    res
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.rfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,ATTRIBUTE_EARTH+ATTRIBUTE_WATER+ATTRIBUTE_FIRE+ATTRIBUTE_WIND)
    let att = ATTRIBUTE_EARTH + ATTRIBUTE_WATER + ATTRIBUTE_FIRE + ATTRIBUTE_WIND;
    let g = duel::get_matching_group(d, Some(&|d, c| rfilter(d, c, att)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return g:IsExists(s.filter,1,nil,g,Group.CreateGroup()) end
    if a.chk == Some(0) {
        let empty = CardSet::new();
        return group::is_exists(d, &g, &|d, c| filter(d, c, &g, &empty), 1, NO_EX);
    }
    // local rg=Group.CreateGroup()
    let mut rg = CardSet::new();
    // while #rg<4 do
    while rg.len() < 4 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local sg=g:FilterSelect(tp,s.filter,1,1,rg,g,rg)
        let rg_snapshot = rg.clone();
        let sg = group::filter_select(d, &g, a.tp, &|d, c| filter(d, c, &g, &rg_snapshot), 1, 1, false, (&rg).into()).nil();
        // rg:Merge(sg)
        group::add_card(&mut rg, &sg);
    }
    // rg:RemoveCard(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    group::remove_card(&mut rg, &CardSet::from_card(h));
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, Exclude::Card(h));
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, Exclude::Card(h));
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let h = effect::get_handler(d, a.e).nil();
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,e:GetHandler())
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, Exclude::Card(h)).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
