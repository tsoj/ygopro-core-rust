//! E・HERO エリクシーラー (Elemental HERO Electrum)
use super::prelude::*;

pub const ID: u32 = 29343734;

pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_HERO, SET_ELEMENTAL_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,21844576,58932615,84327329,79979666)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(21844576), fusion::Mat::Code(58932615), fusion::Mat::Code(84327329), fusion::Mat::Code(79979666)]);
    // spsummon condition
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e2, EFFECT_SPSUMMON_CONDITION);
    // e2:SetValue(aux.fuslimit)
    effect::set_value_fn(d, e2, aux::fuslimit());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // attribute
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_ADD_ATTRIBUTE)
    effect::set_code(d, e3, EFFECT_ADD_ATTRIBUTE);
    // e3:SetValue(0xf)
    effect::set_value(d, e3, 0xf);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // return
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e4, CATEGORY_TODECK);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetCondition(s.retcon)
    effect::set_condition(d, e4, Some(condition(retcon)));
    // e4:SetTarget(s.rettg)
    effect::set_target(d, e4, Some(cost_target(rettg)));
    // e4:SetOperation(s.retop)
    effect::set_operation(d, e4, Some(operation(retop)));
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
    // e5:SetValue(s.val)
    effect::set_value_fn(d, e5, effect_card_value(val));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsFusionSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
/// `s.rettg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,LOCATION_REMOVED,LOCATION_REMOVED,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_REMOVED,LOCATION_REMOVED,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX);
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsAttribute,c:GetAttribute()),c:GetControler(),0,LOCATION_MZONE,nil)*300
    let attr = card::get_attribute(d, c);
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&aux::faceup_filter(move |d, x| card::is_attribute(d, x, attr))), p, 0, LOCATION_MZONE, NO_EX) as i64 * 300
}
