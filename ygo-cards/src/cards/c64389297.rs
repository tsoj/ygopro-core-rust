//! Magical Labyrinth
use super::prelude::*;

pub const ID: u32 = 64389297;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[63162310, 67284908], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip procedure
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsCode,67284908))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_code(d, c, &[67284908]))), None, None, None, None, None, None);
    // Special Summon "Wall Shadow"
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_BOTH_SIDE, 0);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_condition(d, e3, Some(condition(spcon)));
    effect::set_cost(d, e3, Some(cost_target(spcost)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetEquipTarget():IsControler(tp)
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_equip_target(d, c).nil();
    card::is_controler(d, tc, a.tp)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=c:GetEquipTarget()
    let tc = card::get_equip_target(d, c);
    // if chk==0 then return c:IsReleasable() and tc:IsReleasable() end
    if a.chk == Some(0) {
        return card::is_releasable(d, c, None, None) && card::is_releasable(d, tc.nil(), None, None);
    }
    // local g=Group.FromCards(c,tc)
    let mut g = CardSet::from_card(c);
    if let Some(tc) = tc {
        group::add_card(&mut g, &CardSet::from_card(tc));
    }
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(63162310) and c:IsCanBeSpecialSummoned(e,0,tp,true,true)
    card::is_code(d, c, &[63162310]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, true, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if e:GetHandler():GetEquipTarget() and e:GetHandler():GetEquipTarget():GetSequence()<5 then ft=ft+1 end
    let h = effect::get_handler(d, a.e).nil();
    if let Some(eqt) = card::get_equip_target(d, h) {
        if card::get_sequence(d, eqt) < 5 {
            ft += 1;
        }
    }
    // if chk==0 then return ft>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,true,true,POS_FACEUP)
        duel::special_summon(d, g.clone(), 0, a.tp, a.tp, true, true, POS_FACEUP as u8, None);
        // g:GetFirst():CompleteProcedure()
        let first = group::get_first(d, &g).nil();
        card::complete_procedure(d, first);
    }
}
