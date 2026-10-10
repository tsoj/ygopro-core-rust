//! Trap Tracks
use super::prelude::*;

pub const ID: u32 = 70825459;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 monster you control, and if you do, Set 1 Normal Trap directly from your Deck, except "Trap Tracks"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SET);
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
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNormalTrap() and not c:IsCode(id) and c:IsSSetable()
    card::is_normal_trap(d, c) && !card::is_code(d, c, &[ID]) && card::is_s_setable(d, c, false, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp);
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:GetHandler():IsLocation(LOCATION_HAND) then ft=ft-1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_location(d, h, LOCATION_HAND) {
        ft -= 1;
    }
    // if chk==0 then return ft>0 and Duel.IsExistingTarget(nil,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return ft > 0
            && duel::is_existing_target(d, None, a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, a.tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::get_first_target(d).first().copied().nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
        let sg = duel::select_matching_card(d, a.tp, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sg);
        // if sc and Duel.SSet(tp,sc)>0 then
        if let Some(sc) = sc {
            if duel::s_set(d, a.tp, CardSet::from_card(sc), None, true) > 0 {
                // It can be activated this turn
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetDescription(aux.Stringid(id,2))
                effect::set_description(d, e1, aux::stringid(ID, 2));
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
                effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
                // e1:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
                effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // sc:RegisterEffect(e1)
                card::register_effect(d, sc, e1, false);
            }
        }
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // You can only activate 1 Trap Card for the rest of this turn after this card resolves
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsTrapEffect() end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let re = a.re.nil();
        a.ep == a.tp && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_trap_effect(d, re)
    })));
    // e2:SetOperation(function(e,tp) Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1) end)
    effect::set_operation(d, e2, Some(operation(|d, a| {
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    })));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_CHAIN_NEGATED)
    effect::set_code(d, e3, EVENT_CHAIN_NEGATED);
    // e3:SetOperation(function(e,tp) Duel.ResetFlagEffect(tp,id) end)
    effect::set_operation(d, e3, Some(operation(|d, a| {
        duel::reset_flag_effect(d, a.tp, ID);
    })));
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e4:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e4, EFFECT_CANNOT_ACTIVATE);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // e4:SetCondition(function(e) return Duel.HasFlagEffect(e:GetHandlerPlayer(),id) end)
    effect::set_condition(d, e4, Some(condition(|d, a| {
        let p = effect::get_handler_player(d, a.e);
        duel::has_flag_effect(d, p, ID, None)
    })));
    // e4:SetValue(function(e,te) return te:IsHasType(EFFECT_TYPE_ACTIVATE) and te:IsTrapEffect() end)
    effect::set_value_fn(d, e4, lua_fn(value4));
    // e4:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e4,tp)
    duel::register_effect(d, e4, a.tp);
}
/// `s.value(e,te)` of the EFFECT_CANNOT_ACTIVATE effect
fn value4(d: &mut Duel, a: Args) -> bool {
    let te = a.e(1);
    effect::is_has_type(d, te, EFFECT_TYPE_ACTIVATE) && effect::is_trap_effect(d, te)
}
