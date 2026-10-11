//! Puppet Rook
//! Scripted by Nellag
use super::prelude::*;

pub const ID: u32 = 77590412;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 EARTH Warrior from your Deck to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Must attack this card, if able
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_MUST_ATTACK)
    effect::set_code(d, e2, EFFECT_MUST_ATTACK);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_MUST_ATTACK_MONSTER)
    effect::set_code(d, e3, EFFECT_MUST_ATTACK_MONSTER);
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, effect_card_fn(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon and change attack target
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e4, EVENT_BE_BATTLE_TARGET);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetTarget(s.atktg)
    effect::set_target(d, e4, Some(cost_target(atktg)));
    // e4:SetOperation(s.atkop)
    effect::set_operation(d, e4, Some(operation(atkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_WARRIOR) and c:IsAbleToGrave()
    card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_race(d, c, RACE_WARRIOR) && card::is_able_to_grave(d, c, None)
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAttackPos() and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_attack_pos(d, c) && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsAttackPos() and c:IsRelateToEffect(e) and Duel.ChangePosition(c,POS_FACEUP_DEFENSE)>0 then
    if card::is_faceup(d, c)
        && card::is_attack_pos(d, c)
        && card::is_relate_to_effect(d, c, a.e)
        && duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false) > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
        let sg = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SendtoGrave(sg,REASON_EFFECT)
            duel::send_to_grave(d, &sg, REASON_EFFECT, None, None);
        }
    }
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetHandler()
    c == effect::get_handler(d, e).nil()
}
// function s.atkfilter(c,e,tp)
fn atkfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelAbove(6) and c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_WARRIOR)
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_above(d, c, 6)
        && card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && card::is_race(d, c, RACE_WARRIOR)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.atkfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && atkfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetMZoneCount(tp,c)>0
    //   and Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| atkfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let tc = duel::select_target(d, tp, Some(&|d, c| atkfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, tc, 1, tp, 0);
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.SendtoGrave(c,REASON_EFFECT)==0 or not c:IsLocation(LOCATION_GRAVE) then return end
    if !card::is_relate_to_effect(d, c, a.e)
        || duel::send_to_grave(d, c, REASON_EFFECT, None, None) == 0
        || !card::is_location(d, c, LOCATION_GRAVE)
    {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and s.atkfilter(tc,e,tp)
    //   and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && atkfilter(d, tc, a.e, tp)
        && duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.ChangeAttackTarget(tc)
        duel::change_attack_target(d, Some(tc), false);
    }
}
