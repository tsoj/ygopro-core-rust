//! Bujinfidel
use super::prelude::*;

pub const ID: u32 = 66727115;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_BUJIN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rfilter(c,e,tp,ft)`
fn rfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsSetCard(SET_BUJIN) and c:IsRace(RACE_BEASTWARRIOR) and (ft>0 or (c:GetSequence()<5 and c:IsControler(tp)))
    //     and (c:IsFaceup() or c:IsControler(tp))
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,c:GetCode(),e,tp)
    card::is_set_card(d, c, &[SET_BUJIN as u16])
        && card::is_race(d, c, RACE_BEASTWARRIOR)
        && (ft > 0 || (card::get_sequence(d, c) < 5 && card::is_controler(d, c, tp)))
        && (card::is_faceup(d, c) || card::is_controler(d, c, tp))
        && {
            let code = card::get_code(d, c)[0];
            duel::is_existing_target(d, Some(&|d, cc| spfilter(d, cc, code as i64, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        }
}
/// `s.spfilter(c,code,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, code: i64, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_BUJIN) and not c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_BUJIN as u16])
        && !card::is_code(d, c, &[code as u32])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e:GetLabel(),e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp)
            && card::is_location(d, chkc, LOCATION_GRAVE)
            && spfilter(d, chkc, effect::get_label(d, e), e, tp);
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=1 then return false end
        if effect::get_label(d, e) != 1 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return ft>-1 and Duel.CheckReleaseGroupCost(tp,s.rfilter,1,false,nil,nil,e,tp,ft)
        return ft > -1
            && duel::check_release_group_cost(d, tp, Some(&|d, c| rfilter(d, c, e, tp, ft)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.rfilter,1,1,false,nil,nil,e,tp,ft)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| rfilter(d, c, e, tp, ft)), 1, 1, false, None, NO_EX);
    // local code=g:GetFirst():GetCode()
    let first = group::get_first(d, &g).nil();
    let code = card::get_code(d, first)[0];
    // e:SetLabel(code)
    effect::set_label(d, e, &[code as i64]);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,code,e,tp)
    let sg = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, code as i64, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, sg, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
