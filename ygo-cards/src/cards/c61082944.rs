//! Spiritual Power Recovery Potion
//! 霊力回復薬
use super::prelude::*;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_POSSESSED], ..ScriptInfo::DEFAULT };

pub const ID: u32 = 61082944;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish any number of Spellcaster monsters and/or Spells from your GY; all monsters you currently control gain 200 ATK for each card banished this way, and if they do, you gain 400 LP for each card banished this way
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.atkcost)
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // e1:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your Main Phase: You can banish this card from your GY; Special Summon any number of Spellcaster monsters with different Attributes from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkcostfilter(c)`
fn atkcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsRace(RACE_SPELLCASTER) or c:IsSpell()) and c:IsAbleToRemoveAsCost()
    (card::is_race(d, c, RACE_SPELLCASTER) || card::is_spell(d, c)) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkcostfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&atkcostfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local max_count=Duel.GetMatchingGroupCount(s.atkcostfilter,tp,LOCATION_GRAVE,0,nil)
    let max_count = duel::get_matching_group_count(d, Some(&atkcostfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.atkcostfilter,tp,LOCATION_GRAVE,0,1,max_count,nil)
    let g = duel::select_matching_card(d, tp, Some(&atkcostfilter), tp, LOCATION_GRAVE, 0, 1, max_count as u16, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabel(#g)
    effect::set_label(d, e, &[g.len() as i64]);
    true
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // local banish_count=e:GetLabel()
    let banish_count = effect::get_label(d, e);
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,g,1,tp,200*banish_count)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(g), 1, tp, (200 * banish_count) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,400*banish_count)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, (400 * banish_count) as i32);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local prev_atk=0
    // (assigned before it is read in the loop below)
    let mut prev_atk: i32;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local banish_count=e:GetLabel()
    let banish_count = effect::get_label(d, e);
    // local atk=200*banish_count
    let atk = 200 * banish_count;
    // local atk_change_chk=false
    let mut atk_change_chk = false;
    // for atkc in g:Iter() do
    for atkc in group::iter(d, &g) {
        // prev_atk=atkc:GetAttack()
        prev_atk = card::get_attack(d, atkc);
        // All monsters you currently control gain 200 ATK for each card banished this way
        // atkc:UpdateAttack(atk,RESET_EVENT|RESETS_STANDARD,c)
        card::update_attack(d, atkc, atk, Some(RESET_EVENT | RESETS_STANDARD), Some(c), None);
        // if not atk_change_chk and atkc:GetAttack()>prev_atk then
        if !atk_change_chk && card::get_attack(d, atkc) > prev_atk {
            // atk_change_chk=true
            atk_change_chk = true;
        }
    }
    // if not atk_change_chk then return end
    if !atk_change_chk {
        return;
    }
    // Duel.Recover(tp,400*banish_count,REASON_EFFECT)
    duel::recover(d, tp, 400 * banish_count, REASON_EFFECT, false, None);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_SPELLCASTER) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.attribute_value` stands for `Card.GetAttribute` as a value (`aux.dpcheck(Card.GetAttribute)`)
fn attribute_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_attribute(d, c) as i64
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local max_attribute_count=g:GetClassCount(Card.GetAttribute)
    let max_attribute_count = group::get_class_count(d, &g, &attribute_value);
    // ft=math.min(ft,max_attribute_count)
    // (ft is not used after this point)
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    let _ = duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT);
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,max_attribute_count,aux.dpcheck(Card.GetAttribute),1,tp,HINTMSG_SPSUMMON)
    let dp = aux::dpcheck(attribute_value);
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(max_attribute_count), Some(&dp), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
