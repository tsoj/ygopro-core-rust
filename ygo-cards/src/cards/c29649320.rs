//! Mirror Force Launcher
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 29649320;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[44095762], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set 1 "Mirror Force"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetCost(s.thcost)
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Set itself and 1 "Mirror Force"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCondition(s.setcon)
    effect::set_condition(d, e3, Some(condition(setcon)));
    // e3:SetTarget(s.settg)
    effect::set_target(d, e3, Some(cost_target(settg)));
    // e3:SetOperation(s.setop)
    effect::set_operation(d, e3, Some(operation(setop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`: `return Duel.IsTurnPlayer(tp) and Duel.IsMainPhase()`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp) && duel::is_main_phase(d, None)
}
/// `s.thcfilter(c)`: `return c:IsMonster() and c:IsDiscardable()`
fn thcfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_discardable(d, c, None, None)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thcfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thcfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.thcfilter,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&thcfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.thfilter(c)`: `return c:IsCode(44095762) and c:IsAbleToHand()`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[44095762]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil)
        //     and e:GetHandler():GetFlagEffect(id)==0 end
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
            && card::get_flag_effect(d, h, ID) == 0;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESET_PHASE|PHASE_END,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_PHASE | PHASE_END, 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let nf = aux::necro_valley_filter(|d, c| thfilter(d, c));
    let g = duel::select_matching_card(d, tp, Some(&nf), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and c:IsReason(REASON_EFFECT) and c:IsReason(REASON_DESTROY) and c:IsPreviousControler(tp)
    //     and c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousPosition(POS_FACEDOWN)
    a.rp == 1 - a.tp
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_reason(d, c, REASON_DESTROY)
        && card::is_previous_controler(d, c, a.tp)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_position(d, c, POS_FACEDOWN as u32)
}
/// `s.setfilter(c)`: `return c:IsCode(44095762) and c:IsSSetable()`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[44095762]) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>1 and c:IsSSetable()
    //     and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 1
            && card::is_s_setable(d, c, false, None)
            && duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<2 or not c:IsRelateToEffect(e) or not c:IsSSetable() then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 < 2
        || !card::is_relate_to_effect(d, c, a.e)
        || !card::is_s_setable(d, c, false, None)
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.setfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let nf = aux::necro_valley_filter(|d, cc| setfilter(d, cc));
    let g = duel::select_matching_card(d, tp, Some(&nf), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // local sg=Group.FromCards(c,tc)
        let mut sg = CardSet::from_card(c);
        group::add_card(&mut sg, &CardSet::from_card(tc));
        // Duel.SSet(tp,sg)
        duel::s_set(d, tp, sg, None, true);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
        effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
        // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
        effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
