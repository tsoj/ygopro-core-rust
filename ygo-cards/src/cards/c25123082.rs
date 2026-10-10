//! Spellbook of Power
use super::prelude::*;

pub const ID: u32 = 25123082;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPELLBOOK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_SPELLCASTER)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_SPELLCASTER)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) && !card::is_immune_to_effect(d, tc, e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~(RESET_LEAVE|RESET_TODECK|RESET_TEMP_REMOVE|RESET_REMOVE|RESET_TOGRAVE)|RESET_PHASE|PHASE_END,0,1)
        card::register_flag_effect(
            d,
            tc,
            ID,
            RESET_EVENT | (RESETS_STANDARD & !(RESET_LEAVE | RESET_TODECK | RESET_TEMP_REMOVE | RESET_REMOVE | RESET_TOGRAVE)) | RESET_PHASE | PHASE_END,
            0,
            1,
            0,
            0,
        );
        // local e2=Effect.CreateEffect(e:GetHandler())
        let e2 = effect::create_effect(d, h);
        // e2:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e2, aux::stringid(ID, 0));
        // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
        // e2:SetCode(EVENT_BATTLE_DESTROYING)
        effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
        // e2:SetProperty(EFFECT_FLAG_DELAY)
        effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
        // e2:SetLabelObject(tc)
        effect::set_label_object(d, e2, Value::Card(tc));
        // e2:SetCondition(s.shcon)
        effect::set_condition(d, e2, Some(condition(shcon)));
        // e2:SetTarget(s.shtg)
        effect::set_target(d, e2, Some(cost_target(shtg)));
        // e2:SetOperation(s.shop)
        effect::set_operation(d, e2, Some(operation(shop)));
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
    }
}
/// `s.shcon(e,tp,eg,ep,ev,re,r,rp)`
fn shcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // return eg:IsContains(tc) and tc:GetFlagEffect(id)~=0
    let eg = a.eg.clone().nil();
    match tc {
        Some(tc) => group::is_contains(&eg, tc) && card::get_flag_effect(d, tc, ID) != 0,
        None => false,
    }
}
/// `s.shfilter(c)`
fn shfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPELLBOOK) and c:IsSpell() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SPELLBOOK as u16]) && card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.shtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn shtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.shfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&shfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.shop(e,tp,eg,ep,ev,re,r,rp)`
fn shop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.shfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&shfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
