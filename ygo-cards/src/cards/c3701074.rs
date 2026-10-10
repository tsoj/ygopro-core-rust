//! Dark Cure
use super::prelude::*;

pub const ID: u32 = 3701074;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Activate
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e2, CATEGORY_RECOVER);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetTarget(s.rectg1)
    effect::set_target(d, e2, Some(cost_target(rectg1)));
    // e2:SetOperation(s.recop1)
    effect::set_operation(d, e2, Some(operation(recop1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e4, CATEGORY_RECOVER);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetTarget(s.rectg2)
    effect::set_target(d, e4, Some(cost_target(rectg2)));
    // e4:SetOperation(s.recop2)
    effect::set_operation(d, e4, Some(operation(recop2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.rectg1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rectg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return rp~=tp end
    if a.chk == Some(0) {
        return a.rp != a.tp;
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, a.eg());
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, 1 - a.tp, 0);
    true
}
/// `s.recop1(e,tp,eg,ep,ev,re,r,rp)`
fn recop1(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local rec=tc:GetAttack()/2
        let rec = card::get_attack(d, tc) as f64 / 2.0;
        // Duel.Recover(1-tp,rec,REASON_EFFECT)
        duel::recover(d, 1 - a.tp, lua_int(rec), REASON_EFFECT, false, None);
    }
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: Option<EffectId>, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsSummonPlayer(1-tp)
    //     and (not e or c:IsRelateToEffect(e))
    card::is_faceup(d, c)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_summon_player(d, c, 1 - tp)
        && e.is_none_or(|e| card::is_relate_to_effect(d, c, e))
}
/// `s.rectg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rectg2(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return eg:IsExists(s.filter,1,nil,nil,tp)
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| filter(d, c, None, a.tp), 1, NO_EX);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, a.eg());
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, 1 - a.tp, 0);
    true
}
/// `s.recop2(e,tp,eg,ep,ev,re,r,rp)`
fn recop2(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=eg:Filter(s.filter,nil,e,tp)
    let eg = a.eg.clone().nil();
    let mut g = group::filter(d, &eg, &|d, c| filter(d, c, Some(a.e), a.tp), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // if #g>1 then
        if g.len() > 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
            // g=g:Select(tp,1,1,nil)
            g = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        }
        // Duel.Recover(1-tp,g:GetFirst():GetAttack()/2,REASON_EFFECT)
        let tc = group::get_first(d, &g).nil();
        let rec = card::get_attack(d, tc) as f64 / 2.0;
        duel::recover(d, 1 - a.tp, lua_int(rec), REASON_EFFECT, false, None);
    }
}
