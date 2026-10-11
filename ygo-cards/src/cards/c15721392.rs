//! Super Quantal Alphancall Appeal
//! Logical Nonsense
//! Substitute ID
use super::prelude::*;

pub const ID: u32 = 15721392;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUPER_QUANTAL_MECH_BEAST, SET_SUPER_QUANTUM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 "Super Quantal Mech Beast" and 1 "Super Quantum"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.confilter(c,tp)`: "Check for "Super Quant""
fn confilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReason(REASON_BATTLE) and c:IsPreviousSetCard(SET_SUPER_QUANT) and c:IsPreviousControler(tp)
    card::is_reason(d, c, REASON_BATTLE) && card::is_previous_set_card(d, c, &[SET_SUPER_QUANT as u16]) && card::is_previous_controler(d, c, tp)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`: "If it ever happened"
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.confilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| confilter(d, c, a.tp), 1, NO_EX)
}
/// `s.exfilter(c,e,tp,rp)`: "Check for "Super Quantal Mech Beast""
fn exfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rp: u8) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsSetCard(SET_SUPER_QUANTAL_MECH_BEAST)
    //   and Duel.GetLocationCountFromEx(tp,rp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_XYZ)
        && card::is_set_card(d, c, &[SET_SUPER_QUANTAL_MECH_BEAST as u16])
        && duel::get_location_count_from_ex(d, tp, Some(rp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spfilter(c,e,tp,xc)`: "Check for "Super Quantum" to special summon"
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, xc: CardId) -> bool {
    // if not (c:IsSetCard(SET_SUPER_QUANTUM) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) then return false end
    if !(card::is_set_card(d, c, &[SET_SUPER_QUANTUM as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)) {
        return false;
    }
    // for _,code in ipairs(xc.listed_names) do
    //   if c:IsCode(code) then return true end
    // end
    // return false
    // (a script without listed_names reads as an empty list here)
    let codes = crate::card_info(d, xc).listed_names;
    for &code in codes {
        if card::is_code(d, c, &[code]) {
            return true;
        }
    }
    false
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`: "Activation legality"
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let rp = a.rp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.exfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,rp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| exfilter(d, c, a.e, tp, rp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`: "Performing the effect of special summoning"
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let rp = a.rp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.exfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,rp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| exfilter(d, c, a.e, tp, rp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
            // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 or not tc.listed_names then return end
            // (`not tc.listed_names`: a script without listed_names reads as an empty list)
            if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 || crate::card_info(d, tc).listed_names.is_empty() {
                return;
            }
            // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp,tc)
            let g = duel::get_matching_group(
                d,
                Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp, tc))),
                tp,
                LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
                0,
                NO_EX,
            );
            // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
            if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                // local sc=g:Select(tp,1,1,nil):GetFirst()
                let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
                let sc = group::get_first(d, &sg).nil();
                // Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_DISABLE)
                effect::set_code(d, e1, EFFECT_DISABLE);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // sc:RegisterEffect(e1,true)
                card::register_effect(d, sc, e1, true);
                // local e2=Effect.CreateEffect(e:GetHandler())
                let e2 = effect::create_effect(d, h);
                // e2:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                // e2:SetCode(EFFECT_DISABLE_EFFECT)
                effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                // sc:RegisterEffect(e2,true)
                card::register_effect(d, sc, e2, true);
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    }
}
