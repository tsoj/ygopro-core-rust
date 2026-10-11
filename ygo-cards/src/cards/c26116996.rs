//! Cursed Paper Retribution
use super::prelude::*;

pub const ID: u32 = 26116996;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target 1 monster you control and 1 face-up card your opponent controls; destroy your monster, and if you do, negate the effects of that opponent's card until the end of this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this Set card is sent to the GY: You can target 1 face-up card your opponent controls; negate its effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.negatecon)
    effect::set_condition(d, e2, Some(condition(negatecon)));
    // e2:SetTarget(s.negatetg)
    effect::set_target(d, e2, Some(cost_target(negatetg)));
    // e2:SetOperation(s.negateop)
    effect::set_operation(d, e2, Some(operation(negateop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `Card.GetControler` as the value of `aux.dpcheck`
fn card_controler_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_controler(d, c) as i64
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&card::is_negatable), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local g=Duel.GetTargetGroup(nil,tp,LOCATION_MZONE,0,nil)+Duel.GetTargetGroup(Card.IsNegatable,tp,0,LOCATION_ONFIELD,nil)
    let mut g = duel::get_target_group(d, None, tp, LOCATION_MZONE, 0, NO_EX);
    let g2 = duel::get_target_group(d, Some(&card::is_negatable), tp, 0, LOCATION_ONFIELD, NO_EX);
    group::add_card(&mut g, &g2);
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),1,tp,HINTMSG_TARGET)
    let dp = aux::dpcheck(card_controler_value);
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&dp), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // local group_to_destroy,group_to_negate=tg:Split(Card.IsControler,nil,tp)
    let (group_to_destroy, group_to_negate) = group::split(d, &tg, &|d, c| card::is_controler(d, c, tp), NO_EX);
    // e:GetChainData().group_to_destroy=group_to_destroy
    chain::set_effect_data(d, e, None, chain::field::CARD, Value::Group(group_to_destroy.clone()));
    // e:GetChainData().group_to_negate=group_to_negate
    chain::set_effect_data(d, e, None, chain::field::CARD + 1, Value::Group(group_to_negate.clone()));
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,group_to_destroy,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(group_to_destroy), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,group_to_negate,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(group_to_negate), 1, tp, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    let group_to_destroy = chain::effect_data(d, a.e, None, chain::field::CARD).as_group().cloned().nil();
    let group_to_negate = chain::effect_data(d, a.e, None, chain::field::CARD + 1).as_group().cloned().nil();
    // local destroy_tc=(tg&(e:GetChainData().group_to_destroy)):GetFirst()
    let destroy_tc = group::get_first(d, &group::intersection(&tg, &group_to_destroy));
    // local negate_tc=(tg&(e:GetChainData().group_to_negate)):GetFirst()
    let negate_tc = group::get_first(d, &group::intersection(&tg, &group_to_negate));
    // if destroy_tc and destroy_tc:IsControler(tp) and Duel.Destroy(destroy_tc,REASON_EFFECT)>0
    //     and negate_tc and negate_tc:IsControler(1-tp) and negate_tc:IsNegatable() then
    if let Some(destroy_tc) = destroy_tc {
        if card::is_controler(d, destroy_tc, tp) && duel::destroy(d, destroy_tc, REASON_EFFECT, None, None) > 0 {
            if let Some(negate_tc) = negate_tc {
                if card::is_controler(d, negate_tc, 1 - tp) && card::is_negatable(d, negate_tc) {
                    // Negate the effects of that opponent's card until the end of this turn
                    // negate_tc:NegateEffects(e:GetHandler(),RESET_PHASE|PHASE_END,true)
                    let h = effect::get_handler(d, a.e).nil();
                    card::negate_effects(d, negate_tc, h, Some(RESET_PHASE | PHASE_END), true, None);
                }
            }
        }
    }
    let _ = tp;
}
// function s.negatecon(e,tp,eg,ep,ev,re,r,rp)
fn negatecon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousPosition(POS_FACEDOWN)
    card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_previous_position(d, c, POS_FACEDOWN)
}
// function s.negatetg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn negatetg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) and chkc:IsNegatable() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp) && card::is_negatable(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_negatable), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&card::is_negatable), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, tp, 0);
    true
}
// function s.negateop(e,tp,eg,ep,ev,re,r,rp)
fn negateop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // Negate its effects
        // tc:NegateEffects(e:GetHandler(),nil,true)
        let h = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, tc, h, None, true, None);
    }
}
