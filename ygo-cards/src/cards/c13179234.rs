//! Vaylantz Wars - The Place of Beginning
use super::prelude::*;

pub const ID: u32 = 13179234;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[63394872],
    listed_series: &[SET_VAYLANTZ],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(aux.exccon)
    effect::set_condition(d, e2, Some(aux::exccon()));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.pztg)
    effect::set_target(d, e2, Some(cost_target(pztg)));
    // e2:SetOperation(s.pzop)
    effect::set_operation(d, e2, Some(operation(pzop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_FIELD) and c:IsSpell() and c:IsSetCard(SET_VAYLANTZ) and c:IsAbleToHand()
    card::is_type(d, c, TYPE_FIELD) && card::is_spell(d, c) && card::is_set_card(d, c, &[SET_VAYLANTZ as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, LOCATION_ONFIELD as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsOriginalType(TYPE_PENDULUM) and c:IsOriginalType(TYPE_MONSTER)
    card::is_faceup(d, c) && card::is_original_type(d, c, TYPE_PENDULUM) && card::is_original_type(d, c, TYPE_MONSTER)
}
/// `s.senetfilter(c)`
fn senetfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(63394872) and c:IsAbleToHand()
    card::is_code(d, c, &[63394872]) && card::is_able_to_hand(d, c, None)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g<1 or Duel.SendtoHand(g,nil,REASON_EFFECT)<1 or not g:GetFirst():IsLocation(LOCATION_HAND) then return end
    if g.len() < 1 {
        return;
    }
    if duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) < 1 {
        return;
    }
    let first = group::get_first(d, &g).nil();
    if !card::is_location(d, first, LOCATION_HAND) {
        return;
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - a.tp, g);
    // if Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_ONFIELD,0,1,nil)
    //   and Duel.IsExistingMatchingCard(s.senetfilter,tp,LOCATION_DECK,0,1,nil)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::is_existing_matching_card(d, Some(&desfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&senetfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local dg=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
        let dg = duel::select_matching_card(d, a.tp, Some(&desfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
        // if #dg<1 then return end
        if dg.len() < 1 {
            return;
        }
        // Duel.HintSelection(dg,true)
        duel::hint_selection(d, dg.clone(), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.Destroy(dg,REASON_EFFECT)<1 then return end
        if duel::destroy(d, dg, REASON_EFFECT, None, None) < 1 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.senetfilter,tp,LOCATION_DECK,0,1,1,nil)
        let sg = duel::select_matching_card(d, a.tp, Some(&senetfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if sg.len() > 0 {
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - a.tp, sg);
        }
    }
}
/// `s.pzfilter(c)`
fn pzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VAYLANTZ) and not c:IsForbidden()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VAYLANTZ as u16]) && !card::is_forbidden(d, c)
}
/// `s.pztg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pztg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.pzfilter,tp,LOCATION_EXTRA,0,1,nil)
    //   and Duel.CheckPendulumZones(tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&pzfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX) && duel::check_pendulum_zones(d, a.tp);
    }
    true
}
/// `s.pzop(e,tp,eg,ep,ev,re,r,rp)`
fn pzop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.CheckPendulumZones(tp) then return end
    if !duel::check_pendulum_zones(d, a.tp) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.pzfilter,tp,LOCATION_EXTRA,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&pzfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if tc then
    if let Some(tc) = group::get_first(d, &sel) {
        // Duel.MoveToField(tc,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
