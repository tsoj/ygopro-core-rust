//! Skull Lair
use super::prelude::*;

pub const ID: u32 = 6733059;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e0, 0, Some(TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Destroy 1 face-up monster on the field whose Level is equal to the number of the cards you banished
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c)
    card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, false, false)
}
/// `s.rescon(fg)`: `function(sg,e,tp,mg) return fg:IsExists(Card.IsLevel,1,sg,#sg),not fg:IsExists(Card.IsLevelAbove,1,sg,#sg) end`
fn rescon(fg: CardSet) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, _tp, _mg, _c| {
        let n = sg.len() as u32;
        let a = group::is_exists(d, &fg, &|d, c| card::is_level(d, c, &[n]), 1, sg.into());
        let b = group::is_exists(d, &fg, &|d, c| card::is_level_above(d, c, n), 1, sg.into());
        (a, !b)
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local cg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let cg = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local fg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsLevelBelow,#cg),tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let n = cg.len() as u32;
    let ff = aux::faceup_filter(move |d, c| card::is_level_below(d, c, n));
    let fg = duel::get_matching_group(d, Some(&ff), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(cg,e,tp,nil,nil,s.rescon(fg),0) end
    if a.chk == Some(0) {
        let rc = rescon(fg.clone());
        return matches!(
            aux::select_unselect_group(d, &cg, Some(a.e), a.tp, None, None, Some(&rc), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    // local rg=aux.SelectUnselectGroup(cg,e,tp,nil,nil,s.rescon(fg),1,tp,HINTMSG_REMOVE,s.rescon(fg))
    let rc = rescon(fg.clone());
    let rg = aux::select_unselect_group(
        d,
        &cg,
        Some(a.e),
        a.tp,
        None,
        None,
        Some(&rc),
        1,
        a.tp,
        Some(HINTMSG_REMOVE as u64),
        Some(&rc),
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // Duel.SetTargetParam(#rg)
    let m = rg.len() as u32;
    duel::set_target_param(d, m);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,fg:Filter(Card.IsLevel,rg,#rg),1,0,0)
    let targets = group::filter(d, &fg, &|d, c| card::is_level(d, c, &[m]), (&rg).into());
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(targets), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local lv=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let lv = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if lv==0 then return end
    if lv == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsLevel,lv),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let ff = aux::faceup_filter(move |d, c| card::is_level(d, c, &[lv as u32]));
    let g = duel::select_matching_card(d, a.tp, Some(&ff), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
