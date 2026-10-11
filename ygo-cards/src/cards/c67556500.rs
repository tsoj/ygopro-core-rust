//! Phonon Pulse Dragon
use super::prelude::*;

pub const ID: u32 = 67556500;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.lvcon)
    effect::set_condition(d, e1, Some(condition(lvcon)));
    // e1:SetTarget(s.lvtg)
    effect::set_target(d, e1, Some(cost_target(lvtg)));
    // e1:SetOperation(s.lvop)
    effect::set_operation(d, e1, Some(operation(lvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.lvcon(e,tp,eg,ep,ev,re,r,rp)`
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h)
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_LVRANK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_LVRANK as u64);
    // local lv=Duel.AnnounceLevel(tp,1,3,e:GetHandler():GetLevel())
    let h = effect::get_handler(d, a.e).nil();
    let hl = card::get_level(d, h);
    let lv = duel::announce_number_range(d, a.tp, Some(1), Some(3), &[hl]).nil().0 as u32;
    // Duel.SetTargetParam(lv)
    duel::set_target_param(d, lv);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local fid=0
    let mut fid: i64 = 0;
    // local lv=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let lv = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(lv)
        effect::set_value(d, e1, lv);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // fid=c:GetRealFieldID()
        fid = card::get_real_field_id(d, c) as i64;
    }
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // e2:SetTarget(s.splimit)
    effect::set_target(d, e2, Some(lua_fn(splimit)));
    // e2:SetLabel(fid)
    effect::set_label(d, e2, &[fid]);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_MUST_BE_MATERIAL)
    effect::set_code(d, e3, EFFECT_MUST_BE_MATERIAL);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetReset(RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET))
    effect::set_reset(d, e3, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 1);
    // e3:SetValue(REASON_SYNCHRO)
    effect::set_value(d, e3, REASON_SYNCHRO as i64);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`:
/// `return sumtype~=SUMMON_TYPE_SYNCHRO or e:GetOwner():GetRealFieldID()~=e:GetLabel() or e:GetOwner():IsFacedown()`
fn splimit(d: &mut Duel, a: Args) -> bool {
    if a.int(3) != SUMMON_TYPE_SYNCHRO as i64 {
        return true;
    }
    let owner = effect::get_owner(d, a.e(0)).nil();
    if card::get_real_field_id(d, owner) as i64 != effect::get_label(d, a.e(0)) {
        return true;
    }
    card::is_facedown(d, owner)
}
