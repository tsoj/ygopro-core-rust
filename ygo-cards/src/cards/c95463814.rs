//! Phantasm Emperor Trilojig
use super::prelude::*;

pub const ID: u32 = 95463814;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,false,false,aux.FilterBoolFunction(Card.IsLevel,10),3)
    fusion::add_proc_mix_n(d, c, false, false, vec![(fusion::Mat::F(fusion::fus_mat(|d, x, _fc, _st, _tp| card::is_level(d, x, &[10]))), 3)]);
    // Damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.damcon)
    effect::set_condition(d, e1, Some(condition(damcon)));
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp,hc)`
fn filter(d: &mut Duel, c: CardId, tp: u8, hc: CardId) -> bool {
    // return c:IsControler(tp) and (c:GetPreviousLocation()==LOCATION_GRAVE or c==hc)
    card::is_controler(d, c, tp) && (card::get_previous_location(d, c) == LOCATION_GRAVE || c == hc)
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.filter,1,nil,tp,e:GetHandler())
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    group::is_exists(d, &eg, &|d, c| filter(d, c, a.tp, h), 1, NO_EX)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:HasNonZeroAttack() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::has_non_zero_attack(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.HasNonZeroAttack,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::has_non_zero_attack(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,Card.HasNonZeroAttack,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::has_non_zero_attack(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,#g,1-tp,g:GetFirst():GetBaseAttack()/2)
    let n = g.len() as u8;
    let tc = group::get_first(d, &g).nil();
    let atk = card::get_base_attack(d, tc);
    let param = (atk as f64 / 2.0).round() as i32;
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, n, 1 - a.tp, param);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc then
    if let Some(tc) = tc {
        // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
        let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
        // Duel.Damage(p,tc:GetBaseAttack()//2,REASON_EFFECT)
        let base = card::get_base_attack(d, tc) as i64;
        duel::damage(d, p, base.div_euclid(2), REASON_EFFECT, false, None);
    }
}
