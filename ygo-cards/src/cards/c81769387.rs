//! Appliancer Propelion
//! 扇風機塊プロペライオン
use super::prelude::*;

pub const ID: u32 = 81769387;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_APPLIANCER),1)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_APPLIANCER as u16], sc, st, Some(tp)))),
        1,
        None,
        None,
        None,
        None,
    );
    // Cannot be Link Material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_LINK_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_LINK_MATERIAL);
    // e1:SetCondition(s.lkcon)
    effect::set_condition(d, e1, Some(condition(lkcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Direct attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // 0 ATK while co-linked
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e3, EVENT_PRE_DAMAGE_CALCULATE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // 0 ATK while not co-linked
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCondition(s.atkcon2)
    effect::set_condition(d, e4, Some(condition(atkcon2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.lkcon(e)`
fn lkcon(d: &mut Duel, ea: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, ea.e).nil();
    // return c:IsStatus(STATUS_SPSUMMON_TURN) and c:IsLinkSummoned()
    card::is_status(d, c, STATUS_SPSUMMON_TURN) && card::is_link_summoned(d, c)
}
/// The attacker and the attack target, in the order the Lua gives them: `a` is the attacker
/// (or the target when the attacker is the opponent's), `b` the other one. `None` if there is no target.
fn battle_pair(d: &mut Duel, tp: u8) -> Option<(Option<CardId>, Option<CardId>)> {
    // local a=Duel.GetAttacker()
    let a0 = duel::get_attacker(d);
    // local b=Duel.GetAttackTarget()
    // if not b then return false end
    let b0 = duel::get_attack_target(d)?;
    // if a:IsControler(1-tp) then a,b=b,a end
    if card::is_controler(d, a0.nil(), 1 - tp) {
        Some((Some(b0), a0))
    } else {
        Some((a0, Some(b0)))
    }
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, ea: &EffArgs) -> bool {
    let tp = ea.tp;
    let Some((a, b)) = battle_pair(d, tp) else {
        return false;
    };
    // if e:GetHandler():GetMutualLinkedGroupCount()>0
    //     and a:IsControler(tp) and b:IsControler(1-tp)
    //     and Duel.IsPhase(PHASE_DAMAGE_CAL) then
    //     return true else return false end
    let h = effect::get_handler(d, ea.e).nil();
    card::get_mutual_linked_group_count(d, h) > 0
        && card::is_controler(d, a.nil(), tp)
        && card::is_controler(d, b.nil(), 1 - tp)
        && duel::is_phase(d, PHASE_DAMAGE_CAL)
}
/// `s.atkcon2(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon2(d: &mut Duel, ea: &EffArgs) -> bool {
    let tp = ea.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, ea.e);
    let Some((a, b)) = battle_pair(d, tp) else {
        return false;
    };
    // if c:GetMutualLinkedGroupCount()==0 and a==c and b:IsControler(1-tp)
    //     and Duel.IsPhase(PHASE_DAMAGE_CAL) then return true else return false end
    card::get_mutual_linked_group_count(d, c.nil()) == 0
        && a == c
        && card::is_controler(d, b.nil(), 1 - tp)
        && duel::is_phase(d, PHASE_DAMAGE_CAL)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, ea: &EffArgs) {
    let tp = ea.tp;
    // local tc=Duel.GetAttacker()
    // local bc=Duel.GetAttackTarget()
    let tc0 = duel::get_attacker(d);
    let bc0 = duel::get_attack_target(d);
    // if bc:IsControler(1-tp) then bc,tc=tc,bc end
    let tc = if card::is_controler(d, bc0.nil(), 1 - tp) { bc0 } else { tc0 };
    // if tc and tc:IsRelateToBattle() and tc:IsFaceup() and tc:IsControler(1-tp) then
    if let Some(tc) = tc {
        if card::is_relate_to_battle(d, tc) && card::is_faceup(d, tc) && card::is_controler(d, tc, 1 - tp) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, ea.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetReset(RESET_PHASE|PHASE_DAMAGE_CAL)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL, 1);
            // e1:SetValue(0)
            effect::set_value(d, e1, 0);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
