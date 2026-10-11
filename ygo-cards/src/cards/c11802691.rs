//! The Continuing Epic of Charles
//! Scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 11802691;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_INFERNOBLE_CHARLES],
    listed_series: &[SET_NOBLE_ARMS, SET_INFERNOBLE_KNIGHT, SET_NOBLE_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Infernoble Knight" monster from your hand or Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Equip 1 "Noble Knight" monster from your hand or Deck to 1 "Infernoble Knight Emperor Charles" you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e2, Some(condition(stat_change_damage_step_cond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `aux.StatChangeDamageStepCondition`
fn stat_change_damage_step_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    aux::stat_change_damage_step_condition(d)
}
/// `s.revfilter(c,sft,e,tp)`
fn revfilter(d: &mut Duel, c: CardId, sft: i32, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NOBLE_ARMS) and c:IsEquipSpell() and not c:IsPublic()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp,c,sft)
    card::is_set_card(d, c, &[SET_NOBLE_ARMS as u16])
        && card::is_equip_spell(d, c)
        && !card::is_public(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, c, sft)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,eq,sft)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, eq: CardId, sft: i32) -> bool {
    // return c:IsSetCard(SET_INFERNOBLE_KNIGHT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and (eq:IsAbleToGrave() or (sft>0 and eq:CheckEquipTarget(c)
    //     and eq:CheckUniqueOnField(tp) and not eq:IsForbidden()))
    card::is_set_card(d, c, &[SET_INFERNOBLE_KNIGHT as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && (card::is_able_to_grave(d, eq, None)
            || (sft > 0
                && card::check_equip_target(d, eq, c)
                && card::check_unique_on_field(d, eq, tp, None, None)
                && !card::is_forbidden(d, eq)))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local sft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut sft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:GetHandler():IsLocation(LOCATION_HAND) then sft=sft-1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_location(d, h, LOCATION_HAND) {
        sft -= 1;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.revfilter,tp,LOCATION_HAND,0,1,nil,sft,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| revfilter(d, c, sft, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local sft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let sft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local eq=Duel.SelectMatchingCard(tp,s.revfilter,tp,LOCATION_HAND,0,1,1,nil,sft,e,tp):GetFirst()
    let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| revfilter(d, c, sft, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if not eq then return end
    let Some(eq) = group::get_first(d, &sg) else {
        return;
    };
    // Duel.ConfirmCards(1-tp,eq)
    duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(eq));
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp,eq,sft):GetFirst()
    let tg = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, eq, sft)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &tg).nil();
    // if Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0 then
    if duel::special_summon(d, CardSet::from_card(tc), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local b1=sft>0 and eq:CheckEquipTarget(tc) and eq:CheckUniqueOnField(tp) and not eq:IsForbidden()
        let b1 = sft > 0
            && card::check_equip_target(d, eq, tc)
            && card::check_unique_on_field(d, eq, a.tp, None, None)
            && !card::is_forbidden(d, eq);
        // local b2=eq:IsAbleToGrave()
        let b2 = card::is_able_to_grave(d, eq, None);
        // if not (b1 or b2) then return end
        if !(b1 || b2) {
            return;
        }
        // local op=nil
        // if b1 and b2 then
        let op = if b1 && b2 {
            // op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
            duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))])
        } else if b1 {
            // op=(b1 and 1) or (b2 and 2)
            Some(1)
        } else {
            Some(2)
        };
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if op==1 then
        if op == Some(1) {
            // Duel.Equip(tp,eq,tc)
            duel::equip(d, a.tp, eq, tc, true, false);
        // elseif op==2 then
        } else if op == Some(2) {
            // Duel.SendtoGrave(eq,REASON_EFFECT)
            duel::send_to_grave(d, CardSet::from_card(eq), REASON_EFFECT, None, None);
        }
    }
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NOBLE_KNIGHT) and c:IsMonster() and c:CheckUniqueOnField(tp) and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16])
        && card::is_monster(d, c)
        && card::check_unique_on_field(d, c, tp, None, None)
        && !card::is_forbidden(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc:IsCode(CARD_INFERNOBLE_CHARLES) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp)
            && card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_faceup(d, chkc)
            && card::is_code(d, chkc, &[CARD_INFERNOBLE_CHARLES]);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsCode,CARD_INFERNOBLE_CHARLES),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(
                d,
                Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_INFERNOBLE_CHARLES])),
                a.tp,
                LOCATION_MZONE,
                0,
                1,
                NO_EX,
            )
            && duel::is_existing_matching_card(d, Some(&|d, c| eqfilter(d, c, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsCode,CARD_INFERNOBLE_CHARLES),tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(
        d,
        a.tp,
        Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_INFERNOBLE_CHARLES])),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        1,
        false,
        NO_EX,
    );
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFacedown() or not tc:IsRelateToEffect(e) then return end
    if card::is_facedown(d, tc) || !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local eq=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| eqfilter(d, c, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let eq = group::get_first(d, &sg);
    // if eq and Duel.Equip(tp,eq,tc,true,true) then
    if let Some(eq) = eq {
        if duel::equip(d, a.tp, eq, tc, true, true) {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // --Equip Limit
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            // e1:SetValue(function(e,c) return c==tc end)
            effect::set_value_fn(d, e1, effect_card_fn(move |_d, _e, c| c == tc));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // eq:RegisterEffect(e1)
            card::register_effect(d, eq, e1, false);
            // --The equipped monster gains 500 ATK
            // local e2=Effect.CreateEffect(eq)
            let e2 = effect::create_effect(d, eq);
            // e2:SetType(EFFECT_TYPE_EQUIP)
            effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
            // e2:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
            // e2:SetValue(500)
            effect::set_value(d, e2, 500);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // eq:RegisterEffect(e2)
            card::register_effect(d, eq, e2, false);
        }
    }
    // Duel.EquipComplete()
    duel::equip_complete(d);
}
