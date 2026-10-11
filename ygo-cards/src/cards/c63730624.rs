//! Double Tool C&D
//! ダブルツールD&C
use super::prelude::*;

pub const ID: u32 = 63730624;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[2403771], listed_series: &[SET_MORPHTRONIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,0,s.filter,s.eqlimit)
    aux::add_equip_procedure(
        d,
        c,
        Some(0),
        Some(aux::equip_filter_fn(|d, x, _e, _tp| filter(d, x))),
        Some(lua_fn(eqlimit)),
        None,
        None,
        None,
        None,
        None,
    );
    // atk
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetCondition(s.scon1)
    effect::set_condition(d, e3, Some(condition(scon1)));
    // e3:SetValue(1000)
    effect::set_value(d, e3, 1000);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // disable
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e4, EVENT_BE_BATTLE_TARGET);
    // e4:SetCondition(s.scon2)
    effect::set_condition(d, e4, Some(condition(scon2)));
    // e4:SetOperation(s.sop2)
    effect::set_operation(d, e4, Some(operation(sop2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // atk
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e5, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e5, 0, LOCATION_MZONE);
    // e5:SetCondition(s.ocon1)
    effect::set_condition(d, e5, Some(condition(ocon1)));
    // e5:SetValue(s.atlimit)
    effect::set_value_fn(d, e5, effect_card_fn(atlimit));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // destroy
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e6, CATEGORY_DESTROY);
    // e6:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e6, aux::stringid(ID, 0));
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e6:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e6, EVENT_DAMAGE_STEP_END);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetCondition(s.ocon2)
    effect::set_condition(d, e6, Some(condition(ocon2)));
    // e6:SetTarget(s.otg2)
    effect::set_target(d, e6, Some(cost_target(otg2)));
    // e6:SetOperation(s.oop2)
    effect::set_operation(d, e6, Some(operation(oop2)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    // local h = e:GetHandler()
    let h = effect::get_handler(d, a.e(0)).nil();
    let c = a.c(1);
    // return c:GetControler()==e:GetHandler():GetControler()
    //   and (c:IsCode(2403771) or (c:IsSetCard(SET_MORPHTRONIC) and c:GetLevel()>=4 and c:IsRace(RACE_MACHINE)))
    card::get_controler(d, c) == card::get_controler(d, h)
        && (card::is_code(d, c, &[2403771]) || (card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]) && card::get_level(d, c) >= 4 && card::is_race(d, c, RACE_MACHINE)))
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(2403771) or (c:IsSetCard(SET_MORPHTRONIC) and c:GetLevel()>=4 and c:IsRace(RACE_MACHINE))
    card::is_code(d, c, &[2403771]) || (card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]) && card::get_level(d, c) >= 4 && card::is_race(d, c, RACE_MACHINE))
}
/// `s.scon1(e)`
fn scon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetEquipTarget() and Duel.GetTurnPlayer()==e:GetHandler():GetControler()
    let h = effect::get_handler(d, a.e).nil();
    card::get_equip_target(d, h).is_some() && duel::get_turn_player(d) == card::get_controler(d, h)
}
/// `s.scon2(e,tp,eg,ep,ev,re,r,rp)`
fn scon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.GetAttacker()==e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, a.tp) && duel::get_attacker(d) == card::get_equip_target(d, h)
}
/// `s.sop2(e,tp,eg,ep,ev,re,r,rp)`
fn sop2(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local d=Duel.GetAttackTarget()
    let dd = duel::get_attack_target(d).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
    // d:RegisterEffect(e1)
    card::register_effect(d, dd, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DISABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
    // d:RegisterEffect(e2)
    card::register_effect(d, dd, e2, false);
}
/// `s.ocon1(e)`
fn ocon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnPlayer()~=e:GetHandler():GetControler()
    let h = effect::get_handler(d, a.e).nil();
    duel::get_turn_player(d) != card::get_controler(d, h)
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, e).nil();
    Some(c) != card::get_equip_target(d, h)
}
/// `s.ocon2(e,tp,eg,ep,ev,re,r,rp)`
fn ocon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    // local d=Duel.GetAttackTarget()
    // return Duel.IsTurnPlayer(1-tp) and d==e:GetHandler():GetEquipTarget() and a:IsRelateToBattle()
    let h = effect::get_handler(d, a.e).nil();
    let at = duel::get_attacker(d);
    let df = duel::get_attack_target(d);
    duel::is_turn_player(d, 1 - a.tp) && df == card::get_equip_target(d, h) && card::is_relate_to_battle(d, at.nil())
}
/// `s.otg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn otg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Duel.GetAttacker(),1,0,0)
    let at = duel::get_attacker(d);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, at.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.oop2(e,tp,eg,ep,ev,re,r,rp)`
fn oop2(d: &mut Duel, _a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // if a:IsRelateToBattle() then
    if card::is_relate_to_battle(d, at) {
        // Duel.Destroy(a,REASON_EFFECT)
        duel::destroy(d, at, REASON_EFFECT, None, None);
    }
}
