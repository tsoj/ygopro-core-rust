//! Protector Whelp of the Destruction Swordsman
use super::prelude::*;

pub const ID: u32 = 47158777;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_DESTRUCTION_SWORD, SET_BUSTER_BLADER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,nil,2,2)
    link::add_procedure(d, c, None, 2, Some(2), None, None, None);
    // Upon link summon, send 1 "Destruction Sword" from deck to GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.tgcon)
    effect::set_condition(d, e1, Some(condition(tgcon)));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // End of Battle Phase, if opponent has no monsters, burn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.dmgcon)
    effect::set_condition(d, e2, Some(condition(dmgcon)));
    // e2:SetTarget(s.dmgtg)
    effect::set_target(d, e2, Some(cost_target(dmgtg)));
    // e2:SetOperation(s.dmgop)
    effect::set_operation(d, e2, Some(operation(dmgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// --Check for "Destruction Sword" card to send to GY
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DESTRUCTION_SWORD) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_DESTRUCTION_SWORD as u16]) && card::is_able_to_grave(d, c, None)
}
// --Check for "Buster Blader" monster
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_BUSTER_BLADER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_BUSTER_BLADER as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// --If link summoned
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
// --Activation legality
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
// --Send 1 "Destruction Sword" to GY, then can special summon "Buster Blader" monster from hand
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g1 = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.filter,tp,LOCATION_HAND,0,nil,e,tp)
    let g2 = duel::get_matching_group(d, Some(&|d, x| filter(d, x, e, tp)), tp, LOCATION_HAND, 0, NO_EX);
    // if #g1>0 and Duel.SendtoGrave(g1,REASON_EFFECT)>0 and g1:GetFirst():IsLocation(LOCATION_GRAVE)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #g2>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    let g1_count = group::get_count(&g1.clone().nil());
    if g1_count > 0
        && duel::send_to_grave(d, g1.clone().nil(), REASON_EFFECT, None, None) > 0
        && {
            let first = group::get_first(d, &g1.clone().nil()).nil();
            card::is_location(d, first, LOCATION_GRAVE)
        }
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && !g2.is_empty()
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g3=g2:Select(tp,1,1,nil)
        let g3 = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(g3,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g3, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// --If it's player's Battle Phase and opponent has no monsters
/// `s.dmgcon(e,tp,eg,ep,ev,re,r,rp)`
fn dmgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)==0
    duel::is_turn_player(d, a.tp) && duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) == 0
}
// --Check for "Buster Blader" monster that didn't attack
/// `s.dmgfilter(c)`
fn dmgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetAttackAnnouncedCount()==0 and c:IsFaceup() and c:IsSetCard(SET_BUSTER_BLADER)
    card::get_attack_announced_count(d, c) == 0 && card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BUSTER_BLADER as u16])
}
// --Activation legality
/// `s.dmgtg(e,tp,eg,ep,ev,re,r,rp,chk)`: the Lua reads `chkc` as an undefined global (nil), so no
/// `chkc` branch is run here.
fn dmgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingTarget(s.dmgfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&dmgfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.dmgfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&dmgfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,g:GetFirst():GetAttack())
    let first = group::get_first(d, &g.clone().nil()).nil();
    let atk = card::get_attack(d, first);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk);
    true
}
// --Burn equal to a "Buster Blader" monster's ATK
/// `s.dmgop(e,tp,eg,ep,ev,re,r,rp)`
fn dmgop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Damage(1-tp,tc:GetAttack(),REASON_EFFECT)
        let atk = card::get_attack(d, tc) as i64;
        duel::damage(d, 1 - a.tp, atk, REASON_EFFECT, false, None);
    }
}
