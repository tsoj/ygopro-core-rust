//! Time Wizard of Tomorrow
use super::prelude::*;

pub const ID: u32 = 26273196;

pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,71625222,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(71625222),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, Some(fc), st, Some(tp)))),
        ],
    );
    // Toss a coin and destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_COIN+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_COIN + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsFusionSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, c)
}
/// s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,PLAYER_ALL,g:Filter(Card.IsFaceup,nil):GetSum(Card.GetBaseAttack))
    let faceup = group::filter(d, &g, &|d, c| card::is_faceup(d, c), NO_EX);
    let sum = group::get_sum(d, &faceup, &|d, c| card::get_base_attack(d, c) as i64);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, sum as i32);
    true
}
/// s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local damage_oppo=Duel.CallCoin(tp)
    let damage_oppo = duel::call_coin(d, tp);
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
    // local dg=Duel.GetOperatedGroup()
    let dg = duel::get_operated_group(d);
    // local sum=dg:Filter(Card.IsPreviousPosition,nil,POS_FACEUP):GetSum(Card.GetBaseAttack)
    let prev = group::filter(d, &dg, &|d, c| card::is_previous_position(d, c, POS_FACEUP), NO_EX);
    let sum = group::get_sum(d, &prev, &|d, c| card::get_base_attack(d, c) as i64);
    // local p=damage_oppo and 1-tp or tp
    let p = if damage_oppo { 1 - tp } else { tp };
    // Duel.Damage(p,sum/2,REASON_EFFECT)
    duel::damage(d, p, lua_int(sum as f64 / 2.0), REASON_EFFECT, false, None);
}
