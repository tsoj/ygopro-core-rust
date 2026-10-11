//! Magikey Maftea
//! Scripted by Eerie Code
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 99426088;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGIKEY], ..ScriptInfo::DEFAULT };

/// `fparams={fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_MAGIKEY),extrafil=s.fextra,extratg=s.extratg}`
fn fparams() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_MAGIKEY as u16]))),
        extrafil: Some(Arc::new(fextra)),
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    }
}
/// `rparams={filter=aux.FilterBoolFunction(Card.IsSetCard,SET_MAGIKEY),lvtype=RITPROC_GREATER,extrafil=s.rextra,extraop=s.extraop,forcedselection=s.rcheck,extratg=s.extratg}`
fn rparams() -> ritual::RitualProc {
    ritual::RitualProc {
        filter: Some(Arc::new(|d: &mut Duel, c: CardId| card::is_set_card(d, c, &[SET_MAGIKEY as u16]))),
        lvtype: RITPROC_GREATER,
        extrafil: Some(Arc::new(rextra)),
        extraop: Some(Arc::new(extraop)),
        forcedselection: Some(Arc::new(rcheck)),
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    }
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion/Ritual summon
    // local fparams=...
    // local rparams=...
    // local fustg,fusop,rittg,ritop=Fusion.SummonEffTG(fparams),Fusion.SummonEffOP(fparams),Ritual.Target(rparams),Ritual.Operation(rparams)
    let fustg = fusion::summon_eff_tg(d, fparams());
    let fusop = fusion::summon_eff_op(d, fparams());
    let rittg = ritual::target(d, rparams());
    let ritop = ritual::operation(d, rparams());
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target(fustg,rittg))
    let (tfus, trit) = (fustg.clone(), rittg.clone());
    effect::set_target(d, e1, Some(cost_target(move |d, a| target_body(d, a, &tfus, &trit))));
    // e1:SetOperation(s.operation(fustg,fusop,rittg,ritop))
    effect::set_operation(d, e1, Some(operation(move |d, a| op_body(d, a, &fustg, &fusop, &rittg, &ritop))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.fextra(e,tp,mg)
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_NORMAL),tp,LOCATION_MZONE,0,1,nil) then
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_NORMAL)), tp, LOCATION_MZONE, 0, 1, NO_EX) {
        // local sg=Duel.GetMatchingGroup(s.fexfilter,tp,LOCATION_DECK,0,nil)
        let sg = duel::get_matching_group(d, Some(&fexfilter), tp, LOCATION_DECK, 0, NO_EX);
        // if #sg>0 then
        if !sg.is_empty() {
            // return sg,s.fcheck
            return (Some(sg), Some(lua_fn(fcheck)));
        }
    }
    // return nil
    (None, None)
}
// function s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.fexfilter(c)
fn fexfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_NORMAL) and c:IsAbleToGrave()
    card::is_type(d, c, TYPE_NORMAL) && card::is_able_to_grave(d, c, None)
}
// function s.fcheck(tp,sg,fc)
fn fcheck(d: &mut Duel, a: Args) -> bool {
    // return sg:FilterCount(Card.IsLocation,nil,LOCATION_DECK)<=1
    let sg = a.g(1);
    group::filter_count(d, &sg, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX) <= 1
}
// function s.rcheck(e,tp,sg,fc)
fn rcheck(d: &mut Duel, _e: EffectId, _tp: u8, sg: &CardSet, _fc: CardId) -> (bool, bool) {
    // return sg:FilterCount(Card.IsLocation,nil,LOCATION_DECK)<=1
    (group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX) <= 1, false)
}
// function s.rextra(e,tp,mg)
fn rextra(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_NORMAL),tp,LOCATION_MZONE,0,1,nil) then
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_NORMAL)), a.tp, LOCATION_MZONE, 0, 1, NO_EX) {
        // local sg=Duel.GetMatchingGroup(s.fexfilter,tp,LOCATION_DECK,0,nil)
        let sg = duel::get_matching_group(d, Some(&fexfilter), a.tp, LOCATION_DECK, 0, NO_EX);
        // if #sg>0 then
        if !sg.is_empty() {
            // return sg
            return Some(sg);
        }
    }
    // return nil
    None
}
// function s.extraop(mg,e,tp,eg,ep,ev,re,r,rp)
fn extraop(d: &mut Duel, mg: &CardSet, _a: &EffArgs, _tc: CardId) {
    // local mat2=mg:Filter(Card.IsLocation,nil,LOCATION_DECK)
    let mut mg = mg.clone();
    let mat2 = group::filter(d, &mg, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
    // mg:Sub(mat2)
    group::remove_card(&mut mg, &mat2);
    // Duel.ReleaseRitualMaterial(mg)
    duel::release_ritual_material(d, &mg, false);
    // Duel.SendtoGrave(mat2,REASON_EFFECT+REASON_MATERIAL+REASON_RITUAL)
    duel::send_to_grave(d, mat2, REASON_EFFECT + REASON_MATERIAL + REASON_RITUAL, None, None);
}
/// The arguments of a Lua call of the effect functions (`chk` = 0 for the target check).
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
// function s.target(fustg,rittg)
// return function(e,tp,eg,ep,ev,re,r,rp,chk)
//     if chk==0 then return fustg(e,tp,eg,ep,ev,re,r,rp,chk) or rittg(e,tp,eg,ep,ev,re,r,rp,chk) end
// end
fn target_body(d: &mut Duel, a: &EffArgs, fustg: &LuaFn, rittg: &LuaFn) -> bool {
    if a.chk == Some(0) {
        return fustg.call(d, &args(a, Some(0))).truthy() || rittg.call(d, &args(a, Some(0))).truthy();
    }
    true
}
// function s.operation(fustg,fusop,rittg,ritop)
// return function(e,tp,eg,ep,ev,re,r,rp)
fn op_body(d: &mut Duel, a: &EffArgs, fustg: &LuaFn, fusop: &LuaFn, rittg: &LuaFn, ritop: &LuaFn) {
    // local fus=fustg(e,tp,eg,ep,ev,re,r,rp,0)
    let fus = fustg.call(d, &args(a, Some(0))).truthy();
    // local rit=rittg(e,tp,eg,ep,ev,re,r,rp,0)
    let rit = rittg.call(d, &args(a, Some(0))).truthy();
    // if fus or rit then
    if fus || rit {
        // local sel={}
        let mut sel: Vec<u64> = Vec::new();
        // if fus then table.insert(sel,aux.Stringid(id,0)) end
        if fus {
            sel.push(aux::stringid(ID, 0));
        }
        // if rit then table.insert(sel,aux.Stringid(id,1)) end
        if rit {
            sel.push(aux::stringid(ID, 1));
        }
        // local res=Duel.SelectOption(tp,table.unpack(sel))
        let res = duel::select_option(d, a.tp, true, &sel);
        // if res==0 and fus then
        if res == 0 && fus {
            // fusop(e,tp,eg,ep,ev,re,r,rp)
            fusop.call(d, &args(a, None));
        // else
        } else {
            // ritop(e,tp,eg,ep,ev,re,r,rp)
            ritop.call(d, &args(a, None));
        }
    }
}
