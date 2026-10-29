//! Gorgon of Zilofthonia
use super::prelude::*;

pub const ID: u32 = 12067160;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ Effect Monsters
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 2, None, None, None, None);
    // You cannot Summon/Set monsters to a zone(s) this card points to
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    effect::set_property(d, e0, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e0, EFFECT_FORCE_MZONE);
    effect::set_range(d, e0, LOCATION_MZONE);
    effect::set_target_range(d, e0, 1, 0);
    effect::set_value_fn(d, e0, lua_fn(force_mzone_value));
    card::register_effect(d, c, e0, false);
    // Monsters this card points to cannot attack, also their activated effects are negated
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e1, Some(effect_card_fn(|d, e, c| {
        let h = effect::get_handler(d, e).nil();
        let g = card::get_linked_group(d, h);
        group::is_contains(&g, c)
    })));
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(discon)));
    effect::set_operation(d, e2, Some(operation(|d, a| {
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        duel::negate_effect(d, a.ev as u8);
    })));
    card::register_effect(d, c, e2, false);
    // Gains ATK equal to the total original ATK of all monsters this card points to
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_value_fn(d, e3, effect_value(atkval));
    card::register_effect(d, c, e3, false);
    // Cannot be destroyed by battle or monster effects while it points to no monsters
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_value(d, e4, 1);
    effect::set_condition(d, e4, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::get_linked_group_count(d, h) == 0
    })));
    card::register_effect(d, c, e4, false);
    let e5 = effect::clone(d, e4);
    effect::set_code(d, e5, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value_fn(d, e5, lua_fn(e5_value));
    card::register_effect(d, c, e5, false);
}
// function(e,fp,rp,r) return r==LOCATION_REASON_CONTROL and (ZONES_MMZ|ZONES_EMZ)|~(ZONES_MMZ|ZONES_EMZ) or ~e:GetHandler():GetLinkedZone() end
fn force_mzone_value(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let r = a.int(3);
    if r == LOCATION_REASON_CONTROL as i64 {
        // (ZONES_MMZ|ZONES_EMZ)|~(ZONES_MMZ|ZONES_EMZ) has every bit set
        return -1;
    }
    let h = effect::get_handler(d, e).nil();
    !(card::get_linked_zone(d, h, None) as i64)
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    effect::is_monster_effect(d, re)
        && card::is_relate_to_effect(d, rc, re)
        && {
            let lg = card::get_linked_group(d, c);
            group::is_contains(&lg, rc)
        }
        && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
}
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    let h = effect::get_handler(d, e).nil();
    let lg = card::get_linked_group(d, h);
    let fg = group::filter(d, &lg, &|d, c| card::is_faceup(d, c), NO_EX);
    group::get_sum(d, &fg, &|d, c| card::get_base_attack(d, c) as i64)
}
// function(e,re) return re:IsMonsterEffect() end
fn e5_value(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    effect::is_monster_effect(d, re)
}
