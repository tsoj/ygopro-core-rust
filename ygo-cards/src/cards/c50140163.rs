//! Allure Queen LV7
use super::prelude::*;

pub const ID: u32 = 50140163;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[23756165],
    lv: Some((7, SET_ALLURE_QUEEN)),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Register that it was Special Summoned by the effect of "Allure Queen LV5"
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e0, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e0, Some(condition(regcon)));
    effect::set_operation(d, e0, Some(operation(e0_op)));
    card::register_effect(d, c, e0, false);
    // Equip 1 monster your opponent controls to this card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    effect::set_condition(d, e1, Some(condition(eqconignition)));
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    aux::add_er_equip_limit(
        d,
        c,
        Some(condition(eqconignition)),
        lua_fn(|d, a| eqval(d, a.c(0), a.p(2))),
        lua_fn(|d, a| equipop(d, a.c(0), a.e(1), a.p(2), a.c(3))),
        e1,
        None,
        None,
        None,
    );
    // Quick Effect version for when the effect of "Golden Allure Queen" is applied
    let e2 = effect::clone(d, e1);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e2, TIMING_STANDBY_PHASE, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_condition(d, e2, Some(condition(eqconquick)));
    card::register_effect(d, c, e2, false);
    aux::add_er_equip_limit(
        d,
        c,
        Some(condition(eqconquick)),
        lua_fn(|d, a| eqval(d, a.c(0), a.p(2))),
        lua_fn(|d, a| equipop(d, a.c(0), a.e(1), a.p(2), a.c(3))),
        e2,
        None,
        None,
        None,
    );
}
/// `function(e) e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~RESET_TEMP_REMOVE,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1)) end`
fn e0_op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TEMP_REMOVE), EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // local trig_loc,trig_code1,trig_code2=Duel.GetChainInfo(0,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_CODE,CHAININFO_TRIGGERING_CODE2)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_CODE, CHAININFO_TRIGGERING_CODE2]);
    let trig_loc = info[0].int() as u32;
    // if not Duel.IsChainSolving() or (rc:IsRelateToEffect(re) and rc:IsFaceup() and rc:IsLocation(trig_loc)) then
    if !duel::is_chain_solving(d) || (card::is_relate_to_effect(d, rc, re) && card::is_faceup(d, rc) && card::is_location(d, rc, trig_loc)) {
        // return rc:IsCode(23756165)
        return card::is_code(d, rc, &[23756165]);
    }
    // return trig_code1==23756165 or trig_code2==23756165
    info[1].int() == 23756165 || info[2].int() == 23756165
}
/// `s.eqconignition(e,tp,eg,ep,ev,re,r,rp)`
fn eqconignition(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:HasFlagEffect(id) and not c:GetEquipGroup():IsExists(Card.HasFlagEffect,1,nil,id)
    //     and not (c:IsOriginalSetCard(SET_ALLURE_QUEEN) and Duel.IsPlayerAffectedByEffect(tp,EFFECT_GOLDEN_ALLURE_QUEEN))
    card::has_flag_effect(d, c, ID, None)
        && !has_equip_with_flag(d, c)
        && !(card::is_original_set_card(d, c, &[SET_ALLURE_QUEEN as u16]) && duel::is_player_affected_by_effect(d, a.tp, EFFECT_GOLDEN_ALLURE_QUEEN).is_some())
}
/// `s.eqconquick(e,tp,eg,ep,ev,re,r,rp)`
fn eqconquick(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:HasFlagEffect(id) and not c:GetEquipGroup():IsExists(Card.HasFlagEffect,1,nil,id)
    //     and c:IsOriginalSetCard(SET_ALLURE_QUEEN) and Duel.IsPlayerAffectedByEffect(e:GetHandlerPlayer(),EFFECT_GOLDEN_ALLURE_QUEEN)
    let hp = effect::get_handler_player(d, a.e);
    card::has_flag_effect(d, c, ID, None)
        && !has_equip_with_flag(d, c)
        && card::is_original_set_card(d, c, &[SET_ALLURE_QUEEN as u16])
        && duel::is_player_affected_by_effect(d, hp, EFFECT_GOLDEN_ALLURE_QUEEN).is_some()
}
/// `c:GetEquipGroup():IsExists(Card.HasFlagEffect,1,nil,id)`
fn has_equip_with_flag(d: &mut Duel, c: CardId) -> bool {
    let eg = card::get_equip_group(d, c);
    group::is_exists(d, &eg, &|d, x| card::has_flag_effect(d, x, ID, None), 1, NO_EX)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsAbleToChangeControler() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_able_to_change_controler(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(Card.IsAbleToChangeControler,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&card::is_able_to_change_controler), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToChangeControler,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&card::is_able_to_change_controler), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, a.tp, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsFaceup() and Duel.GetLocationCount(tp,LOCATION_SZONE)>0) then
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0) {
        // Duel.SendtoGrave(c,REASON_RULE,PLAYER_NONE,PLAYER_NONE)
        duel::send_to_grave(d, CardSet::from_card(c), REASON_RULE, Some(PLAYER_NONE as u8), Some(PLAYER_NONE as u8));
    } else {
        // s.equipop(c,e,tp,tc)
        equipop(d, c, a.e, a.tp, tc);
    }
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // if not c:EquipByEffectAndLimitRegister(e,tp,tc,id) then return end
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, Some(ID), None) {
        return;
    }
    // If this card would be destroyed by battle, the equipped monster is destroyed instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_OWNER_RELATE, 0);
    effect::set_code(d, e1, EFFECT_DESTROY_SUBSTITUTE);
    // e1:SetValue(function(e,re,r,rp) return r&REASON_BATTLE>0 end)
    effect::set_value_fn(d, e1, lua_fn(|_d, a: Args| (a.int(2) as u32 & REASON_BATTLE) > 0));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
}
/// `s.eqval(ec,c,tp)`
fn eqval(d: &mut Duel, ec: CardId, tp: u8) -> bool {
    // return ec:IsControler(1-tp)
    card::is_controler(d, ec, 1 - tp)
}
