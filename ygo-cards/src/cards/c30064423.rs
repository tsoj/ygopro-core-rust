//! Topologic Blaster Dragon
use super::prelude::*;

pub const ID: u32 = 30064423;

// `s.listed_name={id}` is not declared in INFO: the field name is misspelled in the script and no
// script or core code reads `listed_name` (only this script names it).

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ Effect Monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2)
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_type_ctx(d, x, TYPE_EFFECT, sc, st, Some(tp)))), 2, None, None, None, None);
    // You cannot Summon/Set monsters to any Extra Monster Zone this card points to
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EFFECT_FORCE_MZONE)
    effect::set_code(d, e0, EFFECT_FORCE_MZONE);
    // e0:SetRange(LOCATION_MZONE)
    effect::set_range(d, e0, LOCATION_MZONE);
    // e0:SetTargetRange(1,0)
    effect::set_target_range(d, e0, 1, 0);
    // e0:SetValue(function(e) return ~(e:GetHandler():GetLinkedZone()&ZONES_EMZ) end)
    effect::set_value_fn(d, e0, lua_fn(e0_value));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Apply 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_EMZONE)
    effect::set_range(d, e1, LOCATION_EMZONE);
    // e1:SetCondition(s.effcon)
    effect::set_condition(d, e1, Some(condition(effcon)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function(e) return ~(e:GetHandler():GetLinkedZone()&ZONES_EMZ) end
fn e0_value(d: &mut Duel, a: Args) -> i64 {
    let h = effect::get_handler(d, a.e(0)).nil();
    let zone = card::get_linked_zone(d, h, None);
    !((zone & ZONES_EMZ) as i64)
}
// function s.effcon(e,tp,eg,ep,ev,re,r,rp)
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if eg:IsContains(e:GetHandler()) then return false end
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    if eg.contains(h) {
        return false;
    }
    // local linkg=Duel.GetMatchingGroup(Card.IsLinkMonster,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let linkg = duel::get_matching_group(d, Some(&|d, x| card::is_link_monster(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // for lc in linkg:Iter() do
    for lc in group::iter(d, &linkg) {
        // if aux.zptgroupcon(eg,nil,lc) then return true end
        if aux::zptgroupcon(d, &eg, None, lc, a.tp) {
            return true;
        }
    }
    // return false
    false
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Shuffle all other monsters on the field into the Deck
    // local b1=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_MZONE,LOCATION_MZONE,1,c)
    let b1 = !duel::has_flag_effect(d, a.tp, ID, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_deck(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    // Shuffle all Spells and Traps on the field into the Deck
    // local b2=not Duel.HasFlagEffect(tp,id+1)
    //     and Duel.IsExistingMatchingCard(aux.AND(Card.IsSpellTrap,Card.IsAbleToDeck),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    let b2 = !duel::has_flag_effect(d, a.tp, ID + 1, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_spell_trap(d, x) && card::is_able_to_deck(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    // Look at your opponent's Extra Deck and banish 1 card from it
    // local b3=not Duel.HasFlagEffect(tp,id+2)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_EXTRA,1,nil)
    let b3 = !duel::has_flag_effect(d, a.tp, ID + 2, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, 0, LOCATION_EXTRA, 1, NO_EX);
    // if chk==0 then return (b1 or b2 or b3) end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // if not Duel.HasFlagEffect(tp,id) then
    if !duel::has_flag_effect(d, a.tp, ID, None) {
        // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,LOCATION_MZONE,LOCATION_MZONE,c)
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_deck(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, c.into());
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,g,#g,tp,0)
        let n = g.len() as u8;
        duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, a.tp, 0);
    }
    // if not Duel.HasFlagEffect(tp,id+1) then
    if !duel::has_flag_effect(d, a.tp, ID + 1, None) {
        // local g=Duel.GetMatchingGroup(aux.AND(Card.IsSpellTrap,Card.IsAbleToDeck),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_spell_trap(d, x) && card::is_able_to_deck(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,g,#g,tp,0)
        let n = g.len() as u8;
        duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, a.tp, 0);
    }
    // if not Duel.HasFlagEffect(tp,id+2) then
    if !duel::has_flag_effect(d, a.tp, ID + 2, None) {
        // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_EXTRA,nil,tp)
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove(d, x, Some(a.tp), None, None)), a.tp, 0, LOCATION_EXTRA, NO_EX);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,g,#g,tp,0)
        let n = g.len() as u8;
        duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, a.tp, 0);
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc: Exclude = if card::is_relate_to_effect(d, c, a.e) { c.into() } else { NO_EX };
    // local b1=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_MZONE,LOCATION_MZONE,1,exc)
    let b1 = !duel::has_flag_effect(d, a.tp, ID, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_deck(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, exc);
    // local b2=not Duel.HasFlagEffect(tp,id+1)
    //     and Duel.IsExistingMatchingCard(aux.AND(Card.IsSpellTrap,Card.IsAbleToDeck),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    let b2 = !duel::has_flag_effect(d, a.tp, ID + 1, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_spell_trap(d, x) && card::is_able_to_deck(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    // local b3=not Duel.HasFlagEffect(tp,id+2)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_EXTRA,1,nil)
    let b3 = !duel::has_flag_effect(d, a.tp, ID + 2, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, 0, LOCATION_EXTRA, 1, NO_EX);
    // if not (b1 or b2 or b3) then return end
    if !(b1 || b2 || b3) {
        return;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)},
    //     {b3,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]);
    // if op==1 then
    if op == Some(1) {
        // Shuffle all other monsters on the field into the Deck
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,LOCATION_MZONE,LOCATION_MZONE,exc)
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_deck(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, exc);
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    } else if op == Some(2) {
        // Shuffle all Spells and Traps on the field into the Deck
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // local g=Duel.GetMatchingGroup(aux.AND(Card.IsSpellTrap,Card.IsAbleToDeck),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_spell_trap(d, x) && card::is_able_to_deck(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    } else if op == Some(3) {
        // Look at your opponent's Extra Deck and banish 1 card from it
        // Duel.RegisterFlagEffect(tp,id+2,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID + 2, RESET_PHASE | PHASE_END, 0, 1, 0);
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_EXTRA)
        let g = duel::get_field_group(d, a.tp, 0, LOCATION_EXTRA);
        // Duel.ConfirmCards(tp,g)
        duel::confirm_cards(d, a.tp, &g);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local sg=g:FilterSelect(tp,Card.IsAbleToRemove,1,1,nil)
        let sg = group::filter_select(d, &g, a.tp, &|d, x| card::is_able_to_remove(d, x, None, None, None), 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
        // Duel.ShuffleExtra(1-tp)
        duel::shuffle_extra(d, 1 - a.tp);
    }
}
