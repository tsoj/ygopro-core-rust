//! Galaxy Tyranno
use super::prelude::*;

pub const ID: u32 = 24658418;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GALAXY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When a "Galaxy" monster you control is targeted by an attack, Special Summon itself
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If Summoned by its own effect, Xyz Summon 1 "Galaxy" monster using "Galaxy" monsters you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d).nil();
    // return at:IsFaceup() and at:IsControler(tp) and at:IsSetCard(SET_GALAXY)
    card::is_faceup(d, at) && card::is_controler(d, at, a.tp) && card::is_set_card(d, at, &[SET_GALAXY as u16])
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,1,tp,tp,false,false,POS_FACEUP_DEFENSE)
    duel::special_summon(d, c, 1, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SPECIAL+1
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_SPECIAL + 1
}
// function s.mfilter(c)
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_GALAXY) and not c:IsType(TYPE_TOKEN)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GALAXY as u16]) && !card::is_type(d, c, TYPE_TOKEN)
}
// function s.xyzfilter(c,mg)
fn xyzfilter(d: &mut Duel, c: CardId, mg: &CardSet) -> bool {
    // return c:IsSetCard(SET_GALAXY) and c:IsXyzSummonable(nil,mg)
    card::is_set_card(d, c, &[SET_GALAXY as u16]) && card::is_xyz_summonable(d, c, None, Some(mg.clone()), 0, 0)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.mfilter,tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, Some(&mfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
        // return Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_EXTRA,0,1,nil,g)
        return duel::is_existing_matching_card(d, Some(&|d, c| xyzfilter(d, c, &g)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.mfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&mfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local xyzg=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_EXTRA,0,nil,g)
    let xyzg = duel::get_matching_group(d, Some(&|d, c| xyzfilter(d, c, &g)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // if #xyzg>0 then
    if !xyzg.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local xyz=xyzg:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &xyzg, a.tp, 1, 1, false, NO_EX).nil();
        let xyz = group::get_first(d, &sel).nil();
        // Duel.XyzSummon(tp,xyz,nil,g)
        duel::xyz_summon(d, a.tp, xyz, None, Some(g.clone()), 0, 0);
    }
}
