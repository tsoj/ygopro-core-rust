//! Armor Wizard Pazoo
use super::prelude::*;

pub const ID: u32 = 100200297;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP+CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE+CATEGORY_DISABLE+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_EQUIP + CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE + CATEGORY_DISABLE + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && card::is_faceup(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and c:CheckUniqueOnField(tp)
    //     and Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && card::check_unique_on_field(d, c, tp, None, None)
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup() and Duel.GetLocationCount(tp,LOCATION_SZONE)>0) then
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0) {
        // Duel.SendtoGrave(c,REASON_RULE,PLAYER_NONE,PLAYER_NONE)
        duel::send_to_grave(d, c, REASON_RULE, Some(PLAYER_NONE as u8), Some(PLAYER_NONE as u8));
    } else if duel::equip(d, tp, c, tc, true, false) {
        // local e0=Effect.CreateEffect(c)
        let e0 = effect::create_effect(d, c);
        // e0:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
        // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e0:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e0, EFFECT_EQUIP_LIMIT);
        // e0:SetValue(function(e,c) return c==tc end)
        effect::set_value_fn(d, e0, effect_card_fn(move |_d, _e, x| x == tc));
        // e0:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e0)
        card::register_effect(d, c, e0, false);
        // local e1a=Effect.CreateEffect(c)
        let e1a = effect::create_effect(d, c);
        // e1a:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e1a, EFFECT_TYPE_EQUIP);
        // e1a:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
        // e1a:SetCondition(function(e) return Duel.GetLP(e:GetHandlerPlayer())<=4000 end)
        effect::set_condition(d, e1a, Some(lua_fn(lp_cond)));
        // e1a:SetValue(2100)
        effect::set_value(d, e1a, 2100);
        // e1a:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1a, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1a)
        card::register_effect(d, c, e1a, false);
        // local e1b=e1a:Clone()
        let e1b = effect::clone(d, e1a);
        // e1b:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e1b, EFFECT_UPDATE_DEFENSE);
        // c:RegisterEffect(e1b)
        card::register_effect(d, c, e1b, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_CHAIN_SOLVING)
        effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
        // e2:SetRange(LOCATION_SZONE)
        effect::set_range(d, e2, LOCATION_SZONE);
        // e2:SetCondition(s.discon)
        effect::set_condition(d, e2, Some(condition(discon)));
        // e2:SetOperation(s.disop)
        effect::set_operation(d, e2, Some(operation(disop)));
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
/// `function(e) return Duel.GetLP(e:GetHandlerPlayer())<=4000 end`
fn lp_cond(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let p = effect::get_handler_player(d, e);
    duel::get_lp(d, p) <= 4000
}
/// `s.dicefilter(c)`: `c.roll_dice and c:IsFaceup()`
fn dicefilter(d: &mut Duel, c: CardId) -> bool {
    crate::card_info(d, c).roll_dice && card::is_faceup(d, c)
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and Duel.IsExistingMatchingCard(s.dicefilter,tp,LOCATION_ONFIELD,0,1,nil)
    //     and Duel.IsChainDisablable(ev)
    a.ep as i64 == 1 - a.tp as i64
        && duel::is_existing_matching_card(d, Some(&dicefilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_chain_disablable(d, a.ev as u8)
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsAbleToRemove() and Duel.SelectEffectYesNo(tp,c,aux.Stringid(id,1)) then
    if card::is_able_to_remove(d, c, None, None, None) && duel::select_effect_yes_no(d, a.tp, c, Some(aux::stringid(ID, 1))) {
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // if Duel.NegateEffect(ev) then
        if duel::negate_effect(d, a.ev as u8) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Remove(c,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, c, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
