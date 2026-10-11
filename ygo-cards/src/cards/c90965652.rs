//! Dimer Synthesis
//! 二量合成
use super::prelude::*;

pub const ID: u32 = 90965652;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[65959844, 25669282],
    listed_series: &[SET_CHEMICRITTER],
    listed_card_types: &[TYPE_GEMINI],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.codefilter(c,code)`
fn codefilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsCode(code) and c:IsAbleToHand()
    card::is_code(d, c, &[code]) && card::is_able_to_hand(d, c, None)
}
/// `s.chemfilter(c)`
fn chemfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_CHEMICRITTER) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_CHEMICRITTER as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local f1=Duel.IsExistingMatchingCard(s.codefilter,tp,LOCATION_DECK,0,1,nil,65959844)
    let f1 = duel::is_existing_matching_card(d, Some(&|d, c| codefilter(d, c, 65959844)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local f2=Duel.IsExistingMatchingCard(s.codefilter,tp,LOCATION_DECK,0,1,nil,25669282)
    //     and Duel.IsExistingMatchingCard(s.chemfilter,tp,LOCATION_DECK,0,1,nil)
    let f2 = duel::is_existing_matching_card(d, Some(&|d, c| codefilter(d, c, 25669282)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&chemfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then return f1 or f2 end
    if a.chk == Some(0) {
        return f1 || f2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {f1,aux.Stringid(id,2)},
    //     {f2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(f1, aux::stringid(ID, 2)), (f2, aux::stringid(ID, 3))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,op+1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, (op + 1) as u8, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.threscon(sg)`
fn threscon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return #sg==2 and sg:FilterCount(s.chemfilter,nil)==1
    let res = sg.len() == 2 && group::filter_count(d, sg, &chemfilter, NO_EX) == 1;
    (res, false)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g1=Duel.SelectMatchingCard(tp,s.codefilter,tp,LOCATION_DECK,0,1,1,nil,65959844)
        let g1 = duel::select_matching_card(d, a.tp, Some(&|d, c| codefilter(d, c, 65959844)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g1>0 then
        if !g1.is_empty() {
            // Duel.SendtoHand(g1,nil,REASON_EFFECT)
            duel::send_to_hand(d, g1.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g1)
            duel::confirm_cards(d, 1 - a.tp, g1);
        }
    } else if op == 2 {
        // local g1=Duel.GetMatchingGroup(s.codefilter,tp,LOCATION_DECK,0,nil,25669282)
        let g1 = duel::get_matching_group(d, Some(&|d, c| codefilter(d, c, 25669282)), a.tp, LOCATION_DECK, 0, NO_EX);
        // local g2=Duel.GetMatchingGroup(s.chemfilter,tp,LOCATION_DECK,0,nil)
        let g2 = duel::get_matching_group(d, Some(&chemfilter), a.tp, LOCATION_DECK, 0, NO_EX);
        // if #g1==0 or #g2==0 then return end
        if g1.is_empty() || g2.is_empty() {
            return;
        }
        // local sg=aux.SelectUnselectGroup(g1+g2,e,tp,2,2,s.threscon,1,tp,HINTMSG_ATOHAND)
        let union = group::union(&g1, &g2);
        let sg = aux::select_unselect_group(d, &union, Some(a.e), a.tp, Some(2), Some(2), Some(&threscon), 1, a.tp, Some(HINTMSG_ATOHAND as u64), None, None, false)
            .unwrap_or_default();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - a.tp, sg);
        }
    }
}
/// `s.atkrescon(sg,e,tp,mg)`
fn atkrescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsGeminiStatus,1,nil) and sg:IsExists(Card.IsAttackAbove,1,nil,1)
    let res = group::is_exists(d, sg, &|d, c| card::is_gemini_status(d, c), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::is_attack_above(d, c, 1), 1, NO_EX);
    (res, false)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCanBeEffectTarget,e),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_can_be_effect_target(d, c, Some(a.e))),
        a.tp,
        LOCATION_MZONE,
        0,
        NO_EX,
    );
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,s.atkrescon,0,tp) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&atkrescon), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.atkrescon,1,tp,HINTMSG_TARGET)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&atkrescon), 1, a.tp, Some(HINTMSG_TARGET as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetTargetCards(e)
    let sg = duel::get_target_cards(d, a.e).nil();
    // if #sg~=2 then return end
    if sg.len() != 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,4))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 4));
    // local tc1=sg:FilterSelect(tp,Card.IsAttackAbove,1,1,nil,1):GetFirst()
    let sel = group::filter_select(d, &sg, a.tp, &|d, c| card::is_attack_above(d, c, 1), 1, 1, false, NO_EX).nil();
    let tc1 = group::get_first(d, &sel);
    // if not tc1 or tc1:IsImmuneToEffect(e) then return end
    let Some(tc1) = tc1 else { return };
    if card::is_immune_to_effect(d, tc1, a.e) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e1:SetValue(0)
    effect::set_value(d, e1, 0);
    // tc1:RegisterEffect(e1)
    card::register_effect(d, tc1, e1, false);
    // if not tc1:IsAttack(0) then return end
    if !card::is_attack(d, tc1, &[0]) {
        return;
    }
    // local tc2=(sg-tc1):GetFirst()
    let mut rest = sg.clone();
    group::remove_card(&mut rest, &CardSet::from_card(tc1));
    let Some(tc2) = group::get_first(d, &rest) else { return };
    // if not tc2 or tc2:IsImmuneToEffect(e) then return end
    if card::is_immune_to_effect(d, tc2, a.e) {
        return;
    }
    // tc2:UpdateAttack(tc1:GetBaseAttack(),RESETS_STANDARD_PHASE_END,c)
    let base = card::get_base_attack(d, tc1) as i64;
    card::update_attack(d, tc2, base, Some(RESETS_STANDARD_PHASE_END), Some(c), None);
}
