//! E・HERO ゴッド・ネオス (Elemental HERO Divine Neos)
use super::prelude::*;

pub const ID: u32 = 31111109;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NEOS, SET_NEO_SPACIAN, SET_HERO],
    material_setcode: &[SET_HERO, SET_NEOS, SET_NEO_SPACIAN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Must be Fusion Summoned
    // c:AddMustBeFusionSummoned()
    card::add_must_be_fusion_summoned(d, c);
    // Fusion Summon materials: 5 "Neos", "Neo Space", "Neo-Spacian", or "HERO" monsters
    // including at least 1 "Neos" or "Neo Space" monster, 1 "Neo-Spacian" monster, and 1 "HERO" monster
    // Fusion.AddProcMixRep(c,true,true,s.ffilter,2,2,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_NEOS),aux.FilterBoolFunctionEx(Card.IsSetCard,SET_NEO_SPACIAN),aux.FilterBoolFunctionEx(Card.IsSetCard,SET_HERO))
    fusion::add_proc_mix_rep(
        d,
        c,
        true,
        true,
        fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, fc, st, tp))),
        2,
        2,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_NEOS as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_NEO_SPACIAN as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_HERO as u16], Some(fc), st, Some(tp)))),
        ],
    );
    // Gains 500 ATK and the banished monster's effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(s.copycost)
    effect::set_cost(d, e1, Some(cost_target(copycost)));
    // e1:SetOperation(s.copyop)
    effect::set_operation(d, e1, Some(operation(copyop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ffilter(c,fc,sumtype,tp)`
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_HERO,fc,sumtype,tp) or c:IsSetCard(SET_NEOS,fc,sumtype,tp) or c:IsSetCard(SET_NEO_SPACIAN,fc,sumtype,tp)
    card::is_set_card_ctx(d, c, &[SET_HERO as u16], Some(fc), sumtype, Some(tp))
        || card::is_set_card_ctx(d, c, &[SET_NEOS as u16], Some(fc), sumtype, Some(tp))
        || card::is_set_card_ctx(d, c, &[SET_NEO_SPACIAN as u16], Some(fc), sumtype, Some(tp))
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_NEOS,SET_NEO_SPACIAN,SET_HERO}) and c:IsMonster()
    //     and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_set_card(d, c, &[SET_NEOS as u16, SET_NEO_SPACIAN as u16, SET_HERO as u16])
        && card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.copycost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn copycost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&costfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // Duel.Remove(tc,POS_FACEUP,REASON_COST)
    duel::remove(d, tc, POS_FACEUP as u8, REASON_COST, None, None);
    // Duel.SetTargetParam(tc:GetOriginalCode())
    let code = card::get_original_code(d, tc);
    duel::set_target_param(d, code);
    true
}
/// `s.copyop(e,tp,eg,ep,ev,re,r,rp)`
fn copyop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local code=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let code = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if c:IsRelateToEffect(e) and c:IsFaceup() and code~=0 then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && code != 0 {
        // Gains 500 ATK
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
        effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(500)
        effect::set_value(d, e1, 500);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // Gains the banished monster's effects
        // local cid=c:CopyEffect(code,RESETS_STANDARD_PHASE_END,1)
        let cid = card::copy_effect(d, c, code as u32, RESETS_STANDARD_PHASE_END, 1);
        // Reset the copied effect(s) manually
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e2, aux::stringid(ID, 1));
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
        // e2:SetRange(LOCATION_MZONE)
        effect::set_range(d, e2, LOCATION_MZONE);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // e2:SetCountLimit(1)
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        // e2:SetLabel(cid)
        effect::set_label(d, e2, &[cid as i64]);
        // e2:SetOperation(s.resettop)
        effect::set_operation(d, e2, Some(operation(resettop)));
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
/// `s.resettop(e,tp,eg,ep,ev,re,r,rp)`
fn resettop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local cid=e:GetLabel()
    let cid = effect::get_label(d, a.e);
    // c:ResetEffect(cid,RESET_COPY)
    card::reset_effect(d, c, cid as u32, RESET_COPY);
    // c:ResetEffect(RESET_DISABLE,RESET_EVENT)
    card::reset_effect(d, c, RESET_DISABLE, RESET_EVENT);
    // Duel.HintSelection(Group.FromCards(c))
    duel::hint_selection(d, CardSet::from_card(c), true);
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
}
