//! Battlin' Boxing Cross Counter
use super::prelude::*;

pub const ID: u32 = 19688343;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BATTLIN_BOXER, SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate the activation of a monster effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsMonsterEffect() and Duel.IsChainNegatable(ev)
    a.rp == 1 - a.tp && effect::is_monster_effect(d, a.re.nil()) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.desfilter(c,e)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard({SET_BATTLIN_BOXER,SET_NUMBER}) and c:IsType(TYPE_XYZ)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_BATTLIN_BOXER as u16, SET_NUMBER as u16])
        && card::is_type(d, c, TYPE_XYZ)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&desfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsSetCard(SET_BATTLIN_BOXER) and c:IsType(TYPE_XYZ) and not c:IsCode(code)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    card::is_set_card(d, c, &[SET_BATTLIN_BOXER as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && !card::is_code(d, c, &[code])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local dc=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let dsel = duel::select_matching_card(d, tp, Some(&desfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let dc = group::get_first(d, &dsel);
    // if dc and Duel.Destroy(dc,REASON_EFFECT)>0 and Duel.NegateActivation(ev)
    //     and re:GetHandler():IsRelateToEffect(re) and Duel.Destroy(eg,REASON_EFFECT)>0 then
    let Some(dc) = dc else { return };
    if duel::destroy(d, dc, REASON_EFFECT, None, None) > 0
        && duel::negate_activation(d, a.ev as u8)
        && re_handler_relate(d, a)
        && duel::destroy(d, a.eg(), REASON_EFFECT, None, None) > 0
    {
        // local code=dc:GetCode()
        let code = card::get_code(d, dc)[0];
        // if Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,code) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, code)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,code):GetFirst()
            let scsel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, code)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX)
                .nil();
            // if not sc then return end
            let Some(sc) = group::get_first(d, &scsel) else { return };
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local c=e:GetHandler()
            let c = effect::get_handler(d, e).nil();
            // if Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 and c:IsRelateToEffect(e) then
            if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 && card::is_relate_to_effect(d, c, e) {
                // c:CancelToGrave()
                card::cancel_to_grave(d, c, true);
                // Duel.Overlay(sc,c)
                duel::overlay(d, sc, c, false);
            }
        }
    }
}
/// `re:GetHandler():IsRelateToEffect(re)`
fn re_handler_relate(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    card::is_relate_to_effect(d, rh, re)
}
