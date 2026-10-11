//! Synchro Dilemma
use super::prelude::*;

pub const ID: u32 = 47699948;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SYNCHRON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // To GY + Firewall Lite
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.tgcost)
    effect::set_cost(d, e1, Some(cost_target(tgcost)));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Pop + Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c,e,tp,ft)
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // if not (c:IsSetCard(SET_SYNCHRON) and c:IsMonster() and c:IsAbleToGraveAsCost()) then return false end
    if !(card::is_set_card(d, c, &[SET_SYNCHRON as u16]) && card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c)) {
        return false;
    }
    // if not c:IsLocation(LOCATION_MZONE) then
    if !card::is_location(d, c, LOCATION_MZONE) {
        // return ft>0 and Duel.IsExistingMatchingCard(Card.IsCanBeSpecialSummoned,tp,LOCATION_HAND,0,1,c,e,0,tp,false,false)
        ft > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, x| card::is_can_be_special_summoned(d, x, e, 0, tp, false, false, None, None, None)),
                tp,
                LOCATION_HAND,
                0,
                1,
                c.into(),
            )
    } else {
        // return c:IsFaceup() and ft>-1 and Duel.IsExistingMatchingCard(Card.IsCanBeSpecialSummoned,tp,LOCATION_HAND,0,1,c,e,0,tp,false,false)
        card::is_faceup(d, c)
            && ft > -1
            && duel::is_existing_matching_card(
                d,
                Some(&|d, x| card::is_can_be_special_summoned(d, x, e, 0, tp, false, false, None, None, None)),
                tp,
                LOCATION_HAND,
                0,
                1,
                c.into(),
            )
    }
}
// function s.tgcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,e,tp,ft) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| cfilter(d, c, a.e, a.tp, ft)),
            a.tp,
            LOCATION_HAND | LOCATION_MZONE,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,e,tp,ft)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d, c| cfilter(d, c, a.e, a.tp, ft)),
        a.tp,
        LOCATION_HAND | LOCATION_MZONE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,Card.IsCanBeSpecialSummoned,tp,LOCATION_HAND,0,1,1,nil,e,0,tp,false,false)
    let tc = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)),
        tp,
        LOCATION_HAND,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #tc>0 then
    if !tc.is_empty() {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.filter(c,e,tp,ft)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // if not c:IsLocation(LOCATION_MZONE) then
    if !card::is_location(d, c, LOCATION_MZONE) {
        // return ft>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,c:GetCode(),e,tp)
        ft > 0 && spfilter_exists(d, c, e, tp)
    } else {
        // return ft>-1 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,c:GetCode(),e,tp)
        ft > -1 && spfilter_exists(d, c, e, tp)
    }
}
// Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,c:GetCode(),e,tp)
fn spfilter_exists(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(
        d,
        Some(&|d, x| spfilter(d, x, code, e, tp)),
        tp,
        LOCATION_HAND | LOCATION_GRAVE,
        0,
        1,
        NO_EX,
    )
}
// function s.spfilter(c,code,e,tp)
fn spfilter(d: &mut Duel, c: CardId, code: u32, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SYNCHRON) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and not c:IsOriginalCode(code)
    card::is_set_card(d, c, &[SET_SYNCHRON as u16])
        && card::is_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && !card::is_original_code(d, c, &[code])
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c,ft=e:GetHandler(),Duel.GetLocationCount(tp,LOCATION_MZONE)
    let c = effect::get_handler(d, a.e).nil();
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD) and chkc:IsControler(tp) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD) && card::is_controler(d, chkc, a.tp) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_ONFIELD,0,1,c,e,tp,ft) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| filter(d, x, a.e, a.tp, ft)), a.tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectTarget(tp,s.filter,tp,LOCATION_ONFIELD,0,1,1,c,e,tp,ft)
    let tc = duel::select_target(d, a.tp, Some(&|d, x| filter(d, x, a.e, a.tp, ft)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc, 1, a.tp, LOCATION_ONFIELD as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0
    //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,tc:GetCode(),e,tp)
        let code = card::get_code(d, tc)[0];
        let f = aux::necro_valley_filter(move |d, x| spfilter(d, x, code, a.e, tp));
        let sc = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #sc>0 then
        if !sc.is_empty() {
            // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
