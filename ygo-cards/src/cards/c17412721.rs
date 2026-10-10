//! Elder Entity Norden
use super::prelude::*;

pub const ID: u32 = 17412721;

pub const INFO: ScriptInfo = ScriptInfo {
    miracle_synchro_fusion: true,
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 Synchro or Xyz Monster + 1 Synchro or Xyz Monster
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsType,TYPE_SYNCHRO|TYPE_XYZ),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_type_ctx(d, c, TYPE_SYNCHRO | TYPE_XYZ, Some(fc), st, Some(tp)))), 2)],
    );
    // When this card is Special Summoned from the Extra Deck: ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DISABLE+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DISABLE + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsSummonLocation(LOCATION_EXTRA) end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// The condition of the Special Summon effect: `function(e) return e:GetHandler():IsSummonLocation(LOCATION_EXTRA) end`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_summon_location(d, h, LOCATION_EXTRA)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 4) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // --Its effects are negated
        // tc:NegateEffects(c)
        card::negate_effects(d, tc, c, None, false, None);
        // if c:IsRelateToEffect(e) and c:IsFaceup() then
        if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
            // c:SetCardTarget(tc)
            card::set_card_target(d, c, tc);
            // c:CreateRelation(tc,RESET_EVENT|RESET_OVERLAY|RESET_MSCHANGE|RESET_TURN_SET)
            card::create_relation(d, c, tc, RESET_EVENT | RESET_OVERLAY | RESET_MSCHANGE | RESET_TURN_SET);
            // tc:CreateRelation(c,RESET_EVENT|RESETS_STANDARD|RESET_OVERLAY)
            card::create_relation(d, tc, c, RESET_EVENT | RESETS_STANDARD | RESET_OVERLAY);
            // --Also banish it when this card leaves the field
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_LEAVE_FIELD_P)
            effect::set_code(d, e1, EVENT_LEAVE_FIELD_P);
            // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
            effect::set_operation(d, e1, Some(operation(move |d, _a| leave_p_op(d, c, tc))));
            // e1:SetReset(RESET_EVENT|(RESET_OVERLAY|RESET_MSCHANGE|RESET_TURN_SET))
            effect::set_reset(d, e1, RESET_EVENT | (RESET_OVERLAY | RESET_MSCHANGE | RESET_TURN_SET), 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// The operation of the `EVENT_LEAVE_FIELD_P` effect: registers the banishing effect (Norden leaves, `tc` is still linked).
fn leave_p_op(d: &mut Duel, c: CardId, tc: CardId) {
    // --Register the banishing effect only if its effects aren't negated right before it leaves the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e1, EVENT_LEAVE_FIELD);
    // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e1, Some(operation(move |d, a| remove_op(d, a, c, tc))));
    // e1:SetReset(RESET_EVENT|(RESET_OVERLAY|RESET_MSCHANGE|RESET_TURN_SET))
    effect::set_reset(d, e1, RESET_EVENT | (RESET_OVERLAY | RESET_MSCHANGE | RESET_TURN_SET), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// The operation of the `EVENT_LEAVE_FIELD` effect: banishes `tc` if it is still linked, then resets itself.
fn remove_op(d: &mut Duel, a: &EffArgs, c: CardId, tc: CardId) {
    // if c:IsRelateToCard(tc) and tc:IsRelateToCard(c) then
    if card::is_relate_to_card(d, c, tc) && card::is_relate_to_card(d, tc, c) {
        // c:ReleaseRelation(tc)
        card::release_relation(d, c, tc);
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
    // e:Reset()
    effect::reset(d, a.e);
}
