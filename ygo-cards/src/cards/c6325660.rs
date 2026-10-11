//! Dominus Spark
use super::prelude::*;

pub const ID: u32 = 6325660;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e0, EFFECT_TRAP_ACT_IN_HAND);
    // e0:SetCondition(function(e) return Duel.GetCustomActivityCount(id,1-e:GetHandlerPlayer(),ACTIVITY_CHAIN)>0 end)
    effect::set_condition(d, e0, Some(condition(e0_cond)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,s.chainfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(chainfilter));
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e) return Duel.GetCustomActivityCount(id,1-e:GetHandlerPlayer(),ACTIVITY_CHAIN)>0 end`
fn e0_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let hp = effect::get_handler_player(d, a.e);
    duel::get_custom_activity_count(d, ID, 1 - hp, ACTIVITY_CHAIN) > 0
}
/// `s.chainfilter(re,tp,cid)`
fn chainfilter(d: &mut Duel, a: Args) -> bool {
    // return not (re:IsMonsterEffect() and re:GetActivateLocation()&(LOCATION_HAND|LOCATION_GRAVE)>0)
    let re = a.e(0);
    !(effect::is_monster_effect(d, re) && (effect::get_activate_location(d, re) & (LOCATION_HAND | LOCATION_GRAVE) as u32) > 0)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - tp)
            && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local act_from_hand_chk=e:IsHasType(EFFECT_TYPE_ACTIVATE) and e:GetHandler():IsStatus(STATUS_ACT_FROM_HAND) and 1 or 0
    let act_from_hand = if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        let h = effect::get_handler(d, a.e).nil();
        card::is_status(d, h, STATUS_ACT_FROM_HAND)
    } else {
        false
    };
    let act_from_hand_chk: i64 = if act_from_hand { 1 } else { 0 };
    // e:SetLabel(act_from_hand_chk)
    effect::set_label(d, a.e, &[act_from_hand_chk]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - tp, LOCATION_HAND as i32);
    true
}
/// `s.spfilter(c,e,opp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, opp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,opp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, opp, false, false, None, None, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local opp=1-tp
    let opp = 1 - tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)>0
    //     and not Duel.IsExistingMatchingCard(Card.IsTrap,tp,LOCATION_GRAVE,0,1,nil)
    //     and Duel.GetLocationCount(opp,LOCATION_MZONE,opp)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,opp,LOCATION_HAND,0,1,nil,e,opp)
    //     and Duel.SelectYesNo(opp,aux.Stringid(id,1)) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_trap(d, c)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::get_location_count(d, opp, LOCATION_MZONE as u8, Some(opp), None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, opp)), opp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::select_yes_no(d, opp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,opp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, opp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(opp,s.spfilter,opp,LOCATION_HAND,0,1,1,nil,e,opp)
        let g = duel::select_matching_card(d, opp, Some(&|d, c| spfilter(d, c, a.e, opp)), opp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(g,0,opp,opp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, opp, opp, false, false, POS_FACEUP as u8, None);
        }
    }
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) and e:GetLabel()==1 then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && effect::get_label(d, a.e) == 1 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetValue(function(e,re) return re:IsMonsterEffect() and re:GetHandler():IsAttribute(ATTRIBUTE_EARTH|ATTRIBUTE_WATER|ATTRIBUTE_FIRE|ATTRIBUTE_WIND) end)
        effect::set_value_fn(d, e1, lua_fn(cannot_activate_value));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `function(e,re) return re:IsMonsterEffect() and re:GetHandler():IsAttribute(ATTRIBUTE_EARTH|ATTRIBUTE_WATER|ATTRIBUTE_FIRE|ATTRIBUTE_WIND) end`
fn cannot_activate_value(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_attribute(d, h, ATTRIBUTE_EARTH | ATTRIBUTE_WATER | ATTRIBUTE_FIRE | ATTRIBUTE_WIND)
}
