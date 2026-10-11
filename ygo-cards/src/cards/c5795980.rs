//! Strike of the Monarchs
use super::prelude::*;

pub const ID: u32 = 5795980;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // change attributes
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.attg)
    effect::set_target(d, e2, Some(cost_target(attg)));
    // e2:SetOperation(s.atop)
    effect::set_operation(d, e2, Some(operation(atop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.cfilter(c,e,dg)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, dg: &CardSet) -> bool {
    // if c:IsFacedown() or not c:IsLevelAbove(5) or not c:IsNormalSummoned() then return false end
    if card::is_facedown(d, c) || !card::is_level_above(d, c, 5) || !card::is_normal_summoned(d, c) {
        return false;
    }
    // local a=0
    let mut a: i64 = 0;
    // if dg:IsContains(c) then a=1 end
    if dg.contains(c) {
        a = 1;
    }
    // if c:GetEquipCount()==0 then return #dg-a>=1 end
    let _ = e;
    if card::get_equip_count(d, c) == 0 {
        return dg.len() as i64 - a >= 1;
    }
    // local eg=c:GetEquipGroup()
    let eg = card::get_equip_group(d, c);
    // local tc=eg:GetFirst()
    let _ = group::get_first(d, &eg);
    // for tc in aux.Next(eg) do
    for tc in group::iter(d, &eg) {
        // if dg:IsContains(tc) then a=a+1 end
        if dg.contains(tc) {
            a += 1;
        }
    }
    // return #dg-a>=1
    dg.len() as i64 - a >= 1
}
/// `s.tgfilter(c,e)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsNegatable() and c:IsCanBeEffectTarget(e)
    card::is_negatable(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    let h = effect::get_handler(d, e).nil();
    // if chkc then return chkc:IsOnField() and s.tgfilter(chkc,e) and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && tgfilter(d, chkc, e) && chkc != h;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if not Duel.IsPlayerCanDraw(tp,1) then return false end
        if !duel::is_player_can_draw(d, tp, 1) {
            return false;
        }
        // if e:GetLabel()==1 then
        if effect::get_label(d, e) == 1 {
            // e:SetLabel(0)
            effect::set_label(d, e, &[0]);
            // local rg=Duel.GetReleaseGroup(tp)
            let rg = duel::get_release_group(d, tp, false, false, None);
            // local dg=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler(),e)
            let dg = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, e)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
            // return rg:IsExists(s.cfilter,1,e:GetHandler(),e,dg)
            return group::is_exists(d, &rg, &|d, c| cfilter(d, c, e, &dg), 1, h.into());
        } else {
            // return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler())
            return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
        }
    }
    // if e:GetLabel()==1 then
    if effect::get_label(d, e) == 1 {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // local rg=Duel.GetReleaseGroup(tp)
        let rg = duel::get_release_group(d, tp, false, false, None);
        // local dg=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler(),e)
        let dg = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, e)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
        // local sg=rg:FilterSelect(tp,s.cfilter,1,1,e:GetHandler(),e,dg)
        let sg = group::filter_select(d, &rg, tp, &|d, c| cfilter(d, c, e, &dg), 1, 1, false, h.into()).nil();
        // Duel.Release(sg,REASON_COST)
        duel::release(d, sg, REASON_COST, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,e:GetHandler())
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, h.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and ((tc:IsFaceup() and not tc:IsDisabled()) or tc:IsType(TYPE_TRAPMONSTER)) then
    if card::is_relate_to_effect(d, tc, a.e)
        && ((card::is_faceup(d, tc) && !card::is_disabled(d, tc)) || card::is_type(d, tc, TYPE_TRAPMONSTER))
    {
        // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetValue(RESET_TURN_SET)
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // local e3
        // if tc:IsType(TYPE_TRAPMONSTER) then
        let e3 = if card::is_type(d, tc, TYPE_TRAPMONSTER) {
            // e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_DISABLE_TRAPMONSTER)
            effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
            // e3:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e3)
            card::register_effect(d, tc, e3, false);
            Some(e3)
        } else {
            None
        };
        // if tc:IsImmuneToEffect(e1) or tc:IsImmuneToEffect(e2) or (e3 and tc:IsImmuneToEffect(e3)) then return end
        if card::is_immune_to_effect(d, tc, e1) || card::is_immune_to_effect(d, tc, e2) || e3.is_some_and(|e3| card::is_immune_to_effect(d, tc, e3)) {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
}
/// `s.attg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn attg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTRIBUTE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTRIBUTE as u64);
    // local rc=Duel.AnnounceAttribute(tp,1,ATTRIBUTE_ALL)
    let rc = duel::announce_attribute(d, tp, 1, ATTRIBUTE_ALL);
    // Duel.SetTargetParam(rc)
    duel::set_target_param(d, rc);
    true
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, a: &EffArgs) {
    // local rc=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let rc = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
        // e1:SetValue(rc)
        effect::set_value(d, e1, rc as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
