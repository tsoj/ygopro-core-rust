//! Songs of the Dominators
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 58053438;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DOMINUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate an opponent's activated monster effect on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If you have no monsters in your GY, you can activate this card from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    // e2:SetCondition(function(e) return not Duel.IsExistingMatchingCard(Card.IsMonster,e:GetHandlerPlayer(),LOCATION_GRAVE,0,1,nil) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let p = effect::get_handler_player(d, a.e);
        !duel::is_existing_matching_card(d, Some(&|d, c| card::is_monster(d, c)), p, LOCATION_GRAVE, 0, 1, NO_EX)
    })));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local trig_p,trig_typ,trig_loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_PLAYER,CHAININFO_TRIGGERING_TYPE,CHAININFO_TRIGGERING_LOCATION)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_PLAYER, CHAININFO_TRIGGERING_TYPE, CHAININFO_TRIGGERING_LOCATION]);
    let trig_p = info[0].int();
    let trig_typ = info[1].int();
    let trig_loc = info[2].int();
    // return trig_p==1-tp and trig_typ&TYPE_MONSTER>0 and trig_loc&LOCATION_MZONE>0
    trig_p == (1 - a.tp) as i64 && (trig_typ & TYPE_MONSTER as i64) > 0 && (trig_loc & LOCATION_MZONE as i64) > 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local act_from_hand_chk=(e:IsHasType(EFFECT_TYPE_ACTIVATE) and e:GetHandler():IsStatus(STATUS_ACT_FROM_HAND)) and 1 or 0
    let mut act_from_hand = false;
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        let h = effect::get_handler(d, a.e).nil();
        act_from_hand = card::is_status(d, h, STATUS_ACT_FROM_HAND);
    }
    let act_from_hand_chk: i64 = if act_from_hand { 1 } else { 0 };
    // e:SetLabel(act_from_hand_chk)
    effect::set_label(d, a.e, &[act_from_hand_chk]);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DOMINUS) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_DOMINUS as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.NegateEffect(ev) and Duel.IsExistingMatchingCard(Card.IsTrap,tp,LOCATION_GRAVE,0,1,nil)
    //   and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::negate_effect(d, a.ev as u8)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_trap(d, c)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) and e:GetLabel()==1 then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && effect::get_label(d, a.e) == 1 {
        // --You cannot activate monster effects in the hand, GY, or banishment until the end of the next turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,3))
        effect::set_description(d, e1, aux::stringid(ID, 3));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetValue(function(e,re) return re:IsMonsterEffect() and re:GetHandler():IsLocation(LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED) end)
        effect::set_value_fn(d, e1, lua_fn(cannot_activate_value));
        // e1:SetReset(RESET_PHASE|PHASE_END,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `function(e,re) return re:IsMonsterEffect() and re:GetHandler():IsLocation(LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED) end`
fn cannot_activate_value(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let rc = effect::get_handler(d, re).nil();
    card::is_location(d, rc, LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED)
}
