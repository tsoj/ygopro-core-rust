//! Painful Escape
use super::prelude::*;

pub const ID: u32 = 20513882;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // if chk==0 then return true end
    true
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:GetOriginalLevel()>0 and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,c)
    card::get_original_level(d, c) > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, Some(c))), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.thfilter(c,tc)`: `tc` is nil only when the Lua passes nil (then indexing it is an error)
fn thfilter(d: &mut Duel, c: CardId, tc: Option<CardId>) -> bool {
    // return c:IsMonster() and c:GetOriginalLevel()==tc:GetOriginalLevel() and c:GetOriginalRace()==tc:GetOriginalRace()
    //     and c:GetOriginalAttribute()==tc:GetOriginalAttribute() and c:GetOriginalCode()~=tc:GetOriginalCode() and c:IsAbleToHand()
    if !card::is_monster(d, c) {
        return false;
    }
    let tc = tc.nil();
    card::get_original_level(d, c) == card::get_original_level(d, tc)
        && card::get_original_race(d, c) == card::get_original_race(d, tc)
        && card::get_original_attribute(d, c) == card::get_original_attribute(d, tc)
        && card::get_original_code(d, c) != card::get_original_code(d, tc)
        && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,tp)
        return duel::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,tp)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g);
    effect::set_label_object(d, e, first.map_or(Value::Nil, Value::Card));
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e).as_card();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,tc)
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(move |d, c| thfilter(d, c, tc))), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
