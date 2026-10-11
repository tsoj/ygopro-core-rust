//! Mitsurugi Ritual
//! Scripted by Naim
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 81560239;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.effcost)
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.deckmatfilter(c)
fn deckmatfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_REPTILE) and c:IsReleasableByEffect()
    card::is_race(d, c, RACE_REPTILE) && card::is_releasable_by_effect(d, c, None, None)
}
// function s.extragroup(e,tp,eg,ep,ev,re,r,rp,chk)
fn extragroup(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    // return Duel.GetMatchingGroup(s.deckmatfilter,tp,LOCATION_DECK,0,nil)
    Some(duel::get_matching_group(d, Some(&deckmatfilter), a.tp, LOCATION_DECK, 0, NO_EX))
}
// function s.extraop(mat,e,tp,eg,ep,ev,re,r,rp,tc)
fn extraop(d: &mut Duel, mat: &CardSet) {
    // Duel.ReleaseRitualMaterial(mat,true)
    duel::release_ritual_material(d, mat, true);
}
// function s.tributelimit(e,tp,g,sc)
fn tributelimit(_d: &mut Duel, _e: EffectId, _tp: u8, g: &CardSet, _sc: CardId) -> (bool, bool) {
    // return #g<=2,#g>2
    (g.len() <= 2, g.len() > 2)
}
// local ritual_params1={lvtype=RITPROC_EQUAL,filter=aux.FilterBoolFunction(Card.IsRace,RACE_REPTILE),location=LOCATION_DECK,matfilter=aux.FilterBoolFunction(Card.IsRace,RACE_REPTILE)}
fn params1() -> ritual::RitualProc {
    ritual::RitualProc {
        lvtype: RITPROC_EQUAL,
        filter: Some(Arc::new(|d: &mut Duel, c: CardId| card::is_race(d, c, RACE_REPTILE))),
        location: Some(LOCATION_DECK),
        matfilter: Some(ritual::MatFilt::F(Arc::new(|d: &mut Duel, c: CardId, _e: EffectId, _tp: u8| card::is_race(d, c, RACE_REPTILE)))),
        ..Default::default()
    }
}
// local ritual_params2={lvtype=RITPROC_EQUAL,filter=aux.FilterBoolFunction(Card.IsRace,RACE_REPTILE),matfilter=aux.FilterBoolFunction(Card.IsRace,RACE_REPTILE),extrafil=s.extragroup,extraop=s.extraop,forcedselection=s.tributelimit}
fn params2() -> ritual::RitualProc {
    ritual::RitualProc {
        lvtype: RITPROC_EQUAL,
        filter: Some(Arc::new(|d: &mut Duel, c: CardId| card::is_race(d, c, RACE_REPTILE))),
        matfilter: Some(ritual::MatFilt::F(Arc::new(|d: &mut Duel, c: CardId, _e: EffectId, _tp: u8| card::is_race(d, c, RACE_REPTILE)))),
        extrafil: Some(Arc::new(|d: &mut Duel, a: &EffArgs| extragroup(d, a))),
        extraop: Some(Arc::new(|d: &mut Duel, mat: &CardSet, _a: &EffArgs, _tc: CardId| extraop(d, mat))),
        forcedselection: Some(Arc::new(|d: &mut Duel, e: EffectId, tp: u8, g: &CardSet, sc: CardId| tributelimit(d, e, tp, g, sc))),
        ..Default::default()
    }
}
/// The arguments of the Lua call `f(e,tp,eg,ep,ev,re,r,rp[,chk])`
fn args(a: &EffArgs, chk: Option<i64>) -> Vec<Value> {
    let mut v = vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if let Some(chk) = chk {
        v.push(Value::Int(chk));
    }
    v
}
/// `Ritual.Target(params)(e,tp,eg,ep,ev,re,r,rp,0)` as a boolean
fn ritual_target_chk0(d: &mut Duel, p: ritual::RitualProc, a: &EffArgs) -> bool {
    let f = ritual::target(d, p);
    f.call(d, &args(a, Some(0))).truthy()
}
// function s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // e:SetLabel(-100)
    effect::set_label(d, e, &[-100]);
    // local b1=not Duel.HasFlagEffect(tp,id) and Ritual.Target(ritual_params1)(e,tp,eg,ep,ev,re,r,rp,0)
    let b1 = !duel::has_flag_effect(d, tp, ID, None) && ritual_target_chk0(d, params1(), a);
    // local b2=not Duel.HasFlagEffect(tp,id+1) and Ritual.Target(ritual_params2)(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None) && ritual_target_chk0(d, params2(), a);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    true
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local cost_skip=e:GetLabel()~=-100
    let cost_skip = effect::get_label(d, e) != -100;
    // local b1=(cost_skip or not Duel.HasFlagEffect(tp,id))
    //     and Ritual.Target(ritual_params1)(e,tp,eg,ep,ev,re,r,rp,0)
    let b1 = (cost_skip || !duel::has_flag_effect(d, tp, ID, None)) && ritual_target_chk0(d, params1(), a);
    // local b2=(cost_skip or not Duel.HasFlagEffect(tp,id+1))
    //     and Ritual.Target(ritual_params2)(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = (cost_skip || !duel::has_flag_effect(d, tp, ID + 1, None)) && ritual_target_chk0(d, params2(), a);
    // if chk==0 then e:SetLabel(0) return b1 or b2 end
    if a.chk == Some(0) {
        effect::set_label(d, e, &[0]);
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, e, &[op.map_or(0, |x| x as i64)]);
    // if op==1 then
    if op == Some(1) {
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, tp, ID, RESET_PHASE + PHASE_END, 0, 1, 0);
        }
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
        // Duel.SetOperationInfo(0,CATEGORY_RELEASE,nil,1,tp,LOCATION_HAND|LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_RELEASE, None, 1, tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    } else if op == Some(2) {
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE + PHASE_END, 0, 1, 0);
        }
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
        // Duel.SetOperationInfo(0,CATEGORY_RELEASE,nil,1,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_RELEASE, None, 1, tp, (LOCATION_HAND | LOCATION_MZONE | LOCATION_DECK) as i32);
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Ritual.Operation(ritual_params1)(e,tp,eg,ep,ev,re,r,rp)
        let f = ritual::operation(d, params1());
        f.call(d, &args(a, None));
    } else if op == 2 {
        // elseif op==2 then Ritual.Operation(ritual_params2)(e,tp,eg,ep,ev,re,r,rp)
        let f = ritual::operation(d, params2());
        f.call(d, &args(a, None));
    }
}
