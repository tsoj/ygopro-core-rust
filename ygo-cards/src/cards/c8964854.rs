//! Combination Attack
use super::prelude::*;

pub const ID: u32 = 8964854;

pub const INFO: ScriptInfo = ScriptInfo { listed_card_types: &[TYPE_UNION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_BATTLE_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_END));
    // e1:SetCondition(function() return Duel.IsBattlePhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_battle_phase(d, None))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,e,tp)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if c:GetAttackAnnouncedCount()==0 then return false end
    if card::get_attack_announced_count(d, c) == 0 {
        return false;
    }
    // local g=c:GetEquipGroup():Filter(s.spfilter,nil,e,tp)
    let eg = card::get_equip_group(d, c);
    let g = group::filter(d, &eg, &|d, x| spfilter(d, x, e, tp), NO_EX);
    // if #g==0 or (#g>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) then return false end
    if g.is_empty() || (g.len() > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()) {
        return false;
    }
    // local g1,g2=g:Split(Card.IsControler,nil,tp)
    let (g1, g2) = group::split(d, &g, &|d, x| card::is_controler(d, x, tp), NO_EX);
    // return (#g1==0 or Duel.GetLocationCount(tp,LOCATION_MZONE)>=#g1) and (#g2==0 or Duel.GetLocationCount(1-tp,LOCATION_MZONE)>=#g2)
    (g1.is_empty() || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= g1.len() as i32)
        && (g2.is_empty() || duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 >= g2.len() as i32)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsHasEffect(EFFECT_UNION_STATUS) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,c:GetControler())
    !card::is_has_effect(d, c, EFFECT_UNION_STATUS, None).is_empty()
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(card::get_controler(d, c)), None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,e,tp)
    duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_SZONE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_SZONE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc)) {
        return;
    }
    // local g=tc:GetEquipGroup():Filter(s.spfilter,nil,e,tp)
    let eg = card::get_equip_group(d, tc);
    let g = group::filter(d, &eg, &|d, x| spfilter(d, x, e, tp), NO_EX);
    // if #g==0 or (#g>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) then return false end
    if g.is_empty() || (g.len() > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()) {
        return;
    }
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // Duel.SpecialSummonStep(tc,0,tp,tc:GetControler(),false,false,POS_FACEUP)
        let controler = card::get_controler(d, tc);
        duel::special_summon_step(d, tc, 0, tp, controler, false, false, POS_FACEUP as u8, None);
    }
    // if Duel.SpecialSummonComplete()==0 then return end
    if duel::special_summon_complete(d) == 0 {
        return;
    }
    // --The monster that was equipped with a Union monster can attack again this turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    // e1:SetValue(tc:GetAttackAnnouncedCount())
    let count = card::get_attack_announced_count(d, tc);
    effect::set_value(d, e1, count as i64);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
}
