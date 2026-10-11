//! Mulcharmy Meowls
use super::prelude::*;

pub const ID: u32 = 87126721;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MULCHARMY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply effects for the rest of the turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e1:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0)==0 end)
    effect::set_condition(
        d,
        e1,
        Some(lua_fn(|d, a: Args| duel::get_field_group_count(d, a.p(1), LOCATION_ONFIELD, 0) == 0)),
    );
    // e1:SetCost(s.effcost)
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Keep track of the activations of a "Mulcharmy" monster's effect
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,function(re) return not (re:GetHandler():IsSetCard(SET_MULCHARMY) and re:IsMonsterEffect()) end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(counter_fn));
}
// function(re) return not (re:GetHandler():IsSetCard(SET_MULCHARMY) and re:IsMonsterEffect()) end (the activity counter)
fn counter_fn(d: &mut Duel, a: Args) -> bool {
    let re = a.e(0);
    let h = effect::get_handler(d, re).nil();
    !(card::is_set_card(d, h, &[SET_MULCHARMY as u16]) && effect::is_monster_effect(d, re))
}
// function s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsDiscardable() and Duel.GetCustomActivityCount(id,tp,ACTIVITY_CHAIN)<2 end
    if a.chk == Some(0) {
        return card::is_discardable(d, c, None, None) && duel::get_custom_activity_count(d, ID, tp, ACTIVITY_CHAIN) < 2;
    }
    // Duel.SendtoGrave(c,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, c, REASON_COST | REASON_DISCARD, None, None);
    // You can only activate 1 other "Mulcharmy" monster effect, the turn you activate this effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetCondition(function(e) return Duel.GetCustomActivityCount(id,e:GetHandlerPlayer(),ACTIVITY_CHAIN)>=2 end)
    effect::set_condition(
        d,
        e1,
        Some(lua_fn(|d, a: Args| {
            let p = effect::get_handler_player(d, a.e(0));
            duel::get_custom_activity_count(d, ID, p, ACTIVITY_CHAIN) >= 2
        })),
    );
    // e1:SetValue(function(e,re,tp) return re:GetHandler():IsSetCard(SET_MULCHARMY) and re:IsMonsterEffect() end)
    effect::set_value_fn(
        d,
        e1,
        lua_fn(|d, a: Args| {
            let re = a.e(1);
            let h = effect::get_handler(d, re).nil();
            card::is_set_card(d, h, &[SET_MULCHARMY as u16]) && effect::is_monster_effect(d, re)
        }),
    );
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    true
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Draw 1 card each time your opponent Special Summons a monster(s) from the GY and/or banishment
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.drcon)
    effect::set_condition(d, e1, Some(condition(drcon)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Shuffle random cards from your hand to the Deck during the End Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
// function s.drconfilter(c,tp)
fn drconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(1-tp) and c:IsSummonLocation(LOCATION_GRAVE|LOCATION_REMOVED)
    card::is_summon_player(d, c, 1 - tp) && card::is_summon_location(d, c, LOCATION_GRAVE | LOCATION_REMOVED)
}
// function s.drcon(e,tp,eg,ep,ev,re,r,rp)
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    // return eg:IsExists(s.drconfilter,1,nil,tp)
    group::is_exists(d, &eg, &|d, c| drconfilter(d, c, tp), 1, NO_EX)
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if not Duel.IsChainSolving() then
    if !duel::is_chain_solving(d) {
        // Duel.Hint(HINT_CARD,1-tp,id)
        duel::hint(d, HINT_CARD as u8, 1 - tp, ID as u64);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    } else {
        // local eff=e:GetLabelObject()
        let eff = effect::get_label_object(d, e).as_effect();
        // if eff and not eff:IsDeleted() then
        if let Some(eff) = eff.filter(|x| !effect::is_deleted(d, *x)) {
            // eff:SetLabel(eff:GetLabel()+1)
            let label = effect::get_label(d, eff);
            effect::set_label(d, eff, &[label + 1]);
        } else {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, e).nil();
            // Draw cards when the current Chain Link finishes resolving
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_CHAIN_SOLVED)
            effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
            // e1:SetOperation(s.chainsolvedop)
            effect::set_operation(d, e1, Some(operation(chainsolvedop)));
            // e1:SetLabel(1)
            effect::set_label(d, e1, &[1]);
            // e1:SetLabelObject(e)
            effect::set_label_object(d, e1, Value::Effect(e));
            // e1:SetReset(RESET_CHAIN)
            effect::set_reset(d, e1, RESET_CHAIN, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
            // e:SetLabelObject(e1)
            effect::set_label_object(d, e, Value::Effect(e1));
            // Reset "e1" and the label object of "e" at the end of the Chain Link
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e2:SetCode(EVENT_CHAIN_SOLVED)
            effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
            // e2:SetOperation(function() e:SetLabelObject(nil) e1:Reset() end)
            effect::set_operation(
                d,
                e2,
                Some(lua_fn(move |d, _a: Args| {
                    effect::set_label_object(d, e, Value::Nil);
                    effect::reset(d, e1);
                    Value::Nil
                })),
            );
            // e2:SetReset(RESET_CHAIN)
            effect::set_reset(d, e2, RESET_CHAIN, 1);
            // Duel.RegisterEffect(e2,tp)
            duel::register_effect(d, e2, tp);
        }
    }
}
// function s.chainsolvedop(e,tp,eg,ep,ev,re,r,rp)
fn chainsolvedop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // Duel.Hint(HINT_CARD,1-tp,id)
    duel::hint(d, HINT_CARD as u8, 1 - tp, ID as u64);
    // Duel.Draw(tp,e:GetLabel(),REASON_EFFECT)
    let label = effect::get_label(d, e);
    duel::draw(d, tp, label as u32, REASON_EFFECT);
    // e:Reset()
    effect::reset(d, e);
    // e:GetLabelObject():SetLabelObject(nil)
    let lo = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label_object(d, lo, Value::Nil);
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local dif=Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)-(Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)+6)
    let dif = duel::get_field_group_count(d, tp, LOCATION_HAND, 0) - (duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD) + 6);
    // if dif>0 then
    if dif > 0 {
        // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0):Match(Card.IsAbleToDeck,nil):RandomSelect(tp,dif)
        let hand = duel::get_field_group(d, tp, LOCATION_HAND, 0);
        let able = group::filter(d, &hand, &|d, c| card::is_able_to_deck(d, c), NO_EX);
        let g = group::random_select(d, &able, tp, dif as usize);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.Hint(HINT_CARD,1-tp,id)
        duel::hint(d, HINT_CARD as u8, 1 - tp, ID as u64);
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
