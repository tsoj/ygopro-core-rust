//! Ultimate Ancient Gear Golem
use super::prelude::*;

pub const ID: u32 = 12652643;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ANCIENT_GEAR], listed_names: &[CARD_ANCIENT_GEAR_GOLEM], material_setcode: &[SET_ANCIENT_GEAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: "Ancient Gear Golem" + 2 "Ancient Gear" monsters
    // Fusion.AddProcMixN(c,true,true,CARD_ANCIENT_GEAR_GOLEM,1,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_ANCIENT_GEAR),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![
            (fusion::Mat::Code(CARD_ANCIENT_GEAR_GOLEM), 1),
            (fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_ANCIENT_GEAR as u16], Some(fc), st, Some(tp)))), 2),
        ],
    );
    // spsummon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.fuslimit)
    effect::set_value_fn(d, e1, aux::fuslimit());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // pierce
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e2, EFFECT_PIERCE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card attacks, your opponent cannot activate Spell/Trap Cards until the end of the Damage Step
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,1)
    effect::set_target_range(d, e3, 0, 1);
    // e3:SetValue(s.aclimit)
    effect::set_value_fn(d, e3, lua_fn(aclimit));
    // e3:SetCondition(s.actcon)
    effect::set_condition(d, e3, Some(condition(actcon)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon 1 "Ancient Gear Golem" from your GY
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let re = a.e(1);
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker()==e:GetHandler()
    duel::get_attacker(d) == effect::get_handler(d, a.e)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_ANCIENT_GEAR_GOLEM) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_code(d, c, &[CARD_ANCIENT_GEAR_GOLEM]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) {
        // Duel.SpecialSummon(tc,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    }
}
