//! Dark Time Wizard
use super::prelude::*;

pub const ID: u32 = 40235813;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects (but you can only use each effect of "Dark Time Wizard" once per turn);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(TIMING_END_PHASE,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, TIMING_END_PHASE, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.deckthfilter(c)
fn deckthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:ListsCode(id) and not c:IsCode(id) and c:IsAbleToHand()
    card::lists_code(d, c, &[ID]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local b1=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(s.deckthfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = !duel::has_flag_effect(d, tp, ID, None) && duel::is_existing_matching_card(d, Some(&deckthfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,LOCATION_MZONE)
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, LOCATION_MZONE);
    // local b2=not Duel.HasFlagEffect(tp,id+1) and #g>0
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None) && !g.is_empty();
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, e, &[op.unwrap_or(0) as i64]);
    if op == Some(1) {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
    } else {
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_COIN+CATEGORY_DESTROY+CATEGORY_DAMAGE)
        effect::set_category(d, e, CATEGORY_COIN + CATEGORY_DESTROY + CATEGORY_DAMAGE);
        // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, tp, 1);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,1,1-tp,0)
        duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, 1 - tp, 0);
    }
    true
}
// function s.gythfilter(c)
fn gythfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(id) and c:IsAbleToHand()
    card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.deckthfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&deckthfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,3))
        aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
        effect::set_operation(d, e1, Some(operation(end_phase_op)));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    } else if op == 2 {
        // if Duel.CallCoin(tp) then
        if duel::call_coin(d, tp) {
            // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
            let g = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
            // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>0 then
            if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
                // local dam=Duel.GetOperatedGroup():GetSum(Card.GetBaseAttack)
                let og = duel::get_operated_group(d);
                let dam = group::get_sum(d, &og, &|d, x| card::get_base_attack(d, x) as i64);
                // if dam>0 then
                if dam > 0 {
                    // Duel.Damage(1-tp,dam/2,REASON_EFFECT)
                    duel::damage(d, 1 - tp, lua_int(dam as f64 / 2.0), REASON_EFFECT, false, None);
                }
            }
        } else {
            // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
            let g = duel::get_field_group(d, tp, LOCATION_MZONE, 0);
            // if #g>0 then
            if !g.is_empty() {
                // Duel.Destroy(g,REASON_EFFECT)
                duel::destroy(d, g, REASON_EFFECT, None, None);
            }
        }
    }
}
// function(e,tp,eg,ep,ev,re,r,rp) (the End Phase effect of the first option)
fn end_phase_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.gythfilter),tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c| gythfilter(d, c))), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
