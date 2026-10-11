//! Cyborg Doctor
use super::prelude::*;

pub const ID: u32 = 51020079;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
// function s.rfilter(c,e,tp,ft)
fn rfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsType(TYPE_TUNER) and (ft>0 or (c:GetSequence()<5 and c:IsControler(tp))) and (c:IsFaceup() or c:IsControler(tp))
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,c:GetLevel(),c:GetAttribute())
    card::is_type(d, c, TYPE_TUNER)
        && (ft > 0 || (card::get_sequence(d, c) < 5 && card::is_controler(d, c, tp)))
        && (card::is_faceup(d, c) || card::is_controler(d, c, tp))
        && {
            let lv = card::get_level(d, c);
            let att = card::get_attribute(d, c);
            duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, e, tp, Some(lv), Some(att))), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        }
}
// function s.spfilter(c,e,tp,lv,att)
// (a call without lv/att passes nil: IsLevel(nil) and IsAttribute(nil) are false)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: Option<u32>, att: Option<u32>) -> bool {
    // return c:IsLevel(lv) and c:IsAttribute(att) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    lv.is_some_and(|lv| card::is_level(d, c, &[lv]))
        && att.is_some_and(|att| card::is_attribute(d, c, att))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, a.tp, None, None);
    }
    if a.chk == Some(0) {
        // return Duel.CheckReleaseGroupCost(tp,s.rfilter,1,false,nil,nil,e,tp,ft)
        return duel::check_release_group_cost(d, a.tp, Some(&|d, x| rfilter(d, x, a.e, a.tp, ft)), 1, 1, false, None, NO_EX);
    }
    // local rg=Duel.SelectReleaseGroupCost(tp,s.rfilter,1,1,false,nil,nil,e,tp,ft)
    let rg = duel::select_release_group_cost(d, a.tp, Some(&|d, x| rfilter(d, x, a.e, a.tp, ft)), 1, 1, false, None, NO_EX);
    // local r=rg:GetFirst()
    let r = group::get_first(d, &rg).nil();
    // local lv=r:GetLevel()
    let lv = card::get_level(d, r);
    // local att=r:GetAttribute()
    let att = card::get_attribute(d, r);
    // Duel.Release(rg,REASON_COST)
    duel::release(d, &rg, REASON_COST, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,lv,att)
    let g = duel::select_target(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp, Some(lv), Some(att))), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP) end
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
