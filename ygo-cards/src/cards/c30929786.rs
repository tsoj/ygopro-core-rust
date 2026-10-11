//! Brotherhood of the Fire Fist - Rooster
use super::prelude::*;

pub const ID: u32 = 30929786;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FIRE_FIST, SET_FIRE_FORMATION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // set
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(Cost.Replaceable(s.setcost,s.extracon))
    effect::set_cost(d, e2, Some(cost::replaceable(cost_target(setcost), Some(lua_fn(extracon)))));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:GetHandler():IsSetCard(SET_FIRE_FIST) and re:GetHandler():IsMonster()
    let rh = effect::get_handler(d, a.re.nil()).nil();
    card::is_set_card(d, rh, &[SET_FIRE_FIST as u16]) && card::is_monster(d, rh)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FIRE_FIST) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_FIRE_FIST as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.setfilter(c,field)`: `field` is the Lua argument, where nil and false both give `not field == true`
fn setfilter(d: &mut Duel, c: CardId, field: bool) -> bool {
    // return c:IsSetCard(SET_FIRE_FORMATION) and c:IsSpellTrap() and c:IsSSetable(not field)
    card::is_set_card(d, c, &[SET_FIRE_FORMATION as u16]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, !field, None)
}
/// `s.setcostfilter(c,tp,has_zone)`
fn setcostfilter(d: &mut Duel, c: CardId, tp: u8, has_zone: bool) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_FIRE_FORMATION) and c:IsSpellTrap() and c:IsAbleToGraveAsCost()
    //     --needs S/T equivalent of Duel.GetMZoneCount for proper handling
    //     and (has_zone or c:IsLocation(LOCATION_STZONE) or Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil,true))
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_FIRE_FORMATION as u16])
        && card::is_spell_trap(d, c)
        && card::is_able_to_grave_as_cost(d, c)
        && (has_zone
            || card::is_location(d, c, LOCATION_STZONE)
            || duel::is_existing_matching_card(d, Some(&|d, x| setfilter(d, x, true)), tp, LOCATION_DECK, 0, 1, NO_EX))
}
/// `s.setcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn setcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local has_zone=Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    let has_zone = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setcostfilter,tp,LOCATION_ONFIELD,0,1,nil,tp,has_zone) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| setcostfilter(d, c, tp, has_zone)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setcostfilter,tp,LOCATION_ONFIELD,0,1,1,nil,tp,has_zone)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| setcostfilter(d, c, tp, has_zone)), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        // the filter is called with the card only: field is nil
        return duel::is_existing_matching_card(d, Some(&|d, c| setfilter(d, c, false)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.extracon(base,e,tp)`
fn extracon(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(2);
    // return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     or Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil,true)
    duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        || duel::is_existing_matching_card(d, Some(&|d, c| setfilter(d, c, true)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil,Duel.GetLocationCount(tp,LOCATION_SZONE)==0)
    let field = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 == 0;
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| setfilter(d, c, field)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SSet(tp,g:GetFirst())
        let tc = group::get_first(d, &g).nil();
        duel::s_set(d, a.tp, tc, None, true);
    }
}
