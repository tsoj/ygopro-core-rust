//! Vanquish Soul Heavy Borger
//! ＶＳヘヴィ・ボーガー
use super::prelude::*;

pub const ID: u32 = 92895501;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VANQUISH_SOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return 1 non-Machine "Vanquish Soul" monster to the hand
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
    // Draw 1 card, OR inflict 1500 damage to the opponent
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
    // e2:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END));
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
// function s.opccost(e,tp,eg,ep,ev,re,r,rp,chk)
fn opccost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_CHAIN, 0, 1, 0);
    true
}
// function s.thfilter(c,tp)
//     return c:IsSetCard(SET_VANQUISH_SOUL) and not c:IsRace(RACE_MACHINE) and c:IsFaceup()
//         and c:IsAbleToHand() and Duel.GetMZoneCount(tp,c)>0
// end
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16])
        && !card::is_race(d, c, RACE_MACHINE)
        && card::is_faceup(d, c)
        && card::is_able_to_hand(d, c, None)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.thfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_MZONE,0,1,nil,tp)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| thfilter(d, x, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, x| thfilter(d, x, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
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
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.vscostfilter(c,att) return c:IsAttribute(att) and not c:IsPublic() end
fn vscostfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    card::is_attribute(d, c, att) && !card::is_public(d, c)
}
// function s.vsrescon(sg)
//     return sg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_EARTH)>0
//         and sg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_FIRE)>0
// end
fn vsrescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let r = group::filter_count(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_EARTH), NO_EX) > 0
        && group::filter_count(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_FIRE), NO_EX) > 0;
    (r, false)
}
// function s.vstg(e,tp,eg,ep,ev,re,r,rp,chk)
fn vstg(d: &mut Duel, a: &EffArgs) -> bool {
    // local cg1=Duel.GetMatchingGroup(s.vscostfilter,tp,LOCATION_HAND,0,nil,ATTRIBUTE_DARK)
    let cg1 = duel::get_matching_group(d, Some(&|d, c| vscostfilter(d, c, ATTRIBUTE_DARK)), a.tp, LOCATION_HAND, 0, NO_EX);
    // local b1=#cg1>0 and Duel.IsPlayerCanDraw(tp,1)
    let b1 = !cg1.is_empty() && duel::is_player_can_draw(d, a.tp, 1);
    // local cg2=Duel.GetMatchingGroup(s.vscostfilter,tp,LOCATION_HAND,0,nil,ATTRIBUTE_EARTH|ATTRIBUTE_FIRE)
    let cg2 = duel::get_matching_group(d, Some(&|d, c| vscostfilter(d, c, ATTRIBUTE_EARTH | ATTRIBUTE_FIRE)), a.tp, LOCATION_HAND, 0, NO_EX);
    let rs: aux::Rescon = &vsrescon;
    // local b2=aux.SelectUnselectGroup(cg2,e,tp,1,2,s.vsrescon,0)
    let b2 = matches!(
        aux::select_unselect_group(d, &cg2, Some(a.e), a.tp, Some(1), Some(2), Some(rs), 0, a.tp, None, None, None, false),
        Err(true)
    );
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
        // local g=cg1:Select(tp,1,1,nil)
        let g = group::select(d, &cg1, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // e:SetCategory(CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_DRAW);
        // Duel.SetTargetPlayer(tp)
        duel::set_target_player(d, a.tp);
        // Duel.SetTargetParam(1)
        duel::set_target_param(d, 1);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    } else if op == 2 {
        // local g=aux.SelectUnselectGroup(cg2,e,tp,1,2,s.vsrescon,1,tp,HINTMSG_CONFIRM,s.vsrescon)
        let g = aux::select_unselect_group(
            d,
            &cg2,
            Some(a.e),
            a.tp,
            Some(1),
            Some(2),
            Some(rs),
            1,
            a.tp,
            Some(HINTMSG_CONFIRM as u64),
            Some(rs),
            None,
            false,
        )
        .unwrap_or_default();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // e:SetCategory(CATEGORY_DAMAGE)
        effect::set_category(d, a.e, CATEGORY_DAMAGE);
        // Duel.SetTargetPlayer(1-tp)
        duel::set_target_player(d, 1 - a.tp);
        // Duel.SetTargetParam(1500)
        duel::set_target_param(d, 1500);
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1500)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1500);
    }
    true
}
// function s.vsop(e,tp,eg,ep,ev,re,r,rp)
fn vsop(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let amount = info[1].int();
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Duel.Draw(p,d,REASON_EFFECT)
        duel::draw(d, p, amount as u32, REASON_EFFECT);
    } else if op == 2 {
        // Duel.Damage(p,d,REASON_EFFECT)
        duel::damage(d, p, amount, REASON_EFFECT, false, None);
    }
}
