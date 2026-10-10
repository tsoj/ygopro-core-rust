//! Fusion Deployment
use super::prelude::*;

pub const ID: u32 = 6498706;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FUSION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
/// `s.counterfilter(c)`
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    // return not c:IsSummonLocation(LOCATION_EXTRA) or c:IsType(TYPE_FUSION)
    let c = a.c(0);
    !card::is_summon_location(d, c, LOCATION_EXTRA) || card::is_type(d, c, TYPE_FUSION)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0;
    }
    // Cannot Special Summon from the Extra Deck, except Fusion Monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT+EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Clock Lizard check
    // aux.addTempLizardCheck(e:GetHandler(),tp,s.lizfilter)
    let h = effect::get_handler(d, a.e).nil();
    aux::add_temp_lizard_check(d, h, tp, Some(lua_fn(lizfilter)), None, None, None, None);
    true
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // Can only special summon fusions from extra deck
    // return not c:IsType(TYPE_FUSION) and c:IsLocation(LOCATION_EXTRA)
    let c = a.c(1);
    !card::is_type(d, c, TYPE_FUSION) && card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.lizfilter(e,c)`
fn lizfilter(d: &mut Duel, a: Args) -> bool {
    // return not c:IsOriginalType(TYPE_FUSION)
    let c = a.c(1);
    !card::is_original_type(d, c, TYPE_FUSION)
}
/// `s.exfilter(c,e,tp)`
fn exfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // Check for fusion monster with listed material
    // return c.material and c:IsType(TYPE_FUSION) and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp,c)
    !crate::class::material(d, c).is_empty()
        && card::is_type(d, c, TYPE_FUSION)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, c)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,fc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, fc: CardId) -> bool {
    // Check if listed material can be special summoned
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:IsCode(table.unpack(fc.material))
    if !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) {
        return false;
    }
    let codes = crate::class::material(d, fc);
    card::is_code(d, c, &codes)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // Activation legality
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| exfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local cg=Duel.SelectMatchingCard(tp,s.exfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let cg = duel::select_matching_card(d, tp, Some(&|d, c| exfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #cg==0 then return end
    if cg.is_empty() {
        return;
    }
    // Duel.ConfirmCards(1-tp,cg)
    duel::confirm_cards(d, 1 - tp, cg.clone());
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp,cg:GetFirst())
    let fc = group::get_first(d, &cg).nil();
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, fc)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
