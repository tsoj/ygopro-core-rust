//! Queen of the Blazing Domain
use super::prelude::*;

pub const ID: u32 = 72162751;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be destroyed by effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy up to 2 cards you control, including a face-up Spell/Trap card, and if you, Special Summon this card from hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(aux.FaceupFilter(Card.IsSpellTrap),1,nil) and Duel.GetMZoneCount(tp,sg)>0
    let res = group::is_exists(d, sg, &aux::faceup_filter(|d, c| card::is_spell_trap(d, c)), 1, NO_EX)
        && duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0;
    (res, false)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsCanBeEffectTarget,tp,LOCATION_ONFIELD,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_can_be_effect_target(d, x, Some(a.e))), a.tp, LOCATION_ONFIELD, 0, NO_EX);
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    if a.chk == Some(0) {
        // and aux.SelectUnselectGroup(g,e,tp,1,2,s.rescon,0,tp) end
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(2), Some(&rescon), 0, a.tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,1,2,s.rescon,1,tp,HINTMSG_DESTROY,s.rescon)
    let tg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_DESTROY as u64), Some(&rescon), None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,#tg,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(tg.clone()), tg.len() as u8, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local ct=Duel.Destroy(tg,REASON_EFFECT)
    let ct = duel::destroy(d, tg, REASON_EFFECT, None, None);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP) then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon_step(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
        // if ct==1 then
        if ct == 1 {
            // Return this card to the hand during the End Phase of the next turn
            // local turn_ct=Duel.GetTurnCount()
            let turn_ct = duel::get_turn_count(d, None);
            // aux.DelayedOperation(c,PHASE_END,id,e,tp,
            //   function(tc) Duel.SendtoHand(tc,nil,REASON_EFFECT) end,
            //   function(tc) return Duel.GetTurnCount()==turn_ct+1 end,
            //   nil,2,aux.Stringid(id,1))
            aux::delayed_operation(
                d,
                c,
                PHASE_END,
                ID,
                a.e,
                a.tp,
                Some(std::sync::Arc::new(to_hand_oper)),
                Some(std::sync::Arc::new(move |d: &mut Duel, _ag: &CardSet, _a: &EffArgs| duel::get_turn_count(d, None) == turn_ct + 1)),
                None,
                Some(2),
                Some(aux::stringid(ID, 1)),
                None,
            );
        } else if ct == 2 {
            // Any monster sent from the field to the opponent's GY is banished instead
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(aux.Stringid(id,2))
            effect::set_description(d, e1, aux::stringid(ID, 2));
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_TO_GRAVE_REDIRECT)
            effect::set_code(d, e1, EFFECT_TO_GRAVE_REDIRECT);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
            effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
            // e1:SetTarget(function(e,c) local tp=e:GetHandlerPlayer() return not c:IsOwner(tp) and Duel.IsPlayerCanRemove(tp,c) end)
            effect::set_target(d, e1, Some(effect_card_fn(redirect_target)));
            // e1:SetValue(LOCATION_REMOVED)
            effect::set_value(d, e1, LOCATION_REMOVED as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// function(tc) Duel.SendtoHand(tc,nil,REASON_EFFECT) end
fn to_hand_oper(d: &mut Duel, tc: &CardSet, _a: &EffArgs) -> bool {
    duel::send_to_hand(d, tc.clone(), None, REASON_EFFECT, None);
    true
}
// function(e,c) local tp=e:GetHandlerPlayer() return not c:IsOwner(tp) and Duel.IsPlayerCanRemove(tp,c) end
fn redirect_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let tp = effect::get_handler_player(d, e);
    !card::is_owner(d, c, tp) && duel::is_player_can_remove(d, tp, Some((c, None)))
}
