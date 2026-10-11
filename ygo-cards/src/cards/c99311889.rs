//! Prohibitive Power Patron Purview - Vilaea
use super::prelude::*;

pub const ID: u32 = 99311889;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[53589300, 68231287, 5914858],
    listed_series: &[SET_POWER_PATRON as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Show 1 "Nerva the Power Patron of Creation", 1 "Jupiter the Power Patron of Destruction", and 1 "Junora the Power Patron of Tuning" from your Extra Deck, face-up field, and/or GY, and if you do, your opponent cannot activate cards or effects in response the activation of your "Power Patron" monsters' effects this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If you would banish the top card(s) of your Deck to activate the effect of a "Power Patron" monster you control, you can banish this card from your GY instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_COST_REPLACE)
    effect::set_code(d, e2, EFFECT_COST_REPLACE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsAbleToRemoveAsCost() end)
    effect::set_condition(
        d,
        e2,
        Some(lua_fn(|d, a: Args| {
            let h = effect::get_handler(d, a.e(0)).nil();
            card::is_able_to_remove_as_cost(d, h, None)
        })),
    );
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, lua_fn(repval));
    // e2:SetOperation(function(base) Duel.Remove(base:GetHandler(),POS_FACEUP,REASON_COST) end)
    effect::set_operation(
        d,
        e2,
        Some(lua_fn(|d, a: Args| {
            let h = effect::get_handler(d, a.e(0)).nil();
            duel::remove(d, h, POS_FACEUP as u8, REASON_COST, None, None);
            Value::Nil
        })),
    );
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.showfilter(c)`
fn showfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(53589300,68231287,5914858) and (c:IsFaceup() or not c:IsOnField())
    card::is_code(d, c, &[53589300, 68231287, 5914858]) && (card::is_faceup(d, c) || !card::is_on_field(d, c))
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.showfilter,tp,LOCATION_EXTRA|LOCATION_ONFIELD|LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(d, Some(&showfilter), a.tp, LOCATION_EXTRA | LOCATION_ONFIELD | LOCATION_GRAVE, 0, NO_EX);
        // return g:GetClassCount(Card.GetCode)>=3
        return group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3;
    }
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.showfilter,tp,LOCATION_EXTRA|LOCATION_ONFIELD|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&showfilter), tp, LOCATION_EXTRA | LOCATION_ONFIELD | LOCATION_GRAVE, 0, NO_EX);
    // if #g>=3 then
    if g.len() >= 3 {
        // local rg=aux.SelectUnselectGroup(g,e,tp,3,3,aux.dncheck,1,tp,HINTMSG_CONFIRM)
        let rg = aux::select_unselect_group(
            d,
            &g,
            Some(a.e),
            tp,
            Some(3),
            Some(3),
            Some(&aux::dncheck()),
            1,
            tp,
            Some(HINTMSG_CONFIRM as u64),
            None,
            None,
            false,
        )
        .unwrap_or_default();
        // local fieldgraveg,extrag=rg:Split(Card.IsLocation,nil,LOCATION_ONFIELD|LOCATION_GRAVE)
        let (fieldgraveg, extrag) = group::split(d, &rg, &|d, c| card::is_location(d, c, LOCATION_ONFIELD | LOCATION_GRAVE), NO_EX);
        // if #fieldgraveg>0 then Duel.HintSelection(fieldgraveg) end
        if !fieldgraveg.is_empty() {
            duel::hint_selection(d, fieldgraveg, true);
        }
        // if #extrag>0 then Duel.ConfirmCards(1-tp,extrag) end
        if !extrag.is_empty() {
            duel::confirm_cards(d, 1 - tp, extrag);
        }
        // if rg:IsExists(Card.IsLocation,1,nil,LOCATION_EXTRA) then Duel.ShuffleExtra(tp) end
        if group::is_exists(d, &rg, &|d, c| card::is_location(d, c, LOCATION_EXTRA), 1, NO_EX) {
            duel::shuffle_extra(d, tp);
        }
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,1))
        aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
        // --Your opponent cannot activate cards or effects in response the activation of your "Power Patron" monsters' effects this turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_CHAINING)
        effect::set_code(d, e1, EVENT_CHAINING);
        // e1:SetOperation(s.inactop)
        effect::set_operation(d, e1, Some(operation(inactop)));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE + PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.inactop(e,tp,eg,ep,ev,re,r,rp)`
fn inactop(d: &mut Duel, a: &EffArgs) {
    // if ep==tp and Chain.IsType(ev,TYPE_MONSTER) and Chain.IsSetcode(ev,SET_POWER_PATRON) then
    if a.ep == a.tp
        && chain::is_triggering(d, a.ev as u8, chain::Prop::Type, &[TYPE_MONSTER as i64]).truthy()
        && chain::is_triggering(d, a.ev as u8, chain::Prop::SetCard, &[SET_POWER_PATRON as i64]).truthy()
    {
        // Duel.SetChainLimit(function(e,rp,tp) return rp==tp end)
        duel::set_chain_limit(d, lua_fn(|_d, a: Args| a.p(1) == a.p(2)));
    }
}
/// `s.repval(base,extracon,e,tp,eg,ep,ev,re,r,rp,chk)`
fn repval(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(2)).nil();
    let tp = a.p(3);
    // return c:IsSetCard(SET_POWER_PATRON) and c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE)
    card::is_set_card(d, c, &[SET_POWER_PATRON as u16]) && card::is_faceup(d, c) && card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_MZONE)
}
