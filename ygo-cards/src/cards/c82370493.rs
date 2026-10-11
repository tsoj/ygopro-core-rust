//! Predaplant Dheadevil
use super::prelude::*;

pub const ID: u32 = 82370493;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PREDAPLANT as u32],
    counter_list: &[COUNTER_PREDATOR as u32],
    material_setcode: &[SET_PREDAPLANT as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Predaplant" monster + 1 Level 1 monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_PREDAPLANT),aux.FilterBoolFunction(Card.IsLevel,1))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_PREDAPLANT as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, x, _fc, _st, _tp| card::is_level(d, x, &[1]))),
        ],
    );
    // c:AddMustFirstBeFusionSummoned()
    card::add_must_first_be_fusion_summoned(d, c);
    // Must first be either Fusion Summoned, or Special Summoned (from your Extra Deck) by Tributing 1 monster on either field with a Predator Counter and 1 "Predaplant" monster you control
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0, LOCATION_EXTRA);
    // e0:SetCondition(s.selfspcon)
    effect::set_condition(d, e0, Some(lua_fn(selfspcon)));
    // e0:SetTarget(s.selfsptg)
    effect::set_target(d, e0, Some(lua_fn(selfsptg)));
    // e0:SetOperation(s.selfspop)
    effect::set_operation(d, e0, Some(operation(selfspop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // During the Main Phase, if this card was Special Summoned this turn (Quick Effect): You can target Spells/Traps on the field, up to the number of monsters with a Predator Counter; destroy them
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return Duel.IsMainPhase() and e:GetHandler():IsStatus(STATUS_SPSUMMON_TURN) end)
    effect::set_condition(
        d,
        e1,
        Some(lua_fn(|d, a: Args| {
            if !duel::is_main_phase(d, None) {
                return false;
            }
            let h = effect::get_handler(d, a.e(0)).nil();
            card::is_status(d, h, STATUS_SPSUMMON_TURN)
        })),
    );
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.selfspcostfilter(c,tp,fc)`
fn selfspcostfilter(d: &mut Duel, c: CardId, tp: u8, fc: CardId) -> bool {
    // return ((c:IsSetCard(SET_PREDAPLANT) and c:IsControler(tp)) or c:HasCounter(COUNTER_PREDATOR)) and c:IsReleasable()
    //   and c:IsCanBeFusionMaterial(fc,MATERIAL_FUSION)
    ((card::is_set_card(d, c, &[SET_PREDAPLANT as u16]) && card::is_controler(d, c, tp)) || card::has_counter(d, c, COUNTER_PREDATOR as u16))
        && card::is_releasable(d, c, None, None)
        && card::is_can_be_fusion_material(d, c, Some(fc), Some(MATERIAL_FUSION as u64), None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetLocationCountFromEx(tp,tp,sg,e:GetHandler())>0 and sg:IsExists(s.predaplantchkfilter,1,nil,tp)
    //   and sg:IsExists(Card.HasCounter,1,nil,COUNTER_PREDATOR)
    let h = effect::get_handler(d, e.nil()).nil();
    let res = duel::get_location_count_from_ex(d, tp, Some(tp), sg.into(), duel::ExtraCard::Card(h), None).0 > 0
        && group::is_exists(d, sg, &|d, c| predaplantchkfilter(d, c, tp), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::has_counter(d, c, COUNTER_PREDATOR as u16), 1, NO_EX);
    (res, false)
}
/// `s.predaplantchkfilter(c,tp)`
fn predaplantchkfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_PREDAPLANT) and c:IsControler(tp)
    card::is_set_card(d, c, &[SET_PREDAPLANT as u16]) && card::is_controler(d, c, tp)
}
/// `s.selfspcon(e,c)`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.selfspcostfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,tp,c)
    let g = duel::get_matching_group(d, Some(&|d, x| selfspcostfilter(d, x, tp, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // return aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
    matches!(
        aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
        Err(true)
    )
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn selfsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    // local g=Duel.GetMatchingGroup(s.selfspcostfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,tp,c)
    let g = duel::get_matching_group(d, Some(&|d, x| selfspcostfilter(d, x, tp, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_RELEASE,nil,nil,true)
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(e),
        tp,
        Some(2),
        Some(2),
        Some(&rescon),
        1,
        tp,
        Some(HINTMSG_RELEASE as u64),
        None,
        None,
        true,
    )
    .unwrap_or_default();
    // if #sg==2 then
    if sg.len() == 2 {
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(sg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    // if g then
    if let Some(g) = effect::get_label_object(d, a.e).as_group().cloned() {
        // Duel.Release(g,REASON_COST|REASON_MATERIAL)
        duel::release(d, g, REASON_COST | REASON_MATERIAL, None);
    }
}
/// `s.destg(e,tp,...,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsSpellTrap() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_spell_trap(d, chkc);
    }
    // local max_count=Duel.GetMatchingGroupCount(Card.HasCounter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,COUNTER_PREDATOR)
    let max_count = duel::get_matching_group_count(d, Some(&|d, c| card::has_counter(d, c, COUNTER_PREDATOR as u16)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return max_count>0 and Duel.IsExistingTarget(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return max_count > 0 && duel::is_existing_target(d, Some(&card::is_spell_trap), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,max_count,nil)
    let g = duel::select_target(d, tp, Some(&card::is_spell_trap), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, max_count as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, 0);
    true
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Destroy(tg,REASON_EFFECT)
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
