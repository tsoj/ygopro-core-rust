//! Number iC1000: Numerounius Numerounia
use super::prelude::*;

pub const ID: u32 = 15862758;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[89477759], xyz_number: Some(1000), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(13), 5, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // atk/def
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(lua_fn(adcon)));
    effect::set_value(d, e1, 100000);
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e2, false);
    // atk restriction
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_MUST_ATTACK);
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_condition(d, e3, Some(lua_fn(atcon)));
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EFFECT_MUST_ATTACK_MONSTER);
    effect::set_value_fn(d, e4, effect_card_fn(attg));
    card::register_effect(d, c, e4, false);
    // win
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_DELAY + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_code(d, e5, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_condition(d, e5, Some(condition(wincon)));
    effect::set_operation(d, e5, Some(operation(winop)));
    card::register_effect(d, c, e5, false);
    // negate attack
    let e7 = effect::create_effect(d, c);
    effect::set_description(d, e7, aux::stringid(ID, 0));
    effect::set_category(d, e7, CATEGORY_RECOVER);
    effect::set_type(d, e7, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e7, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e7, LOCATION_MZONE);
    effect::set_condition(d, e7, Some(condition(nacon)));
    effect::set_cost(d, e7, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e7, Some(cost_target(natg)));
    effect::set_operation(d, e7, Some(operation(naop)));
    card::register_effect(d, c, e7, false);
}
/// `s.adcon(e)`
fn adcon(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsSummonType(SUMMON_TYPE_SPECIAL+0x53b) and not Duel.IsTurnPlayer(e:GetHandlerPlayer())
    let e = a.e(0);
    let h = effect::get_handler(d, e).nil();
    card::is_summon_type(d, h, &[SUMMON_TYPE_SPECIAL + 0x53b]) && !duel::is_turn_player(d, effect::get_handler_player(d, e))
}
/// `s.atcon(e)`
fn atcon(d: &mut Duel, a: Args) -> bool {
    atcon_of(d, a.e(0))
}
fn atcon_of(d: &mut Duel, e: EffectId) -> bool {
    // return e:GetHandler():IsSummonType(SUMMON_TYPE_SPECIAL+0x53b) and Duel.GetTurnCount()>e:GetHandler():GetTurnID()
    let h = effect::get_handler(d, e).nil();
    card::is_summon_type(d, h, &[SUMMON_TYPE_SPECIAL + 0x53b]) && duel::get_turn_count(d, None) > card::get_turn_id(d, h) as i32
}
/// `s.attg(e,c)`
fn attg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetHandler()
    c == effect::get_handler(d, e).nil()
}
/// `s.wincon(e,tp,eg,ep,ev,re,r,rp)`
fn wincon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsTurnPlayer(tp) and e:GetHandler():GetBattledGroupCount()==0 and s.atcon(e)
    !duel::is_turn_player(d, a.tp)
        && card::get_battled_group_count(d, effect::get_handler(d, a.e).nil()) == 0
        && atcon_of(d, a.e)
}
/// `s.winop(e,tp,eg,ep,ev,re,r,rp)`
fn winop(d: &mut Duel, a: &EffArgs) {
    // Duel.Win(tp,WIN_REASON_NUMBER_iC1000)
    duel::win(d, a.tp, WIN_REASON_NUMBER_iC1000);
}
/// `s.nacon(e,tp,eg,ep,ev,re,r,rp)`
fn nacon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.GetAttacker():IsControler(tp)
    !card::is_controler(d, duel::get_attacker(d).nil(), a.tp)
}
/// `s.natg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn natg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetAttacker():IsOnField() end
    if a.chk == Some(0) {
        return card::is_on_field(d, duel::get_attacker(d).nil());
    }
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,Duel.GetAttacker():GetAttack())
    let atk = card::get_attack(d, duel::get_attacker(d).nil());
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, atk);
    true
}
/// `s.naop(e,tp,eg,ep,ev,re,r,rp)`
fn naop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateAttack() then
    if duel::negate_attack(d) {
        // Duel.Recover(tp,Duel.GetAttacker():GetAttack(),REASON_EFFECT)
        let atk = card::get_attack(d, duel::get_attacker(d).nil());
        duel::recover(d, a.tp, atk as i64, REASON_EFFECT, false, None);
    }
}
