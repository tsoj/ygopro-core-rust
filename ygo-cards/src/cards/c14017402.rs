//! Dragon Knight Draco-Equiste
use super::prelude::*;

pub const ID: u32 = 14017402;
pub const INFO: ScriptInfo = ScriptInfo { miracle_synchro_fusion: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,s.ffilter,aux.FilterBoolFunctionEx(Card.IsRace,RACE_WARRIOR))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, fc, st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_WARRIOR, Some(fc), st, Some(tp)))),
        ],
    );
    // special summon condition
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(s.splimit)
    effect::set_value_fn(d, e0, lua_fn(splimit));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // copy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // reflect damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_REFLECT_DAMAGE)
    effect::set_code(d, e2, EFFECT_REFLECT_DAMAGE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetValue(s.refcon)
    effect::set_value_fn(d, e2, lua_fn(refcon));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.splimit(e,se,sp,st)
fn splimit(d: &mut Duel, a: Args) -> bool {
    // if e:GetHandler():IsLocation(LOCATION_EXTRA) then
    let c = effect::get_handler(d, a.e(0)).nil();
    if card::is_location(d, c, LOCATION_EXTRA) {
        // return (st&SUMMON_TYPE_FUSION)==SUMMON_TYPE_FUSION
        return (a.int(3) as u32 & SUMMON_TYPE_FUSION) == SUMMON_TYPE_FUSION;
    }
    // return true
    true
}
// function s.refcon(e,re,val,r,rp,rc)
fn refcon(d: &mut Duel, a: Args) -> bool {
    // return (r&REASON_EFFECT)~=0 and rp==1-e:GetHandler():GetControler() and e:GetHandler():IsAttackPos()
    let h = effect::get_handler(d, a.e(0)).nil();
    let r = a.int(3) as u32;
    let rp = a.int(4);
    (r & REASON_EFFECT) != 0 && rp == 1 - card::get_controler(d, h) as i64 && card::is_attack_pos(d, h)
}
// function s.ffilter(c,fc,sumtype,tp)
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON,fc,sumtype,tp) and c:IsType(TYPE_SYNCHRO,fc,sumtype,tp)
    card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), sumtype, Some(tp)) && card::is_type_ctx(d, c, TYPE_SYNCHRO, Some(fc), sumtype, Some(tp))
}
// function s.cpfilter(c)
fn cpfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsType(TYPE_SYNCHRO) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::is_race(d, c, RACE_DRAGON) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.cpfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && cpfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.cpfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&cpfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.cpfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&cpfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and c:IsFaceup() and tc and tc:IsRelateToEffect(e) then
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c)) {
        return;
    }
    let Some(tc) = tc else { return };
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // if Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)==1 then
    if duel::remove(d, CardSet::from_card(tc), POS_FACEUP as u8, REASON_EFFECT, None, None) == 1 {
        // local code=tc:GetOriginalCode()
        let code = card::get_original_code(d, tc);
        // local reset_flag=RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_END
        let reset_flag = RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_END;
        // c:CopyEffect(code,reset_flag,1)
        card::copy_effect(d, c, code, reset_flag, 1);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetReset(reset_flag)
        effect::set_reset(d, e1, reset_flag, 1);
        // e1:SetCode(EFFECT_CHANGE_CODE)
        effect::set_code(d, e1, EFFECT_CHANGE_CODE);
        // e1:SetValue(code)
        effect::set_value(d, e1, code as i64);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
