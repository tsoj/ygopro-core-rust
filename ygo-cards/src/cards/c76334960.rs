//! Invocation - "Chalice"
//! 召喚魔術－「杯」
//! scripted by pyrQ
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 76334960;

// s.listed_seriess={SET_ALEISTER,SET_INVOKED} is a misspelled field: the core never reads it
// (only this script names it), so it is left out.

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you have an "Aleister" monster in your field, GY, or banishment: Activate 1 of these effects;
    // ● Fusion Summon 1 "Invoked" Fusion Monster from your Extra Deck, by banishing 1 monster from your hand or field and 1 monster from your Deck
    // local fusion_params1={
    //     fusfilter=function(c) return c:IsSetCard(SET_INVOKED) end,
    //     exactcount=2,
    //     matfilter=Card.IsAbleToRemove,
    //     extrafil=function(e,tp,mg) ... end,
    //     extratg=function(e,tp,eg,ep,ev,re,r,rp,chk) ... end,
    //     extraop=Fusion.BanishMaterial
    // }
    let fusion_params1 = fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_INVOKED as u16]))),
        exactcount: Some(2),
        matfilter: Some(fusion::mat_filter_fn(|d, c, _e, tp, chk| card::is_able_to_remove(d, c, None, Some(tp), Some(chk as u32)))),
        extrafil: Some(Arc::new(extrafil1)),
        extratg: Some(cost_target(extratg1)),
        extraop: Some(fusion::banish_material()),
        ..Default::default()
    };
    // ● Fusion Summon 1 "Invoked" Fusion Monster from your Extra Deck, by banishing 1 monster you control and 1 face-up monster your opponent controls
    // local fusion_params2={
    //     fusfilter=function(c) return c:IsSetCard(SET_INVOKED) end,
    //     exactcount=2,
    //     matfilter=Fusion.OnFieldMat(Card.IsAbleToRemove),
    //     extrafil=function(e,tp,mg) ... end,
    //     extratg=function(e,tp,eg,ep,ev,re,r,rp,chk) ... end,
    //     extraop=Fusion.BanishMaterial
    // }
    let fusion_params2 = fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_INVOKED as u16]))),
        exactcount: Some(2),
        matfilter: Some(fusion::on_field_mat_with(fusion::mat_filter_fn(|d, c, _e, tp, chk| {
            card::is_able_to_remove(d, c, None, Some(tp), Some(chk as u32))
        }))),
        extrafil: Some(Arc::new(extrafil2)),
        extratg: Some(cost_target(extratg2)),
        extraop: Some(fusion::banish_material()),
        ..Default::default()
    };
    // local fustg1=Fusion.SummonEffTG(fusion_params1)
    let fustg1 = fusion::summon_eff_tg(d, fusion_params1.clone());
    // local fusop1=Fusion.SummonEffOP(fusion_params1)
    let fusop1 = fusion::summon_eff_op(d, fusion_params1);
    // local fustg2=Fusion.SummonEffTG(fusion_params2)
    let fustg2 = fusion::summon_eff_tg(d, fusion_params2.clone());
    // local fusop2=Fusion.SummonEffOP(fusion_params2)
    let fusop2 = fusion::summon_eff_op(d, fusion_params2);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target(fustg1,fustg2))
    effect::set_target(d, e1, Some(cost_target(move |d, a| target(d, a, &fustg1, &fustg2))));
    // e1:SetOperation(s.activate(fusop1,fusop2))
    effect::set_operation(d, e1, Some(operation(move |d, a| activate(d, a, &fusop1, &fusop2))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function(tp,sg,fc) return sg:FilterCount(Card.IsLocation,nil,LOCATION_DECK)==1 end
fn check1(d: &mut Duel, a: Args) -> bool {
    let sg = a.g(1);
    group::filter_count(d, &sg, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX) == 1
}
// function(e,tp,mg) return Duel.GetMatchingGroup(...),function(tp,sg,fc) ... end end (fusion_params1)
fn extrafil1(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)),
        tp,
        LOCATION_DECK,
        0,
        NO_EX,
    );
    (Some(g), Some(lua_fn(check1)))
}
// function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return true end ... end (fusion_params1)
fn extratg1(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 2, a.tp, (LOCATION_HAND | LOCATION_ONFIELD | LOCATION_DECK) as i32);
    true
}
// function(tp,sg,fc) return sg:FilterCount(Card.IsControler,nil,1-tp)==1 end
fn check2(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(0);
    let sg = a.g(1);
    group::filter_count(d, &sg, &|d, c| card::is_controler(d, c, 1 - tp), NO_EX) == 1
}
// function(e,tp,mg) ... end (fusion_params2)
fn extrafil2(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_remove(d, c, None, None, None)),
        tp,
        0,
        LOCATION_ONFIELD,
        NO_EX,
    );
    (Some(g), Some(lua_fn(check2)))
}
// function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return true end ... end (fusion_params2)
fn extratg2(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 2, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    true
}
// function s.confilter(c)
fn confilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ALEISTER) and c:IsMonster() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_ALEISTER as u16]) && card::is_monster(d, c) && card::is_faceup(d, c)
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.confilter,tp,LOCATION_MZONE|LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil)
    duel::is_existing_matching_card(d, Some(&confilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX)
}
// The arguments of a Lua call `f(e,tp,eg,ep,ev,re,r,rp[,chk])` made with the arguments of `a`
fn call_args(a: &EffArgs, chk: Option<i64>) -> Vec<Value> {
    let mut v = vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map_or(Value::Nil, Value::Effect),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if let Some(chk) = chk {
        v.push(Value::Int(chk));
    }
    v
}
// function s.target(fustg1,fustg2) return function(e,tp,eg,ep,ev,re,r,rp,chk) ... end end
fn target(d: &mut Duel, a: &EffArgs, fustg1: &LuaFn, fustg2: &LuaFn) -> bool {
    // local b1=fustg1(e,tp,eg,ep,ev,re,r,rp,0)
    let b1 = fustg1.call(d, &call_args(a, Some(0))).truthy();
    // local b2=fustg2(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = fustg2.call(d, &call_args(a, Some(0))).truthy();
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local cd=e:GetChainData()
    // cd.choice=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let choice = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    chain::set_effect_data(d, a.e, None, chain::field::CARD, choice_value(choice));
    // if cd.choice==1 then
    //     fustg1(e,tp,eg,ep,ev,re,r,rp,1)
    // elseif cd.choice==2 then
    //     fustg2(e,tp,eg,ep,ev,re,r,rp,1)
    // end
    if choice == Some(1) {
        fustg1.call(d, &call_args(a, Some(1)));
    } else if choice == Some(2) {
        fustg2.call(d, &call_args(a, Some(1)));
    }
    true
}
// The chosen option of `Duel.SelectEffect` as the value stored in `cd.choice` (nil when none)
fn choice_value(choice: Option<usize>) -> Value {
    match choice {
        Some(n) => Value::Int(n as i64),
        None => Value::Nil,
    }
}
// function s.activate(fusop1,fusop2) return function(e,tp,eg,ep,ev,re,r,rp) ... end end
fn activate(d: &mut Duel, a: &EffArgs, fusop1: &LuaFn, fusop2: &LuaFn) {
    // local cd=e:GetChainData()
    // if cd.choice==1 then
    //     fusop1(e,tp,eg,ep,ev,re,r,rp)
    // elseif cd.choice==2 then
    //     fusop2(e,tp,eg,ep,ev,re,r,rp)
    // end
    match chain::effect_data(d, a.e, None, chain::field::CARD) {
        Value::Int(1) => {
            fusop1.call(d, &call_args(a, None));
        }
        Value::Int(2) => {
            fusop2.call(d, &call_args(a, None));
        }
        _ => {}
    }
}
