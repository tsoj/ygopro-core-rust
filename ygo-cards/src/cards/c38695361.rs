//! Envoy of Chaos
use super::prelude::*;

pub const ID: u32 = 38695361;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_BLACK_LUSTER_SOLDIER, SET_GAIA_THE_FIERCE_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Increase ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.atkcon1)
    effect::set_condition(d, e1, Some(condition(atkcon1)));
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(s.atktg1)
    effect::set_target(d, e1, Some(cost_target(atktg1)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Add itself to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(s.thcost)
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkcon1(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon1(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsBattlePhase() and aux.StatChangeDamageStepCondition()
    duel::is_battle_phase(d, None) && aux::stat_change_damage_step_condition(d)
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard({SET_BLACK_LUSTER_SOLDIER,SET_GAIA_THE_FIERCE_KNIGHT})
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BLACK_LUSTER_SOLDIER as u16, SET_GAIA_THE_FIERCE_KNIGHT as u16])
}
/// `s.atktg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.atkfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && atkfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
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
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1500)
        effect::set_value(d, e1, 1500);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(e:GetHandler())
        let e2 = effect::create_effect(d, h);
        // e2:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        // e2:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e2, EFFECT_SET_ATTACK_FINAL);
        // e2:SetTargetRange(0,LOCATION_MZONE)
        effect::set_target_range(d, e2, 0, LOCATION_MZONE);
        // e2:SetCondition(s.atkcon2)
        effect::set_condition(d, e2, Some(condition(atkcon2)));
        // e2:SetTarget(s.atktg2)
        effect::set_target(d, e2, Some(effect_card_fn(atktg2)));
        // e2:SetValue(s.atkval)
        effect::set_value_fn(d, e2, effect_card_value(atkval));
        // e2:SetLabelObject(tc)
        effect::set_label_object(d, e2, Value::Card(tc));
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    }
}
/// `s.atkcon2(e)`
fn atkcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // return Duel.IsPhase(PHASE_DAMAGE_CAL) and tc:GetFlagEffect(id)~=0 and tc:GetBattleTarget()
    duel::is_phase(d, PHASE_DAMAGE_CAL)
        && card::get_flag_effect(d, tc, ID) != 0
        && card::get_battle_target(d, tc).is_some()
}
/// `s.atktg2(e,c)`
fn atktg2(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetLabelObject():GetBattleTarget()
    let tc = effect::get_label_object(d, e).as_card().nil();
    card::get_battle_target(d, tc) == Some(c)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetBaseAttack()
    card::get_base_attack(d, c) as i64
}
/// `s.cfilter(c,att)`
fn cfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsAttribute(att) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, att) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,c,ATTRIBUTE_LIGHT)
    //     and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,c,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| cfilter(d, x, ATTRIBUTE_LIGHT)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into())
            && duel::is_existing_matching_card(d, Some(&|d, x| cfilter(d, x, ATTRIBUTE_DARK)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,c,ATTRIBUTE_LIGHT)
    let mut g1 = duel::select_matching_card(d, a.tp, Some(&|d, x| cfilter(d, x, ATTRIBUTE_LIGHT)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g2=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,c,ATTRIBUTE_DARK)
    let g2 = duel::select_matching_card(d, a.tp, Some(&|d, x| cfilter(d, x, ATTRIBUTE_DARK)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.Remove(g1,POS_FACEUP,REASON_COST)
    duel::remove(d, g1, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToHand() end
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,c)
        duel::confirm_cards(d, 1 - a.tp, &CardSet::from_card(c));
    }
}
