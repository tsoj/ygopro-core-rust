//! Zombie World
use super::prelude::*;

pub const ID: u32 = 4064256;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // race
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE);
    effect::set_code(d, e2, EFFECT_CHANGE_RACE);
    effect::set_value(d, e2, RACE_ZOMBIE as i64);
    effect::set_target(d, e2, Some(effect_card_fn(tg)));
    card::register_effect(d, c, e2, false);
    // summon limit
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_code(d, e3, EFFECT_CANNOT_SUMMON);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e3, 1, 1);
    effect::set_target(d, e3, Some(lua_fn(sumlimit)));
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_CANNOT_MSET)
    effect::set_code(d, e4, EFFECT_CANNOT_MSET);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // id chk
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    effect::set_code(d, e5, ID);
    effect::set_range(d, e5, LOCATION_FZONE);
    effect::set_property(d, e5, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e5, 1, 1);
    effect::set_value_fn(d, e5, lua_fn(val));
    card::register_effect(d, c, e5, false);
}
/// `s.tg(e,c)`
fn tg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // if c:GetFlagEffect(1)==0 then
    if card::get_flag_effect(d, c, 1) == 0 {
        // c:RegisterFlagEffect(1,0,0,0)
        card::register_flag_effect(d, c, 1, 0, 0, 0, 0, 0);
        // local eff
        // if c:IsLocation(LOCATION_MZONE) then eff={Duel.GetPlayerEffect(c:GetControler(),EFFECT_NECRO_VALLEY)}
        let eff = if card::is_location(d, c, LOCATION_MZONE) {
            let tp = card::get_controler(d, c);
            duel::get_player_effect(d, tp, EFFECT_NECRO_VALLEY)
        } else {
            // else eff={c:GetCardEffect(EFFECT_NECRO_VALLEY)} end
            card::get_card_effect(d, c, EFFECT_NECRO_VALLEY)
        };
        // c:ResetFlagEffect(1)
        card::reset_flag_effect(d, c, 1);
        // for _,te in ipairs(eff) do
        for te in eff {
            // local op=te:GetOperation()
            // if not op or op(e,c) then return false end
            match effect::get_operation(d, te) {
                None => return false,
                Some(op) => {
                    if op.call(d, &[Value::Effect(e), Value::Card(c)]).truthy() {
                        return false;
                    }
                }
            }
        }
    }
    // return true
    true
}
/// `s.val(e,c,re,chk)`
fn val(_d: &mut Duel, a: Args) -> Value {
    // if chk==0 then return true end
    if matches!(a.v(3), Value::Int(0)) {
        return Value::Bool(true);
    }
    // return RACE_ZOMBIE
    Value::Int(RACE_ZOMBIE as i64)
}
/// `s.sumlimit(e,c,tp,sumtp)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    // return sumtp&SUMMON_TYPE_TRIBUTE==SUMMON_TYPE_TRIBUTE and not c:IsRace(RACE_ZOMBIE)
    let sumtp = a.int(3) as u32;
    (sumtp & SUMMON_TYPE_TRIBUTE) == SUMMON_TYPE_TRIBUTE && !card::is_race(d, a.c(1), RACE_ZOMBIE)
}
