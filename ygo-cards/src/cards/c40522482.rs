//! Skydive Scorcher
use super::prelude::*;

pub const ID: u32 = 40522482;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELEMENTAL_HERO, SET_SKYSCRAPER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy monsters and inflict damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ELEMENTAL_HERO) and c:IsType(TYPE_FUSION)
    //     and Duel.IsExistingMatchingCard(s.desfilter,tp,0,LOCATION_MZONE,1,nil,c:GetAttack())
    if !card::is_faceup(d, c) || !card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]) || !card::is_type(d, c, TYPE_FUSION) {
        return false;
    }
    let atk = card::get_attack(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| desfilter(d, x, atk)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.desfilter(c,atk)`
fn desfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:GetAttack()>atk
    card::is_faceup(d, c) && card::get_attack(d, c) > atk
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tg=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let tg = duel::select_target(d, tp, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local atk=tg:GetFirst():GetAttack()
    let tg_first = group::get_first(d, &tg).nil();
    let atk = card::get_attack(d, tg_first);
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil,atk)
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, atk)), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
    let fc = duel::get_field_card(d, tp, LOCATION_FZONE, 0);
    // local dam=0
    let dam: i64;
    // if fc and s.ffilter(fc) then
    let fc_ok = match fc {
        Some(fc) => ffilter(d, fc),
        None => false,
    };
    if fc_ok {
        // dam=g:GetSum(Card.GetBaseAttack)
        dam = group::get_sum(d, &g, &|d, c| card::get_base_attack(d, c) as i64);
    } else {
        // g,dam=g:GetMaxGroup(Card.GetBaseAttack)
        dam = match group::get_max_group(d, &g, &|d, c| card::get_base_attack(d, c) as i64) {
            Some((_, v)) => v,
            None => 0,
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,1,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, 1 - tp, dam as i32);
    true
}
/// `s.ffilter(c)`
fn ffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SKYSCRAPER)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SKYSCRAPER as u16])
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    // if not tc or not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
    let Some(tc) = duel::first_target(d) else { return };
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil,tc:GetAttack())
    let tc_atk = card::get_attack(d, tc);
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, tc_atk)), tp, 0, LOCATION_MZONE, NO_EX);
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        // local og=Duel.GetOperatedGroup():Filter(Card.IsLocation,nil,LOCATION_GRAVE)
        let opg = duel::get_operated_group(d);
        let og = group::filter(d, &opg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
        // if #og==0 then return end
        if og.is_empty() {
            return;
        }
        // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
        let fc = duel::get_field_card(d, tp, LOCATION_FZONE, 0);
        // local dam=0
        let dam: i64;
        // if fc and s.ffilter(fc) then
        let fc_ok = match fc {
            Some(fc) => ffilter(d, fc),
            None => false,
        };
        if fc_ok {
            // dam=og:GetSum(Card.GetBaseAttack)
            dam = group::get_sum(d, &og, &|d, c| card::get_base_attack(d, c) as i64);
        } else {
            // g,dam=og:GetMaxGroup(Card.GetBaseAttack)
            dam = match group::get_max_group(d, &og, &|d, c| card::get_base_attack(d, c) as i64) {
                Some((_, v)) => v,
                None => 0,
            };
        }
        // Duel.Damage(1-tp,dam,REASON_EFFECT)
        duel::damage(d, 1 - tp, dam, REASON_EFFECT, false, None);
    }
}
