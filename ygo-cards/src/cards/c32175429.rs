//! Heroic Call
use super::prelude::*;

pub const ID: u32 = 32175429;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HEROIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Warrior monster from hand or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase the ATK of a target by 500 per "Heroic" card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(function(_,tp) return Duel.GetLP(tp)<=500 end)
    effect::set_condition(d, e2, Some(condition(lp_cond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_WARRIOR) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_WARRIOR as u64) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `function(_,tp) return Duel.GetLP(tp)<=500 end`
fn lp_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_lp(d, a.tp) <= 500
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) and not tc:IsSetCard(SET_HEROIC) then
    if let Some(tc) = tc {
        if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None)
            && !card::is_set_card(d, tc, &[SET_HEROIC as u16])
        {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, e).nil();
            // Negate its effects
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetValue(RESET_TURN_SET)
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
            // Cannot attack
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetDescription(3206)
            effect::set_description(d, e3, 3206);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e3, EFFECT_CANNOT_ATTACK);
            // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e3)
            card::register_effect(d, tc, e3, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.heroiccount(tp)`
fn heroiccount(d: &mut Duel, tp: u8) -> usize {
    // local mct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_HEROIC),tp,LOCATION_ONFIELD,0,nil)
    let mct = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_HEROIC as u16])), tp, LOCATION_ONFIELD, 0, NO_EX);
    // local oct=Duel.GetOverlayGroup(tp,1,0):FilterCount(Card.IsSetCard,nil,SET_HEROIC)
    let og = duel::get_overlay_group(d, tp, 1, 0, None);
    let oct = group::filter_count(d, &og, &|d, c| card::is_set_card(d, c, &[SET_HEROIC as u16]), NO_EX);
    // return mct+oct
    mct + oct
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::get_controler(d, chkc) == tp && card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // local ct=s.heroiccount(tp)
    let ct = heroiccount(d, tp);
    // if chk==0 then return ct>0 and Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,tc,1,0,ct*500)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, tc.map(CardSet::from_card), 1, 0, (ct * 500) as i32);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsFaceup() and tc:IsRelateToEffect(e)) then return end
    if !(card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e)) {
        return;
    }
    // local ct=s.heroiccount(tp)
    let ct = heroiccount(d, tp);
    // if ct>0 then
    if ct > 0 {
        // Increase ATK
        // local e1=Effect.CreateEffect(e:GetHandler())
        let c = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(ct*500)
        effect::set_value(d, e1, (ct * 500) as i64);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
