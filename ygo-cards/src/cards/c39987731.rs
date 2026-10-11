//! Jinzo the Machine Menace
use super::prelude::*;

pub const ID: u32 = 39987731;
// s.listed_names={id,CARD_JINZO}
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, CARD_JINZO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon itself from hand
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
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
    // Special summon "Jinzo" from hand or GY, then you can destroy all of opponent's traps
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetHintTiming(0,TIMING_SSET|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_SSET | TIMING_MAIN_END));
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetCondition(s.sscon)
    effect::set_condition(d, e2, Some(condition(sscon)));
    // e2:SetTarget(s.sstg)
    effect::set_target(d, e2, Some(cost_target(sstg)));
    // e2:SetOperation(s.ssop)
    effect::set_operation(d, e2, Some(operation(ssop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsTrap),tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_trap(d, c)),
        a.tp,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        1,
        NO_EX,
    )
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let mut g = CardSet::new();
    group::add_card(&mut g, &CardSet::from_card(c));
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp,c)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,1,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, c, 1, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    // Becomes level 6 until end of this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
    // e1:SetValue(6)
    effect::set_value(d, e1, 6);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.sscon(e,tp,eg,ep,ev,re,r,rp)
fn sscon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
// function s.ssfilter(c,e,tp)
fn ssfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_JINZO) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP)
    card::is_code(d, c, &[CARD_JINZO]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
}
// function s.sstg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sstg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetMZoneCount(tp,e:GetHandler())>0 and Duel.IsExistingMatchingCard(s.ssfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        let zone = duel::get_m_zone_count(d, a.tp, c.into(), None, None, None).0;
        let e = a.e;
        let tp = a.tp;
        return zone > 0 && duel::is_existing_matching_card(d, Some(&|d, c| ssfilter(d, c, e, tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
// function s.ssop(e,tp,eg,ep,ev,re,r,rp)
fn ssop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.ssfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let e = a.e;
    let tp = a.tp;
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(move |d, c| ssfilter(d, c, e, tp))), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local dg=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let dg = duel::get_matching_group(d, Some(&card::is_spell_trap), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)~=0 and #dg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !g.is_empty() && duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 && !dg.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.ConfirmCards(tp,dg)
        duel::confirm_cards(d, a.tp, dg.clone());
        // local sg=dg:Filter(Card.IsTrap,nil)
        let sg = group::filter(d, &dg, &|d, c| card::is_trap(d, c), NO_EX);
        // Duel.Destroy(sg,REASON_EFFECT)
        duel::destroy(d, sg, REASON_EFFECT, None, None);
    }
}
