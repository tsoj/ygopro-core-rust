//! コクーン・パーティ / Cocoon Party
use super::prelude::*;

pub const ID: u32 = 80368942;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NEO_SPACIAN, SET_CHRYSALIS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.gfilter(c)`
fn gfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NEO_SPACIAN) and c:IsMonster()
    card::is_set_card(d, c, &[SET_NEO_SPACIAN as u16]) && card::is_monster(d, c)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_CHRYSALIS) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_CHRYSALIS as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.gfilter,tp,LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(d, Some(&gfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
        // local ct=s.count_unique_code(g)
        let ct = count_unique_code(d, &g);
        // e:SetLabel(ct)
        effect::set_label(d, a.e, &[ct as i64]);
        // return ct>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>=ct
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,ct,nil,e,tp)
        return ct > 0
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 >= ct as i32
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, ct as u32, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,e:GetLabel(),tp,LOCATION_DECK)
    let label = effect::get_label(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, label as u8, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.gfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&gfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local ct=s.count_unique_code(g)
    let ct = count_unique_code(d, &g);
    // if ct==0 and Duel.GetLocationCount(tp,LOCATION_MZONE)<ct then return end
    if ct == 0 && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < ct as i32 {
        return;
    }
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #sg<ct then return end
    if sg.len() < ct {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local spg=sg:Select(tp,ct,ct,nil)
    let spg = group::select(d, &sg, a.tp, ct as u16, ct as u16, false, NO_EX).nil();
    // Duel.SpecialSummon(spg,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, spg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
/// `s.count_unique_code(g)`: the number of different codes the cards of `g` have.
fn count_unique_code(d: &mut Duel, g: &CardSet) -> usize {
    // local check={}
    let mut check: Vec<u32> = Vec::new();
    // local count=0
    let mut count = 0;
    // local tc=g:GetFirst()
    let _ = group::get_first(d, g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, g) {
        // for i,code in ipairs({tc:GetCode()}) do
        for code in card::get_code(d, tc) {
            // if not check[code] then
            if !check.contains(&code) {
                // check[code]=true
                check.push(code);
                // count=count+1
                count += 1;
            }
        }
    }
    // return count
    count
}
