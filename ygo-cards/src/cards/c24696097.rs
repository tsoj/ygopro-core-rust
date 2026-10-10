//! Shooting Star Dragon
use super::prelude::*;

pub const ID: u32 = 24696097;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon Procedure
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_SYNCHRO),1,1,aux.FilterSummonCode(CARD_STARDUST_DRAGON),1,1)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_SYNCHRO, sc, st, Some(tp)))),
        1,
        1,
        Some(aux::filter_summon_code(&[CARD_STARDUST_DRAGON])),
        1,
        1,
        None,
        None,
        None,
        None,
        None,
    );
    // Excavate the top 5 cards of your deck and gain extra attack per Tuner monster excavated
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.mtcon)
    effect::set_condition(d, e1, Some(condition(mtcon)));
    // e1:SetOperation(s.mtop)
    effect::set_operation(d, e1, Some(operation(mtop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate effects that would destroy cards on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish itself and negate an attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.dacon)
    effect::set_condition(d, e3, Some(condition(dacon)));
    // e3:SetTarget(s.datg)
    effect::set_target(d, e3, Some(cost_target(datg)));
    // e3:SetOperation(s.daop)
    effect::set_operation(d, e3, Some(operation(daop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon itself after it is banished by its own effect
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e4, aux::stringid(ID, 3));
    // e4:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_FIELD);
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    // e4:SetRange(LOCATION_REMOVED)
    effect::set_range(d, e4, LOCATION_REMOVED);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetTarget(s.sumtg)
    effect::set_target(d, e4, Some(cost_target(sumtg)));
    // e4:SetOperation(s.sumop)
    effect::set_operation(d, e4, Some(operation(sumop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Multiple tuners
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e5, EFFECT_MATERIAL_CHECK);
    // e5:SetValue(s.valcheck)
    effect::set_value_fn(d, e5, effect_card_value(valcheck));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}

pub const INFO: ScriptInfo = ScriptInfo {
    material: &[CARD_STARDUST_DRAGON],
    listed_names: &[CARD_STARDUST_DRAGON],
    synchro_tuner_required: Some(1),
    synchro_nt_required: Some(1),
    ..ScriptInfo::DEFAULT
};

fn mtcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_able_to_enter_bp(d) && duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) >= 5
}
fn mtop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::confirm_decktop(d, a.tp, 5);
    let g = duel::get_decktop_group(d, a.tp, 5);
    let ct = group::filter_count(d, &g, &|d, c| card::is_type(d, c, TYPE_TUNER), NO_EX);
    duel::shuffle_deck(d, a.tp);
    if ct > 1 {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        effect::set_value(d, e1, ct as i64 - 1);
        card::register_effect(d, c, e1, false);
    } else if ct == 0 {
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, c, e2, false);
    }
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if card::is_status(d, h, STATUS_BATTLE_DESTROYED) || !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    let re = a.re.nil();
    if effect::is_has_category(d, re, CATEGORY_NEGATE) {
        let te = duel::get_chain_info(d, (a.ev - 1) as u8, &[CHAININFO_TRIGGERING_EFFECT])
            .first()
            .and_then(|v| v.as_effect())
            .nil();
        if effect::is_has_type(d, te, EFFECT_TYPE_ACTIVATE) {
            return false;
        }
    }
    let Some(op) = duel::get_operation_info(d, a.ev as u32, CATEGORY_DESTROY) else {
        return false;
    };
    let Some(tg) = op.op_cards else {
        return false;
    };
    let tc = op.op_count as i64;
    let n = group::filter_count(d, &tg, &|d, c| card::is_on_field(d, c), NO_EX) as i64;
    tc + n - tg.len() as i64 > 0
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rh, re) && card::is_destructable(d, rh, None) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    if duel::negate_effect(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
        }
    }
}
fn dacon(d: &mut Duel, a: &EffArgs) -> bool {
    let att = duel::get_attacker(d).nil();
    card::is_controler(d, att, 1 - a.tp)
}
fn datg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return duel::get_attacker(d) == Some(chkc);
    }
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None)
            && card::is_can_be_effect_target(d, duel::get_attacker(d).nil(), Some(a.e))
            && !card::is_status(d, c, STATUS_CHAINING);
    }
    duel::set_target_card(d, duel::get_attacker(d).nil());
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn daop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::remove(d, c, POS_FACEUP as u8, REASON_EFFECT, None, None) != 0 {
        duel::negate_attack(d);
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 0, 0, 0);
    }
}
fn sumtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::has_flag_effect(d, c, ID, None)
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn sumop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
/// `function s.valcheck(e,c)`: the value is not used, so it returns 0 (Lua nil).
fn valcheck(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let g = card::get_material(d, c);
    if group::is_exists(d, &g, &|d, c| card::is_type(d, c, TYPE_TUNER), 2, NO_EX) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
        effect::set_code(d, e1, EFFECT_MULTIPLE_TUNERS);
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD) | RESET_PHASE | PHASE_END, 1);
        card::register_effect(d, c, e1, false);
    }
    0
}
