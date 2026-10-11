//! Sangen Summoning
use super::prelude::*;

pub const ID: u32 = 30336082;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TENPAI_DRAGON], ..ScriptInfo::DEFAULT };

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
    // FIRE Dragon monsters you control are unaffected by your opponent's activated effects during your Main Phase 1
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCondition(function(e) return Duel.IsTurnPlayer(e:GetHandlerPlayer()) and Duel.IsPhase(PHASE_MAIN1) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let p = effect::get_handler_player(d, a.e);
        duel::is_turn_player(d, p) && duel::is_phase(d, PHASE_MAIN1)
    })));
    // e2:SetTarget(function(e,c) return c:IsAttribute(ATTRIBUTE_FIRE) and c:IsRace(RACE_DRAGON) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_race(d, c, RACE_DRAGON as u64))));
    // e2:SetValue(s.immval)
    effect::set_value_fn(d, e2, lua_fn(|d, a| immval(d, a.e(0), a.e(1))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Search 1 "Tenpai Dragon" monster from your Deck to your hand, then discard 1 card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_HANDES)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_HANDES);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Double the ATK of 1 Dragon Synchro Monster you control
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e4, CATEGORY_ATKCHANGE);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetCondition(function() return Duel.IsBattlePhase() end)
    effect::set_condition(d, e4, Some(condition(|d, _a| duel::is_battle_phase(d, None))));
    // e4:SetTarget(s.atktg)
    effect::set_target(d, e4, Some(cost_target(atktg)));
    // e4:SetOperation(s.atkop)
    effect::set_operation(d, e4, Some(operation(atkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.immval(e,te)`: `return te:GetOwnerPlayer()==1-e:GetHandlerPlayer() and te:IsActivated()`
fn immval(d: &mut Duel, e: EffectId, te: EffectId) -> bool {
    let owner = effect::get_owner_player(d, te);
    let hp = effect::get_handler_player(d, e);
    owner == 1 - hp && effect::is_activated(d, te)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TENPAI_DRAGON) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_TENPAI_DRAGON as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,1,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 1, tp, 1);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD,nil)
        duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_DRAGON) and c:IsType(TYPE_SYNCHRO)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_DRAGON as u64) && card::is_type(d, c, TYPE_SYNCHRO)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.atkfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && atkfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&atkfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    // local tc=Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let tc = duel::select_target(d, tp, Some(&atkfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,tc,1,tp,tc:GetFirst():GetAttack())
    let first = group::get_first(d, &tc.clone().nil()).nil();
    let atk = card::get_attack(d, first);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, tc, 1, tp, atk);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(tc:GetAttack()*2)
        let atk = card::get_attack(d, tc);
        effect::set_value(d, e1, atk as i64 * 2);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
