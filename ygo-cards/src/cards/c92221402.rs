//! Chaotic Elements
//! カオティック・エレメンツ
use super::prelude::*;

pub const ID: u32 = 92221402;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 Level 5 or higher LIGHT or DARK Pyro or Aqua monster from your Deck or GY to your hand
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
    // Take control of 1 monster your opponent controls until the End Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e2, CATEGORY_CONTROL);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.ctrlcon)
    effect::set_condition(d, e2, Some(condition(ctrlcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.crtltg)
    effect::set_target(d, e2, Some(cost_target(crtltg)));
    // e2:SetOperation(s.crtlop)
    effect::set_operation(d, e2, Some(operation(crtlop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(5) and c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK) and c:IsRace(RACE_PYRO|RACE_AQUA) and c:IsAbleToHand()
    card::is_level_above(d, c, 5)
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK)
        && card::is_race(d, c, RACE_PYRO | RACE_AQUA)
        && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,PLAYER_EITHER,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let f = aux::necro_valley_filter(thfilter);
    let g = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_HAND) then
    if let Some(tc) = tc {
        if duel::send_to_hand(d, tc, None, REASON_EFFECT, None) > 0 && card::is_location(d, tc, LOCATION_HAND) {
            // Duel.ConfirmCards(1-tp,tc)
            duel::confirm_cards(d, 1 - tp, tc);
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // local exc=c:IsRelateToEffect(e) and c or nil
            let exc: Option<CardId> = if card::is_relate_to_effect(d, c, a.e) { Some(c) } else { None };
            // if Duel.IsExistingMatchingCard(Card.IsRace,tp,LOCATION_GRAVE,0,3,nil,RACE_PYRO|RACE_AQUA)
            //     and Duel.IsExistingMatchingCard(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,exc)
            //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if duel::is_existing_matching_card(d, Some(&|d, x| card::is_race(d, x, RACE_PYRO | RACE_AQUA)), tp, LOCATION_GRAVE, 0, 3, NO_EX)
                && duel::is_existing_matching_card(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, exc.into())
                && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
            {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
                // local sg=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,exc)
                let sg = duel::select_matching_card(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, exc.into()).nil();
                // if #sg==0 then return end
                if sg.is_empty() {
                    return;
                }
                // Duel.HintSelection(sg)
                duel::hint_selection(d, sg.clone(), true);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Destroy(sg,REASON_EFFECT)
                duel::destroy(d, sg, REASON_EFFECT, None, None);
            }
        }
    }
}
/// `s.ctrlconfilter(c)`
fn ctrlconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK) and c:IsRace(RACE_PYRO|RACE_AQUA) and c:IsFaceup()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK) && card::is_race(d, c, RACE_PYRO | RACE_AQUA) && card::is_faceup(d, c)
}
/// `s.ctrlcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctrlcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.ctrlconfilter,tp,0,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&ctrlconfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.crtltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn crtltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsControlerCanBeChanged() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - tp)
            && card::is_controler_can_be_changed(d, chkc, false, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, tp, 0);
    true
}
/// `s.crtlop(e,tp,eg,ep,ev,re,r,rp)`
fn crtlop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.GetControl(tc,tp,PHASE_END,1)
        duel::get_control(d, tc, a.tp, PHASE_END as u16, 1, None, None);
    }
}
