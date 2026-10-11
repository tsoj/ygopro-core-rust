//! Mimesis Elephant
use super::prelude::*;

pub const ID: u32 = 10632284;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself
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
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change type and attribute
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:IsHasType(EFFECT_TYPE_ACTIVATE)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER+TYPE_EFFECT,0,2000,2,RACE_BEAST,ATTRIBUTE_EARTH) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && duel::is_player_can_special_summon_monster(d, tp, ID, mimesis_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER+TYPE_EFFECT,0,2000,2,RACE_BEAST,ATTRIBUTE_EARTH)) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && duel::is_player_can_special_summon_monster(d, tp, ID, mimesis_data(), None, None, 0)) {
        return;
    }
    // c:AddMonsterAttribute(TYPE_EFFECT+TYPE_TRAP)
    card::add_monster_attribute(d, c, TYPE_EFFECT + TYPE_TRAP, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// The monster data of the card itself (`TYPE_MONSTER+TYPE_EFFECT`, ...)
fn mimesis_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPE_MONSTER + TYPE_EFFECT),
        attack: Some(0),
        defense: Some(2000),
        level: Some(2),
        race: Some(RACE_BEAST),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
/// `s.chfilter(c,e)`
fn chfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.tgfilter(c,rc,att)`
fn tgfilter(d: &mut Duel, c: CardId, rc: u64, att: u32) -> bool {
    // return c:IsRaceExcept(rc) and c:IsAttributeExcept(att)
    card::is_race_except(d, c, rc, None, 0, None) && card::is_attribute_except(d, c, att, None, 0, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsFaceup() and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,e:GetLabel()) end
    if let Some(chkc) = a.chkc() {
        let labels = effect::get_labels(d, a.e);
        let rc = labels.first().copied().unwrap_or(0) as u64;
        let att = labels.get(1).copied().unwrap_or(0) as u32;
        return card::is_faceup(d, chkc) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, rc, att);
    }
    // local g=Duel.GetMatchingGroup(s.chfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| chfilter(d, c, a.e)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local rc=Duel.AnnounceAnotherRace(g,tp)
    let rc = duel::announce_another_race(d, &g, tp);
    // local att=Duel.AnnounceAnotherAttribute(g,tp)
    let att = duel::announce_another_attribute(d, &g, tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local sg=g:FilterSelect(tp,s.tgfilter,1,1,nil,rc,att)
    let sg = group::filter_select(d, &g, tp, &|d, c| tgfilter(d, c, rc, att), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(sg:GetFirst())
    let first = group::get_first(d, &sg).nil();
    duel::set_target_card(d, first);
    // e:SetLabel(rc,att)
    effect::set_label(d, a.e, &[rc as i64, att as i64]);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) {
        return;
    }
    // local rc,att=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let rc = labels.first().copied().unwrap_or(0);
    let att = labels.get(1).copied().unwrap_or(0);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if tc:IsRaceExcept(rc) then
    if card::is_race_except(d, tc, rc as u64, None, 0, None) {
        // Change monster type
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_RACE)
        effect::set_code(d, e1, EFFECT_CHANGE_RACE);
        // e1:SetValue(rc)
        effect::set_value(d, e1, rc);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
    // if tc:IsAttributeExcept(att) then
    if card::is_attribute_except(d, tc, att as u32, None, 0, None) {
        // Change attribute
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
        // e1:SetValue(att)
        effect::set_value(d, e1, att);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
