//! Dominus Spiral
use super::prelude::*;

pub const ID: u32 = 42091632;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If your opponent activated a monster effect in the hand or GY this turn, you can activate this card from your hand
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
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
    // Return 1 monster your opponent controls to the hand/Extra Deck, then if you have no Traps in your GY, they can Special Summon 1 monster from their GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_TOEXTRA+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e) return Duel.GetCustomActivityCount(id,1-e:GetHandlerPlayer(),ACTIVITY_CHAIN)>0 end`
fn e0_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::get_custom_activity_count(d, ID, 1 - p, ACTIVITY_CHAIN) > 0
}
/// `s.chainfilter(re,tp,cid)`
fn chainfilter(d: &mut Duel, a: Args) -> bool {
    // return not (re:IsMonsterEffect() and re:GetActivateLocation()&(LOCATION_HAND|LOCATION_GRAVE)>0)
    let re = a.e(0);
    !(effect::is_monster_effect(d, re) && (effect::get_activate_location(d, re) & (LOCATION_HAND | LOCATION_GRAVE)) > 0)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsAbleToHand() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_able_to_hand(d, chkc, None) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToHand,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local act_from_hand_chk=e:IsHasType(EFFECT_TYPE_ACTIVATE) and e:GetHandler():IsStatus(STATUS_ACT_FROM_HAND) and 1 or 0
    let act_from_hand_chk = if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        let h = effect::get_handler(d, a.e).nil();
        if card::is_status(d, h, STATUS_ACT_FROM_HAND) { 1 } else { 0 }
    } else {
        0
    };
    // e:SetLabel(act_from_hand_chk)
    effect::set_label(d, a.e, &[act_from_hand_chk]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_HAND|LOCATION_EXTRA)
    //     and not Duel.IsExistingMatchingCard(Card.IsTrap,tp,LOCATION_GRAVE,0,1,nil)
    //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE,1-tp)>0
    //     and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spfilter),tp,0,LOCATION_GRAVE,1,nil,e,1-tp)
    //     and Duel.SelectYesNo(1-tp,aux.Stringid(id,2)) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_HAND | LOCATION_EXTRA)
        && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_trap(d, c)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(1 - a.tp), None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, 1 - a.tp))), a.tp, 0, LOCATION_GRAVE, 1, NO_EX)
        && duel::select_yes_no(d, 1 - a.tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(1-tp,s.spfilter,tp,0,LOCATION_GRAVE,1,1,nil,e,1-tp)
        let g = duel::select_matching_card(d, 1 - a.tp, Some(&|d, c| spfilter(d, c, a.e, 1 - a.tp)), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(g,0,1-tp,1-tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, 1 - a.tp, 1 - a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) and e:GetLabel()==1 then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && effect::get_label(d, a.e) == 1 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,3))
        effect::set_description(d, e1, aux::stringid(ID, 3));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetValue(function(e,re) return re:IsMonsterEffect() and re:GetHandler():IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK) end)
        effect::set_value_fn(d, e1, lua_fn(cannot_activate_value));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
/// `function(e,re) return re:IsMonsterEffect() and re:GetHandler():IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK) end`
fn cannot_activate_value(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_attribute(d, h, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK)
}
