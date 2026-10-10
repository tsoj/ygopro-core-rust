//! Enigmaster Packbit
use super::prelude::*;

pub const ID: u32 = 72444406;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Place 1 monster from your GY or your opponent's field in the Spell & Trap Zone as a Continuous Trap
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.pltg)
    effect::set_target(d, e1, Some(cost_target(pltg)));
    // e1:SetOperation(s.plop)
    effect::set_operation(d, e1, Some(operation(plop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::is_synchro_summoned(d, c)
    })));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 Monster Card from your Spell & Trap Zone
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e3:SetCondition(function(e) return e:GetHandler():IsContinuousTrap() end)
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::is_continuous_trap(d, c)
    })));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // local owner_p=c:GetOwner()
    let owner_p = card::get_owner(d, c);
    // return c:IsMonster() and c:IsFaceup() and Duel.GetLocationCount(owner_p,LOCATION_SZONE)>0 and not c:IsForbidden()
    //   and c:CheckUniqueOnField(owner_p)
    card::is_monster(d, c)
        && card::is_faceup(d, c)
        && duel::get_location_count(d, owner_p, LOCATION_SZONE as u8, None, None, None).0 > 0
        && !card::is_forbidden(d, c)
        && card::check_unique_on_field(d, c, owner_p, None, None)
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return s.plfilter(chkc) and ((chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE))
    //   or (chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE))) end
    if let Some(chkc) = a.chkc() {
        return plfilter(d, chkc)
            && ((card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE))
                || (card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE)));
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil,REASON_EFFECT)
    //   and Duel.IsExistingTarget(s.plfilter,tp,LOCATION_GRAVE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_discardable(d, x, Some(REASON_EFFECT), None)), tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&plfilter), tp, LOCATION_GRAVE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectTarget(tp,s.plfilter,tp,LOCATION_GRAVE,LOCATION_MZONE,1,1,nil):GetFirst()
    let g = duel::select_target(d, tp, Some(&plfilter), tp, LOCATION_GRAVE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    // if tc:IsLocation(LOCATION_GRAVE) then
    if card::is_location(d, tc, LOCATION_GRAVE) {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(tc)), 1, tp, 0);
    }
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_EFFECT|REASON_DISCARD,nil,REASON_EFFECT)==0 then return end
    if duel::discard_hand(d, tp, Some(&|d, x| card::is_discardable(d, x, Some(REASON_EFFECT), None)), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) == 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local owner_p=tc:GetOwner()
    let owner_p = card::get_owner(d, tc);
    // if not tc:IsRelateToEffect(e) or tc:IsImmuneToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, e) || card::is_immune_to_effect(d, tc, e) {
        return;
    }
    // if Duel.GetLocationCount(owner_p,LOCATION_SZONE)==0 then
    if duel::get_location_count(d, owner_p, LOCATION_SZONE as u8, None, None, None).0 == 0 {
        // Duel.SendtoGrave(tc,REASON_RULE,nil,PLAYER_NONE)
        duel::send_to_grave(d, tc, REASON_RULE, None, Some(PLAYER_NONE as u8));
    } else {
        let is_monster = card::is_monster_card(d, tc);
        // elseif tc:CheckUniqueOnField(owner_p) and Duel.MoveToField(tc,tp,owner_p,LOCATION_SZONE,POS_FACEUP,tc:IsMonsterCard()) then
        if card::check_unique_on_field(d, tc, owner_p, None, None)
            && duel::move_to_field(d, tc, tp, owner_p, LOCATION_SZONE as u16, POS_FACEUP as u8, is_monster, None)
        {
            // Treated as a Continuous Trap
            // local e1=Effect.CreateEffect(e:GetHandler())
            let c = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_TYPE)
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            // e1:SetValue(TYPE_TRAP|TYPE_CONTINUOUS)
            effect::set_value(d, e1, (TYPE_TRAP | TYPE_CONTINUOUS) as i64);
            // e1:SetReset((RESET_EVENT|RESETS_STANDARD)&~RESET_TURN_SET)
            effect::set_reset(d, e1, (RESET_EVENT | RESETS_STANDARD) & !RESET_TURN_SET, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsOriginalType(TYPE_MONSTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_faceup(d, c)
        && card::is_original_type(d, c, TYPE_MONSTER)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_STZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_STZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_STZONE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_STZONE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_STZONE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_STZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
