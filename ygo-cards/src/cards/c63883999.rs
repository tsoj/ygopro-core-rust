//! Archfiend Palabyrinth
use super::prelude::*;

pub const ID: u32 = 63883999;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atkup
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_FIEND))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_FIEND))));
    // e2:SetValue(500)
    effect::set_value(d, e2, 500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // spsummon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c,e,tp,ft)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return lv>0 and c:IsFaceup() and c:IsSetCard(SET_ARCHFIEND)
    //     and Duel.IsExistingMatchingCard(s.rfilter,tp,LOCATION_MZONE,0,1,c,ft)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_DECK,0,1,nil,lv,e,tp)
    lv > 0
        && card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_ARCHFIEND as u16])
        && duel::is_existing_matching_card(d, Some(&|d, x| rfilter(d, x, ft)), tp, LOCATION_MZONE, 0, 1, c.into())
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, lv, e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.rfilter(c,ft)`
fn rfilter(d: &mut Duel, c: CardId, ft: i32) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_FIEND) and c:IsAbleToRemove() and (ft>0 or c:GetSequence()<5)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_FIEND) && card::is_able_to_remove(d, c, None, None, None) && (ft > 0 || card::get_sequence(d, c) < 5)
}
/// `s.spfilter(c,lv,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, lv: u32, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ARCHFIEND) and c:IsLevel(lv) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ARCHFIEND as u16]) && card::is_level(d, c, &[lv]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,e,tp,ft) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.e, a.tp, ft);
    }
    // if chk==0 then return ft>-1 and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,e,tp,ft) end
    if a.chk == Some(0) {
        return ft > -1 && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp, ft)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,e,tp,ft)
    let _ = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp, ft)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp,chk)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
            // local rg=Duel.SelectMatchingCard(tp,s.rfilter,tp,LOCATION_MZONE,0,1,1,tc,Duel.GetLocationCount(tp,LOCATION_MZONE))
            let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
            let rg = duel::select_matching_card(d, a.tp, Some(&|d, x| rfilter(d, x, ft)), a.tp, LOCATION_MZONE, 0, 1, 1, false, tc.into()).nil();
            // if #rg>0 and Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
            if !rg.is_empty()
                && duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
                && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
                // local sg=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_DECK,0,1,1,nil,tc:GetLevel(),e,tp)
                let tc_lv = card::get_level(d, tc);
                let nvf = aux::necro_valley_filter(|d, x| spfilter(d, x, tc_lv, a.e, a.tp));
                let sg = duel::select_matching_card(d, a.tp, Some(&nvf), a.tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
                // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
