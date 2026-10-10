//! Armed Dragon Lightning
//! scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 97091969;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gain ATK or add to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.opttarget)
    effect::set_target(d, e2, Some(cost_target(opttarget)));
    // e2:SetOperation(s.opteffect)
    effect::set_operation(d, e2, Some(operation(opteffect)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destruction replacemente
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTarget(s.reptg)
    effect::set_target(d, e3, Some(cost_target(reptg)));
    // e3:SetValue(s.repval)
    effect::set_value_fn(d, e3, effect_card_fn(repval));
    // e3:SetOperation(s.repop)
    effect::set_operation(d, e3, Some(operation(repop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.optfilter(c)`
fn optfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ARMED_DRAGON) and c:HasLevel()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ARMED_DRAGON as u16]) && card::has_level(d, c)
}
/// `s.thfilter(c,lvl)`
fn thfilter(d: &mut Duel, c: CardId, lvl: u32) -> bool {
    // return c:IsSetCard(SET_ARMED_DRAGON) and c:IsMonster() and c:IsLevelBelow(lvl) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ARMED_DRAGON as u16]) && card::is_monster(d, c) && card::is_level_below(d, c, lvl) && card::is_able_to_hand(d, c, None)
}
/// `s.opttarget(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn opttarget(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.optfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && optfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.optfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&optfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.optfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let tc = duel::select_target(d, tp, Some(&optfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local op=0
    // if Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil,tc:GetFirst():GetLevel()) then
    let first = group::get_first(d, &tc).nil();
    let lvl = card::get_level(d, first);
    let op = if duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, lvl)), tp, LOCATION_GRAVE, 0, 1, NO_EX) {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)])
    } else {
        // op=0
        // Duel.Hint(HINT_OPSELECTED,1-tp,aux.Stringid(id,1))
        duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, aux::stringid(ID, 1));
        0
    };
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    true
}
/// `s.opteffect(e,tp,eg,ep,ev,re,r,rp)`
fn opteffect(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not c:IsRelateToEffect(e) or not tc:IsRelateToEffect(e) or tc:IsImmuneToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) || !card::is_relate_to_effect(d, tc, a.e) || card::is_immune_to_effect(d, tc, a.e) {
        return;
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op~=1 then
    if op != 1 {
        // local val=tc:GetLevel()*100
        let val = card::get_level(d, tc) as i64 * 100;
        // if val~=0 then
        if val != 0 {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(val)
            effect::set_value(d, e1, val);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
    // if op~=0 then
    if op != 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil,tc:GetLevel())
        let lvl = card::get_level(d, tc);
        let g = duel::select_matching_card(d, tp, Some(&|d, x| thfilter(d, x, lvl)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, &g);
        }
    }
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE)
    //     and c:IsSetCard(SET_ARMED_DRAGON) and c:IsReason(REASON_EFFECT) and not c:IsReason(REASON_REPLACE)
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_set_card(d, c, &[SET_ARMED_DRAGON as u16])
        && card::is_reason(d, c, REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| repfilter(d, c, tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,e:GetHandler(),96)
    let h = effect::get_handler(d, a.e).nil();
    duel::select_effect_yes_no(d, tp, h, Some(96))
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.SendtoGrave(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::send_to_grave(d, CardSet::from_card(h), REASON_EFFECT, None, None);
}
