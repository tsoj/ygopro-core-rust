//! Ursarctic Departure
use super::prelude::*;

pub const ID: u32 = 16471775;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_URSARCTIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 2 "Ursarctic" monsters from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.Discard(nil,true))
    effect::set_cost(d, e1, Some(cost::discard(None, true, None, None, None)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If you would Tribute a monster(s) to activate an "Ursarctic" monster's effect, except the turn this card was sent to the GY, you can banish this card from your GY instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_COST_REPLACE)
    effect::set_code(d, e2, EFFECT_COST_REPLACE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(aux.AND(aux.exccon,function(e) return e:GetHandler():IsAbleToRemoveAsCost() end))
    effect::set_condition(d, e2, Some(lua_fn(ccond)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, lua_fn(repval));
    // e2:SetOperation(function(base) Duel.Remove(base:GetHandler(),POS_FACEUP,REASON_COST|REASON_REPLACE) end)
    effect::set_operation(d, e2, Some(operation(repop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// aux.AND(aux.exccon, function(e) return e:GetHandler():IsAbleToRemoveAsCost() end)
fn ccond(d: &mut Duel, a: Args) -> bool {
    if !aux::exccon().call(d, a.0).truthy() {
        return false;
    }
    let c = effect::get_handler(d, a.e(0)).nil();
    card::is_able_to_remove_as_cost(d, c, None)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_URSARCTIC) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_URSARCTIC as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,2,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 2, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,2,2,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 2, 2, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
// function(base) Duel.Remove(base:GetHandler(),POS_FACEUP,REASON_COST|REASON_REPLACE) end
fn repop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::remove(d, c, POS_FACEUP as u8, REASON_COST | REASON_REPLACE, None, None);
}
// function s.repval(base,extracon,e,tp,eg,ep,ev,re,r,rp,chk)
fn repval(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(2)).nil();
    // return c:IsSetCard(SET_URSARCTIC) and c:IsMonster() and extracon(base,e,tp,eg,ep,ev,re,r,rp)
    if !(card::is_set_card(d, c, &[SET_URSARCTIC as u16]) && card::is_monster(d, c)) {
        return false;
    }
    let mut full = vec![a.v(0).clone(), a.v(2).clone()];
    full.extend(a.0.iter().skip(3).cloned());
    match a.v(1) {
        Value::Fn(f) => f.call(d, &full).truthy(),
        _ => false,
    }
}
