//! Asset Mountis
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 98520301;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsLevelBelow,6),tp,0,LOCATION_MZONE,1,nil) end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_level_below(d, c, 6)), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
    })));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change the battle positions of all monsters on the field, also make all other Insect monsters on the field become the same Attribute and Level as this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_POSITION+CATEGORY_LVCHANGE)
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_LVCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)>Duel.GetFieldGroupCount(tp,0,LOCATION_HAND) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) > duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND)
    })));
    // e2:SetTarget(s.postg)
    effect::set_target(d, e2, Some(cost_target(postg)));
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 Level 8 or higher Insect monster from your Deck to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local posg=Duel.GetMatchingGroup(Card.IsCanChangePosition,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let posg = duel::get_matching_group(d, Some(&|d, c| card::is_can_change_position(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #posg>0 end
    if a.chk == Some(0) {
        return !posg.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,posg,#posg,tp,0)
    let n = posg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(posg), n, tp, 0);
    true
}
/// `s.insectfilter(c,attr,lv)`: `return c:IsRace(RACE_INSECT) and c:IsFaceup() and not (c:IsAttribute(attr) and c:IsLevel(lv))`
fn insectfilter(d: &mut Duel, c: CardId, attr: u32, lv: u32) -> bool {
    card::is_race(d, c, RACE_INSECT as u64) && card::is_faceup(d, c) && !(card::is_attribute(d, c, attr) && card::is_level(d, c, &[lv]))
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local posg=Duel.GetMatchingGroup(Card.IsCanChangePosition,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let posg = duel::get_matching_group(d, Some(&|d, c| card::is_can_change_position(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #posg>0 then
    if !posg.is_empty() {
        // Duel.ChangePosition(posg,POS_FACEUP_DEFENSE,POS_FACEDOWN_DEFENSE,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK)
        duel::change_position(d, posg, POS_FACEUP_DEFENSE as u8, Some(POS_FACEDOWN_DEFENSE as u8), Some(POS_FACEUP_ATTACK as u8), Some(POS_FACEUP_ATTACK as u8), false);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local attr=c:GetAttribute()
        let attr = card::get_attribute(d, c);
        // local lv=c:GetLevel()
        let lv = card::get_level(d, c);
        // local insg=Duel.GetMatchingGroup(s.insectfilter,tp,LOCATION_MZONE,LOCATION_MZONE,c,attr,lv)
        let insg = duel::get_matching_group(d, Some(&|d, x| insectfilter(d, x, attr, lv)), tp, LOCATION_MZONE, LOCATION_MZONE, c.into());
        // for tc in insg:Iter() do
        for tc in group::iter(d, &insg) {
            // if not tc:IsAttribute(attr) then
            if !card::is_attribute(d, tc, attr) {
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
                effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
                // e1:SetValue(attr)
                effect::set_value(d, e1, attr as i64);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
            }
            // if c:HasLevel() and tc:HasLevel() and not tc:IsLevel(lv) then
            if card::has_level(d, c) && card::has_level(d, tc) && !card::is_level(d, tc, &[lv]) {
                // local e2=Effect.CreateEffect(c)
                let e2 = effect::create_effect(d, c);
                // e2:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e2:SetCode(EFFECT_CHANGE_LEVEL_FINAL)
                effect::set_code(d, e2, EFFECT_CHANGE_LEVEL_FINAL);
                // e2:SetValue(lv)
                effect::set_value(d, e2, lv as i64);
                // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                // tc:RegisterEffect(e2)
                card::register_effect(d, tc, e2, false);
            }
        }
    }
    // You cannot Special Summon monsters for the rest of this turn, except Insect monsters
    // local e3=Effect.CreateEffect(c)
    let c = effect::get_handler(d, a.e).nil();
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e3, aux::stringid(ID, 3));
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetTarget(function(e,c) return not c:IsRace(RACE_INSECT) end)
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_INSECT as u64))));
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
}
/// `s.thfilter(c)`: `return c:IsLevelAbove(8) and c:IsRace(RACE_INSECT) and c:IsAbleToHand()`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_level_above(d, c, 8) && card::is_race(d, c, RACE_INSECT as u64) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
