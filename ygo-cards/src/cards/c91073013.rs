//! Vanquish Soul Caesar Valius
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 91073013;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VANQUISH_SOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return 1 non-Dragon "Vanquish Soul" monster to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetCost(s.opccost)
    effect::set_cost(d, e1, Some(cost_target(opccost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make this card unaffected by effects, OR destroy 1 other card on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.opccost)
    effect::set_cost(d, e2, Some(cost_target(opccost)));
    // e2:SetTarget(s.vstg)
    effect::set_target(d, e2, Some(cost_target(vstg)));
    // e2:SetOperation(s.vsop)
    effect::set_operation(d, e2, Some(operation(vsop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.opccost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn opccost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_CHAIN, 0, 1, 0);
    true
}
/// `s.thfilter(c,tp)`
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_VANQUISH_SOUL) and not c:IsRace(RACE_DRAGON) and c:IsFaceup()
    //     and c:IsAbleToHand() and Duel.GetMZoneCount(tp,c)>0
    card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16])
        && !card::is_race(d, c, RACE_DRAGON)
        && card::is_faceup(d, c)
        && card::is_able_to_hand(d, c, None)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.thfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && thfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_MZONE,0,1,nil,tp)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| thfilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| thfilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0
    //     and tc:IsLocation(LOCATION_HAND) and c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_HAND)
        && card::is_relate_to_effect(d, c, a.e)
    {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.vscostfilter(c,att)`
fn vscostfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsAttribute(att) and not c:IsPublic()
    card::is_attribute(d, c, att) && !card::is_public(d, c)
}
/// `s.vsrescon(sg)`
fn vsrescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_EARTH)>0
    //     and sg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_DARK)>0
    //     and sg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_FIRE)>0
    let res = group::filter_count(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_EARTH), NO_EX) > 0
        && group::filter_count(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_DARK), NO_EX) > 0
        && group::filter_count(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_FIRE), NO_EX) > 0;
    (res, false)
}
/// `s.vstg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn vstg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local cg1=Duel.GetMatchingGroup(s.vscostfilter,tp,LOCATION_HAND,0,nil,ATTRIBUTE_EARTH)
    let cg1 = duel::get_matching_group(d, Some(&|d, c| vscostfilter(d, c, ATTRIBUTE_EARTH)), tp, LOCATION_HAND, 0, NO_EX);
    // local b1=#cg1>0
    let b1 = cg1.len() > 0;
    // local cg2=cg1+Duel.GetMatchingGroup(s.vscostfilter,tp,LOCATION_HAND,0,nil,ATTRIBUTE_DARK|ATTRIBUTE_FIRE)
    let mut cg2 = cg1.clone();
    let cg_df = duel::get_matching_group(d, Some(&|d, c| vscostfilter(d, c, ATTRIBUTE_DARK | ATTRIBUTE_FIRE)), tp, LOCATION_HAND, 0, NO_EX);
    group::add_card(&mut cg2, &cg_df);
    // local dg=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let dg = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    // local b2=#dg>0 and aux.SelectUnselectGroup(cg2,e,tp,1,3,s.vsrescon,0)
    let rs: aux::Rescon = &vsrescon;
    let b2 = dg.len() > 0
        && matches!(aux::select_unselect_group(d, &cg2, Some(a.e), tp, Some(1), Some(3), Some(rs), 0, tp, None, None, None, false), Err(true));
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |v| v as i64)]);
    // if op==1 then
    if op == Some(1) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local g=cg1:Select(tp,1,1,nil)
        let g = group::select(d, &cg1, tp, 1, 1, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    // elseif op==2 then
    } else if op == Some(2) {
        // local g=aux.SelectUnselectGroup(cg2,e,tp,1,3,s.vsrescon,1,tp,HINTMSG_CONFIRM,s.vsrescon)
        let g = aux::select_unselect_group(
            d,
            &cg2,
            Some(a.e),
            tp,
            Some(1),
            Some(3),
            Some(rs),
            1,
            tp,
            Some(HINTMSG_CONFIRM as u64),
            Some(rs),
            None,
            false,
        )
        .unwrap_or_default();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 1, 0, 0);
    }
    true
}
/// `s.vsop(e,tp,eg,ep,ev,re,r,rp)`
fn vsop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if op==1 then
    if op == 1 {
        // if c:IsFacedown() or not c:IsRelateToEffect(e) then return end
        if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,4))
        effect::set_description(d, e1, aux::stringid(ID, 4));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_IMMUNE_EFFECT)
        effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetValue(function(e,te) return te:IsActivated() and te:GetOwnerPlayer()~=e:GetHandlerPlayer() end)
        effect::set_value_fn(d, e1, lua_fn(immune_value));
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    // elseif op==2 then
    } else if op == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,c)
        let g = duel::select_matching_card(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into()).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `function(e,te) return te:IsActivated() and te:GetOwnerPlayer()~=e:GetHandlerPlayer() end`
fn immune_value(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    // te:IsActivated() and te:GetOwnerPlayer()~=e:GetHandlerPlayer()
    if !effect::is_activated(d, te) {
        return false;
    }
    let owner = effect::get_owner_player(d, te);
    owner != effect::get_handler_player(d, e)
}
