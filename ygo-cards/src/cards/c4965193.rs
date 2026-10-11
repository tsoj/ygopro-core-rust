//! Chasma, Dragon Ruler of Auroras
use super::prelude::*;

pub const ID: u32 = 4965193;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGON_RULER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 7 Dragon monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON),7,2)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_DRAGON as u64, sc, st, Some(tp)))),
        Some(7),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // This effect becomes the effect of a Level 7 "Dragon Ruler" monster that activates by discarding itself
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.AND(s.applycost,Cost.DetachFromSelf(1)))
    effect::set_cost(
        d,
        e1,
        Some(cost::and(vec![cost_target(applycost), cost::detach_from_self(cost::Count::N(1), None, None)])),
    );
    // e1:SetTarget(s.applytg)
    effect::set_target(d, e1, Some(cost_target(applytg)));
    // e1:SetOperation(s.applyop)
    effect::set_operation(d, e1, Some(operation(applyop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Level 7 "Dragon Ruler" from your GY or banishment
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.runfn(fn,eff,tp,chk)`: `not fn or fn(eff,tp,Group.CreateGroup(),PLAYER_NONE,0,nil,REASON_EFFECT,PLAYER_NONE,chk)`
fn runfn(d: &mut Duel, f: Option<LuaFn>, eff: EffectId, tp: u8, chk: i64) -> bool {
    match f {
        None => true,
        Some(f) => f
            .call(
                d,
                &[
                    Value::Effect(eff),
                    Value::Int(tp as i64),
                    Value::Group(CardSet::new()),
                    Value::Int(PLAYER_NONE as i64),
                    Value::Int(0),
                    Value::Nil,
                    Value::Int(REASON_EFFECT as i64),
                    Value::Int(PLAYER_NONE as i64),
                    Value::Int(chk),
                ],
            )
            .truthy(),
    }
}
/// `s.applyfilter(c,tp)`
fn applyfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // if not (c:IsSetCard(SET_DRAGON_RULER) and c:IsLevel(7) and c:IsAbleToGraveAsCost()) then return false end
    if !(card::is_set_card(d, c, &[SET_DRAGON_RULER as u16]) && card::is_level(d, c, &[7]) && card::is_able_to_grave_as_cost(d, c)) {
        return false;
    }
    // for _,eff in ipairs({c:GetOwnEffects()}) do
    for eff in card::get_own_effects(d, c) {
        // if eff:HasSelfDiscardCost() and s.runfn(eff:GetCondition(),eff,tp,0) and s.runfn(eff:GetTarget(),eff,tp,0) then return true end
        if effect::has_self_discard_cost(d, eff) {
            let cond = effect::get_condition(d, eff);
            if runfn(d, cond, eff, tp, 0) {
                let targ = effect::get_target(d, eff);
                if runfn(d, targ, eff, tp, 0) {
                    return true;
                }
            }
        }
    }
    // end
    false
}
/// `s.applycost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn applycost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.applyfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| applyfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.applyfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| applyfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel).nil();
    // local effs={}
    let mut effs: Vec<EffectId> = Vec::new();
    // local options={}
    let mut options: Vec<(bool, u64)> = Vec::new();
    // for _,eff in ipairs({sc:GetOwnEffects()}) do
    for eff in card::get_own_effects(d, sc) {
        // if eff:HasSelfDiscardCost() then
        if effect::has_self_discard_cost(d, eff) {
            // table.insert(effs,eff)
            effs.push(eff);
            // local eff_chk=s.runfn(eff:GetCondition(),eff,tp,0) and s.runfn(eff:GetTarget(),eff,tp,0)
            let cond = effect::get_condition(d, eff);
            let mut eff_chk = runfn(d, cond, eff, tp, 0);
            if eff_chk {
                let targ = effect::get_target(d, eff);
                eff_chk = runfn(d, targ, eff, tp, 0);
            }
            // table.insert(options,{eff_chk,eff:GetDescription()})
            let desc = effect::get_description(d, eff);
            options.push((eff_chk, desc));
        }
    }
    // local op=#options==1 and 1 or Duel.SelectEffect(tp,table.unpack(options))
    let op = if options.len() == 1 { Some(1) } else { duel::select_effect(d, tp, &options) };
    // local te=effs[op]
    let te = effs[op.nil() - 1];
    // e:SetLabelObject(te)
    effect::set_label_object(d, e, Value::Effect(te));
    // Duel.SendtoGrave(sc,REASON_COST)
    duel::send_to_grave(d, sc, REASON_COST, None, None);
    true
}
/// `s.applytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn applytg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local te=e:GetLabelObject()
    let te = effect::get_label_object(d, e).as_effect();
    // if chkc then return te and te:GetTarget() and te:GetTarget()(e,tp,eg,ep,ev,re,r,rp,0,chkc) end
    if let Some(chkc) = a.chkc() {
        let Some(te) = te else {
            return false;
        };
        let Some(tg) = effect::get_target(d, te) else {
            return false;
        };
        return tg
            .call(
                d,
                &[
                    Value::Effect(e),
                    Value::Int(tp as i64),
                    eg_value(a),
                    Value::Int(a.ep as i64),
                    Value::Int(a.ev as i64),
                    re_value(a),
                    Value::Int(a.r as i64),
                    Value::Int(a.rp as i64),
                    Value::Int(0),
                    Value::Card(chkc),
                ],
            )
            .truthy();
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let te = te.nil();
    // e:SetLabel(te:GetLabel())
    let l = effect::get_label(d, te);
    effect::set_label(d, e, &[l]);
    // e:SetLabelObject(te:GetLabelObject())
    let lo = effect::get_label_object(d, te);
    effect::set_label_object(d, e, lo);
    // e:SetProperty(te:IsHasProperty(EFFECT_FLAG_CARD_TARGET) and EFFECT_FLAG_CARD_TARGET or 0)
    let prop = if effect::is_has_property(d, te, EFFECT_FLAG_CARD_TARGET, 0) { EFFECT_FLAG_CARD_TARGET } else { 0 };
    effect::set_property(d, e, prop, 0);
    // local tg=te:GetTarget()
    // if tg then
    if let Some(tg) = effect::get_target(d, te) {
        // tg(e,tp,eg,ep,ev,re,r,rp,1)
        tg.call(
            d,
            &[
                Value::Effect(e),
                Value::Int(tp as i64),
                eg_value(a),
                Value::Int(a.ep as i64),
                Value::Int(a.ev as i64),
                re_value(a),
                Value::Int(a.r as i64),
                Value::Int(a.rp as i64),
                Value::Int(1),
            ],
        );
        // te:SetLabel(e:GetLabel())
        let l = effect::get_label(d, e);
        effect::set_label(d, te, &[l]);
        // te:SetLabelObject(e:GetLabelObject())
        let lo = effect::get_label_object(d, e);
        effect::set_label_object(d, te, lo);
        // Duel.ClearOperationInfo(0)
        duel::clear_operation_info(d, 0);
    }
    // e:SetLabelObject(te)
    effect::set_label_object(d, e, Value::Effect(te));
    true
}
/// `s.applyop(e,tp,eg,ep,ev,re,r,rp)`
fn applyop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local te=e:GetLabelObject()
    // if not te then return end
    let Some(te) = effect::get_label_object(d, e).as_effect() else {
        return;
    };
    // local op=te:GetOperation()
    // if op then
    if let Some(op) = effect::get_operation(d, te) {
        // e:SetLabel(te:GetLabel())
        let l = effect::get_label(d, te);
        effect::set_label(d, e, &[l]);
        // e:SetLabelObject(te:GetLabelObject())
        let lo = effect::get_label_object(d, te);
        effect::set_label_object(d, e, lo);
        // op(e,tp,eg,ep,ev,re,r,rp)
        op.call(
            d,
            &[
                Value::Effect(e),
                Value::Int(tp as i64),
                eg_value(a),
                Value::Int(a.ep as i64),
                Value::Int(a.ev as i64),
                re_value(a),
                Value::Int(a.r as i64),
                Value::Int(a.rp as i64),
            ],
        );
    }
    // e:SetProperty(0)
    effect::set_property(d, e, 0, 0);
    // e:SetLabel(0)
    effect::set_label(d, e, &[0]);
    // e:SetLabelObject(nil)
    effect::set_label_object(d, e, Value::Nil);
}
/// The `eg` argument as a Lua value (`nil` when the core passed none)
fn eg_value(a: &EffArgs) -> Value {
    match &a.eg {
        Some(g) => Value::Group(g.clone()),
        None => Value::Nil,
    }
}
/// The `re` argument as a Lua value (`nil` when the core passed none)
fn re_value(a: &EffArgs) -> Value {
    match a.re {
        Some(re) => Value::Effect(re),
        None => Value::Nil,
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_BATTLE) or (rp==1-tp and c:IsReason(REASON_EFFECT) and c:IsPreviousControler(tp))
    card::is_reason(d, c, REASON_BATTLE)
        || (a.rp == 1 - tp && card::is_reason(d, c, REASON_EFFECT) && card::is_previous_controler(d, c, tp))
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DRAGON_RULER) and c:IsLevel(7) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_DRAGON_RULER as u16])
        && card::is_level(d, c, &[7])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_faceup(d, c)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))),
        tp,
        LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
