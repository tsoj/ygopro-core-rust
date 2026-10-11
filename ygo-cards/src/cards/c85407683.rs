//! New World Stars
use super::prelude::*;

pub const ID: u32 = 85407683;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_VISAS_STARFROST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent destruction of monsters by battle once each turn while you control "Visas Starfrost"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_COUNT);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCondition(s.indescond)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_VISAS_STARFROST),e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil)
        let p = effect::get_handler_player(d, a.e);
        duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_VISAS_STARFROST]))), p, LOCATION_ONFIELD, 0, 1, NO_EX)
    })));
    // e2:SetValue(s.indesval)
    effect::set_value_fn(d, e2, lua_fn(indesval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Shuffle monster into the Deck during your End Phase
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TODECK+CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TODECK + CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetHintTiming(TIMING_END_PHASE,0)
    effect::set_hint_timing(d, e3, TIMING_END_PHASE, Some(0));
    // e3:SetCondition(s.tdcond)
    effect::set_condition(d, e3, Some(condition(tdcond)));
    // e3:SetTarget(s.tdtg)
    effect::set_target(d, e3, Some(cost_target(tdtg)));
    // e3:SetOperation(s.tdop)
    effect::set_operation(d, e3, Some(operation(tdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.indesval(e,re,r,rp)
fn indesval(_d: &mut Duel, a: Args) -> i64 {
    // if (r&REASON_BATTLE)~=0 then return 1 else return 0 end
    if (a.int(2) as u32 & REASON_BATTLE) != 0 {
        1
    } else {
        0
    }
}
// function s.tdcond(e,tp,eg,ep,ev,re,r,rp)
fn tdcond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_END)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_VISAS_STARFROST),tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_turn_player(d, tp)
        && duel::is_phase(d, PHASE_END)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_VISAS_STARFROST]))), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
// function s.tdfilter(c,tohand)
fn tdfilter(d: &mut Duel, c: CardId, tohand: bool) -> bool {
    // return c:IsMonster() and (c:IsAbleToDeck() or (tohand and c:IsAbleToHand()))
    card::is_monster(d, c) && (card::is_able_to_deck(d, c) || (tohand && card::is_able_to_hand(d, c, None)))
}
// function s.tunersyncfilter(c)
fn tunersyncfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_TUNER) and c:IsType(TYPE_SYNCHRO)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_TUNER) && card::is_type(d, c, TYPE_SYNCHRO)
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local tohand=Duel.IsExistingMatchingCard(s.tunersyncfilter,tp,LOCATION_MZONE,0,1,nil)
    let tohand = duel::is_existing_matching_card(d, Some(&tunersyncfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tdfilter(chkc,tohand) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tdfilter(d, chkc, tohand);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,1,nil,tohand) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tdfilter(d, c, tohand)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE,0,1,1,nil,tohand)
    let g = duel::select_target(d, tp, Some(&|d, c| tdfilter(d, c, tohand)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // if not tohand then
    if !tohand {
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, g.clone(), 1, tp, 0);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, g.clone(), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, g, 1, tp, 0);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local tohand=Duel.IsExistingMatchingCard(s.tunersyncfilter,tp,LOCATION_MZONE,0,1,nil)
    let tohand = duel::is_existing_matching_card(d, Some(&tunersyncfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if tohand then
    if tohand {
        // aux.ToHandOrElse(tc,tp,
        //     function() return tc:IsAbleToDeck() end,
        //     function() Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT) end,
        //     aux.Stringid(id,1))
        aux::to_hand_or_else(
            d,
            tc,
            tp,
            Some(&|d, _c| card::is_able_to_deck(d, tc)),
            Some(&|d, _g| duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None)),
            Some(aux::stringid(ID, 1)),
        );
    } else {
        // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
