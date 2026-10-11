//! Hu-Li the Jewel Mikanko
//! 珠の御巫 フゥリ
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 6327734;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MIKANKO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Take no battle damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    // e1:SetCondition(aux.NOT(s.eqcon))
    effect::set_condition(d, e1, Some(condition(not_eqcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Cannot be destroyed by battle
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetCondition(s.eqcon)
    effect::set_condition(d, e2, Some(condition(eqcon_cond)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Reflect battle damage
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_REFLECT_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_REFLECT_BATTLE_DAMAGE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --Opponent cannot target "Mikanko" cards
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e4, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e4:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e4, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(LOCATION_ONFIELD,0)
    effect::set_target_range(d, e4, LOCATION_ONFIELD, 0);
    // e4:SetCondition(s.immcon)
    effect::set_condition(d, e4, Some(lua_fn(immcon)));
    // e4:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_MIKANKO))
    effect::set_target(d, e4, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_MIKANKO as u16]))));
    // e4:SetValue(aux.tgoval)
    effect::set_value_fn(d, e4, aux::tgoval());
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // --Search 1 "Mikanko" Trap
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e5, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e5:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e5, EFFECT_FLAG_DELAY, 0);
    // e5:SetCode(EVENT_EQUIP)
    effect::set_code(d, e5, EVENT_EQUIP);
    // e5:SetCountLimit(1,id)
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    // e5:SetTarget(s.thtg)
    effect::set_target(d, e5, Some(cost_target(thtg)));
    // e5:SetOperation(s.thop)
    effect::set_operation(d, e5, Some(operation(thop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.eqcon(e)`
fn eqcon(d: &mut Duel, e: EffectId) -> bool {
    // return e:GetHandler():GetEquipCount()>0
    let h = effect::get_handler(d, e).nil();
    card::get_equip_count(d, h) > 0
}
/// `s.eqcon` as a condition
fn eqcon_cond(d: &mut Duel, a: &EffArgs) -> bool {
    eqcon(d, a.e)
}
/// `aux.NOT(s.eqcon)` as a condition
fn not_eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    !eqcon(d, a.e)
}
/// `s.immcon(e)`
fn immcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsType,e:GetHandlerPlayer(),LOCATION_STZONE,0,1,nil,TYPE_EQUIP)
    let p = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&|d, x| card::is_type(d, x, TYPE_EQUIP)), p, LOCATION_STZONE, 0, 1, NO_EX)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MIKANKO) and c:IsTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MIKANKO as u16]) && card::is_trap(d, c) && card::is_able_to_hand(d, c, None)
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
