//! The Dragon that Devours the Dogma
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 76666602;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_ALBAZ],
    listed_series: &[SET_ECCLESIA, SET_DOGMATIKA, SET_TRI_BRIGADE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: "Fallen of Albaz" + 1 LIGHT or DARK monster + 1 Effect Monster
    // Fusion.AddProcMix(c,true,true,CARD_ALBAZ,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_LIGHT|ATTRIBUTE_DARK),
    //     aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(CARD_ALBAZ),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| {
                card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK, Some(fc), st, Some(tp))
            })),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, Some(fc), st, Some(tp)))),
        ],
    );
    // While an "Ecclesia" monster is on the field or in either GY, this card gains 500 ATK, also it is unaffected by other cards' effects
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE);
    // e1a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1a:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetCondition(function(e) return Duel.IsExistingMatchingCard(s.atkimmconfilter,e:GetHandlerPlayer(),LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,nil) end)
    effect::set_condition(
        d,
        e1a,
        Some(condition(|d, a| {
            let p = effect::get_handler_player(d, a.e);
            duel::is_existing_matching_card(d, Some(&atkimmconfilter), p, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX)
        })),
    );
    // e1a:SetValue(500)
    effect::set_value(d, e1a, 500);
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1b, EFFECT_IMMUNE_EFFECT);
    // e1b:SetValue(function(e,te) return te:GetOwner()~=e:GetOwner() end)
    effect::set_value_fn(
        d,
        e1b,
        lua_fn(|d, a: Args| {
            let (e, te) = (a.e(0), a.e(1));
            effect::get_owner(d, te) != effect::get_owner(d, e)
        }),
    );
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Shuffle up to 2 cards from any GY(s) and/or banishment into the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 "Dogmatika" or "Tri-Brigade" card from your Deck to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(e) return e:GetHandler():GetTurnID()==Duel.GetTurnCount() end)
    effect::set_condition(
        d,
        e3,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::get_turn_id(d, h) as i64 == duel::get_turn_count(d, None) as i64
        })),
    );
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// s.atkimmconfilter(c)
fn atkimmconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ECCLESIA) and c:IsMonster()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ECCLESIA as u16]) && card::is_monster(d, c)
}
/// s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_GRAVE|LOCATION_REMOVED,LOCATION_GRAVE|LOCATION_REMOVED,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_able_to_deck(d, c)),
            tp,
            LOCATION_GRAVE | LOCATION_REMOVED,
            LOCATION_GRAVE | LOCATION_REMOVED,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,PLAYER_EITHER,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, PLAYER_EITHER as u8, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
/// s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(Card.IsAbleToDeck),tp,LOCATION_GRAVE|LOCATION_REMOVED,LOCATION_GRAVE|LOCATION_REMOVED,1,2,nil)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| card::is_able_to_deck(d, c))),
        tp,
        LOCATION_GRAVE | LOCATION_REMOVED,
        LOCATION_GRAVE | LOCATION_REMOVED,
        1,
        2,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
/// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_DOGMATIKA,SET_TRI_BRIGADE}) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_DOGMATIKA as u16, SET_TRI_BRIGADE as u16]) && card::is_able_to_hand(d, c, None)
}
/// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
