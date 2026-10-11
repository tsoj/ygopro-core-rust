//! Dream Tower of Princess Nemleria
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 18458255;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_DREAMING_NEMLERIA],
    listed_series: &[SET_NEMLERIA],
    ..ScriptInfo::DEFAULT
};

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
    // Search 2 Level 10 Beast monsters with different names
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.thcost)
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,function(c) return not (c:IsSummonLocation(LOCATION_EXTRA) and not c:IsType(TYPE_PENDULUM)) end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(activity_counter));
    // Banish 1 face-down card from your Extra Deck to prevent destruction
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, effect_card_fn(|d, e, c| repfilter(d, c, effect::get_handler_player(d, e))));
    // e2:SetOperation(s.repop)
    effect::set_operation(d, e2, Some(operation(repop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(c) return not (c:IsSummonLocation(LOCATION_EXTRA) and not c:IsType(TYPE_PENDULUM)) end
fn activity_counter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    !(card::is_summon_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_PENDULUM))
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsAbleToRemoveAsCost(POS_FACEDOWN)
    card::is_facedown(d, c) && card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8))
}
// function s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_EXTRA,0,2,nil)
    //     and Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_EXTRA, 0, 2, NO_EX)
            && duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_EXTRA,0,2,2,nil)
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_EXTRA, 0, 2, 2, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEDOWN,REASON_COST)
    duel::remove(d, &g, POS_FACEDOWN as u8, REASON_COST, None, None);
    // Cannot Special Summon from the Extra Deck, except Pendulum Monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(_,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_PENDULUM) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_PENDULUM))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    true
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_BEAST) and c:IsLevel(10) and c:IsAbleToHand()
    card::is_race(d, c, RACE_BEAST) && card::is_level(d, c, &[10]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&aux::dncheck()), 0, tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_ATOHAND)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false)
        .unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoHand(sg,tp,REASON_EFFECT)
        duel::send_to_hand(d, &sg, Some(tp), REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, &sg);
    }
}
// function s.repfilter(c,tp)
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsControler(tp) and c:IsSetCard(SET_NEMLERIA)
    //     and not c:IsReason(REASON_REPLACE) and (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp))
    card::is_faceup(d, c)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && card::is_set_card(d, c, &[SET_NEMLERIA as u16])
        && !card::is_reason(d, c, REASON_REPLACE)
        && (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp))
}
// function s.rmfilter(c,tp)
fn rmfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFacedown() and c:IsAbleToRemove(tp,POS_FACEDOWN,REASON_EFFECT|REASON_REPLACE)
    card::is_facedown(d, c) && card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), Some(REASON_EFFECT | REASON_REPLACE))
}
// function s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_DREAMING_NEMLERIA),tp,LOCATION_EXTRA,0,1,nil)
    //     and eg:IsExists(s.repfilter,1,nil,tp)
    //     and Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_EXTRA,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_DREAMING_NEMLERIA]))),
            tp,
            LOCATION_EXTRA,
            0,
            1,
            NO_EX,
        ) && group::is_exists(d, &a.eg.clone().nil(), &|d, c| repfilter(d, c, tp), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| rmfilter(d, c, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,e:GetHandler(),96)
    let h = effect::get_handler(d, a.e).nil();
    duel::select_effect_yes_no(d, tp, h, Some(96))
}
// function s.repop(e,tp,eg,ep,ev,re,r,rp)
fn repop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_EXTRA,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| rmfilter(d, c, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT|REASON_REPLACE)
    duel::remove(d, &g, POS_FACEDOWN as u8, REASON_EFFECT | REASON_REPLACE, None, None);
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
}
