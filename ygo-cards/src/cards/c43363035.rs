//! Wandering Titan of Tartarus
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 43363035;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send as many cards you control as possible to the GY, and if you do, Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_DECK) end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_previous_location(d, h, LOCATION_DECK)
    })));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send 1 card on the field to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return not e:GetHandler():IsPreviousLocation(LOCATION_DECK) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        !card::is_previous_location(d, h, LOCATION_DECK)
    })));
    // e2:SetCost(s.tgcost)
    effect::set_cost(d, e2, Some(cost_target(tgcost)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, tp, 0);
    // Duel.SetChainLimit(function(te) return not te:IsMonsterEffect() end)
    duel::set_chain_limit(d, lua_fn(|d, a: Args| !effect::is_monster_effect(d, a.e(0))));
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local your_fg=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_ONFIELD,0,nil)
    let your_fg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_ONFIELD, 0, NO_EX);
    // if #your_fg==0 or Duel.SendtoGrave(your_fg,REASON_EFFECT)==0 then return end
    if your_fg.is_empty() || duel::send_to_grave(d, your_fg.clone(), REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local ct=your_fg:FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)
    let ct = group::filter_count(d, &your_fg, &|d, x| card::is_location(d, x, LOCATION_GRAVE), NO_EX);
    // if ct>0 and c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if ct > 0
        && card::is_relate_to_effect(d, c, e)
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
    {
        // local opp_fg=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,0,LOCATION_ONFIELD,nil)
        let mut opp_fg = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), tp, 0, LOCATION_ONFIELD, NO_EX);
        // if #opp_fg==0 then return end
        if opp_fg.is_empty() {
            return;
        }
        // if #opp_fg>1 then
        if opp_fg.len() > 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
            // opp_fg=opp_fg:Select(tp,1,math.min(ct,#opp_fg),nil)
            let max = (ct as usize).min(opp_fg.len()) as u16;
            opp_fg = group::select(d, &opp_fg, tp, 1, max, false, NO_EX).nil();
        }
        // Duel.HintSelection(opp_fg)
        duel::hint_selection(d, &opp_fg, true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoGrave(opp_fg,REASON_EFFECT)
        duel::send_to_grave(d, opp_fg, REASON_EFFECT, None, None);
    }
}
/// `s.tgcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_grave_as_cost(d, x)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| card::is_able_to_grave_as_cost(d, x)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), 1, tp, 0);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGrave,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| card::is_able_to_grave(d, x, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, &g, true);
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
