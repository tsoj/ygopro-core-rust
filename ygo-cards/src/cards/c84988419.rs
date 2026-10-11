//! Bloom Diva the Melodious Choir
use super::prelude::*;

pub const ID: u32 = 84988419;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MELODIOUS_MAESTRA, SET_MELODIOUS],
    material_setcode: &[SET_MELODIOUS, SET_MELODIOUS_MAESTRA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_MELODIOUS_MAESTRA),aux.FilterBoolFunctionEx(Card.IsSetCard,SET_MELODIOUS))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_MELODIOUS_MAESTRA as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_MELODIOUS as u16], Some(fc), st, Some(tp)))),
        ],
    );
    // indes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // avoid battle damage
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_AVOID_BATTLE_DAMAGE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e4, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_BATTLED)
    effect::set_code(d, e4, EVENT_BATTLED);
    // e4:SetCondition(s.condition)
    effect::set_condition(d, e4, Some(condition(cond)));
    // e4:SetTarget(s.target)
    effect::set_target(d, e4, Some(cost_target(target)));
    // e4:SetOperation(s.operation)
    effect::set_operation(d, e4, Some(operation(op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return bc and (bc:GetSummonType()&SUMMON_TYPE_SPECIAL)==SUMMON_TYPE_SPECIAL and c:GetBaseAttack()~=bc:GetBaseAttack()
    let Some(bc) = bc else {
        return false;
    };
    let st = card::get_summon_type(d, bc);
    if (st & SUMMON_TYPE_SPECIAL as u32) != SUMMON_TYPE_SPECIAL as u32 {
        return false;
    }
    let c_atk = card::get_base_attack(d, c);
    let bc_atk = card::get_base_attack(d, bc);
    c_atk != bc_atk
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local bc=e:GetHandler():GetBattleTarget()
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c).nil();
    // if chk==0 then return bc:IsRelateToBattle() end
    if a.chk == Some(0) {
        return card::is_relate_to_battle(d, bc);
    }
    // local atk=math.abs(e:GetHandler():GetBaseAttack()-bc:GetBaseAttack())
    let c_atk = card::get_base_attack(d, c);
    let bc_atk = card::get_base_attack(d, bc);
    let atk = (c_atk - bc_atk).abs();
    // Duel.SetTargetCard(bc)
    duel::set_target_card(d, bc);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,atk)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, atk);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(bc)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local bc=Duel.GetFirstTarget()
    // if not bc then return false end
    let Some(bc) = duel::first_target(d) else {
        return;
    };
    // local atk=math.abs(e:GetHandler():GetBaseAttack()-bc:GetBaseAttack())
    let c = effect::get_handler(d, a.e).nil();
    let c_atk = card::get_base_attack(d, c);
    let bc_atk = card::get_base_attack(d, bc);
    let atk = (c_atk - bc_atk).abs();
    // if bc:IsRelateToEffect(e) and bc:IsFaceup() and Duel.Damage(1-tp,atk,REASON_EFFECT)~=0 then
    if card::is_relate_to_effect(d, bc, a.e) && card::is_faceup(d, bc) && duel::damage(d, 1 - tp, atk as i64, REASON_EFFECT, false, None) != 0 {
        // Duel.Destroy(bc,REASON_EFFECT)
        duel::destroy(d, bc, REASON_EFFECT, None, None);
    }
}
