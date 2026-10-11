//! Corrosive Scales
use super::prelude::*;

pub const ID: u32 = 13235258;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x1045], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_INSECT)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_INSECT)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let _g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsLocation(LOCATION_SZONE) then return end
    if !card::is_location(d, c, LOCATION_SZONE) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if !card::is_relate_to_effect(d, c, a.e) || !card::is_relate_to_effect(d, tc.nil(), a.e) || !card::is_faceup(d, tc.nil()) {
        return;
    }
    let tc = tc.nil();
    // Duel.Equip(tp,c,tc)
    duel::equip(d, tp, c, tc, true, false);
    // c:CancelToGrave()
    card::cancel_to_grave(d, c, true);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetValue(s.eqlimit)
    effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetCondition(s.atkcon1)
    effect::set_condition(d, e2, Some(condition(atkcon1)));
    // e2:SetValue(s.atktg)
    effect::set_value_fn(d, e2, effect_card_fn(atktg));
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCondition(s.ctcon1)
    effect::set_condition(d, e3, Some(condition(ctcon1)));
    // e3:SetOperation(s.ctop)
    effect::set_operation(d, e3, Some(operation(ctop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_CHAINING)
    effect::set_code(d, e5, EVENT_CHAINING);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e5:SetOperation(s.regop)
    effect::set_operation(d, e5, Some(operation(regop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e6:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e6, EVENT_CHAIN_SOLVED);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetCondition(s.ctcon2)
    effect::set_condition(d, e6, Some(condition(ctcon2)));
    // e6:SetOperation(s.ctop)
    effect::set_operation(d, e6, Some(operation(ctop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e7, EFFECT_TYPE_FIELD);
    // e7:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e7, EFFECT_UPDATE_ATTACK);
    // e7:SetRange(LOCATION_SZONE)
    effect::set_range(d, e7, LOCATION_SZONE);
    // e7:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e7, 0, LOCATION_MZONE);
    // e7:SetCondition(s.atkcon2)
    effect::set_condition(d, e7, Some(condition(atkcon2)));
    // e7:SetValue(s.atkval)
    effect::set_value_fn(d, e7, effect_card_value(atkval));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // local e8=e7:Clone()
    let e8 = effect::clone(d, e7);
    // e8:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e8, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetControler()==e:GetHandlerPlayer() or e:GetHandler():GetEquipTarget()==c
    let p = effect::get_handler_player(d, e);
    if card::get_controler(d, c) == p {
        return true;
    }
    let h = effect::get_handler(d, e).nil();
    card::get_equip_target(d, h) == Some(c)
}
/// `s.atkcon1(e)`
fn atkcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    // return ec and ec:GetControler()==e:GetHandlerPlayer()
    match ec {
        Some(ec) => card::get_controler(d, ec) == effect::get_handler_player(d, a.e),
        None => false,
    }
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsRace(RACE_INSECT) and c~=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, e).nil();
    card::is_race(d, c, RACE_INSECT) && Some(c) != card::get_equip_target(d, h)
}
/// `s.ctcon1(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetEquipTarget() and eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp)
    let h = effect::get_handler(d, a.e).nil();
    if card::get_equip_target(d, h).is_none() {
        return false;
    }
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - tp), 1, NO_EX)
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // tc:AddCounter(0x1045,1)
        card::add_counter(d, tc, 0x1045, 1, false);
    }
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET)|RESET_CHAIN,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET) | RESET_CHAIN, 0, 1, 0, 0);
}
/// `s.ctcon2(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetEquipTarget() and ep~=tp and e:GetHandler():GetFlagEffect(id)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_equip_target(d, h).is_some() && a.ep != a.tp && card::get_flag_effect(d, h, ID) != 0
}
/// `s.atkcon2(e)`
fn atkcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    card::get_equip_target(d, h).is_some()
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetCounter(0x1045)*-100
    card::get_counter(d, c, 0x1045) as i64 * -100
}
