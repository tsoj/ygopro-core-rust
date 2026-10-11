//! Ravenous Vendread
//! scripted by pyrQ
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 13386407;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[29348048], listed_series: &[SET_VENDREAD], ..ScriptInfo::DEFAULT };

// local rparams={handler=c,lvtype=RITPROC_GREATER,filter=s.ritualfil,matfilter=s.matfilter,extrafil=s.extrafil,
//     location=LOCATION_HAND|LOCATION_GRAVE,forcedselection=s.ritcheck}
fn rparams() -> ritual::RitualProc {
    ritual::RitualProc {
        lvtype: RITPROC_GREATER,
        filter: Some(Arc::new(|d: &mut Duel, c: CardId| ritualfil(d, c))),
        matfilter: Some(ritual::MatFilt::F(Arc::new(|d: &mut Duel, c: CardId, e: EffectId, tp: u8| matfilter(d, c, e, tp)))),
        extrafil: Some(Arc::new(|d: &mut Duel, a: &EffArgs| extrafil(d, a))),
        location: Some(LOCATION_HAND | LOCATION_GRAVE),
        forcedselection: Some(Arc::new(|d: &mut Duel, e: EffectId, tp: u8, g: &CardSet, sc: CardId| ritcheck(d, e, tp, g, sc))),
        ..Default::default()
    }
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_RELEASE+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_RELEASE + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target(Ritual.Target(rparams)))
    let rittg = ritual::target(d, rparams());
    effect::set_target(d, e1, Some(target_wrap(rittg)));
    // e1:SetOperation(s.operation(Ritual.Target(rparams),Ritual.Operation(rparams)))
    let rittg = ritual::target(d, rparams());
    let ritop = ritual::operation(d, rparams());
    effect::set_operation(d, e1, Some(op_wrap(rittg, ritop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.ritualfil(c)
fn ritualfil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VENDREAD) and c:IsRitualMonster()
    card::is_set_card(d, c, &[SET_VENDREAD as u16]) && card::is_ritual_monster(d, c)
}
// function s.matfilter(c,e,tp)
fn matfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (Duel.IsPlayerCanRelease(tp,c) and c:IsLocation(LOCATION_MZONE)) or s.extramatfil(c,e,tp)
    (duel::is_player_can_release(d, tp, Some((c, None))) && card::is_location(d, c, LOCATION_MZONE)) || extramatfil(d, c, e, tp)
}
// function s.extramatfil(c,e,tp)
fn extramatfil(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_VENDREAD) and not c:IsCode(29348048) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE)
    card::is_set_card(d, c, &[SET_VENDREAD as u16])
        && !card::is_code(d, c, &[29348048])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
}
// function s.extrafil(e,tp,mg)
fn extrafil(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    let (e, tp) = (a.e, a.tp);
    // if not Duel.IsPlayerCanRelease(tp) then return nil end
    if !duel::is_player_can_release(d, tp, None) {
        return None;
    }
    // return Duel.GetMatchingGroup(s.extramatfil,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp)
    Some(duel::get_matching_group(d, Some(&|d, c| extramatfil(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX))
}
// function s.ritcheck(e,tp,g,sc)
fn ritcheck(d: &mut Duel, e: EffectId, _tp: u8, g: &CardSet, _sc: CardId) -> (bool, bool) {
    // local obj=e:GetLabelObject()
    let obj = effect::get_label_object(d, e).as_card();
    // local res=obj and g:IsContains(obj)
    let res = obj.is_some_and(|o| g.contains(o));
    // local ct=g:FilterCount(aux.NOT(Card.IsLocation),obj,LOCATION_MZONE)
    let not_mzone = |d: &mut Duel, c: CardId| !card::is_location(d, c, LOCATION_MZONE);
    let ct = match obj {
        Some(o) => group::filter_count(d, g, &not_mzone, o.into()),
        None => group::filter_count(d, g, &not_mzone, NO_EX),
    };
    // return res and ct==0,not res and ct~=0
    (res && ct == 0, !res && ct != 0)
}
/// The arguments of the Lua call `f(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ritual_args(a: &EffArgs, chk: Option<i64>) -> [Value; 9] {
    [
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
        chk.map_or(Value::Nil, Value::Int),
    ]
}
// function s.spfilter(c,e,tp,eg,ep,ev,re,r,rp,rittg)
fn spfilter(d: &mut Duel, c: CardId, a: &EffArgs, rittg: &LuaFn) -> bool {
    let e = a.e;
    // if not s.extramatfil(c,e,tp) then return false end
    if !extramatfil(d, c, e, a.tp) {
        return false;
    }
    // e:SetLabelObject(c)
    effect::set_label_object(d, e, Value::Card(c));
    // local res=rittg(e,tp,eg,ep,ev,re,r,rp,0)
    let res = rittg.call(d, &ritual_args(a, Some(0))).truthy();
    // e:SetLabelObject(nil)
    effect::set_label_object(d, e, Value::Nil);
    // return res
    res
}
// function s.target(rittg)
//   return function(e,tp,eg,ep,ev,re,r,rp,chk)
fn target_wrap(rittg: LuaFn) -> LuaFn {
    cost_target(move |d, a| {
        let tp = a.tp;
        // if chk==0 then return Duel.IsPlayerCanSpecialSummonCount(tp,2)
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp,eg,ep,ev,re,r,rp,rittg) end
        if a.chk == Some(0) {
            return duel::is_player_can_special_summon_count(d, tp, 2)
                && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a, &rittg)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
        }
        // rittg(e,tp,eg,ep,ev,re,r,rp,1)
        rittg.call(d, &ritual_args(a, Some(1)));
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
        true
    })
}
// function s.operation(rittg,ritop)
//   return function(e,tp,eg,ep,ev,re,r,rp)
fn op_wrap(rittg: LuaFn, ritop: LuaFn) -> LuaFn {
    operation(move |d, a| {
        let tp = a.tp;
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp,eg,ep,ev,re,r,rp,rittg)
        let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a, &rittg)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #sg==0 then return end
        if sg.is_empty() {
            return;
        }
        // if Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)>0 then
        if duel::special_summon(d, sg.clone(), 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None) > 0 {
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - tp, sg.clone());
            // Duel.BreakEffect()
            duel::break_effect(d);
            // e:SetLabelObject(sg:GetFirst())
            let first = group::get_first(d, &sg);
            effect::set_label_object(d, a.e, first.map_or(Value::Nil, Value::Card));
            // ritop(e,tp,eg,ep,ev,re,r,rp)
            ritop.call(d, &ritual_args(a, None));
            // e:SetLabelObject(nil)
            effect::set_label_object(d, a.e, Value::Nil);
        }
    })
}
