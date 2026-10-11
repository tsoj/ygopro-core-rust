//! Venom Shot
use super::prelude::*;

pub const ID: u32 = 60728397;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_VENOM],
    counter_place_list: &[COUNTER_VENOM],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DECKDES);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsCode(72677437,8062132) or c:IsSetCard(SET_VENOM))
    card::is_faceup(d, c) && (card::is_code(d, c, &[72677437, 8062132]) || card::is_set_card(d, c, &[SET_VENOM as u16]))
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_REPTILE) and c:IsAbleToGrave()
    card::is_race(d, c, RACE_REPTILE) && card::is_able_to_grave(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsCanAddCounter(COUNTER_VENOM,2) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_can_add_counter(d, chkc, COUNTER_VENOM as u16, 2, false, 0);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsCanAddCounter,tp,0,LOCATION_MZONE,1,nil,COUNTER_VENOM,2)
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_VENOM as u16, 2, false, 0)), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,Card.IsCanAddCounter,tp,0,LOCATION_MZONE,1,1,nil,COUNTER_VENOM,2)
    let _g = duel::select_target(d, tp, Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_VENOM as u16, 2, false, 0)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None);
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) and tc:IsCanAddCounter(COUNTER_VENOM,2) and g:GetFirst():IsLocation(LOCATION_GRAVE) then
        if card::is_relate_to_effect(d, tc, a.e)
            && card::is_can_add_counter(d, tc, COUNTER_VENOM as u16, 2, false, 0)
            && {
                let first = group::get_first(d, &g).nil();
                card::is_location(d, first, LOCATION_GRAVE)
            }
        {
            // local atk=tc:GetAttack()
            let atk = card::get_attack(d, tc);
            // tc:AddCounter(COUNTER_VENOM,2)
            card::add_counter(d, tc, COUNTER_VENOM as u16, 2, false);
            // if atk>0 and tc:GetAttack()==0 then
            if atk > 0 && card::get_attack(d, tc) == 0 {
                // Duel.RaiseEvent(tc,EVENT_CUSTOM+54306223,e,0,0,0,0)
                duel::raise_event(d, CardSet::from_card(tc), EVENT_CUSTOM + 54306223, Some(a.e), 0, 0, 0, 0);
            }
        }
    }
}
