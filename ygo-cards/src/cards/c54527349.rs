//! Darklord Uprising
//! Scripted by edo9300
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 54527349;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DARKLORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON+CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON + CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.matfilter(exc)`: `function(c) return c~=exc and c:IsAbleToGrave() end`
fn matfilter(exc: CardId) -> fusion::MatFilterFn {
    fusion::mat_filter_fn(move |d, c, _e, _tp, _x| c != exc && card::is_able_to_grave(d, c, None))
}
/// `s.costcardcheck(exc)`: `function(tp,sg,fc) return Duel.GetLocationCountFromEx(tp,tp,sg+exc,fc)>0 end`
fn costcardcheck(exc: CardId) -> LuaFn {
    lua_fn(move |d, a| {
        let tp = a.p(0);
        // sg+exc
        let mut sg = a.g(1);
        group::add_card(&mut sg, &CardSet::from_card(exc));
        let fc = match a.card(2) {
            Some(fc) => duel::ExtraCard::Card(fc),
            None => duel::ExtraCard::None,
        };
        duel::get_location_count_from_ex(d, tp, Some(tp), (&sg).into(), fc, None).0 > 0
    })
}
/// `s.fextra(exc)`: `function(e,tp,mg) return nil,s.costcardcheck(exc) end`
fn fextra(exc: CardId) -> fusion::ExtraFil {
    Arc::new(move |_d: &mut Duel, _e: EffectId, _tp: u8, _mg: &CardSet| (None, Some(costcardcheck(exc))))
}
/// `aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_DARK)` as the `fusfilter`
fn dark_filter() -> fusion::FusFilter {
    fusion::fus_filter(|d, c, _tp| card::is_attribute(d, c, ATTRIBUTE_DARK))
}
/// `s.costfilter(c,e,tp,eg,ep,ev,re,r,rp)`: the cost's arguments are passed on by `a`.
fn costfilter(d: &mut Duel, c: CardId, a: &EffArgs) -> bool {
    // if not (c:IsSetCard(SET_DARKLORD) and c:IsMonster() and (c:IsLocation(LOCATION_HAND) or c:IsFaceup()) and c:IsAbleToGraveAsCost()) then return false end
    if !(card::is_set_card(d, c, &[SET_DARKLORD as u16])
        && card::is_monster(d, c)
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_able_to_grave_as_cost(d, c))
    {
        return false;
    }
    // local params={fusfilter=aux.FilterBoolFunction(Card.IsAttribute,ATTRIBUTE_DARK),matfilter=s.matfilter(c),extrafil=s.fextra(c),chkf=PLAYER_NONE}
    let params = fusion::SummonEff {
        fusfilter: Some(dark_filter()),
        matfilter: Some(matfilter(c)),
        extrafil: Some(fextra(c)),
        chkf: Some(PLAYER_NONE as u64),
        ..Default::default()
    };
    // return Fusion.SummonEffTG(params)(e,tp,eg,ep,ev,re,r,rp,0)
    let mut call = a.clone();
    call.chk = Some(0);
    fusion::summon_eff_tg_call(d, &params, &call)
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // e:SetLabel(100)
    effect::set_label(d, e, &[100]);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,e,tp,eg,ep,ev,re,r,rp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| costfilter(d, c, a)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,e,tp,eg,ep,ev,re,r,rp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| costfilter(d, c, a)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g.clone(), REASON_COST, None, None);
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g);
    effect::set_label_object(d, e, match first {
        Some(c) => Value::Card(c),
        None => Value::Nil,
    });
    true
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local params={fusfilter=aux.FilterBoolFunction(Card.IsAttribute,ATTRIBUTE_DARK)}
        // return e:GetLabel()==100 or Fusion.SummonEffTG(params)(e,tp,eg,ep,ev,re,r,rp,0)
        if effect::get_label(d, e) == 100 {
            return true;
        }
        let params = fusion::SummonEff { fusfilter: Some(dark_filter()), ..Default::default() };
        return fusion::summon_eff_tg_call(d, &params, a);
    }
    // local cost_card=e:GetLabelObject()
    // if cost_card then
    if let Some(cost_card) = effect::get_label_object(d, e).as_card() {
        // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,1,tp,cost_card:GetBaseAttack())
        let atk = card::get_base_attack(d, cost_card);
        duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 1, tp, atk);
    }
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp,chk)
fn spop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local cost_card=e:GetLabelObject()
    let has_cost_card = effect::get_label_object(d, e).as_card().is_some();
    // local params={fusfilter=aux.FilterBoolFunction(Card.IsAttribute,ATTRIBUTE_DARK),stage2=cost_card and s.stage2 or nil}
    let params = fusion::SummonEff {
        fusfilter: Some(dark_filter()),
        stage2: if has_cost_card { Some(Arc::new(stage2)) } else { None },
        ..Default::default()
    };
    // Fusion.SummonEffOP(params)(e,tp,eg,ep,ev,re,r,rp,1)
    let mut call = a.clone();
    call.chk = Some(1);
    fusion::summon_eff_op_call(d, &params, &call);
}
// function s.stage2(e,tc,tp,sg,chk)
fn stage2(d: &mut Duel, e: EffectId, _tc: Option<CardId>, tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==1 and e:IsHasType(EFFECT_TYPE_ACTIVATE) and e:GetLabelObject() then
    if chk == 1 && effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        if let Some(cost_card) = effect::get_label_object(d, e).as_card() {
            // local lp=e:GetLabelObject():GetAttack()
            let lp = card::get_attack(d, cost_card);
            // if lp>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
            if lp > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Recover(tp,lp,REASON_EFFECT)
                duel::recover(d, tp, lp as i64, REASON_EFFECT, false, None);
            }
            // e:SetLabelObject(nil)
            effect::set_label_object(d, e, Value::Nil);
        }
    }
}
