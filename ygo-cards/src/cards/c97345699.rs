//! Vesper Girsu
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 97345699;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Your opponent can send any number of monsters they control to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_RECOVER+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_RECOVER + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>=Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)+2 end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let opp = duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE);
            let own = duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0);
            opp >= own + 2
        })),
    );
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - tp, LOCATION_MZONE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_RECOVER,nil,1,1-tp,2000)
    duel::set_possible_operation_info(d, 0, CATEGORY_RECOVER, None, 1, 1 - tp, 2000);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_HAND as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_MZONE, NO_EX);
    // if #g>0 and Duel.SelectYesNo(1-tp,aux.Stringid(id,1)) then
    if !g.is_empty() && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(1-tp,Card.IsAbleToGrave,1-tp,LOCATION_MZONE,0,1,#g,nil)
        let g = duel::select_matching_card(d, 1 - tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), 1 - tp, LOCATION_MZONE, 0, 1, g.len() as u16, false, NO_EX)
            .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoGrave(g,REASON_EFFECT,PLAYER_NONE,1-tp)
            duel::send_to_grave(d, g, REASON_EFFECT, Some(PLAYER_NONE as u8), Some(1 - tp));
        }
    }
    // local ct=Duel.GetMatchingGroupCount(nil,tp,0,LOCATION_MZONE,nil)
    let ct = duel::get_matching_group_count(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    // if ct==0 then
    if ct == 0 {
        // Halve your LP
        // Duel.SetLP(tp,Duel.GetLP(tp)//2)
        let lp = duel::get_lp(d, tp).div_euclid(2);
        duel::set_lp(d, tp, lp);
    } else if ct == 1 {
        // Your opponent gains 2000 LP
        // Duel.Recover(1-tp,2000,REASON_EFFECT)
        duel::recover(d, 1 - tp, 2000, REASON_EFFECT, false, None);
    } else if ct == 2 {
        // Banish your opponent's hand face-up until the End Phase
        // local rmg=Duel.GetMatchingGroup(nil,tp,0,LOCATION_HAND,nil)
        let rmg = duel::get_matching_group(d, None, tp, 0, LOCATION_HAND, NO_EX);
        // if #rmg==0 then return end
        if rmg.is_empty() {
            return;
        }
        // aux.RemoveUntil(rmg,POS_FACEUP,REASON_EFFECT,PHASE_END,id,e,tp,function(rg) Duel.SendtoHand(rg,nil,REASON_EFFECT) end)
        aux::remove_until(
            d,
            rmg,
            Some(POS_FACEUP as u8),
            REASON_EFFECT,
            PHASE_END,
            ID,
            a.e,
            tp,
            Some(std::sync::Arc::new(|d: &mut Duel, rg: &CardSet, _a: &EffArgs| {
                duel::send_to_hand(d, rg.clone(), None, REASON_EFFECT, None);
                true
            })),
            None,
            None,
            None,
            None,
            None,
        );
    } else if ct >= 3 {
        // Your opponent cannot activate monster effects this turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(0,1)
        effect::set_target_range(d, e1, 0, 1);
        // e1:SetValue(function(e,re,rp) return re:IsMonsterEffect() end)
        effect::set_value_fn(
            d,
            e1,
            lua_fn(|d: &mut Duel, a: Args| {
                let re = a.v(1).as_effect().nil();
                effect::is_monster_effect(d, re)
            }),
        );
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
