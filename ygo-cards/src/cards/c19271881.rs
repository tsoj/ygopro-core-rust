//! Brightest, Blazing, Branded King
use super::prelude::*;

pub const ID: u32 = 19271881;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate the effects of cards on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add itself to the hand during the End Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(TIMING_END_PHASE,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, TIMING_END_PHASE, Some(TIMING_END_PHASE));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(_,tp) return Duel.HasFlagEffect(tp,id) and Duel.IsPhase(PHASE_END) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Check for Fusion Monsters sent to the GY
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_TO_GRAVE)
        effect::set_code(d, ge1, EVENT_TO_GRAVE);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `function(_,tp) return Duel.HasFlagEffect(tp,id) and Duel.IsPhase(PHASE_END) end` (condition of e2)
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::has_flag_effect(d, a.tp, ID, None) && duel::is_phase(d, PHASE_END)
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // for tc in eg:Iter() do
    for tc in group::iter(d, &a.eg()) {
        // if tc:IsType(TYPE_FUSION) then
        if card::is_type(d, tc, TYPE_FUSION) {
            // Duel.RegisterFlagEffect(tc:GetControler(),id,RESET_PHASE|PHASE_END,0,1)
            let p = card::get_controler(d, tc);
            duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    }
}
/// `s.cfilter(c,sc)`
fn cfilter(d: &mut Duel, c: CardId, sc: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_FUSION) and c:ListsCodeAsMaterial(CARD_ALBAZ)
    //     and Duel.IsExistingMatchingCard(Card.IsNegatable,0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,Group.FromCards(c,sc))
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_FUSION)
        && card::lists_code_as_material(d, c, &[CARD_ALBAZ])
        && {
            let ex = CardSet::from_iter([c, sc]);
            duel::is_existing_matching_card(d, Some(&|d, cc| card::is_negatable(d, cc)), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, (&ex).into())
        }
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, cc| cfilter(d, cc, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,c)
    let g = duel::get_matching_group(d, Some(&|d, cc| card::is_negatable(d, cc)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,(#g-1),0,0)
    let count = (g.len() as i64 - 1) as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g), count, 0, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local exc=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE,0,1,1,nil,c)
    let mut exc = duel::select_matching_card(d, tp, Some(&|d, cc| cfilter(d, cc, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #exc==0 then return end
    if exc.is_empty() {
        return;
    }
    // Duel.HintSelection(exc,true)
    duel::hint_selection(d, exc.clone(), true);
    // local g=Duel.GetMatchingGroup(Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,exc:AddCard(c))
    group::add_card(&mut exc, &CardSet::from_card(c));
    let g = duel::get_matching_group(d, Some(&|d, cc| card::is_negatable(d, cc)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, (&exc).into());
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
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
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetValue(RESET_TURN_SET)
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,c)
        duel::confirm_cards(d, 1 - a.tp, c);
    }
}
