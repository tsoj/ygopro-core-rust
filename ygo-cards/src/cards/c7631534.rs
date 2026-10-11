//! Dual Avatar Feet - Armored Un-Gyo
use super::prelude::*;

pub const ID: u32 = 7631534;

pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_DUAL_AVATAR], listed_series: &[SET_DUAL_AVATAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion summon procedure
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcFunRep(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_DUAL_AVATAR),2,true)
    fusion::add_proc_fun_rep(
        d,
        c,
        fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_DUAL_AVATAR as u16], Some(fc), st, Some(tp))),
        2,
        true,
    );
    // Destruction replacement
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetValue(s.value)
    effect::set_value_fn(d, e1, effect_card_fn(value));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate effects from monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetHintTiming(0,TIMING_SPSUMMON|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_SPSUMMON | TIMING_MAIN_END));
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.desfilter(c,e,tp)`
fn desfilter(d: &mut Duel, c: CardId, e: EffectId, _tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DUAL_AVATAR) and c:IsDestructable(e) and not c:IsStatus(STATUS_DESTROY_CONFIRMED)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_DUAL_AVATAR as u16])
        && card::is_destructable(d, c, Some(e))
        && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsReason(REASON_EFFECT)
    //   and not c:IsReason(REASON_REPLACE) and c:IsSetCard(SET_DUAL_AVATAR) and c:IsType(TYPE_FUSION)
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_reason(d, c, REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_set_card(d, c, &[SET_DUAL_AVATAR as u16])
        && card::is_type(d, c, TYPE_FUSION)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return #(eg:Filter(s.repfilter,nil,tp))==1
    //   and Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_MZONE,0,1,eg,e,tp) end
    if a.chk == Some(0) {
        return group::filter(d, &eg, &|d, c| repfilter(d, c, tp), NO_EX).len() == 1
            && duel::is_existing_matching_card(d, Some(&|d, c| desfilter(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, (&eg).into());
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_MZONE,0,1,1,eg,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| desfilter(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, (&eg).into()).nil();
        // Duel.SetTargetCard(g)
        duel::set_target_card(d, &g);
        // g:GetFirst():SetStatus(STATUS_DESTROY_CONFIRMED,true)
        let first = group::get_first(d, &g).nil();
        card::set_status(d, first, STATUS_DESTROY_CONFIRMED, true);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.value(e,c)`
fn value(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, _a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned().nil();
    // g:GetFirst():SetStatus(STATUS_DESTROY_CONFIRMED,false)
    let first = group::get_first(d, &g).nil();
    card::set_status(d, first, STATUS_DESTROY_CONFIRMED, false);
    // Duel.Destroy(g,REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, &g, REASON_EFFECT | REASON_REPLACE, None, None);
}
/// `s.negfil(c)`
fn negfil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DUAL_AVATAR) and c:IsFusionSummoned()
    //   and c:GetMaterial():IsExists(Card.IsType,1,nil,TYPE_EFFECT)
    if !(card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DUAL_AVATAR as u16]) && card::is_fusion_summoned(d, c)) {
        return false;
    }
    let mat = card::get_material(d, c);
    group::is_exists(d, &mat, &|d, cc| card::is_type(d, cc, TYPE_EFFECT), 1, NO_EX)
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsMainPhase() and Duel.IsExistingMatchingCard(s.negfil,tp,LOCATION_MZONE,0,1,nil)
    duel::is_main_phase(d, None) && duel::is_existing_matching_card(d, Some(&negfil), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.filter(c)`
fn negfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSummonLocation(LOCATION_EXTRA) and c:IsFaceup() and not c:IsDisabled()
    card::is_summon_location(d, c, LOCATION_EXTRA) && card::is_faceup(d, c) && !card::is_disabled(d, c)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && negfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&negfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let _g = duel::select_target(d, tp, Some(&negfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
            duel::negate_related_chain(d, tc, RESET_TURN_SET);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e2:SetValue(RESET_TURN_SET)
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
