//! Realm Eulogy
use super::prelude::*;

pub const ID: u32 = 54851325;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_VISAS_STARFROST], listed_series: &[SET_VEDA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 monster you control and negate the effects of 1 of your opponent's monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.negttg)
    effect::set_target(d, e1, Some(cost_target(negttg)));
    // e1:SetOperation(s.negtop)
    effect::set_operation(d, e1, Some(operation(negtop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 card you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.descond)
    effect::set_condition(d, e2, Some(condition(descond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsControler(tp) or (c:IsControler(1-tp) and c:IsNegatableMonster() and c:IsType(TYPE_EFFECT)))
    //   and c:IsCanBeEffectTarget(e)
    (card::is_controler(d, c, tp) || (card::is_controler(d, c, 1 - tp) && card::is_negatable_monster(d, c) && card::is_type(d, c, TYPE_EFFECT)))
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsControler,nil,tp)==1
    let res = group::filter_count(d, sg, &|d, c| card::is_controler(d, c, tp), NO_EX) == 1;
    (res, false)
}
/// `s.negttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn negttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local rg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e,tp)
    let rg = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
    let tg = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // local dg,ng=tg:Split(Card.IsControler,nil,tp)
    let (dg, ng) = group::split(d, &tg, &|d, c| card::is_controler(d, c, tp), NO_EX);
    // e:SetLabelObject(dg:GetFirst())
    let first = group::get_first(d, &dg);
    effect::set_label_object(d, e, first.map_or(Value::Nil, Value::Card));
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,ng,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(ng.clone()), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,ng,1,0,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, Some(ng), 1, 0, 0);
    true
}
/// `s.negtop(e,tp,eg,ep,ev,re,r,rp)`
fn negtop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local dc=g:GetFirst()
    let items = group::iter(d, &g);
    let mut dc = items.first().copied();
    // local negc=g:GetNext()
    let mut negc = items.get(1).copied();
    // if negc==e:GetLabelObject() then dc,negc=negc,dc end
    let label = effect::get_label_object(d, a.e).as_card();
    if negc == label {
        std::mem::swap(&mut dc, &mut negc);
    }
    // if dc and dc:IsControler(tp) and Duel.Destroy(dc,REASON_EFFECT)>0
    //   and negc and negc:IsControler(1-tp) and negc:IsFaceup() and negc:IsCanBeDisabledByEffect(e) then
    if let Some(dc) = dc {
        if card::is_controler(d, dc, tp) && duel::destroy(d, dc, REASON_EFFECT, None, None) > 0 {
            if let Some(negc) = negc {
                if card::is_controler(d, negc, 1 - tp) && card::is_faceup(d, negc) && card::is_can_be_disabled_by_effect(d, negc, a.e, true) {
                    // negc:NegateEffects(e:GetHandler(),RESET_PHASE|PHASE_END)
                    let h = effect::get_handler(d, a.e).nil();
                    card::negate_effects(d, negc, h, Some(RESET_PHASE | PHASE_END), false, None);
                    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_VISAS_STARFROST),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
                    //   and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
                    if duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_code(d, x, &[CARD_VISAS_STARFROST]))), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
                        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
                    {
                        // Duel.AdjustInstantly()
                        duel::adjust_instantly(d, None);
                        // Duel.BreakEffect()
                        duel::break_effect(d);
                        // Duel.Destroy(negc,REASON_EFFECT)
                        duel::destroy(d, negc, REASON_EFFECT, None, None);
                    }
                }
            }
        }
    }
}
/// `s.vedafilter(c)`
fn vedafilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VEDA) and c:IsOriginalType(TYPE_MONSTER)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VEDA as u16]) && card::is_original_type(d, c, TYPE_MONSTER)
}
/// `s.descond(e,tp,eg,ep,ev,re,r,rp)`
fn descond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.vedafilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    duel::is_existing_matching_card(d, Some(&vedafilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, None, tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
