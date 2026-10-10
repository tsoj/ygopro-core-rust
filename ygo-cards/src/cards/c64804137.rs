//! Trickstar Band Drumatis
use super::prelude::*;

pub const ID: u32 = 64804137;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRICKSTAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_TRICKSTAR),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_TRICKSTAR as u16], Some(fc), st, Some(tp)))), 2)],
    );
    // Your "Trickstar" Link Monsters that point to this card gain 1000 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.linkatktg)
    effect::set_target(d, e1, Some(effect_card_fn(linkatktg)));
    // e1:SetValue(1000)
    effect::set_value(d, e1, 1000);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Trickstar" card from your Deck to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Change the ATK of 1 monster on the field to 0
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e3, EVENT_DAMAGE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return (r&REASON_EFFECT)>0 end)
    effect::set_condition(d, e3, Some(condition(cond3)));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.linkatktg(e,c)`
fn linkatktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsLinkMonster() and c:IsSetCard(SET_TRICKSTAR) and c:GetLinkedGroup():IsContains(e:GetHandler())
    if !(card::is_link_monster(d, c) && card::is_set_card(d, c, &[SET_TRICKSTAR as u16])) {
        return false;
    }
    let lg = card::get_linked_group(d, c);
    match effect::get_handler(d, e) {
        Some(h) => group::is_contains(&lg, h),
        None => false,
    }
}
/// `s.thfilter(c,tp)`
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_TRICKSTAR) and c:IsAbleToHand()
    //     and not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,c:GetCode()),tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,nil)
    if !(card::is_set_card(d, c, &[SET_TRICKSTAR as u16]) && card::is_able_to_hand(d, c, None)) {
        return false;
    }
    let codes = card::get_code(d, c);
    !duel::is_existing_matching_card(
        d,
        Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &codes)),
        tp,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        0,
        1,
        NO_EX,
    )
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,tp,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), Some(a.tp), REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:HasNonZeroAttack() and chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::has_non_zero_attack(d, chkc) && card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.HasNonZeroAttack,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::has_non_zero_attack(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // Duel.SelectTarget(tp,Card.HasNonZeroAttack,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, c| card::has_non_zero_attack(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `function(e) return e:GetHandler():IsFusionSummoned() end` (e2's condition)
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return (r&REASON_EFFECT)>0 end` (e3's condition)
fn cond3(_d: &mut Duel, a: &EffArgs) -> bool {
    (a.r & REASON_EFFECT) > 0
}
