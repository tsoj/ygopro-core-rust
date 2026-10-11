//! Double Interlock
//! Scripted by Eerie Code
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 38129297;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ROKKET],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local fusion_params={handler=c,matfilter=aux.FALSE,extrafil=s.fextra,extraop=Fusion.BanishMaterial,extratg=s.extratg}
    // local ritual_params={handler=c,lvtype=RITPROC_GREATER,forcedselection=s.forcedselection,
    //     matfilter=aux.FilterBoolFunction(Card.IsLocation,LOCATION_GRAVE),extrafil=s.rextra,extratg=s.extratg}
    // (the parameters are built by fusion_params() and ritual_params() below)
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.effcost(fusion_params,ritual_params))
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.efftg(fusion_params,ritual_params))
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop(fusion_params,ritual_params))
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `fusion_params` of this card
fn fusion_params() -> fusion::SummonEff {
    fusion::SummonEff {
        // matfilter=aux.FALSE
        matfilter: Some(fusion::mat_filter_fn(|_d, _c, _e, _tp, _chk| false)),
        // extrafil=s.fextra
        extrafil: Some(Arc::new(fextra)),
        // extraop=Fusion.BanishMaterial
        extraop: Some(fusion::banish_material()),
        // extratg=s.extratg
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    }
}
/// `ritual_params` of this card
fn ritual_params() -> ritual::RitualProc {
    ritual::RitualProc {
        // lvtype=RITPROC_GREATER
        lvtype: RITPROC_GREATER,
        // forcedselection=s.forcedselection
        forcedselection: Some(Arc::new(forcedselection)),
        // matfilter=aux.FilterBoolFunction(Card.IsLocation,LOCATION_GRAVE)
        matfilter: Some(ritual::MatFilt::F(Arc::new(|d, c, _e, _tp| card::is_location(d, c, LOCATION_GRAVE)))),
        // extrafil=s.rextra
        extrafil: Some(Arc::new(rextra)),
        // extratg=s.extratg
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    }
}
/// The check of the Fusion Summon's materials: `s.matcheck(tp,sg,fc)` (as a Lua function value)
fn matcheck_fn() -> LuaFn {
    lua_fn(|d, a: Args| -> bool {
        let tp = a.p(0);
        let sg = a.g(1);
        let fc = a.card(2);
        // return sg:IsExists(Card.IsSetCard,1,nil,SET_ROKKET,fc,SUMMON_TYPE_FUSION,tp)
        group::is_exists(
            d,
            &sg,
            &|d, c| card::is_set_card_ctx(d, c, &[SET_ROKKET as u16], fc, SUMMON_TYPE_FUSION as u64, Some(tp)),
            1,
            NO_EX,
        )
    })
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil),s.matcheck
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)),
            tp,
            LOCATION_GRAVE,
            0,
            NO_EX,
        );
        return (Some(g), Some(matcheck_fn()));
    }
    // return nil,s.matcheck
    (None, Some(matcheck_fn()))
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.rextra(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rextra(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, a.tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // return Duel.GetMatchingGroup(aux.AND(Card.HasLevel,Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil)
        return Some(duel::get_matching_group(
            d,
            Some(&|d, c| card::has_level(d, c) && card::is_able_to_remove(d, c, None, None, None)),
            a.tp,
            LOCATION_GRAVE,
            0,
            NO_EX,
        ));
    }
    None
}
/// `s.forcedselection(e,tp,sg,sc)`
fn forcedselection(d: &mut Duel, _e: EffectId, _tp: u8, sg: &CardSet, _sc: CardId) -> (bool, bool) {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_ROKKET)
    (group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_ROKKET as u16]), 1, NO_EX), false)
}
/// `Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,chk)`
fn fusion_tg(d: &mut Duel, a: &EffArgs, chk: i64) -> bool {
    let mut args = a.clone();
    args.chk = Some(chk);
    fusion::summon_eff_tg_call(d, &fusion_params(), &args)
}
/// `Ritual.Target(ritual_params)(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ritual_tg(d: &mut Duel, a: &EffArgs, chk: i64) -> bool {
    let f = ritual::target(d, ritual_params());
    call_lua(d, &f, a, Some(chk)).truthy()
}
/// Calls a Lua function of a card effect with the arguments of `a` (`chk` only when given)
fn call_lua(d: &mut Duel, f: &LuaFn, a: &EffArgs, chk: Option<i64>) -> Value {
    let mut vals = vec![
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
        vals.push(Value::Int(chk));
    }
    f.call(d, &vals)
}
/// `s.effcost(fusion_params,ritual_params)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // e:SetLabel(-100)
    effect::set_label(d, a.e, &[-100]);
    // local b1=not Duel.HasFlagEffect(tp,id) and Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,0)
    let b1 = !duel::has_flag_effect(d, tp, ID, None) && fusion_tg(d, a, 0);
    // local b2=not Duel.HasFlagEffect(tp,id+1) and Ritual.Target(ritual_params)(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None) && ritual_tg(d, a, 0);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    true
}
/// `s.efftg(fusion_params,ritual_params)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local cost_skip=e:GetLabel()~=-100
    let cost_skip = effect::get_label(d, e) != -100;
    // local b1=(cost_skip or not Duel.HasFlagEffect(tp,id)) and Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,0)
    let b1 = (cost_skip || !duel::has_flag_effect(d, tp, ID, None)) && fusion_tg(d, a, 0);
    // local b2=(cost_skip or not Duel.HasFlagEffect(tp,id+1)) and Ritual.Target(ritual_params)(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = (cost_skip || !duel::has_flag_effect(d, tp, ID + 1, None)) && ritual_tg(d, a, 0);
    // if chk==0 then e:SetLabel(0) return b1 or b2 end
    if a.chk == Some(0) {
        effect::set_label(d, e, &[0]);
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, e, &[op.map_or(0, |v| v as i64)]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
        effect::set_category(d, e, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        // Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,1)
        fusion_tg(d, a, 1);
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        // Ritual.Target(ritual_params)(e,tp,eg,ep,ev,re,r,rp,1)
        ritual_tg(d, a, 1);
    }
    true
}
/// `s.effop(fusion_params,ritual_params)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Fusion Summon 1 Fusion Monster from your Extra Deck, by banishing its materials from your GY, including a "Rokket" monster
        // Fusion.SummonEffOP(fusion_params)(e,tp,eg,ep,ev,re,r,rp)
        fusion::summon_eff_op_call(d, &fusion_params(), a);
    } else if op == 2 {
        // Ritual Summon 1 Ritual Monster from your hand, by banishing monsters from your GY whose total Levels equal or exceed that monster's Level, including a "Rokket" monster
        // Ritual.Operation(ritual_params)(e,tp,eg,ep,ev,re,r,rp)
        let f = ritual::operation(d, ritual_params());
        call_lua(d, &f, a, None);
    }
}
