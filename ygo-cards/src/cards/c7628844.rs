//! Number C32: Shark Drake LeVeiss
//! ＣＮｏ．３２ 海咬龍シャーク・ドレイク・リバイス
use super::prelude::*;

pub const ID: u32 = 7628844;
pub const INFO: ScriptInfo = ScriptInfo {
    xyz_number: Some(32),
    listed_series: &[SET_SHARK_DRAKE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon Procedure: 4 Level 5 monsters
    // Xyz.AddProcedure(c,nil,5,4,s.altmatfilter,aux.Stringid(id,0),4,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(5),
        4,
        Some(xyz::alter_filter(altmatfilter)),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(4)),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // Negate the effects of an Effect Monster your opponent controls and change its ATK/DEF to 0
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(function() return not (Duel.IsPhase(PHASE_DAMAGE) and Duel.IsDamageCalculated()) end)
    effect::set_condition(d, e1, Some(condition(|d, _a| !(duel::is_phase(d, PHASE_DAMAGE) && duel::is_damage_calculated(d)))));
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // This card can make a second attack during each Battle Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e2, EFFECT_EXTRA_ATTACK);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Inflicts piercing battle damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e3, EFFECT_PIERCE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.altmatfilter(c,tp,xyzc)`
fn altmatfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsRank(4) and c:IsSetCard(SET_SHARK_DRAKE,xyzc,SUMMON_TYPE_XYZ,tp) and c:IsType(TYPE_XYZ,xyzc,SUMMON_TYPE_XYZ,tp) and c:IsFaceup()
    card::is_rank(d, c, &[4])
        && card::is_set_card_ctx(d, c, &[SET_SHARK_DRAKE as u16], Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_faceup(d, c)
}
/// `s.altproccostfilter(c)`
fn altproccostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsDiscardable()
    card::is_spell(d, c) && card::is_discardable(d, c, None, None)
}
/// `s.xyzop(e,tp,chk,mc)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.altproccostfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.int(2) == 0 {
        return duel::is_existing_matching_card(d, Some(&altproccostfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local sc=Duel.GetMatchingGroup(s.altproccostfilter,tp,LOCATION_HAND,0,nil):SelectUnselect(Group.CreateGroup(),tp,false,Xyz.ProcCancellable)
    let g = duel::get_matching_group(d, Some(&altproccostfilter), tp, LOCATION_HAND, 0, NO_EX);
    let cancellable = xyz::proc_cancellable(d);
    let sc = group::select_unselect(d, &g, Some(&CardSet::new()), tp, false, cancellable, None, None);
    // return sc and Duel.SendtoGrave(sc,REASON_DISCARD|REASON_COST)>0
    match sc {
        Some(sc) => duel::send_to_grave(d, sc, REASON_DISCARD | REASON_COST, None, None) > 0,
        None => false,
    }
}
/// `s.disfilter(c)`
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNegatableMonster() and c:IsType(TYPE_EFFECT)
    card::is_negatable_monster(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.disfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && disfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.disfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&disfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    // local tc=Duel.SelectTarget(tp,s.disfilter,tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
    let g = duel::select_target(d, a.tp, Some(&disfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(tc)), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,tc,1,tp,-tc:GetAttack())
    let atk = -card::get_attack(d, tc);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(tc)), 1, a.tp, atk);
    // Duel.SetOperationInfo(0,CATEGORY_DEFCHANGE,tc,1,tp,-tc:GetDefense())
    let def = -card::get_defense(d, tc);
    duel::set_operation_info(d, 0, CATEGORY_DEFCHANGE, Some(CardSet::from_card(tc)), 1, a.tp, def);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and tc:IsCanBeDisabledByEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::is_can_be_disabled_by_effect(d, tc, a.e, true) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // Negate its effects
        // tc:NegateEffects(c)
        card::negate_effects(d, tc, c, None, false, None);
        // Duel.AdjustInstantly(tc)
        duel::adjust_instantly(d, Some(tc));
        // if not tc:IsDisabled() then return end
        if !card::is_disabled(d, tc) {
            return;
        }
        // Change its ATK/DEF to 0
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
