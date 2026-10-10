//! Primite Imperial Dragon
use super::prelude::*;

pub const ID: u32 = 81418467;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PRIMITE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Requires 1 Normal Monster Tribute to Normal Summon face-up
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_LIMIT_SUMMON_PROC)
    effect::set_code(d, e0, EFFECT_LIMIT_SUMMON_PROC);
    // e0:SetCondition(s.selfnssumcon)
    effect::set_condition(d, e0, Some(lua_fn(selfnssumcon)));
    // e0:SetTarget(s.selfnssumtg)
    effect::set_target(d, e0, Some(lua_fn(selfnssumtg)));
    // e0:SetOperation(s.selfnssumop)
    effect::set_operation(d, e0, Some(lua_fn(selfnssumop)));
    // e0:SetValue(SUMMON_TYPE_TRIBUTE)
    effect::set_value(d, e0, SUMMON_TYPE_TRIBUTE as i64);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Normal Summon 1 "Primoredial" monster
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e1, CATEGORY_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) and Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetCost(Cost.SelfReveal)
    effect::set_cost(d, e1, Some(cost::self_reveal()));
    // e1:SetTarget(s.sumtg)
    effect::set_target(d, e1, Some(cost_target(sumtg)));
    // e1:SetOperation(s.sumop)
    effect::set_operation(d, e1, Some(operation(sumop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Apply effects if it is Tribute Summoned
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCondition(function(e) return e:GetHandler():IsTributeSummoned() end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp) return Duel.IsTurnPlayer(1-tp) and Duel.IsMainPhase() end` (e1)
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && duel::is_main_phase(d, None)
}
/// `function(e) return e:GetHandler():IsTributeSummoned() end` (e2)
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
/// `s.rescon(sg,e,tp,mg)`: `aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.tributefilter,1,nil,tp)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    let (res, _) = aux::chkf_mmz(1)(d, sg, e, tp, mg, c);
    (res && group::is_exists(d, sg, &|d, x| tributefilter(d, x, tp), 1, NO_EX), false)
}
/// `s.tributefilter(c,tp)`
fn tributefilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_NORMAL) and (c:IsControler(tp) or c:IsFaceup())
    card::is_type(d, c, TYPE_NORMAL) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.selfnssumcon(e,c)`
fn selfnssumcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return aux.SelectUnselectGroup(Duel.GetReleaseGroup(tp),e,tp,1,1,s.rescon,0)
    let rg = duel::get_release_group(d, tp, false, false, None);
    let r = aux::select_unselect_group(d, &rg, Some(a.e(0)), tp, Some(1), Some(1), Some(&rescon), 0, tp, None, None, None, false);
    matches!(r, Err(true))
}
/// `s.selfnssumtg(e,tp,eg,ep,ev,re,r,rp,c)`
fn selfnssumtg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=aux.SelectUnselectGroup(Duel.GetReleaseGroup(tp),e,tp,1,1,s.rescon,1,tp,HINTMSG_RELEASE,nil,nil,Duel.IsSummonCancelable())
    let rg = duel::get_release_group(d, tp, false, false, None);
    let cancelable = duel::is_summon_cancelable(d);
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&rescon), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, cancelable).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.selfnssumop(e,tp,eg,ep,ev,re,r,rp,c)`
fn selfnssumop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        // if not g then return end
        return;
    };
    let c = a.c(8);
    // c:SetMaterial(g)
    card::set_material(d, c, g.clone());
    // Duel.Release(g,REASON_SUMMON|REASON_COST|REASON_MATERIAL)
    duel::release(d, g.clone(), REASON_SUMMON | REASON_COST | REASON_MATERIAL, None);
    // g:DeleteGroup()
}
/// `s.sumfilter(c)`
fn sumfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PRIMITE) and c:IsSummonable(true,nil)
    card::is_set_card(d, c, &[SET_PRIMITE as u16]) && card::is_summonable(d, c, true, None, 0, None)
}
/// `s.sumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sumtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.sumfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&sumfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.sumfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&sumfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, Some(g), 1, a.tp, 0);
    true
}
/// `s.sumop(e,tp,eg,ep,ev,re,r,rp)`
fn sumop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.sumfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&sumfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.Summon(tp,tc,true,nil)
        duel::summon(d, a.tp, tc, true, None, 0, None);
    }
}
/// `s.rmvfilter(c,tp)`
fn rmvfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsAbleToRemove()
    //     and Duel.IsExistingMatchingCard(s.matchfilter,tp,LOCATION_GRAVE,0,1,nil,c:GetRace(),c:GetAttribute())
    if !(card::is_faceup(d, c) && card::is_able_to_remove(d, c, None, None, None)) {
        return false;
    }
    let race = card::get_race(d, c);
    let att = card::get_attribute(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| matchfilter(d, x, race, att)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.matchfilter(c,race,att)`
fn matchfilter(d: &mut Duel, c: CardId, race: u64, att: u32) -> bool {
    // return c:IsType(TYPE_NORMAL) and (c:IsRace(race) or c:IsAttribute(att))
    card::is_type(d, c, TYPE_NORMAL) && (card::is_race(d, c, race) || card::is_attribute(d, c, att))
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local disg=Duel.GetMatchingGroup(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,nil)
    let disg = duel::get_matching_group(d, Some(&card::is_negatable_monster), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local rmvg=Duel.GetMatchingGroup(s.rmvfilter,tp,0,LOCATION_MZONE,nil,tp)
    let tp = a.tp;
    let rmvg = duel::get_matching_group(d, Some(&|d, c| rmvfilter(d, c, tp)), tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #disg>0 or #rmvg>0 end
    if a.chk == Some(0) {
        return !disg.is_empty() || !rmvg.is_empty();
    }
    // if #disg>0 then
    if !disg.is_empty() {
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,disg,#disg,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(disg.clone()), disg.len() as u8, tp, 0);
    }
    // if #rmvg>0 then
    if !rmvg.is_empty() {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rmvg,#rmvg,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(rmvg.clone()), rmvg.len() as u8, tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local break_chk=false
    let mut break_chk = false;
    // local disg=Duel.GetMatchingGroup(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,nil):Filter(Card.IsCanBeDisabledByEffect,nil,e)
    let disg = duel::get_matching_group(d, Some(&card::is_negatable_monster), a.tp, 0, LOCATION_MZONE, NO_EX);
    let disg = group::filter(d, &disg, &|d, x| card::is_can_be_disabled_by_effect(d, x, a.e, true), NO_EX);
    // Negate the effects of all face-up monsters your opponent controls
    // for tc in disg:Iter() do
    for tc in group::iter(d, &disg) {
        // tc:NegateEffects(c)
        card::negate_effects(d, tc, c, None, false, None);
        // Duel.AdjustInstantly(tc)
        duel::adjust_instantly(d, Some(tc));
        // break_chk=true
        break_chk = true;
    }
    // Banish all monsters with the same Type/Attributes as Normal Monsters in your GY
    // local rmvg=Duel.GetMatchingGroup(s.rmvfilter,tp,0,LOCATION_MZONE,nil,tp)
    let tp = a.tp;
    let rmvg = duel::get_matching_group(d, Some(&|d, x| rmvfilter(d, x, tp)), tp, 0, LOCATION_MZONE, NO_EX);
    // if #rmvg>0 then
    if !rmvg.is_empty() {
        // if break_chk then Duel.BreakEffect() end
        if break_chk {
            duel::break_effect(d);
        }
        // Duel.Remove(rmvg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, &rmvg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
