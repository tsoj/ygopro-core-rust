//! Firewall Dragon Singularity
//! ファイアウォール・ドラゴン・シンギュラリティ
use super::prelude::*;

pub const ID: u32 = 21637210;

const TYPES: u32 = TYPE_RITUAL | TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),3)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 3, None, None, None, None);
    // Return cards from field/GY to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Cyberse monster from your GY
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
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e4, EVENT_CUSTOM + ID);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `getcount(tp)`
fn getcount(d: &mut Duel, tp: u8) -> u32 {
    // return Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsMonster),tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil):GetBinClassCount(function(c) return c:GetType()&TYPES end)
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_monster(d, c))), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    group::get_bin_class_count(d, &g, &|d, c| (card::get_type(d, c) & TYPES) as i64)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e);
    // local ct=getcount(tp)
    let ct = getcount(d, tp);
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_ONFIELD|LOCATION_GRAVE) and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_ONFIELD | LOCATION_GRAVE) && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return ct>0 and Duel.IsExistingTarget(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_target(d, Some(&|d, x| card::is_able_to_hand(d, x, None)), tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,ct,nil)
    let g = duel::select_target(d, tp, Some(&|d, x| card::is_able_to_hand(d, x, None)), tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, ct as u16, false, NO_EX).nil();
    let n = g.len() as u8;
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g==0 or Duel.SendtoHand(g,nil,REASON_EFFECT)==0 then return end
    if g.is_empty() || duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) == 0 {
        return;
    }
    // local ct=Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_HAND)
    let og = duel::get_operated_group(d);
    let ct = group::filter_count(d, &og, &|d, x| card::is_location(d, x, LOCATION_HAND), NO_EX);
    // if ct>0 then
    if ct > 0 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(ct*500)
        effect::set_value(d, e1, ct as i64 * 500);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
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
    card::is_previous_location(d, c, LOCATION_MZONE) && ((zone >> seq) & 1) != 0
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // return eg:IsExists(s.cfilter,1,nil,tp,e:GetHandler():GetLinkedZone())
    let c = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, c, None);
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, x| cfilter(d, x, tp, zone), 1, NO_EX)
}
/// `s.cfilter2(c,tp,zone)`
fn cfilter2(d: &mut Duel, c: CardId, tp: u8, zone: u32) -> bool {
    // return not c:IsReason(REASON_BATTLE) and s.cfilter(c,tp,zone)
    !card::is_reason(d, c, REASON_BATTLE) && cfilter(d, c, tp, zone)
}
/// `s.regcon2(e,tp,eg,ep,ev,re,r,rp)`
fn regcon2(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // return eg:IsExists(s.cfilter2,1,nil,tp,e:GetHandler():GetLinkedZone())
    let c = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, c, None);
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, x| cfilter2(d, x, tp, zone), 1, NO_EX)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,0,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::raise_single_event(d, c, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, 0, 0);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_CYBERSE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_CYBERSE) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
