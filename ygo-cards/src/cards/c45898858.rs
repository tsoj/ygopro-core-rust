//! Bonding - H2O
use super::prelude::*;

pub const ID: u32 = 45898858;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[58071123, 22587018, 85066822], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon "Water Dragon" from the hand, Deck or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
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
/// `s.spcheck(sg,tp)`
fn spcheck(d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet) -> (bool, bool) {
    // return aux.ReleaseCheckMMZ(sg,tp) and sg:IsExists(s.chk,1,nil,sg)
    let (ok, _) = aux::release_check_mmz(d, sg, tp, exg);
    (ok && group::is_exists(d, sg, &|d, x| chk(d, x, sg), 1, NO_EX), false)
}
/// `s.chk(c,sg)`
fn chk(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsCode(58071123) and sg:IsExists(Card.IsCode,2,c,22587018)
    card::is_code(d, c, &[58071123]) && group::is_exists(d, sg, &|d, x| card::is_code(d, x, &[22587018]), 2, c.into())
}
/// `Card.IsCode` with the extra codes of the cost call (`22587018, 58071123`)
fn cost_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[22587018, 58071123])
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsCode,3,nil,s.spcheck,nil,22587018,58071123) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&cost_filter), 3, 3, false, Some(&spcheck), NO_EX);
    }
    // local sg=Duel.SelectReleaseGroupCost(tp,Card.IsCode,3,3,nil,s.spcheck,nil,22587018,58071123)
    let sg = duel::select_release_group_cost(d, a.tp, Some(&cost_filter), 3, 3, false, Some(&spcheck), NO_EX);
    // Duel.Release(sg,REASON_COST)
    duel::release(d, sg, REASON_COST, None);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(85066822) and c:IsCanBeSpecialSummoned(e,0,tp,true,true)
    card::is_code(d, c, &[85066822]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, true, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()==0 and Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return false end
        if effect::get_label(d, a.e) == 0 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp)
        let e = a.e;
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| spfilter(d, c, e, tp)),
            tp,
            LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))(d, c)),
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
        // Duel.SpecialSummon(g,0,tp,tp,true,true,POS_FACEUP)
        duel::special_summon(d, &g, 0, tp, tp, true, true, POS_FACEUP as u8, None);
        // g:GetFirst():CompleteProcedure()
        let tc = group::get_first(d, &g).nil();
        card::complete_procedure(d, tc);
    }
}
