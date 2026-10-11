//! Hyper Galaxy
use super::prelude::*;

pub const ID: u32 = 83512285;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_GALAXYEYES_P_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Tribute opponent's monster, special summon "Galaxy-Eyes Photon Dragon"
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
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter(c,ft,tp)`: "Check for a monster with 2000+ ATK, besides "Galaxy-Eyes Photon Dragon""
fn costfilter(d: &mut Duel, c: CardId, ft: i32, tp: u8) -> bool {
    // return (c:IsControler(tp) or c:IsFaceup()) and c:IsAttackAbove(2000) and not c:IsCode(CARD_GALAXYEYES_P_DRAGON)
    //     and (ft>0 or c:GetSequence()<5)
    //     and Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,c,e)
    // (`e` is an undefined global in the Lua: nil)
    (card::is_controler(d, c, tp) || card::is_faceup(d, c))
        && card::is_attack_above(d, c, 2000)
        && !card::is_code(d, c, &[CARD_GALAXYEYES_P_DRAGON])
        && (ft > 0 || card::get_sequence(d, c) < 5)
        && duel::is_existing_target(d, Some(&|d, x| filter(d, x, None)), tp, 0, LOCATION_MZONE, 1, c.into())
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`: "Tribute cost"
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return ft>-1 and Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,aux.ReleaseCheckMMZ,nil,ft,tp) end
    if a.chk == Some(0) {
        return ft > -1
            && duel::check_release_group_cost(
                d,
                tp,
                Some(&|d, c| costfilter(d, c, ft, tp)),
                1,
                1,
                false,
                Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_mmz(d, sg, tp, exg)),
                NO_EX,
            );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // local g=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,aux.ReleaseCheckMMZ,nil,ft,tp)
    let g = duel::select_release_group_cost(
        d,
        tp,
        Some(&|d, c| costfilter(d, c, ft, tp)),
        1,
        1,
        false,
        Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_mmz(d, sg, tp, exg)),
        NO_EX,
    );
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.filter(c,e)`: "Opponent's monster with 2000+ ATK and can be tributed"
fn filter(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    // return c:IsFaceup() and c:IsAttackAbove(2000) and c:IsReleasableByEffect(e)
    card::is_faceup(d, c) && card::is_attack_above(d, c, 2000) && card::is_releasable_by_effect(d, c, e, None)
}
/// `s.spfilter(c,e,tp)`: "Check for "Galaxy-Eyes Photon Dragon""
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_GALAXYEYES_P_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_GALAXYEYES_P_DRAGON]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`: "Activation legality"
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc, Some(a.e));
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| spfilter(d, c, a.e, a.tp)),
            a.tp,
            LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil,e)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, Some(a.e))), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`: "Tribute opponent's monster, special summon "Galaxy-Eyes Photon Dragon""
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.Release(tc,REASON_EFFECT)~=0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e)
            && duel::release(d, tc, REASON_EFFECT, None) != 0
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
            let g = duel::select_matching_card(
                d,
                tp,
                Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp))),
                tp,
                LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
                0,
                1,
                1,
                false,
                NO_EX,
            )
            .nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
