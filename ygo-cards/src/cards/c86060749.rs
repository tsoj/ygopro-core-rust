//! Stall Turn
use super::prelude::*;

pub const ID: u32 = 86060749;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if not Duel.IsChainNegatable(ev) or not (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE)) then return false end
    let re = a.re.nil();
    if !duel::is_chain_negatable(d, a.ev as u8) || !(effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)) {
        return false;
    }
    // if re:IsHasCategory(CATEGORY_DISABLE_SUMMON) then return true end
    if effect::is_has_category(d, re, CATEGORY_DISABLE_SUMMON) {
        return true;
    }
    // local ex,tg,ct=Duel.GetOperationInfo(ev,CATEGORY_NEGATE)
    let info = duel::get_operation_info(d, a.ev as u32, CATEGORY_NEGATE);
    // if not ex then return false end
    let Some(info) = info else {
        return false;
    };
    let ct = info.op_count;
    if ct > 1 {
        // for i=1,ev-1 do
        for i in 1..a.ev {
            // local ce=Duel.GetChainInfo(ev-i,CHAININFO_TRIGGERING_EFFECT)
            let ce = duel::get_chain_info(d, (a.ev - i) as u8, &[CHAININFO_TRIGGERING_EFFECT]).into_iter().next().and_then(|v| v.as_effect());
            // if ce and ce:IsSpellTrapEffect() and ce:IsHasType(EFFECT_TYPE_ACTIVATE) then
            if let Some(ce) = ce {
                if effect::is_spell_trap_effect(d, ce) && effect::is_has_type(d, ce, EFFECT_TYPE_ACTIVATE) {
                    return true;
                }
            }
        }
        false
    } else if ct == 1 {
        // local ce=Duel.GetChainInfo(ev-1,CHAININFO_TRIGGERING_EFFECT)
        // return ce and ce:IsSpellTrapEffect() and ce:IsHasType(EFFECT_TYPE_ACTIVATE)
        let ce = duel::get_chain_info(d, (a.ev - 1) as u8, &[CHAININFO_TRIGGERING_EFFECT]).into_iter().next().and_then(|v| v.as_effect());
        match ce {
            Some(ce) => effect::is_spell_trap_effect(d, ce) && effect::is_has_type(d, ce, EFFECT_TYPE_ACTIVATE),
            None => false,
        }
    } else {
        false
    }
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetDecktopGroup(tp,3)
    let g = duel::get_decktop_group(d, a.tp, 3);
    // if chk==0 then return g:FilterCount(Card.IsAbleToRemoveAsCost,nil,POS_FACEDOWN)==3 end
    if a.chk == Some(0) {
        return group::filter_count(d, &g, &|d, c| card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8)), NO_EX) == 3;
    }
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // Duel.Remove(g,POS_FACEDOWN,REASON_COST)
    duel::remove(d, g, POS_FACEDOWN as u8, REASON_COST, None, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsAbleToDeck() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_able_to_deck(d, rh) && card::is_relate_to_effect(d, rh, re) {
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, effect::get_handler(d, a.re.nil()).nil(), a.re.nil()) {
        // Duel.SendtoDeck(eg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, a.eg(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
