//! Start for VS!
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 35550352;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VANQUISH_SOUL], ..ScriptInfo::DEFAULT };

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
    // Add 1 "Vanquish Soul" monster from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If a "Vanquish Soul" monster(s) you control would be destroyed by battle or card effect, you can reveal 1 "Vanquish Soul" monster in your hand instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.repltg)
    effect::set_target(d, e2, Some(cost_target(repltg)));
    // e2:SetOperation(s.replop)
    effect::set_operation(d, e2, Some(operation(replop)));
    // e2:SetValue(function(e,c) return s.replfilter(c,e:GetHandlerPlayer()) end)
    effect::set_value_fn(d, e2, effect_card_fn(|d, e, c| {
        let p = effect::get_handler_player(d, e);
        replfilter(d, c, p)
    }));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Set 1 "Vanquish Soul" Trap from your Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.setcon)
    effect::set_condition(d, e3, Some(condition(setcon)));
    // e3:SetTarget(s.settg)
    effect::set_target(d, e3, Some(cost_target(settg)));
    // e3:SetOperation(s.setop)
    effect::set_operation(d, e3, Some(operation(setop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thtgfilter(c,tp)`
fn thtgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_VANQUISH_SOUL) and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,c:GetAttribute())
    if !(card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16]) && card::is_faceup(d, c)) {
        return false;
    }
    let attr = card::get_attribute(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, attr)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.thfilter(c,attr)`
fn thfilter(d: &mut Duel, c: CardId, attr: u32) -> bool {
    // return c:IsSetCard(SET_VANQUISH_SOUL) and c:IsMonster() and not c:IsAttribute(attr) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16])
        && card::is_monster(d, c)
        && !card::is_attribute(d, c, attr)
        && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then ... end
    // (`chkc` is not a parameter of this function: in Lua it is an undefined global, nil)
    // if chk==0 then return Duel.IsExistingTarget(s.thtgfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| thtgfilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.thtgfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let _g = duel::select_target(d, tp, Some(&|d, x| thtgfilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tc:GetAttribute())
    let attr = card::get_attribute(d, tc);
    let g = duel::select_matching_card(d, tp, Some(&|d, x| thfilter(d, x, attr)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.replfilter(c,tp)`
fn replfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_VANQUISH_SOUL) and c:IsLocation(LOCATION_MZONE) and c:IsFaceup()
    //     and c:IsControler(tp) and not c:IsReason(REASON_REPLACE)
    card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16])
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.rvfilter(c)`
fn rvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VANQUISH_SOUL) and c:IsMonster() and not c:IsPublic()
    card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16]) && card::is_monster(d, c) && !card::is_public(d, c)
}
/// `s.repltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn repltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return eg:IsExists(s.replfilter,1,nil,tp)
    //     and Duel.IsExistingMatchingCard(s.rvfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| replfilter(d, c, tp), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&rvfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    let h = effect::get_handler(d, a.e).nil();
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local tg=Duel.SelectMatchingCard(tp,s.rvfilter,tp,LOCATION_HAND,0,1,1,nil)
        let tg = duel::select_matching_card(d, tp, Some(&rvfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // e:SetLabelObject(tg:GetFirst())
        let label = match group::get_first(d, &tg) {
            Some(first) => Value::Card(first),
            None => Value::Nil,
        };
        effect::set_label_object(d, a.e, label);
        // return true
        return true;
    }
    // return false
    false
}
/// `s.replop(e,tp,eg,ep,ev,re,r,rp)`
fn replop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_CARD,1-tp,id)
    duel::hint(d, HINT_CARD as u8, 1 - tp, ID as u64);
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.ConfirmCards(1-tp,tc)
    duel::confirm_cards(d, 1 - tp, tc);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.IsTurnPlayer(tp)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_VANQUISH_SOUL),tp,LOCATION_MZONE,0,2,nil)
    duel::is_turn_player(d, tp)
        && duel::is_existing_matching_card(
            d,
            Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16]))),
            tp,
            LOCATION_MZONE,
            0,
            2,
            NO_EX,
        )
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VANQUISH_SOUL) and c:IsTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SSet(tp,g) end
    if !g.is_empty() {
        duel::s_set(d, tp, g, None, true);
    }
}
