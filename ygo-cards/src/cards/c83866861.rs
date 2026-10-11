//! Frightfur Chimera
use super::prelude::*;

pub const ID: u32 = 83866861;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FRIGHTFUR], material_setcode: &[SET_FRIGHTFUR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_FRIGHTFUR),3)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_FRIGHTFUR as u16], Some(fc), st, Some(tp)))), 3)],
    );
    // spsummon condition
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, e1, aux::fuslimit());
    card::register_effect(d, c, e1, false);
    // actlimit
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, 0, 1);
    effect::set_value(d, e2, 1);
    effect::set_condition(d, e2, Some(condition(actcon)));
    card::register_effect(d, c, e2, false);
    // special summon
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_condition(d, e3, Some(condition(spcon)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
    // atkup
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e4, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_value_fn(d, e4, effect_card_value(atkval));
    card::register_effect(d, c, e4, false);
}
// function s.actcon(e)
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker()==e:GetHandler() or Duel.GetAttackTarget()==e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    duel::get_attacker(d) == Some(c) || duel::get_attack_target(d) == Some(c)
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return c:IsRelateToBattle() and c:IsFaceup() and bc:IsLocation(LOCATION_GRAVE) and bc:IsMonster()
    card::is_relate_to_battle(d, c) && card::is_faceup(d, c) && card::is_location(d, bc.nil(), LOCATION_GRAVE) && card::is_monster(d, bc.nil())
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetBattleTarget()
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and bc:IsCanBeSpecialSummoned(e,0,tp,false,false)
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, bc.nil(), a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetTargetCard(bc)
    duel::set_target_card(d, bc.nil());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(bc.nil())), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local atk=tc:GetAttack()
    let atk = card::get_attack(d, tc.nil());
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    if card::is_relate_to_effect(d, tc.nil(), a.e) && duel::special_summon_step(d, tc.nil(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        effect::set_code(d, e1, EFFECT_SET_ATTACK);
        // e1:SetValue(atk/2)
        effect::set_value(d, e1, lua_int(atk as f64 / 2.0));
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc.nil(), e1, true);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// function s.atkfilter(c,p)
fn atkfilter(d: &mut Duel, c: CardId, p: u8) -> bool {
    // return c:GetOwner()==p
    card::get_owner(d, c) == p
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetMatchingGroupCount(s.atkfilter,tp,LOCATION_MZONE,0,nil,1-tp)*300
    duel::get_matching_group_count(d, Some(&|d, x| atkfilter(d, x, 1 - tp)), tp, LOCATION_MZONE, 0, NO_EX) as i64 * 300
}
