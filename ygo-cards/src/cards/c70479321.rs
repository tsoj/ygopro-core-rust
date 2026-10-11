//! Performapal Drummerilla
use super::prelude::*;

pub const ID: u32 = 70479321;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // atk change effect
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // summon without using tributes
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e2, EFFECT_SUMMON_PROC);
    effect::set_condition(d, e2, Some(lua_fn(ntcon)));
    card::register_effect(d, c, e2, false);
    // change level
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_SUMMON_COST);
    effect::set_operation(d, e3, Some(operation(lvop)));
    card::register_effect(d, c, e3, false);
    // increase ATK
    let e4 = effect::clone(d, e1);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_operation(d, e4, Some(operation(atkop2)));
    card::register_effect(d, c, e4, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let at = duel::get_attacker(d);
    let dt = duel::get_attack_target(d);
    if let Some(dt) = dt {
        let at = at.nil();
        if card::get_controler(d, at) != card::get_controler(d, dt) {
            if card::is_controler(d, at, a.tp) {
                effect::set_label_object(d, a.e, Value::Card(at));
            } else {
                effect::set_label_object(d, a.e, Value::Card(dt));
            }
            return true;
        }
    }
    false
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tc = effect::get_label_object(d, a.e).as_card();
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tc;
    }
    if a.chk == Some(0) {
        let tc = tc.nil();
        return card::is_on_field(d, tc) && card::is_can_be_effect_target(d, tc, Some(a.e));
    }
    duel::set_target_card(d, tc.nil());
    true
}
/// The ATK gain of `s.atkop` / `s.atkop2`: an effect on `tc` (owned by `h`).
fn atk_boost(d: &mut Duel, h: CardId, tc: CardId) {
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, 600);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
    card::register_effect(d, tc, e1, false);
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            atk_boost(d, h, tc);
        }
    }
}
/// `s.ntcon(e,c,minc)`: EFFECT_SUMMON_PROC condition
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    let minc = a.int(2);
    let tp = card::get_controler(d, c);
    // return minc==0 and c:GetLevel()>4 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    //     and Duel.GetFieldGroupCount(c:GetControler(),LOCATION_MZONE,LOCATION_MZONE)==0
    minc == 0
        && card::get_level(d, c) > 4
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_field_group_count(d, tp, LOCATION_MZONE, LOCATION_MZONE) == 0
}
/// `s.lvcon(e)`
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_material_count(d, h) == 0
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(lvcon)));
    effect::set_value(d, e1, 4);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE&~RESET_TOFIELD)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    card::register_effect(d, c, e1, false);
}
/// `s.atkop2(e,tp,eg,ep,ev,re,r,rp)`
fn atkop2(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            atk_boost(d, h, tc);
        }
    }
}
