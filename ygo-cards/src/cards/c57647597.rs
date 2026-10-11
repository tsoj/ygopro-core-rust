//! Reptilianne Lamia
use super::prelude::*;

pub const ID: u32 = 57647597;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_SPECIAL_SUMMON+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_SPECIAL_SUMMON + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk to 0
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local fc=Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)
    let fc = duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0);
    // return fc>0 and Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_REPTILE),tp,LOCATION_MZONE,0,nil)==fc
    fc > 0
        && duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_REPTILE))), a.tp, LOCATION_MZONE, 0, NO_EX) as i32 == fc
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:HasNonZeroAttack() and c:GetBaseAttack()>0
    card::has_non_zero_attack(d, c) && card::get_base_attack(d, c) > 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.tgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && tgfilter(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and Duel.IsExistingTarget(s.tgfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_existing_target(d, Some(&tgfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&tgfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,g:GetFirst():GetBaseAttack())
    let first = group::get_first(d, &g).nil();
    let base = card::get_base_attack(d, first);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, base);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not (tc and tc:IsFaceup() and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) and tc:GetAttack()>0) then return end
    let Some(tc) = tc else { return };
    if !(card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) && card::get_attack(d, tc) > 0) {
        return;
    }
    // local atk=tc:GetBaseAttack()
    let atk = card::get_base_attack(d, tc);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetValue(0)
    effect::set_value(d, e1, 0);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Damage(tp,atk,REASON_EFFECT)
        duel::damage(d, a.tp, atk as i64, REASON_EFFECT, false, None);
    }
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and r & REASON_SYNCHRO == REASON_SYNCHRO
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE) && (a.r & REASON_SYNCHRO) == REASON_SYNCHRO
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:HasNonZeroAttack() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::has_non_zero_attack(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.HasNonZeroAttack,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::has_non_zero_attack), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.HasNonZeroAttack,tp,LOCATION_MZONE,LOCATION_MZONE,1,2,nil)
    duel::select_target(d, a.tp, Some(&card::has_non_zero_attack), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 2, false, NO_EX);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS):Filter(aux.FaceupFilter(Card.IsRelateToEffect,e),nil)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let e = a.e;
    let g = group::filter(d, &info[0].as_group().cloned().nil(), &aux::faceup_filter(move |d, c| card::is_relate_to_effect(d, c, e)), NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // g:ForEach(s.zaop,e:GetHandler())
    let hc = effect::get_handler(d, a.e).nil();
    for tc in group::iter(d, &g) {
        zaop(d, tc, hc);
    }
}
/// `s.zaop(c,hc)`
fn zaop(d: &mut Duel, c: CardId, hc: CardId) {
    // local e1=Effect.CreateEffect(hc)
    let e1 = effect::create_effect(d, hc);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetValue(0)
    effect::set_value(d, e1, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
