//! Guard Mines
//! ボム・ガード
use super::prelude::*;

pub const ID: u32 = 88928798;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let re = a.re.nil();
    // if not (re:IsHasType(EFFECT_TYPE_ACTIVATE) or re:IsMonsterEffect()) or
    //     not Duel.IsChainNegatable(ev) then return false end
    if !(effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) || effect::is_monster_effect(d, re)) || !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let g = info.first().and_then(|v| v.as_group().cloned());
    // if not g or #g~=1 then return false end
    let Some(g) = g else {
        return false;
    };
    if g.len() != 1 {
        return false;
    }
    // local gc=g:GetFirst()
    let gc = group::get_first(d, &g).nil();
    // if not gc:IsControler(tp) or not gc:IsLocation(LOCATION_MZONE) then return false end
    if !card::is_controler(d, gc, tp) || !card::is_location(d, gc, LOCATION_MZONE) {
        return false;
    }
    // local ex,tg,tc=Duel.GetOperationInfo(ev,CATEGORY_DESTROY)
    // return ex and tg~=nil and #tg==1 and tg:GetFirst()==gc
    match duel::get_operation_info(d, a.ev as u32, CATEGORY_DESTROY) {
        Some(op) => match op.op_cards {
            Some(tg) => tg.len() == 1 && group::get_first(d, &tg) == Some(gc),
            None => false,
        },
        None => false,
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 500);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re)
    //     and Duel.Destroy(eg,REASON_EFFECT)>0 then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) && duel::destroy(d, a.eg(), REASON_EFFECT, None, None) > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Damage(1-tp,500,REASON_EFFECT)
            duel::damage(d, 1 - tp, 500, REASON_EFFECT, false, None);
        }
    }
}
