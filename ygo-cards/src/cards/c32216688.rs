//! Ｒ．Ｂ．ブルート・ブルース
//! R.B. The Brute Blues
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 32216688;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RB], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ Machine monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_MACHINE),2)
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_MACHINE as u64, sc, st, Some(tp)))), 2, None, None, None, None);
    // Gains total original ATK of "R.B." monsters this card points to while it points to an "R.B." monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e) return e:GetHandler():GetLinkedGroup():IsExists(aux.FaceupFilter(Card.IsSetCard,SET_RB),1,nil) end)
    effect::set_condition(d, e1, Some(lua_fn(|d, a: Args| linked_rb(d, a.e(0)))));
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can make a second attack during each Battle Phase while it points to a "R.B." monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e2, EFFECT_EXTRA_ATTACK);
    // e2:SetCondition(function(e) return e:GetHandler():GetLinkedGroup():IsExists(aux.FaceupFilter(Card.IsSetCard,SET_RB),1,nil) end)
    effect::set_condition(d, e2, Some(lua_fn(|d, a: Args| linked_rb(d, a.e(0)))));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Cannot be destroyed by battle or card effects while it points to a "R.B." monster
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3a, EFFECT_TYPE_SINGLE);
    // e3a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3a:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3a, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3a, LOCATION_MZONE);
    // e3a:SetCondition(function(e) return e:GetHandler():GetLinkedGroup():IsExists(aux.FaceupFilter(Card.IsSetCard,SET_RB),1,nil) end)
    effect::set_condition(d, e3a, Some(lua_fn(|d, a: Args| linked_rb(d, a.e(0)))));
    // e3a:SetValue(1)
    effect::set_value(d, e3a, 1);
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local e3b=e3a:Clone()
    let e3b = effect::clone(d, e3a);
    // e3b:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3b, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
    // Add 1 "R.B." card from your Deck to your hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_RB)` on the linked group of the card of effect `e`
fn linked_rb(d: &mut Duel, e: EffectId) -> bool {
    // e:GetHandler():GetLinkedGroup():IsExists(aux.FaceupFilter(Card.IsSetCard,SET_RB),1,nil)
    let c = effect::get_handler(d, e).nil();
    let lg = card::get_linked_group(d, c);
    group::is_exists(d, &lg, &aux::faceup_filter(|d, x| card::is_set_card(d, x, &[SET_RB as u16])), 1, NO_EX)
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_RB) and c:GetBaseAttack()>=0
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_RB as u16]) && card::get_base_attack(d, c) >= 0
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetLinkedGroup():Filter(s.atkfilter,nil):GetSum(Card.GetBaseAttack)
    let lg = card::get_linked_group(d, c);
    let f = group::filter(d, &lg, &|d, x| atkfilter(d, x), NO_EX);
    group::get_sum(d, &f, &|d, x| card::get_base_attack(d, x) as i64)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RB) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_RB as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
