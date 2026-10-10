//! Knightmare Corruptor Iblee
use super::prelude::*;

pub const ID: u32 = 10158145;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon a link monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(s.splimit)
    effect::set_target(d, e2, Some(lua_fn(splimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // special summon to the opponent's field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Cannot Special Summon from the Main Deck check
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCode(CARD_EHERO_BLAZEMAN)
    effect::set_code(d, e4, CARD_EHERO_BLAZEMAN);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spfilter(c,e,tp,ec)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ec: CardId) -> bool {
    // local zone=ec:GetToBeLinkedZone(c,tp)
    let zone = card::get_to_be_linked_zone(d, ec, c, tp, false, false);
    // return zone~=0 and c:IsLinkMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
    zone != 0
        && card::is_link_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp,e:GetHandler()) end
    if let Some(chkc) = a.chkc() {
        let ec = effect::get_handler(d, e).nil();
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp, ec);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,e:GetHandler()) end
    if a.chk == Some(0) {
        let ec = effect::get_handler(d, e).nil();
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, e, tp, ec)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,e:GetHandler())
    let ec = effect::get_handler(d, e).nil();
    let g = duel::select_target(d, tp, Some(&|d, x| spfilter(d, x, e, tp, ec)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) {
                // local zone=c:GetToBeLinkedZone(tc,tp)
                let zone = card::get_to_be_linked_zone(d, c, tc, tp, false, false);
                // if zone~=0 and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP,zone) then
                if zone != 0 && duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, Some(zone)) {
                    // local e1=Effect.CreateEffect(c)
                    let e1 = effect::create_effect(d, c);
                    // e1:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                    // e1:SetCode(EFFECT_DISABLE)
                    effect::set_code(d, e1, EFFECT_DISABLE);
                    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                    // tc:RegisterEffect(e1)
                    card::register_effect(d, tc, e1, false);
                    // local e2=Effect.CreateEffect(c)
                    let e2 = effect::create_effect(d, c);
                    // e2:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                    // e2:SetCode(EFFECT_DISABLE_EFFECT)
                    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
                    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                    // tc:RegisterEffect(e2)
                    card::register_effect(d, tc, e2, false);
                    // local e3=Effect.CreateEffect(c)
                    let e3 = effect::create_effect(d, c);
                    // e3:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
                    // e3:SetCode(EFFECT_SET_ATTACK_FINAL)
                    effect::set_code(d, e3, EFFECT_SET_ATTACK_FINAL);
                    // e3:SetValue(0)
                    effect::set_value(d, e3, 0);
                    // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
                    effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
                    // tc:RegisterEffect(e3)
                    card::register_effect(d, tc, e3, false);
                }
                // Duel.SpecialSummonComplete()
                duel::special_summon_complete(d);
            }
        }
    }
}
/// `s.splimit(e,c,tp,sumtp,sumpos)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    // if c:IsMonster() then return not c:IsType(TYPE_LINK)
    if card::is_monster(d, c) {
        !card::is_type(d, c, TYPE_LINK)
    // else return not c:IsOriginalType(TYPE_LINK) end
    } else {
        !card::is_original_type(d, c, TYPE_LINK)
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) and e:GetHandler():IsPreviousControler(tp)
    card::is_previous_location(d, h, LOCATION_ONFIELD) && card::is_previous_controler(d, h, a.tp)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE,1-tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), Some(1 - a.tp), None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // Duel.SpecialSummon(e:GetHandler(),0,tp,1-tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, h, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
