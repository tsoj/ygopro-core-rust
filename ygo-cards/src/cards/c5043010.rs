//! Firewall Dragon
use super::prelude::*;

pub const ID: u32 = 5043010;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon procedure
    // Link.AddProcedure(c,nil,2)
    link::add_procedure(d, c, None, 2, None, None, None, None);
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Return monsters from field/GY to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_NO_TURN_RESET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_NO_TURN_RESET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Cyberse monster from hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetCondition(s.regcon)
    effect::set_condition(d, e2, Some(condition(regcon)));
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(s.regcon2)
    effect::set_condition(d, e3, Some(condition(regcon2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_DELAY, 0);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e4, EVENT_CUSTOM + ID);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.thfilter(c)`: `return c:IsMonster() and c:IsAbleToHand()`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=#(c:GetMutualLinkedGroup():Filter(Card.IsMonster,nil))
    let mlg = card::get_mutual_linked_group(d, c);
    let ct = group::filter(d, &mlg, &|d, cc| card::is_monster(d, cc), NO_EX).len();
    if let Some(chkc) = a.chkc() {
        // return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.thfilter(chkc)
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && thfilter(d, chkc);
    }
    // if chk==0 then return ct>0 and Duel.IsExistingTarget(s.thfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,ct,nil)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    // c:RegisterFlagEffect(0,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
    card::register_flag_effect(d, c, 0, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).unwrap_or_default();
    // Duel.SendtoHand(g,nil,REASON_EFFECT)
    duel::send_to_hand(d, g, None, REASON_EFFECT, None);
}
/// `s.cfilter(c,tp,zone)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8, zone: u32) -> bool {
    // local seq=c:GetPreviousSequence()
    let mut seq = card::get_previous_sequence(d, c);
    // if not c:IsPreviousControler(tp) then seq=seq+16 end
    if !card::is_previous_controler(d, c, tp) {
        seq += 16;
    }
    // return c:IsPreviousLocation(LOCATION_MZONE) and bit.extract(zone,seq)~=0
    card::is_previous_location(d, c, LOCATION_MZONE) && (zone >> seq) & 1 != 0
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp,e:GetHandler():GetLinkedZone())
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, h, None);
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp, zone), 1, NO_EX)
}
/// `s.cfilter2(c,tp,zone)`: `return not c:IsReason(REASON_BATTLE) and s.cfilter(c,tp,zone)`
fn cfilter2(d: &mut Duel, c: CardId, tp: u8, zone: u32) -> bool {
    !card::is_reason(d, c, REASON_BATTLE) && cfilter(d, c, tp, zone)
}
/// `s.regcon2(e,tp,eg,ep,ev,re,r,rp)`
fn regcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter2,1,nil,tp,e:GetHandler():GetLinkedZone())
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, h, None);
    group::is_exists(d, &eg, &|d, c| cfilter2(d, c, a.tp, zone), 1, NO_EX)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, 0, 0);
}
/// `s.spfilter(c,e,tp)`: `return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:IsRace(RACE_CYBERSE)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) && card::is_race(d, c, RACE_CYBERSE)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
