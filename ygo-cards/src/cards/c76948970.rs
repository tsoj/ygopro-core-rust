//! Mitsurugi no Miko, Wousu
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 76948970;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can Special Summon this card (from your hand) by Tributing 1 other Reptile mosnter from your hand and 1 monster your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(|d, a| spcon(d, a))));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(|d, a| sptg(d, a))));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(|d, a| spop(d, a))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Discard 1 card, and if you do, add this card to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_HANDES+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_HANDES + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_RELEASE)
    effect::set_code(d, e2, EVENT_RELEASE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spconfilter(c,tp)
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsRace(RACE_REPTILE) or c:IsControler(1-tp)) and c:IsReleasable()
    (card::is_race(d, c, RACE_REPTILE) || card::is_controler(d, c, 1 - tp)) && card::is_releasable(d, c, None, None)
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args<'_>) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local g=Duel.GetMatchingGroup(s.spconfilter,tp,LOCATION_HAND,LOCATION_MZONE,c,tp)
    let g = duel::get_matching_group(d, Some(&|d, x| spconfilter(d, x, tp)), tp, LOCATION_HAND, LOCATION_MZONE, c.into());
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #g>0
    //   and aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),0)
    let dp = aux::dpcheck(controler_key);
    let rs: aux::Rescon = &dp;
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && !g.is_empty()
        && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true))
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)
fn sptg(d: &mut Duel, a: Args<'_>) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.card(9);
    // local rg=Duel.GetMatchingGroup(s.spconfilter,tp,LOCATION_HAND,LOCATION_MZONE,c,tp)
    let ex = match c {
        Some(c) => c.into(),
        None => NO_EX,
    };
    let rg = duel::get_matching_group(d, Some(&|d, x| spconfilter(d, x, tp)), tp, LOCATION_HAND, LOCATION_MZONE, ex);
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,aux.dpcheck(Card.GetControler),1,tp,HINTMSG_RELEASE,nil,nil,true)
    let dp = aux::dpcheck(controler_key);
    let rs: aux::Rescon = &dp;
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp,c)
fn spop(d: &mut Duel, a: Args<'_>) {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(8);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // g:DeleteGroup()
    // --You cannot Special Summon nor activate monster effects for the rest of this turn, except Reptile monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return not c:IsRace(RACE_REPTILE) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_REPTILE))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetTarget(aux.TRUE)
    effect::set_target(d, e2, Some(aux_true()));
    // e2:SetValue(function(e,re) return re:IsMonsterEffect() and not re:GetHandler():IsRace(RACE_REPTILE) end)
    effect::set_value_fn(d, e2, lua_fn(|d, a| {
        let re = a.e(1);
        if !effect::is_monster_effect(d, re) {
            return false;
        }
        let rh = effect::get_handler(d, re).nil();
        !card::is_race(d, rh, RACE_REPTILE)
    }));
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil,REASON_EFFECT)
    //   and c:IsAbleToHand() end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_discardable(d, x, Some(REASON_EFFECT), None)), tp, LOCATION_HAND, 0, 1, NO_EX)
            && card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_EFFECT|REASON_DISCARD)>0 and c:IsRelateToEffect(e) then
    if duel::discard_hand(d, tp, Some(&|d, x| card::is_discardable(d, x, None, None)), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) > 0
        && card::is_relate_to_effect(d, c, a.e)
    {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,c)
        duel::confirm_cards(d, 1 - tp, c);
    }
}
// Card.GetControler as a number key for aux.dpcheck
fn controler_key(d: &mut Duel, c: CardId) -> i64 {
    card::get_controler(d, c) as i64
}
