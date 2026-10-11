//! Virtual World Tiger - Fufu
//! 電脳堺虎－虎々
use super::prelude::*;

pub const ID: u32 = 95243515;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VIRTUAL_WORLD, SET_VIRTUAL_WORLD_GATE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Xyz Summon procedure
    // Xyz.AddProcedure(c,nil,9,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(9), 2, None, None, Some(xyz::MaxCt::Infinite), None, false, None);
    // --Negate the effects of 2 monsters on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Can attack directly
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
    // e2:SetCondition(s.effcond(2))
    effect::set_condition(d, e2, Some(effcond(2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Unaffected by activated effects, except "Virtual World" cards
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.effcond(4))
    effect::set_condition(d, e3, Some(effcond(4)));
    // e3:SetValue(function(e,te) return te:IsActivated() and not te:IsCardSetcode(SET_VIRTUAL_WORLD) end)
    effect::set_value_fn(
        d,
        e3,
        lua_fn(|d, a: Args| {
            let te = a.e(1);
            effect::is_activated(d, te) && !effect::is_card_setcode(d, te, &[SET_VIRTUAL_WORLD as u16])
        }),
    );
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.effcond(value)`: the count of face-up "Virtual World Gate" cards on the field, compared with `value`
fn effcond(value: i64) -> LuaFn {
    condition(move |d, a| {
        // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_VIRTUAL_WORLD_GATE),e:GetHandlerPlayer(),LOCATION_ONFIELD,0,nil)
        let p = effect::get_handler_player(d, a.e);
        let ct = duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_VIRTUAL_WORLD_GATE as u16]))), p, LOCATION_ONFIELD, 0, NO_EX);
        // return (value==2 and ct>=value) or (value==4 and ct==value)
        (value == 2 && ct as i64 >= value) || (value == 4 && ct as i64 == value)
    })
}
/// `s.tgfilter(c,e)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT) and not c:IsDisabled() and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && !card::is_disabled(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.propfilter(c,sc)`: `return sc and not c:IsRace(sc:GetRace()) and not c:IsAttribute(sc:GetAttribute())`
fn propfilter(d: &mut Duel, c: CardId, sc: Option<CardId>) -> bool {
    match sc {
        None => false,
        Some(sc) => {
            let race = card::get_race(d, sc);
            let attr = card::get_attribute(d, sc);
            !card::is_race(d, c, race) && !card::is_attribute(d, c, attr)
        }
    }
}
/// `s.rescon(sg,e,tp,mg)`: `sg:IsExists(Card.IsControler,1,nil,tp) and sg:IsExists(s.propfilter,1,nil,sg:GetFirst())`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let first = group::get_first(d, sg);
    let res = group::is_exists(d, sg, &|d, c| card::is_controler(d, c, tp), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| propfilter(d, c, first), 1, NO_EX);
    (res, false)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, a.e)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_NEGATE)
    let tg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_NEGATE as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,tg,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(tg), 2, 0, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local dg=Duel.GetTargetCards(e)
    let dg = duel::get_target_cards(d, a.e).nil();
    // if #dg==0 then return end
    if dg.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in dg:Iter() do
    for tc in group::iter(d, &dg) {
        // if tc:IsFaceup() and not tc:IsDisabled() then
        if card::is_faceup(d, tc) && !card::is_disabled(d, tc) {
            // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
            duel::negate_related_chain(d, tc, RESET_TURN_SET as u32);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetValue(RESET_TURN_SET)
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
