//! Varar, Vaalmonican Concord
//! ヴァルモニカの神奏－ヴァーラル
use super::prelude::*;

pub const ID: u32 = 1340142;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_VAALMONICA],
    counter_list: &[COUNTER_RESONANCE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 2 monsters, including a "Vaalmonica" Link Monster
    // Link.AddProcedure(c,nil,2,2,s.matcheck)
    link::add_procedure(d, c, None, 2, Some(2), Some(std::sync::Arc::new(matcheck)), None, None);
    // Unaffected by non-"Vaalmonica" cards' effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e) return Duel.GetCounter(e:GetHandlerPlayer(),1,0,COUNTER_RESONANCE)>=6 end)
    effect::set_condition(
        d,
        e1,
        Some(lua_fn(|d, a: Args| {
            let p = effect::get_handler_player(d, a.e(0));
            duel::get_counter(d, p, 1, 0, COUNTER_RESONANCE as u16) >= 6
        })),
    );
    // e1:SetValue(s.immval)
    effect::set_value_fn(d, e1, lua_fn(immval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gains additional attack for each Level 4 "Vaalmonica" monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e2, EFFECT_EXTRA_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, lua_fn(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Negate an opponent's Special Summon and destroy that monster(s), then remove 3 Resonance Counters from your field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DISABLE_SUMMON + CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e3, EVENT_SPSUMMON);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp end)
    effect::set_condition(d, e3, Some(condition(|_d, a| a.ep == 1 - a.tp)));
    // e3:SetTarget(s.negsumtg)
    effect::set_target(d, e3, Some(cost_target(negsumtg)));
    // e3:SetOperation(s.negsumop)
    effect::set_operation(d, e3, Some(operation(negsumop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.matfilter(c,lc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_VAALMONICA,lc,sumtype,tp) and c:IsType(TYPE_LINK,lc,sumtype,tp)
    card::is_set_card_ctx(d, c, &[SET_VAALMONICA as u16], lc, sumtype, Some(tp))
        && card::is_type_ctx(d, c, TYPE_LINK, lc, sumtype, Some(tp))
}
/// `s.matcheck(g,lc,sumtype,tp)`
fn matcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(s.matfilter,1,nil,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| matfilter(d, c, Some(lc), sumtype, tp), 1, NO_EX)
}
/// `s.immval(e,te)`
fn immval(d: &mut Duel, a: Args) -> bool {
    // return not te:IsCardSetcode(SET_VAALMONICA)
    let te = a.e(1);
    !effect::is_card_setcode(d, te, &[SET_VAALMONICA as u16])
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(4) and c:IsSetCard(SET_VAALMONICA) and c:IsFaceup()
    card::is_level(d, c, &[4]) && card::is_set_card(d, c, &[SET_VAALMONICA as u16]) && card::is_faceup(d, c)
}
/// `s.atkval(e)`
fn atkval(d: &mut Duel, a: Args) -> i64 {
    // return Duel.GetMatchingGroupCount(s.atkfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,nil)
    let p = effect::get_handler_player(d, a.e(0));
    duel::get_matching_group_count(d, Some(&atkfilter), p, LOCATION_MZONE, 0, NO_EX) as i64
}
/// `s.negsumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negsumtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,0,COUNTER_RESONANCE,3,REASON_EFFECT) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, a.tp, 1, 0, COUNTER_RESONANCE as u16, 3, REASON_EFFECT);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,0,0)
    let n = a.eg.clone().nil().len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, a.eg.clone(), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,#eg,0,0)
    let n = a.eg.clone().nil().len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), n, 0, 0);
    true
}
/// `s.negsumop(e,tp,eg,ep,ev,re,r,rp)`
fn negsumop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, a.eg.clone().nil());
    // if Duel.Destroy(eg,REASON_EFFECT)>0 then
    if duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None) > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.RemoveCounter(tp,1,0,COUNTER_RESONANCE,3,REASON_EFFECT)
        duel::remove_counter(d, tp, 1, 0, COUNTER_RESONANCE as u16, 3, REASON_EFFECT);
    }
}
