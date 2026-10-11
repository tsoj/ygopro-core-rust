//! Malefic Parallel Gear
use super::prelude::*;

pub const ID: u32 = 74509280;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MALEFIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // hand synchro
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e3, EFFECT_HAND_SYNCHRO);
    effect::set_label(d, e3, &[ID as i64]);
    effect::set_value_fn(d, e3, lua_fn(synval));
    card::register_effect(d, c, e3, false);
    let e4 = effect::create_effect(d, c);
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e4, EFFECT_SYNCHRO_MAT_RESTRICTION);
    effect::set_value_fn(d, e4, effect_card_fn(synfilter));
    card::register_effect(d, c, e4, false);
    let e1 = effect::create_effect(d, c);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SYNCHRO_MATERIAL_CUSTOM);
    effect::set_operation(d, e1, Some(lua_fn(synop)));
    card::register_effect(d, c, e1, false);
}
/// `s.synfilter(e,c)`
fn synfilter(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let p = effect::get_handler_player(d, e);
    card::is_location(d, c, LOCATION_HAND) && card::is_set_card(d, c, &[SET_MALEFIC as u16]) && card::is_controler(d, c, p)
}
/// `s.synval(e,c,sc)`
fn synval(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    // if c:IsSetCard(SET_MALEFIC) and c:IsLocation(LOCATION_HAND) then
    if card::is_set_card(d, c, &[SET_MALEFIC as u16]) && card::is_location(d, c, LOCATION_HAND) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e(0)).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_HAND_SYNCHRO+EFFECT_SYNCHRO_CHECK)
        effect::set_code(d, e1, EFFECT_HAND_SYNCHRO + EFFECT_SYNCHRO_CHECK);
        // e1:SetLabel(id)
        effect::set_label(d, e1, &[ID as i64]);
        // e1:SetTarget(s.synchktg)
        effect::set_target(d, e1, Some(lua_fn(synchktg)));
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        return true;
    }
    false
}
/// `s.chk2(c)`
fn chk2(d: &mut Duel, c: CardId) -> bool {
    if card::is_has_effect(d, c, EFFECT_HAND_SYNCHRO, None).is_empty() || !card::is_has_effect(d, c, EFFECT_HAND_SYNCHRO + EFFECT_SYNCHRO_CHECK, None).is_empty() {
        return false;
    }
    let te = card::get_card_effect(d, c, EFFECT_HAND_SYNCHRO);
    for e in te {
        if effect::get_label(d, e) == ID as i64 {
            return true;
        }
    }
    false
}
/// `s.synchktg(e,c,sg,tg,ntg,tsg,ntsg)`
fn synchktg(d: &mut Duel, a: Args) -> Value {
    let sg = a.g(2);
    if let Some(c) = a.card(1) {
        let res = true;
        let tg = a.g(3);
        let ntg = a.g(4);
        if sg.len() >= 2
            || (!group::is_exists(d, &tg, &chk2, 1, c.into())
                && !group::is_exists(d, &ntg, &chk2, 1, c.into())
                && !group::is_exists(d, &sg, &chk2, 1, c.into()))
        {
            return Value::Bool(false);
        }
        let ttg = group::filter(d, &tg, &chk2, NO_EX);
        let nttg = group::filter(d, &ntg, &chk2, NO_EX);
        let mut trg = tg.clone();
        let mut ntrg = ntg.clone();
        group::remove_card(&mut trg, &ttg);
        group::remove_card(&mut ntrg, &nttg);
        return Value::Multi(vec![Value::Bool(res), Value::Group(trg), Value::Group(ntrg)]);
    }
    Value::Bool(sg.len() < 2)
}
/// `s.synop(e,tg,ntg,sg,lv,sc,tp)`
fn synop(d: &mut Duel, a: Args) -> Value {
    let _ = d;
    let sg = a.g(3);
    Value::Multi(vec![Value::Bool(sg.len() == 2), Value::Bool(false)])
}
