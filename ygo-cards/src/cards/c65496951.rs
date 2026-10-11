//! Vaalmonica Disarmonia
use super::prelude::*;

pub const ID: u32 = 65496951;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_VAALMONICA], counter_place_list: &[COUNTER_RESONANCE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER+CATEGORY_RECOVER+CATEGORY_DAMAGE+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_COUNTER + CATEGORY_RECOVER + CATEGORY_DAMAGE + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsCanAddCounter,tp,LOCATION_PZONE,0,1,nil,COUNTER_RESONANCE,1) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d: &mut Duel, c: CardId| card::is_can_add_counter(d, c, COUNTER_RESONANCE as u16, 1, false, 0)), a.tp, LOCATION_PZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,0,COUNTER_RESONANCE)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, 0, COUNTER_RESONANCE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,500)
    duel::set_possible_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 500);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,500)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, 500);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAALMONICA) and not c:IsCode(id) and c:IsFaceup() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_VAALMONICA as u16]) && !card::is_code(d, c, &[ID]) && card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp,angello_or_dimonno)
fn activate(d: &mut Duel, a: &EffArgs) {
    activate_with(d, a, None);
}
/// `s.activate` with the extra argument that "Angello Vaalmonica" and "Dimonno Vaalmonica" pass
pub fn activate_with(d: &mut Duel, a: &EffArgs, angello_or_dimonno: Option<i64>) {
    // local op=nil
    let op: Option<i64> = if angello_or_dimonno.is_some() {
        // op=angello_or_dimonno
        angello_or_dimonno
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_COUNTER)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_COUNTER as u64);
        // local tc=Duel.SelectMatchingCard(tp,Card.IsCanAddCounter,tp,LOCATION_PZONE,0,1,1,nil,COUNTER_RESONANCE,1):GetFirst()
        let sel = duel::select_matching_card(d, a.tp, Some(&|d: &mut Duel, c: CardId| card::is_can_add_counter(d, c, COUNTER_RESONANCE as u16, 1, false, 0)), a.tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel);
        // if not (tc and tc:AddCounter(COUNTER_RESONANCE,1)) then return end
        let Some(tc) = tc else { return };
        if !card::add_counter(d, tc, COUNTER_RESONANCE as u16, 1, false) {
            return;
        }
        // Duel.RaiseEvent(tc,EVENT_CUSTOM+39210885,e,0,tp,tp,1)
        duel::raise_event(d, tc, EVENT_CUSTOM + 39210885, Some(a.e), 0, a.tp, a.tp, 1);
        // op=Duel.SelectEffect(tp,{true,aux.Stringid(id,1)},{true,aux.Stringid(id,2)})
        let op = duel::select_effect(d, a.tp, &[(true, aux::stringid(ID, 1)), (true, aux::stringid(ID, 2))]).map(|x| x as i64);
        // Duel.BreakEffect()
        duel::break_effect(d);
        op
    };
    // if op==1 then
    if op == Some(1) {
        // Gain 500 LP, then you can add 1 of your banished "Vaalmonica" cards to your hand
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_REMOVED,0,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_REMOVED, 0, NO_EX);
        // if Duel.Recover(tp,500,REASON_EFFECT)>0 and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if duel::recover(d, a.tp, 500, REASON_EFFECT, false, None) > 0 && !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - a.tp, sg);
        }
    } else if op == Some(2) {
        // Take 500 damage, then you can add 1 "Vaalmonica" card from your GY to your hand
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
        // if Duel.Damage(tp,500,REASON_EFFECT)>0 and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,4)) then
        if duel::damage(d, a.tp, 500, REASON_EFFECT, false, None) > 0 && !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 4)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - a.tp, sg);
        }
    }
}
