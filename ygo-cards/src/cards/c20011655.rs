//! Beast of Talwar - The Sword Summit
use super::prelude::*;

pub const ID: u32 = 20011655;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you control no monsters, you can Special Summon this card (from your hand)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    card::register_effect(d, c, e1, false);
    // The first time this card would be destroyed by an opponent's card effect each turn, it is not destroyed
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_COUNT);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_value_fn(d, e2, lua_fn(e2val));
    card::register_effect(d, c, e2, false);
    // Neither player can target other monsters with Equip Spell Cards or effects
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target(d, e3, Some(effect_card_fn(e3target)));
    effect::set_value_fn(d, e3, lua_fn(tgval));
    card::register_effect(d, c, e3, false);
}
/// `function(e,c)` with `c == nil` when the core only checks the effect
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    if a.card(1).is_none() {
        return true;
    }
    let tp = effect::get_handler_player(d, e);
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `function(e,re,r,rp)`
fn e2val(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let r = a.int(2);
    let rp = a.int(3);
    (r & REASON_EFFECT as i64) > 0 && rp == 1 - effect::get_handler_player(d, e) as i64
}
/// `function(e,c) return c~=e:GetHandler() and c:IsMonster() and (c:IsFaceup() or not c:IsLocation(LOCATION_REMOVED)) end`
fn e3target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    Some(c) != effect::get_handler(d, e) && card::is_monster(d, c) && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_REMOVED))
}
/// `function(e,re,rp)`
fn tgval(d: &mut Duel, a: Args) -> bool {
    let (e, re) = (a.e(0), a.e(1));
    if !(effect::is_spell_effect(d, re) && effect::is_active_type(d, re, TYPE_EQUIP)) {
        return false;
    }
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_TYPE, CHAININFO_TRIGGERING_EFFECT]);
    let typ = info[0].int();
    let eff = info[1].as_effect();
    let _ = e;
    if eff == Some(re) {
        (typ & (TYPE_SPELL | TYPE_EQUIP) as i64) == (TYPE_SPELL | TYPE_EQUIP) as i64
    } else {
        let h = effect::get_handler(d, re).nil();
        card::is_equip_spell(d, h)
    }
}
