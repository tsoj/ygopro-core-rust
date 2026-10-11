//! Drill Armed Dragon
use super::prelude::*;

pub const ID: u32 = 46221535;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply a "for the rest of this turn, all WIND Dragon monsters you control will gain 300 ATK" effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.atkcost)
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 Dragon monster from your Deck to your hand with a Level equal to the number of banished monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.thcost)
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkcostfilter(c,tp)`
fn atkcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND) and c:IsAbleToGraveAsCost()
    card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() and Duel.IsExistingMatchingCard(s.atkcostfilter,tp,LOCATION_HAND,0,1,c) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c) && duel::is_existing_matching_card(d, Some(&atkcostfilter), tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.atkcostfilter,tp,LOCATION_HAND,0,1,1,c)
    let mut g = duel::select_matching_card(d, tp, Some(&atkcostfilter), tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    // Duel.SendtoGrave(g+c,REASON_COST)
    group::add_card(&mut g, &CardSet::from_card(c));
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return c:IsAttribute(ATTRIBUTE_WIND) and c:IsRace(RACE_DRAGON) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_race(d, c, RACE_DRAGON as u64))));
    // e1:SetValue(300)
    effect::set_value(d, e1, 300);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.thcostfilter(c)`
fn thcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and (c:IsLevelAbove(7) or c:IsAttribute(ATTRIBUTE_WIND)) and c:IsAbleToRemoveAsCost()
    //   and aux.SpElimFilter(c,true)
    card::is_race(d, c, RACE_DRAGON as u64)
        && (card::is_level_above(d, c, 7) || card::is_attribute(d, c, ATTRIBUTE_WIND))
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,#sg)
    let lv = sg.len() as u32;
    (duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, lv)), tp, LOCATION_DECK, 0, 1, NO_EX), false)
}
/// `s.thfilter(c,lv)`
fn thfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsLevel(lv) and not c:IsCode(id)
    card::is_race(d, c, RACE_DRAGON as u64) && card::is_level(d, c, &[lv]) && !card::is_code(d, c, &[ID])
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // e:SetLabel(-100)
    effect::set_label(d, e, &[-100]);
    // local rg=Duel.GetMatchingGroup(s.thcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&thcostfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(rg,e,tp,1,#rg,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(rg.len()), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,#rg,s.rescon,1,tp,HINTMSG_REMOVE)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(rg.len()), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    // e:SetLabel(#g)
    effect::set_label(d, e, &[g.len() as i64]);
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then local res=e:GetLabel()==-100 e:SetLabel(0) return res end
    if a.chk == Some(0) {
        let res = effect::get_label(d, a.e) == -100;
        effect::set_label(d, a.e, &[0]);
        return res;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,e:GetLabel())
    let lv = effect::get_label(d, a.e) as u32;
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, lv)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
