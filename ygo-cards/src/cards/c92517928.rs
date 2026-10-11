//! Galaxy-Eyes Antimatter Dragon
use super::prelude::*;

pub const ID: u32 = 92517928;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,9,2,function(c) return c:IsXyzMonster() and c:GetOverlayCount()>=3 and c:IsFaceup() end,aux.Stringid(id,0),2,s.altprocop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(9),
        2,
        Some(xyz::alter_filter(|d, sc, _tp, _mc| card::is_xyz_monster(d, sc) && card::get_overlay_count(d, sc) >= 3 && card::is_faceup(d, sc))),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(2)),
        Some(lua_fn(altprocop)),
        false,
        None,
    );
    // Any battle damage this card with material inflicts to your opponent is halved
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
    effect::set_condition(d, e1, Some(lua_fn(damcon)));
    effect::set_value_fn(d, e1, aux::change_battle_damage(1, HALF_DAMAGE as i64));
    card::register_effect(d, c, e1, false);
    // Make this card gain the ability this turn to make up to 2 attacks on monsters during each Battle Phase, then ...
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(lua_fn(bpcon)));
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), Some(std::sync::Arc::new(detach_label) as cost::DetachOp))));
    effect::set_target(d, e2, Some(cost_target(efftg)));
    effect::set_operation(d, e2, Some(operation(effop)));
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():GetOverlayCount()>0 end`
fn damcon(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_overlay_count(d, h) > 0
}
/// `function() return Duel.IsAbleToEnterBP() end`
fn bpcon(d: &mut Duel, _a: Args) -> bool {
    duel::is_able_to_enter_bp(d)
}
/// `s.altprocop(e,tp,chk)`
fn altprocop(d: &mut Duel, a: Args) -> Value {
    let tp = a.p(1);
    if a.int(2) == 0 {
        return Value::Bool(!duel::has_flag_effect(d, tp, ID, None));
    }
    match duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0) {
        Some(e) => Value::Effect(e),
        None => Value::Nil,
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)` in the cost of e2: `function(e,og) local oc=og:GetFirst() e:SetLabel(oc:IsMonster() and oc:GetRace() or 0) end`
fn detach_label(d: &mut Duel, e: EffectId, og: &CardSet) {
    let oc = group::get_first(d, og).nil();
    let lv = if card::is_monster(d, oc) { card::get_race(d, oc) as i64 } else { 0 };
    effect::set_label(d, e, &[lv]);
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_has_effect(d, h, EFFECT_EXTRA_ATTACK_MONSTER, None).is_empty();
    }
    let detached_type = effect::get_label(d, a.e);
    effect::set_label(d, a.e, &[0]);
    duel::set_target_param(d, detached_type as u32);
    if detached_type > 0 {
        duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    }
    true
}
/// `s.tgfilter(c,detached_type)`
fn tgfilter(d: &mut Duel, c: CardId, detached_type: u64) -> bool {
    card::is_race(d, c, detached_type) && card::is_able_to_grave(d, c, None)
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_has_effect(d, c, EFFECT_EXTRA_ATTACK_MONSTER, None).is_empty() {
        // This card can make up to 2 attacks on monsters this turn
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, aux::stringid(3202, 0));
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK_MONSTER);
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
        let detached_type = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u64;
        if detached_type > 0
            && duel::is_existing_matching_card(d, Some(&|d, sc| tgfilter(d, sc, detached_type)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
        {
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
            let g = duel::select_matching_card(d, a.tp, Some(&|d, sc| tgfilter(d, sc, detached_type)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            if !g.is_empty() {
                duel::break_effect(d);
                duel::send_to_grave(d, g, REASON_EFFECT, None, None);
            }
        }
    }
}
