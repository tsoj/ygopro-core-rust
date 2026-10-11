//! Assault Sentinel
//! バスター・スナイパー
use super::prelude::*;

pub const ID: u32 = 39015;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ASSAULT_MODE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // reveal
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.rvtg)
    effect::set_target(d, e2, Some(cost_target(rvtg)));
    // e2:SetOperation(s.rvop)
    effect::set_operation(d, e2, Some(operation(rvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:ListsCode(CARD_ASSAULT_MODE) and not c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::lists_code(d, c, &[CARD_ASSAULT_MODE]) && !card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    // if chk==0 then return ft>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_HAND) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler(c)
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK|LOCATION_HAND,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // lizard check
    // aux.addTempLizardCheck(e:GetHandler(),tp,s.lizfilter)
    aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(lizfilter)), None, None, None, None);
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    // return not c:IsType(TYPE_SYNCHRO) and c:IsLocation(LOCATION_EXTRA)
    !card::is_type(d, c, TYPE_SYNCHRO) && card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.lizfilter(e,c)`
fn lizfilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsOriginalType(TYPE_SYNCHRO)
    !card::is_original_type(d, c, TYPE_SYNCHRO)
}
/// `s.exfilter1(c,tp)`
fn exfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_SYNCHRO) and not c:IsPublic() and Duel.IsExistingTarget(s.rvfilter2,tp,LOCATION_MZONE,0,1,nil,c)
    card::is_type(d, c, TYPE_SYNCHRO) && !card::is_public(d, c) && duel::is_existing_target(d, Some(&|d, x| rvfilter2(d, x, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.exfilter2(c,oc)`
fn exfilter2(d: &mut Duel, c: CardId, oc: CardId) -> bool {
    // return c:IsType(TYPE_SYNCHRO) and not c:IsPublic() and (not c:IsRace(oc:GetRace()) or not c:IsAttribute(oc:GetAttribute()))
    if !(card::is_type(d, c, TYPE_SYNCHRO) && !card::is_public(d, c)) {
        return false;
    }
    let race = card::get_race(d, oc);
    if !card::is_race(d, c, race) {
        return true;
    }
    let attr = card::get_attribute(d, oc);
    !card::is_attribute(d, c, attr)
}
/// `s.rvfilter1(c,tp)`
fn rvfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and Duel.IsExistingMatchingCard(s.exfilter2,tp,LOCATION_EXTRA,0,1,nil,c)
    card::is_faceup(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| exfilter2(d, x, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.rvfilter2(c,oc)`
fn rvfilter2(d: &mut Duel, c: CardId, oc: CardId) -> bool {
    // return c:IsFaceup() and (not c:IsRace(oc:GetRace()) or not c:IsAttribute(oc:GetAttribute()))
    if !card::is_faceup(d, c) {
        return false;
    }
    let race = card::get_race(d, oc);
    if !card::is_race(d, c, race) {
        return true;
    }
    let attr = card::get_attribute(d, oc);
    !card::is_attribute(d, c, attr)
}
/// `s.rvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.rvfilter1(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && rvfilter1(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.exfilter1,tp,LOCATION_EXTRA,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| exfilter1(d, x, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.rvfilter1,tp,LOCATION_MZONE,0,1,1,nil,tp)
    duel::select_target(d, tp, Some(&|d, x| rvfilter1(d, x, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.rvop(e,tp,eg,ep,ev,re,r,rp)`
fn rvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local g=Duel.SelectMatchingCard(tp,s.exfilter2,tp,LOCATION_EXTRA,0,1,1,nil,tc)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| exfilter2(d, x, tc)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g.clone());
            // local oc=g:GetFirst()
            let oc = group::get_first(d, &g).nil();
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_RACE)
            effect::set_code(d, e1, EFFECT_CHANGE_RACE);
            // e1:SetValue(oc:GetRace())
            let race = card::get_race(d, oc);
            effect::set_value(d, e1, race as i64);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1,true)
            card::register_effect(d, tc, e1, true);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_CHANGE_ATTRIBUTE)
            effect::set_code(d, e2, EFFECT_CHANGE_ATTRIBUTE);
            // e2:SetValue(oc:GetAttribute())
            let attr = card::get_attribute(d, oc);
            effect::set_value(d, e2, attr as i64);
            // tc:RegisterEffect(e2,true)
            card::register_effect(d, tc, e2, true);
        }
    }
}
