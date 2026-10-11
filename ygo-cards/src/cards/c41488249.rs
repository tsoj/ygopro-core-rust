//! Primite Howl
use super::prelude::*;

pub const ID: u32 = 41488249;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Prevent battle damage and Special Summon 1 Normal monster
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_START|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_START | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Take control of 1 monster your opponent controls
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e2, CATEGORY_CONTROL);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_START|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_START | TIMINGS_CHECK_MONSTER));
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.controltg)
    effect::set_target(d, e2, Some(cost_target(controltg)));
    // e2:SetOperation(s.controlop)
    effect::set_operation(d, e2, Some(operation(controlop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // s.announce_filter={TYPE_NORMAL,OPCODE_ISTYPE}
    // local code=Duel.AnnounceCard(tp,s.announce_filter)
    let announce_filter = [TYPE_NORMAL as u64, OPCODE_ISTYPE as u64];
    crate::class::set_announce_filter(d, ID, &announce_filter);
    let code = duel::announce_card(d, a.tp, &announce_filter);
    // Duel.SetTargetParam(code)
    duel::set_target_param(d, code as u32);
    // Duel.SetOperationInfo(0,CATEGORY_ANNOUNCE,nil,0,tp,ANNOUNCE_CARD_FILTER)
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, a.tp, ANNOUNCE_CARD_FILTER as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsType(TYPE_NORMAL) and c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_NORMAL) && card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local code=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let code = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    // You take no damage from battles involving your Normal Monsters with the declared name or "Primite" monsters this turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return c:IsSetCard(SET_PRIMITE) or (c:IsCode(code) and c:IsType(TYPE_NORMAL)) end)
    effect::set_target(d, e1, Some(effect_card_fn(move |d, _e, c| {
        card::is_set_card(d, c, &[SET_PRIMITE as u16]) || (card::is_code(d, c, &[code]) && card::is_type(d, c, TYPE_NORMAL))
    })));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(e:GetHandler(),0,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, h, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    // if Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)>0 or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) > 0 || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Special Summon 1 Normal Monster with the declared name
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp,code)
    let e = a.e;
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp, code))), tp, LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_NORMAL) and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(s.controlfilter,tp,0,LOCATION_MZONE,1,nil,c:GetAttack())
    let atk = card::get_attack(d, c);
    card::is_type(d, c, TYPE_NORMAL)
        && card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| controlfilter(d, x, atk)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.controlfilter(c,atk)`
fn controlfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsControlerCanBeChanged() and c:IsFaceup() and c:GetAttack()>atk
    card::is_controler_can_be_changed(d, c, false, None) && card::is_faceup(d, c) && card::get_attack(d, c) > atk
}
/// `s.cond2(e,tp)`: `Duel.IsTurnPlayer(1-tp)`
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.controltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn controltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsControler(tp) and s.tgfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && tgfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,tp)
    duel::select_target(d, a.tp, Some(&|d, c| tgfilter(d, c, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 1, a.tp, 0);
    true
}
/// `s.controlop(e,tp,eg,ep,ev,re,r,rp)`
fn controlop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        return;
    }
    // local atk=tc:GetAttack()
    let atk = card::get_attack(d, tc);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectMatchingCard(tp,s.controlfilter,tp,0,LOCATION_MZONE,1,1,nil,atk)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| controlfilter(d, c, atk)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // if #g>0 then
    if let Some(g) = g {
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, &g, true);
            // Duel.GetControl(g,tp,PHASE_END,1)
            duel::get_control(d, &g, a.tp, PHASE_END as u16, 1, None, None);
        }
    }
}
