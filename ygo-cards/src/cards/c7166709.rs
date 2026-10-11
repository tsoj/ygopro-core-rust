//! 鉄絃の犠打職人
//! Steel-Stringed Sacrifice
use super::prelude::*;

pub const ID: u32 = 7166709;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCost(s.effcost)
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.effcostfilter(c)`
fn effcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(5) and not c:IsPublic()
    card::is_level_above(d, c, 5) && !card::is_public(d, c)
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.effcostfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&effcostfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.effcostfilter,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
    let sg = duel::select_matching_card(d, a.tp, Some(&effcostfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).unwrap_or_default();
    let rc = group::get_first(d, &sg).nil();
    // Duel.ConfirmCards(1-tp,rc)
    duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(rc));
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // e:SetLabelObject(rc)
    effect::set_label_object(d, a.e, Value::Card(rc));
    // e:SetLabel(rc:GetOriginalCodeRule())
    let code = card::get_original_code_rule(d, rc)[0];
    effect::set_label(d, a.e, &[code as i64]);
    // rc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1))
    card::register_flag_effect(d, rc, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(5) and c:IsAbleToHand()
    card::is_level_above(d, c, 5) && card::is_able_to_hand(d, c, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local b1=c:IsLocation(LOCATION_HAND) and not Duel.HasFlagEffect(tp,id)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let b1 = card::is_location(d, c, LOCATION_HAND)
        && duel::get_flag_effect(d, tp, ID) == 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    // local b2=c:IsLocation(LOCATION_GRAVE) and not Duel.HasFlagEffect(tp,id+1) and c:IsAbleToHand()
    let b2 = card::is_location(d, c, LOCATION_GRAVE)
        && duel::get_flag_effect(d, tp, ID + 1) == 0
        && card::is_able_to_hand(d, c, None);
    // local b3=c:IsLocation(LOCATION_GRAVE) and not Duel.HasFlagEffect(tp,id+2)
    //     and c:IsAbleToRemove() and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,c)
    let b3 = card::is_location(d, c, LOCATION_GRAVE)
        && duel::get_flag_effect(d, tp, ID + 2) == 0
        && card::is_able_to_remove(d, c, None, None, None)
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, c.into());
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)},{b3,aux.Stringid(id,4)})
    let op = duel::select_effect(
        d,
        tp,
        &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3)), (b3, aux::stringid(ID, 4))],
    )
    .nil() as u32;
    // Duel.SetTargetParam(op)
    duel::set_target_param(d, op);
    // Duel.RegisterFlagEffect(tp,id+op-1,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID + op - 1, RESET_PHASE | PHASE_END, 0, 1, 0);
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, tp, 0);
    } else if op == 3 {
        // e:SetCategory(CATEGORY_REMOVE+CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_REMOVE + CATEGORY_TOHAND);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(c)), 1, tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local code=e:GetLabel()
    let code = effect::get_label(d, a.e);
    // local rc=e:GetLabelObject()
    let rc = effect::get_label_object(d, a.e).as_card().nil();
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,5))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 5)), 0, None);
    // Register if the player Normal Summons the revealed monster or a monster with the same original name
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep) return ep==tp and ((rc:HasFlagEffect(id) and eg:IsContains(rc) and rc:IsFaceup())
    //     or eg:IsExists(aux.FaceupFilter(Card.IsOriginalCodeRule,code),1,nil)) end)
    effect::set_condition(
        d,
        e1,
        Some(condition(move |d, a| {
            if a.ep != a.tp {
                return false;
            }
            let eg = a.eg();
            let first = card::get_flag_effect(d, rc, ID) != 0 && eg.contains(rc) && card::is_faceup(d, rc);
            first
                || group::is_exists(
                    d,
                    &eg,
                    &aux::faceup_filter(|d, x| card::is_original_code_rule(d, x, &[code as u32])),
                    1,
                    NO_EX,
                )
        })),
    );
    // e1:SetOperation(function(e) e:SetLabel(1) end)
    effect::set_operation(d, e1, Some(operation(|d, a| effect::set_label(d, a.e, &[1]))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetDescription(aux.Stringid(id,6))
    effect::set_description(d, e2, aux::stringid(ID, 6));
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetCondition(function() return e1:GetLabel()==0 end)
    effect::set_condition(d, e2, Some(condition(move |d, _a| effect::get_label(d, e1) == 0)));
    // e2:SetOperation(function(e,tp) Duel.SetLP(tp,Duel.GetLP(tp)-1000) end)
    effect::set_operation(
        d,
        e2,
        Some(operation(|d, a| {
            let lp = duel::get_lp(d, a.tp);
            duel::set_lp(d, a.tp, lp - 1000);
        })),
    );
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local op=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let op = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    if op == 1 {
        // Special Summon this card from your hand
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    } else if op == 2 {
        // Add this card from your GY to your hand
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,c)
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(c));
    } else if op == 3 {
        // Banish this card from your GY, and if you do, add 1 Level 5 or higher monster from your GY to your hand
        // if Duel.Remove(c,POS_FACEUP,REASON_EFFECT)==0 or not c:IsLocation(LOCATION_REMOVED) then return end
        if duel::remove(d, CardSet::from_card(c), POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 || !card::is_location(d, c, LOCATION_REMOVED) {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).unwrap_or_default();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g, None, REASON_EFFECT, None);
        }
    }
}
