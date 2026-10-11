//! Assault Blackwing - Onimaru the Divine Thunder
//! (Lua: A BF－神立のオニマル)
use super::prelude::*;

pub const ID: u32 = 80773359;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLACKWING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // add type
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.tncon)
    effect::set_condition(d, e1, Some(condition(tncon)));
    // e1:SetOperation(s.tnop)
    effect::set_operation(d, e1, Some(operation(tnop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e2, EFFECT_MATERIAL_CHECK);
    // e2:SetValue(s.valcheck)
    effect::set_value_fn(d, e2, lua_fn(valcheck));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // indes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // lvchange
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e4, CATEGORY_LVCHANGE);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCountLimit(1,id,EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e4, 1, ID, 0, EFFECT_COUNT_CODE_DUEL as u8);
    // e4:SetTarget(s.lvtg)
    effect::set_target(d, e4, Some(cost_target(lvtg)));
    // e4:SetOperation(s.lvop)
    effect::set_operation(d, e4, Some(operation(lvop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // atk
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e5, EFFECT_UPDATE_ATTACK);
    // e5:SetCondition(s.condtion)
    effect::set_condition(d, e5, Some(condition(condtion)));
    // e5:SetValue(3000)
    effect::set_value(d, e5, 3000);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) -> Value {
    let (e, c) = (a.e(0), a.c(1));
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsSetCard,1,nil,SET_BLACKWING) then
    if group::is_exists(d, &g, &|d, x| card::is_set_card(d, x, &[SET_BLACKWING as u16]), 1, NO_EX) {
        // e:GetLabelObject():SetLabel(1)
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, lo, &[1]);
    } else {
        // e:GetLabelObject():SetLabel(0)
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, lo, &[0]);
    }
    Value::Nil
}
/// `s.tncon(e,tp,eg,ep,ev,re,r,rp)`
fn tncon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SYNCHRO and e:GetLabel()==1
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_SYNCHRO && effect::get_label(d, a.e) == 1
}
/// `s.tnop(e,tp,eg,ep,ev,re,r,rp)`
fn tnop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_ADD_TYPE)
    effect::set_code(d, e1, EFFECT_ADD_TYPE);
    // e1:SetValue(TYPE_TUNER)
    effect::set_value(d, e1, TYPE_TUNER as i64);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.lvfilter(c,lv)`
fn lvfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsSetCard(SET_BLACKWING) and c:HasLevel() and c:GetLevel()~=lv
    card::is_set_card(d, c, &[SET_BLACKWING as u16]) && card::has_level(d, c) && card::get_level(d, c) != lv
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let lv = card::get_level(d, h);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.lvfilter(chkc,e:GetHandler():GetLevel()) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && lvfilter(d, chkc, lv);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.lvfilter,tp,LOCATION_GRAVE,0,1,nil,e:GetHandler():GetLevel()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| lvfilter(d, c, lv)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.lvfilter,tp,LOCATION_GRAVE,0,1,1,nil,e:GetHandler():GetLevel())
    let _ = duel::select_target(d, a.tp, Some(&|d, c| lvfilter(d, c, lv)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and c:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            // e1:SetValue(tc:GetLevel())
            let tl = card::get_level(d, tc);
            effect::set_value(d, e1, tl as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
/// `s.mfilter(c)`
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsType(TYPE_SYNCHRO)
    !card::is_type(d, c, TYPE_SYNCHRO)
}
/// `s.condtion(e)`
fn condtion(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local mg=c:GetMaterial()
    let mg = card::get_material(d, c);
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // return (ph==PHASE_DAMAGE or ph==PHASE_DAMAGE_CAL) and Duel.GetAttacker()==c
    //   and c:GetSummonType()==SUMMON_TYPE_SYNCHRO and #mg>0 and not mg:IsExists(s.mfilter,1,nil)
    (ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL)
        && duel::get_attacker(d) == Some(c)
        && card::get_summon_type(d, c) == SUMMON_TYPE_SYNCHRO
        && !mg.is_empty()
        && !group::is_exists(d, &mg, &|d, x| mfilter(d, x), 1, NO_EX)
}
