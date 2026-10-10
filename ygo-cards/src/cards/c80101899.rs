//! Trap Trick
use super::prelude::*;

pub const ID: u32 = 80101899;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Set 1 Normal Trap Card from the Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE | CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn rmfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsNormalTrap() and not c:IsCode(id) and c:IsAbleToRemove()
    //     and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,c,c:GetCode())
    if !(card::is_normal_trap(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_remove(d, c, None, None, None)) {
        return false;
    }
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, cc| setfilter(d, cc, code)), tp, LOCATION_DECK, 0, 1, c.into())
}
fn setfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsCode(code) and c:IsSSetable()
    card::is_code(d, c, &[code]) && card::is_s_setable(d, c, false, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| rmfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, cc| rmfilter(d, cc, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let rc = group::get_first(d, &sel);
    // if rc and Duel.Remove(rc,POS_FACEUP,REASON_EFFECT)>0 then
    if let Some(rc) = rc {
        if duel::remove(d, rc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
            // local sc=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil,rc:GetCode()):GetFirst()
            let rc_code = card::get_code(d, rc)[0];
            let sel = duel::select_matching_card(d, a.tp, Some(&|d, cc| setfilter(d, cc, rc_code)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            let sc = group::get_first(d, &sel);
            // if sc and Duel.SSet(tp,sc)>0 then
            if let Some(sc) = sc {
                if duel::s_set(d, a.tp, sc, None, true) > 0 {
                    // local e1=Effect.CreateEffect(c)
                    let e1 = effect::create_effect(d, c.nil());
                    // e1:SetDescription(aux.Stringid(id,2))
                    effect::set_description(d, e1, aux::stringid(ID, 2));
                    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
                    effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
                    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                    // sc:RegisterEffect(e1)
                    card::register_effect(d, sc, e1, false);
                }
            }
        }
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c.nil());
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS | EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_operation(d, e2, Some(operation(aclimit1)));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c.nil());
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS | EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EVENT_CHAIN_NEGATED);
    effect::set_operation(d, e3, Some(operation(aclimit2)));
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c.nil());
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e4, EFFECT_CANNOT_ACTIVATE);
    effect::set_target_range(d, e4, 1, 0);
    effect::set_condition(d, e4, Some(condition(e4_cond)));
    effect::set_value_fn(d, e4, lua_fn(e4_value));
    effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e4,tp)
    duel::register_effect(d, e4, a.tp);
}
fn e4_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFlagEffect(e:GetHandlerPlayer(),id)>0
    let p = effect::get_handler_player(d, a.e);
    duel::get_flag_effect(d, p, ID) > 0
}
fn e4_value(d: &mut Duel, a: Args) -> bool {
    // function(_,te) return te:IsHasType(EFFECT_TYPE_ACTIVATE) and te:IsTrapEffect() end
    let te = a.e(1);
    effect::is_has_type(d, te, EFFECT_TYPE_ACTIVATE) && effect::is_trap_effect(d, te)
}
/// `re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsTrapEffect()`
fn re_is_activated_trap(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_trap_effect(d, re)
}
fn aclimit1(d: &mut Duel, a: &EffArgs) {
    // if not (ep==tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsTrapEffect()) then return end
    if !(a.ep == a.tp && re_is_activated_trap(d, a)) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
}
fn aclimit2(d: &mut Duel, a: &EffArgs) {
    // if not (ep==tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsTrapEffect()) then return end
    if !(a.ep == a.tp && re_is_activated_trap(d, a)) {
        return;
    }
    // Duel.ResetFlagEffect(tp,id)
    duel::reset_flag_effect(d, a.tp, ID);
}
