//! Archfiend Black Skull Dragon
use super::prelude::*;

pub const ID: u32 = 45349196;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_RED_EYES],
    material_setcode: &[SET_RED_EYES, SET_ARCHFIEND],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // fusion material
    // Fusion.AddProcMix(c,true,true,s.mfilter1,s.mfilter2)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| mfilter1(d, c, fc, st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| mfilter2(d, c, fc, st, tp))),
        ],
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // aclimit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetCondition(s.accon)
    effect::set_condition(d, e1, Some(condition(accon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_DAMAGE+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_DAMAGE + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(condition(damcon)));
    // e2:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.mfilter1(c,fc,sumtype,tp)`
fn mfilter1(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_ARCHFIEND,fc,sumtype,tp) and c:IsType(TYPE_NORMAL,fc,sumtype,tp) and c:GetLevel()==6
    card::is_set_card_ctx(d, c, &[SET_ARCHFIEND as u16], Some(fc), sumtype, Some(tp))
        && card::is_type_ctx(d, c, TYPE_NORMAL, Some(fc), sumtype, Some(tp))
        && card::get_level(d, c) == 6
}
/// `s.mfilter2(c,fc,sumtype,tp)`
fn mfilter2(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_RED_EYES,fc,sumtype,tp) and c:IsType(TYPE_NORMAL,fc,sumtype,tp)
    card::is_set_card_ctx(d, c, &[SET_RED_EYES as u16], Some(fc), sumtype, Some(tp))
        && card::is_type_ctx(d, c, TYPE_NORMAL, Some(fc), sumtype, Some(tp))
}
/// `s.accon(e)`
fn accon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker()==e:GetHandler() or Duel.GetAttackTarget()==e:GetHandler()
    let h = effect::get_handler(d, a.e);
    duel::get_attacker(d) == h || duel::get_attack_target(d) == h
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFusionSummoned() and c:GetBattledGroupCount()>0
    card::is_fusion_summoned(d, c) && card::get_battled_group_count(d, c) > 0
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RED_EYES) and c:IsType(TYPE_NORMAL) and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_RED_EYES as u16]) && card::is_type(d, c, TYPE_NORMAL) && card::is_able_to_deck(d, c)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local atk=g:GetFirst():GetBaseAttack()
    let target = group::get_first(d, &g).nil();
    let atk = card::get_base_attack(d, target);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,atk)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, atk);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 1, 0, 0);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.Damage(1-tp,tc:GetBaseAttack(),REASON_EFFECT)~=0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            let atk = card::get_base_attack(d, tc);
            if duel::damage(d, 1 - tp, atk as i64, REASON_EFFECT, false, None) != 0 {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
                duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
            }
        }
    }
}
