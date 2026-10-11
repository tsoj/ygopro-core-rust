//! Synchro Chase
use super::prelude::*;

pub const ID: u32 = 23442438;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WARRIOR, SET_SYNCHRON, SET_STARDUST], ..ScriptInfo::DEFAULT };

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
    // Special Summon 1 monster used as material for a Synchro Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Prevent activations in response to your monsters' effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetOperation(s.chainop)
    effect::set_operation(d, e3, Some(operation(chainop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_SYNCHRO) and c:IsSynchroSummoned()
    //     and c:IsSetCard({SET_WARRIOR,SET_SYNCHRON,SET_STARDUST}) and c:IsControler(tp)
    //     and c:GetMaterial():IsExists(s.spfilter,1,nil,e,tp,c)
    if !(card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_synchro_summoned(d, c)
        && card::is_set_card(d, c, &[SET_WARRIOR as u16, SET_SYNCHRON as u16, SET_STARDUST as u16])
        && card::is_controler(d, c, tp))
    {
        return false;
    }
    let mats = card::get_material(d, c);
    group::is_exists(d, &mats, &|d, x| spfilter(d, x, e, tp, c), 1, NO_EX)
}
/// `s.spfilter(c,e,tp,sync)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sync: CardId) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_GRAVE)
    //     and (c:GetReason()&(REASON_SYNCHRO+REASON_MATERIAL))==(REASON_SYNCHRO+REASON_MATERIAL)
    //     and c:GetReasonCard()==sync and c:IsCanBeEffectTarget(e)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_GRAVE)
        && (card::get_reason(d, c) & (REASON_SYNCHRO + REASON_MATERIAL)) == (REASON_SYNCHRO + REASON_MATERIAL)
        && card::get_reason_card(d, c) == Some(sync)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,e,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.e, a.tp), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ec=eg:Filter(s.cfilter,nil,e,tp):GetFirst()
    let eg = a.eg.clone().nil();
    let filtered = group::filter(d, &eg, &|d, c| cfilter(d, c, e, tp), NO_EX);
    let ec = group::get_first(d, &filtered).nil();
    // if chkc then return s.spfilter(chkc,e,tp,ec) and ec:GetMaterial():IsContains(chkc) end
    if let Some(chkc) = a.chkc() {
        return spfilter(d, chkc, e, tp, ec) && group::is_contains(&card::get_material(d, ec), chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=ec:GetMaterial():FilterSelect(tp,s.spfilter,1,1,nil,e,tp,ec)
    let mats = card::get_material(d, ec);
    let g = group::filter_select(d, &mats, tp, &|d, c| spfilter(d, c, e, tp, ec), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
/// `s.chainop(e,tp,eg,ep,ev,re,r,rp)`
fn chainop(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if ep==tp and re:IsMonsterEffect() and rc:IsType(TYPE_SYNCHRO)
    //     and (rc:IsOriginalSetCard(SET_WARRIOR) or rc:IsOriginalSetCard(SET_SYNCHRON) or rc:IsOriginalSetCard(SET_STARDUST)) then
    if a.ep == a.tp
        && effect::is_monster_effect(d, re)
        && card::is_type(d, rc, TYPE_SYNCHRO)
        && (card::is_original_set_card(d, rc, &[SET_WARRIOR as u16])
            || card::is_original_set_card(d, rc, &[SET_SYNCHRON as u16])
            || card::is_original_set_card(d, rc, &[SET_STARDUST as u16]))
    {
        // Duel.SetChainLimit(s.chainlm)
        duel::set_chain_limit(d, lua_fn(chainlm));
    }
}
/// `s.chainlm(e,rp,tp)`
fn chainlm(_d: &mut Duel, a: Args) -> bool {
    // return tp==rp
    a.p(2) == a.p(1)
}
