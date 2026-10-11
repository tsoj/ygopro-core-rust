//! Justice of Prophecy
use super::prelude::*;

pub const ID: u32 = 26732909;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPELLBOOK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetCost(s.thcost)
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,s.chainfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(chainfilter));
}
/// `s.chainfilter(re,tp,cid)`
fn chainfilter(d: &mut Duel, a: Args) -> bool {
    let re = a.e(0);
    // return not (re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and re:GetHandler():IsSetCard(SET_SPELLBOOK))
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return true;
    }
    if !effect::is_spell_effect(d, re) {
        return true;
    }
    let rc = effect::get_handler(d, re).nil();
    !card::is_set_card(d, rc, &[SET_SPELLBOOK as u16])
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_CHAIN)>0 and e:GetHandler():IsAbleToRemoveAsCost() end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::get_custom_activity_count(d, ID, tp, ACTIVITY_CHAIN) > 0 && card::is_able_to_remove_as_cost(d, c, None);
    }
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_COST)
    let c = effect::get_handler(d, a.e).nil();
    duel::remove(d, c, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(5) and c:IsAttribute(ATTRIBUTE_LIGHT+ATTRIBUTE_DARK) and c:IsRace(RACE_SPELLCASTER) and c:IsAbleToHand()
    card::is_level_above(d, c, 5)
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT + ATTRIBUTE_DARK)
        && card::is_race(d, c, RACE_SPELLCASTER)
        && card::is_able_to_hand(d, c, None)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsSetCard(SET_SPELLBOOK) and c:IsAbleToHand()
    card::is_spell(d, c) && card::is_set_card(d, c, &[SET_SPELLBOOK as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter1), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&filter2), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g1=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_DECK,0,nil)
    let g1 = duel::get_matching_group(d, Some(&filter1), tp, LOCATION_DECK, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.filter2,tp,LOCATION_DECK,0,nil)
    let g2 = duel::get_matching_group(d, Some(&filter2), tp, LOCATION_DECK, 0, NO_EX);
    // if #g1>0 and #g2>0 then
    if !g1.is_empty() && !g2.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg1=g1:Select(tp,1,1,nil)
        let mut sg1 = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg2=g2:Select(tp,1,1,nil)
        let sg2 = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
        // sg1:Merge(sg2)
        group::add_card(&mut sg1, &sg2);
        // Duel.SendtoHand(sg1,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg1.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg1)
        duel::confirm_cards(d, 1 - tp, sg1);
    }
}
