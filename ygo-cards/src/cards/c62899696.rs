//! Speedroid Maliciousmagnet
use super::prelude::*;

pub const ID: u32 = 62899696;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot be synchro
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_BE_SYNCHRO_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCondition(s.smcon)
    effect::set_condition(d, e1, Some(lua_fn(|d, a: Args| smcon(d, a.e(0)))));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // synchro summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.smcon(e)`
fn smcon(d: &mut Duel, e: EffectId) -> bool {
    // return e:GetHandler():GetFlagEffect(id)==0
    let h = effect::get_handler(d, e).nil();
    card::get_flag_effect(d, h, ID) == 0
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.IsMainPhase()
    duel::is_turn_player(d, a.tp) && duel::is_main_phase(d, None)
}
/// `s.filter(tc,c,tp)`
fn filter(d: &mut Duel, tc: CardId, c: CardId, tp: u8) -> bool {
    // if not tc:IsFaceup() or not tc:IsCanBeSynchroMaterial() then return false end
    if !card::is_faceup(d, tc) || !card::is_can_be_synchro_material(d, tc, None, None, None) {
        return false;
    }
    // c:RegisterFlagEffect(id,0,0,1)
    card::register_flag_effect(d, c, ID, 0, 0, 1, 0, 0);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SYNCHRO_MATERIAL)
    effect::set_code(d, e1, EFFECT_SYNCHRO_MATERIAL);
    // tc:RegisterEffect(e1,true)
    card::register_effect(d, tc, e1, true);
    // local mg=Group.FromCards(c,tc)
    let mg = group::create_group(&[c, tc]);
    // local res=Duel.IsExistingMatchingCard(s.synfilter,tp,LOCATION_EXTRA,0,1,nil,mg)
    let res = duel::is_existing_matching_card(d, Some(&|d, x| synfilter(d, x, &mg)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    // c:ResetFlagEffect(id)
    card::reset_flag_effect(d, c, ID);
    // e1:Reset()
    effect::reset(d, e1);
    // return res
    res
}
/// `s.synfilter(c,mg)`
fn synfilter(d: &mut Duel, c: CardId, mg: &CardSet) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND) and c:IsSynchroSummonable(nil,mg)
    card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_synchro_summonable(d, c, None, Some(mg.clone()), 0, 0)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc,e:GetHandler(),tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && filter(d, chkc, c, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil,e:GetHandler(),tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| filter(d, x, c, tp)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil,e:GetHandler(),tp)
    let _ = duel::select_target(d, tp, Some(&|d, x| filter(d, x, c, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and c:IsFaceup() and tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if !(card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c)) {
        return;
    }
    let Some(tc) = tc else { return };
    if !(card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e)) {
        return;
    }
    // c:RegisterFlagEffect(id,0,0,1)
    card::register_flag_effect(d, c, ID, 0, 0, 1, 0, 0);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SYNCHRO_MATERIAL)
    effect::set_code(d, e1, EFFECT_SYNCHRO_MATERIAL);
    // tc:RegisterEffect(e1,true)
    card::register_effect(d, tc, e1, true);
    // local mg=Group.FromCards(c,tc)
    let mg = group::create_group(&[c, tc]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.synfilter,tp,LOCATION_EXTRA,0,1,1,nil,mg)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| synfilter(d, x, &mg)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local sc=g:GetFirst()
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
        // e3:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e3, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // e3:SetOperation(s.regop)
        effect::set_operation(d, e3, Some(operation(regop)));
        // e3:SetLabelObject(e1)
        effect::set_label_object(d, e3, Value::Effect(e1));
        // sc:RegisterEffect(e3,true)
        card::register_effect(d, sc, e3, true);
        // local e4=e3:Clone()
        let e4 = effect::clone(d, e3);
        // e4:SetCode(EVENT_SPSUMMON_NEGATED)
        effect::set_code(d, e4, EVENT_SPSUMMON_NEGATED);
        // sc:RegisterEffect(e4,true)
        card::register_effect(d, sc, e4, true);
        // Duel.SynchroSummon(tp,sc,nil,mg)
        duel::synchro_summon(d, tp, sc, None, Some(mg), 0, 0);
    } else {
        // c:ResetFlagEffect(id)
        card::reset_flag_effect(d, c, ID);
        // e1:Reset()
        effect::reset(d, e1);
    }
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local rc=e:GetOwner()
    let rc = effect::get_owner(d, e).nil();
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, e);
    // rc:ResetFlagEffect(id)
    card::reset_flag_effect(d, rc, ID);
    // e:GetLabelObject():Reset()
    let lo = effect::get_label_object(d, e).as_effect().nil();
    effect::reset(d, lo);
    // e:Reset()
    effect::reset(d, e);
}
