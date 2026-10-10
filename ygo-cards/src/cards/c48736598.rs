//! Codebreaker Virus Berserker
use super::prelude::*;

pub const ID: u32 = 48736598;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CODEBREAKER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,nil,2,3,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(3), Some(group_check(lcheck)), None, None);
    // special summon
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsSetCard,1,nil,SET_CODEBREAKER,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_set_card_ctx(d, c, &[SET_CODEBREAKER as u16], Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.spcon(e,...)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetMutualLinkedGroupCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_mutual_linked_group_count(d, h) > 0
}
/// `s.spfilter(c,e,tp,p,zones)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, p: u8, zones: [u32; 2]) -> bool {
    // return c:IsSetCard(SET_CODEBREAKER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,p,zones[p])
    card::is_set_card(d, c, &[SET_CODEBREAKER as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(p), Some(zones[p as usize]))
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local zones={}
    // zones[0]=aux.GetMMZonesPointedTo(0)
    // zones[1]=aux.GetMMZonesPointedTo(1)
    let zones = [aux::get_mm_zones_pointed_to(d, 0, None, None, None, None), aux::get_mm_zones_pointed_to(d, 1, None, None, None, None)];
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp,tp,zones)
    //     or Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp,1-tp,zones) end
    if a.chk == Some(0) {
        let e = a.e;
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, tp, zones)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
            || duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, 1 - tp, zones)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local zones={}
    // zones[0]=aux.GetMMZonesPointedTo(0)
    // zones[1]=aux.GetMMZonesPointedTo(1)
    let zones = [aux::get_mm_zones_pointed_to(d, 0, None, None, None, None), aux::get_mm_zones_pointed_to(d, 1, None, None, None, None)];
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,nil,e,tp,tp,zones)
    //     +Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,nil,e,tp,1-tp,zones)
    let f_own = aux::necro_valley_filter(move |d, x| spfilter(d, x, e, tp, tp, zones));
    let mut g = duel::get_matching_group(d, Some(&f_own), tp, LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
    let f_opp = aux::necro_valley_filter(move |d, x| spfilter(d, x, e, tp, 1 - tp, zones));
    let g2 = duel::get_matching_group(d, Some(&f_opp), tp, LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
    group::add_card(&mut g, &g2);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local ft=Duel.GetLocationCount(0,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zones[0])
    //     +Duel.GetLocationCount(1,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zones[1])
    let ft = duel::get_location_count(d, 0, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zones[0])).0
        + duel::get_location_count(d, 1, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zones[1])).0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=g:Select(tp,1,Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and 1 or math.min(ft,2),nil)
    let max = if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() { 1 } else { ft.min(2) };
    let sg = group::select(d, &g, tp, 1, max as u16, false, NO_EX).nil();
    // for tc in aux.Next(sg) do
    for tc in group::iter(d, &sg) {
        // local p
        // if s.spfilter(tc,e,tp,tp,zones) and s.spfilter(tc,e,tp,1-tp,zones) then
        //     p=Duel.SelectYesNo(tp,aux.Stringid(id,2)) and 1-tp or tp
        // elseif s.spfilter(tc,e,tp,tp,zones) then p=tp
        // else p=1-tp end
        let p = if spfilter(d, tc, e, tp, tp, zones) && spfilter(d, tc, e, tp, 1 - tp, zones) {
            if duel::select_yes_no(d, tp, aux::stringid(ID, 2)) { 1 - tp } else { tp }
        } else if spfilter(d, tc, e, tp, tp, zones) {
            tp
        } else {
            1 - tp
        };
        // Duel.SpecialSummonStep(tc,0,tp,p,false,false,POS_FACEUP,zones[p])
        duel::special_summon_step(d, tc, 0, tp, p, false, false, POS_FACEUP as u8, Some(zones[p as usize]));
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CODEBREAKER) and c:IsLinked()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_CODEBREAKER as u16]) && card::is_linked(d, c)
}
/// `s.destg(e,...,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,1,nil)
    //     and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_spell_trap(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,1,Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil),nil)
    let max = duel::get_matching_group_count(d, Some(&cfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_spell_trap(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, max as u16, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
