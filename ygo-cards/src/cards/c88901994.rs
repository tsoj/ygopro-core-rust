//! Synchro Rumble
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 88901994;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_CRIMSON_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Tuner or 1 Level 7/8 Dragon Synchro Monster from your GY
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
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destruction replacement for "Crimson Dragon" or Level 7 or 8 Dragon Synchro Monster(s)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, effect_card_fn(repval));
    // e2:SetOperation(s.repop)
    effect::set_operation(d, e2, Some(operation(repop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsType(TYPE_TUNER) or (c:IsLevel(7,8) and c:IsRace(RACE_DRAGON) and c:IsType(TYPE_SYNCHRO)))
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    (card::is_type(d, c, TYPE_TUNER) || (card::is_level(d, c, &[7, 8]) && card::is_race(d, c, RACE_DRAGON) && card::is_type(d, c, TYPE_SYNCHRO)))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        // return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp)
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(_,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_SYNCHRO) end)
    effect::set_target(d, e1, Some(effect_card_fn(cannot_sp_target)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // aux.addTempLizardCheck(c,tp,function(_,c) return not c:IsOriginalType(TYPE_SYNCHRO) end)
    aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(lizard_filter)), None, None, None, None);
}
/// `function(_,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_SYNCHRO) end` (the target of e1)
fn cannot_sp_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_SYNCHRO)
}
/// `function(_,c) return not c:IsOriginalType(TYPE_SYNCHRO) end` (the Clock Lizard check)
fn lizard_filter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_original_type(d, c, TYPE_SYNCHRO)
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_ONFIELD)
    //     and (c:IsCode(CARD_CRIMSON_DRAGON) or (c:IsLevel(7,8) and c:IsRace(RACE_DRAGON) and c:IsType(TYPE_SYNCHRO) and c:IsLocation(LOCATION_MZONE)))
    //     and not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_ONFIELD)
        && (card::is_code(d, c, &[CARD_CRIMSON_DRAGON])
            || (card::is_level(d, c, &[7, 8]) && card::is_race(d, c, RACE_DRAGON) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_location(d, c, LOCATION_MZONE)))
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        let eg = a.eg.clone().nil();
        return card::is_able_to_remove(d, h, None, None, None) && group::is_exists(d, &eg, &|d, c| repfilter(d, c, a.tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,e:GetHandler(),96)
    let h = effect::get_handler(d, a.e).nil();
    duel::select_effect_yes_no(d, a.tp, h, Some(96))
}
/// `s.repval(e,c)`: `return s.repfilter(c,e:GetHandlerPlayer())`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT|REASON_REPLACE)
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
}
