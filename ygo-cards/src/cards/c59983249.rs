//! Monarchic Perfection
use super::prelude::*;

pub const ID: u32 = 59983249;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MONARCH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let eg = a.eg();
        group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - a.tp), 1, NO_EX)
    })));
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
    // return c:IsSetCard(SET_MONARCH) and c:IsSpellTrap() and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_MONARCH as u16]) && card::is_spell_trap(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(-100)
    effect::set_label(d, a.e, &[-100]);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.effcostfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| effcostfilter(d, c)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.effcostfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| effcostfilter(d, c)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // local cost_skip=e:GetLabel()~=-100
    let cost_skip = effect::get_label(d, a.e) != -100;
    // Destroy 1 monster on the field
    // local b1=(cost_skip or not Duel.HasFlagEffect(tp,id)) and Duel.IsExistingMatchingCard(nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let b1 = (cost_skip || !duel::has_flag_effect(d, tp, ID, None))
        && duel::is_existing_matching_card(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    // Destroy up to 2 Spells/Traps on the field
    // local b2=(cost_skip or not Duel.HasFlagEffect(tp,id+1)) and Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    let b2 = (cost_skip || !duel::has_flag_effect(d, tp, ID + 1, None))
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    // Discard 1 random card from your opponent's hand
    // local b3=(cost_skip or not Duel.HasFlagEffect(tp,id+2)) and Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,0,LOCATION_HAND,1,nil,REASON_EFFECT)
    let b3 = (cost_skip || !duel::has_flag_effect(d, tp, ID + 2, None))
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), tp, 0, LOCATION_HAND, 1, NO_EX);
    // Destroy 1 face-down card on the field
    // local b4=(cost_skip or not Duel.HasFlagEffect(tp,id+3)) and Duel.IsExistingMatchingCard(Card.IsFacedown,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler())
    let b4 = (cost_skip || !duel::has_flag_effect(d, tp, ID + 3, None))
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
    // Place 1 card on the field on top of the Deck
    // local b5=(cost_skip or not Duel.HasFlagEffect(tp,id+4)) and Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    let b5 = (cost_skip || !duel::has_flag_effect(d, tp, ID + 4, None))
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    // Banish 1 card on the field
    // local b6=(cost_skip or not Duel.HasFlagEffect(tp,id+5)) and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    let b6 = (cost_skip || !duel::has_flag_effect(d, tp, ID + 5, None))
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    // if chk==0 then e:SetLabel(0) return Duel.IsExistingMatchingCard(Card.IsTributeSummoned,tp,LOCATION_MZONE,0,1,nil)
    //     and (b1 or b2 or b3 or b4 or b5 or b6) end
    if a.chk == Some(0) {
        effect::set_label(d, a.e, &[0]);
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_tribute_summoned(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && (b1 || b2 || b3 || b4 || b5 || b6);
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)},{b3,aux.Stringid(id,3)},
    //     {b4,aux.Stringid(id,4)},{b5,aux.Stringid(id,5)},{b6,aux.Stringid(id,6)})
    let op = duel::select_effect(
        d,
        tp,
        &[
            (b1, aux::stringid(ID, 1)),
            (b2, aux::stringid(ID, 2)),
            (b3, aux::stringid(ID, 3)),
            (b4, aux::stringid(ID, 4)),
            (b5, aux::stringid(ID, 5)),
            (b6, aux::stringid(ID, 6)),
        ],
    );
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |v| v as i64)]);
    match op {
        // Destroy 1 monster on the field
        Some(1) => {
            // e:SetCategory(CATEGORY_DESTROY)
            effect::set_category(d, a.e, CATEGORY_DESTROY);
            // if not cost_skip then Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1) end
            if !cost_skip {
                duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
            }
            // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
            let g = duel::get_matching_group(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
            // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
            duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
        }
        // Destroy up to 2 Spells/Traps on the field
        Some(2) => {
            effect::set_category(d, a.e, CATEGORY_DESTROY);
            if !cost_skip {
                duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
            }
            let g = duel::get_matching_group(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
            duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
        }
        // Discard 1 random card from your opponent's hand
        Some(3) => {
            effect::set_category(d, a.e, CATEGORY_HANDES);
            if !cost_skip {
                duel::register_flag_effect(d, tp, ID + 2, RESET_PHASE | PHASE_END, 0, 1, 0);
            }
            duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - tp, 1);
        }
        // Destroy 1 face-down card on the field
        Some(4) => {
            effect::set_category(d, a.e, CATEGORY_DESTROY);
            if !cost_skip {
                duel::register_flag_effect(d, tp, ID + 3, RESET_PHASE | PHASE_END, 0, 1, 0);
            }
            let g = duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
            duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
        }
        // Place 1 card on the field on top of the Deck
        Some(5) => {
            effect::set_category(d, a.e, CATEGORY_TODECK);
            if !cost_skip {
                duel::register_flag_effect(d, tp, ID + 4, RESET_PHASE | PHASE_END, 0, 1, 0);
            }
            duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_ONFIELD as i32);
        }
        // Banish 1 card on the field
        Some(6) => {
            effect::set_category(d, a.e, CATEGORY_REMOVE);
            if !cost_skip {
                duel::register_flag_effect(d, tp, ID + 5, RESET_PHASE | PHASE_END, 0, 1, 0);
            }
            duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_ONFIELD as i32);
        }
        _ => {}
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Destroy 1 monster on the field
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, tp, None, tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    } else if op == 2 {
        // Destroy up to 2 Spells/Traps on the field
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 2, false, NO_EX).nil();
        if !g.is_empty() {
            duel::hint_selection(d, g.clone(), true);
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    } else if op == 3 {
        // Discard 1 random card from your opponent's hand
        // local hg=Duel.GetMatchingGroup(Card.IsDiscardable,tp,0,LOCATION_HAND,nil,REASON_EFFECT)
        let hg = duel::get_matching_group(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), tp, 0, LOCATION_HAND, NO_EX);
        // if #hg>0 then
        if !hg.is_empty() {
            // local g=hg:RandomSelect(tp,1,1,nil)
            let g = group::random_select(d, &hg, tp, 1);
            // Duel.SendtoGrave(g,REASON_DISCARD|REASON_EFFECT)
            duel::send_to_grave(d, g, REASON_DISCARD | REASON_EFFECT, None, None);
        }
    } else if op == 4 {
        // Destroy 1 face-down card on the field
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::hint_selection(d, g.clone(), true);
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    } else if op == 5 {
        // Place 1 card on the field on top of the Deck
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_deck(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::hint_selection(d, g.clone(), true);
            duel::send_to_deck(d, g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
        }
    } else if op == 6 {
        // Banish 1 card on the field
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::hint_selection(d, g.clone(), true);
            duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
