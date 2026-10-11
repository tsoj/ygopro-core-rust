//! Orbital 7
use super::prelude::*;

pub const ID: u32 = 71071546;
// local COUNTER_YOU_GOT_IT_BOSS=0x2c
const COUNTER_YOU_GOT_IT_BOSS: u16 = 0x2c;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PHOTON, SET_GALAXY],
    counter_place_list: &[0x2c],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can hold You Got It Boss! counters
    // c:EnableCounterPermit(COUNTER_YOU_GOT_IT_BOSS)
    card::enable_counter_permit(d, c, COUNTER_YOU_GOT_IT_BOSS, None, None);
    // Place 1 You Got It Boss! counter on itselef
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_FLIP)
    effect::set_code(d, e1, EVENT_FLIP);
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make its ATK become 2000
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(s.atkcost)
    effect::set_cost(d, e2, Some(cost_target(atkcost)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 "Photon" or "Galaxy" monster from GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e3, Some(cost::self_tribute()));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // c:AddCounter(COUNTER_YOU_GOT_IT_BOSS,1)
        card::add_counter(d, c, COUNTER_YOU_GOT_IT_BOSS, 1, false);
    }
}
// function s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=e:GetHandler():GetCounter(COUNTER_YOU_GOT_IT_BOSS)
    let h = effect::get_handler(d, a.e).nil();
    let ct = card::get_counter(d, h, COUNTER_YOU_GOT_IT_BOSS);
    // if chk==0 then return ct>0 and e:GetHandler():IsCanRemoveCounter(tp,COUNTER_YOU_GOT_IT_BOSS,ct,REASON_COST) end
    if a.chk == Some(0) {
        return ct > 0 && card::is_can_remove_counter(d, h, a.tp, COUNTER_YOU_GOT_IT_BOSS, ct as u16, REASON_COST);
    }
    // e:GetHandler():RemoveCounter(tp,COUNTER_YOU_GOT_IT_BOSS,ct,REASON_COST)
    card::remove_counter(d, h, a.tp, COUNTER_YOU_GOT_IT_BOSS, ct as u16, REASON_COST);
    true
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():GetAttack()~=2000 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::get_attack(d, h) != 2000;
    }
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // ATK becomes 2000
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
        effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(2000)
        effect::set_value(d, e1, 2000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // Cannot attack directly
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(3207)
        effect::set_description(d, e2, 3207);
        // e2:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
        effect::set_code(d, e2, EFFECT_CANNOT_DIRECT_ATTACK);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
        // Send itself to GY
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e3:SetRange(LOCATION_MZONE)
        effect::set_range(d, e3, LOCATION_MZONE);
        // e3:SetCountLimit(1)
        effect::set_count_limit(d, e3, 1, 0, 0, 0);
        // e3:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
        // e3:SetOperation(s.tgop)
        effect::set_operation(d, e3, Some(operation(tgop)));
        // e3:SetReset(RESET_EVENT|(RESETS_STANDARD|RESET_MSCHANGE|RESET_OVERLAY)&~(RESET_TOFIELD|RESET_LEAVE|RESET_TEMP_REMOVE))
        effect::set_reset(
            d,
            e3,
            RESET_EVENT | ((RESETS_STANDARD | RESET_MSCHANGE | RESET_OVERLAY) & !(RESET_TOFIELD | RESET_LEAVE | RESET_TEMP_REMOVE)),
            1,
        );
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
    }
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.SendtoGrave(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::send_to_grave(d, h, REASON_EFFECT, None, None);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_PHOTON,SET_GALAXY}) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_PHOTON as u16, SET_GALAXY as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - a.tp, tc);
    }
}
