//! Joker's Wild
use super::prelude::*;

pub const ID: u32 = 81945678;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_QUEEN_KNIGHT, CARD_KING_KNIGHT, CARD_JACK_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // This effect becomes the activation effect of a Spell that mentions all of "Queen's Knight", "King's Knight", and "Jack's Knight"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_START|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_START | TIMINGS_CHECK_MONSTER));
    // e1:SetCondition(function() return Duel.IsMainPhase() or Duel.IsBattlePhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None) || duel::is_battle_phase(d, None))));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle 1 LIGHT Warrior monster from your GY into the Deck, and if you do, add this card to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetHintTiming(TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, TIMING_END_PHASE, None);
    // e2:SetCondition(function() return Duel.IsPhase(PHASE_END) end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_phase(d, PHASE_END))));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:ListsCode(CARD_QUEEN_KNIGHT,CARD_KING_KNIGHT,CARD_JACK_KNIGHT) and c:IsAbleToGraveAsCost()
    //     and c:CheckActivateEffect(true,true,false)~=nil
    card::is_spell(d, c)
        && card::lists_code(d, c, &[CARD_QUEEN_KNIGHT, CARD_KING_KNIGHT, CARD_JACK_KNIGHT])
        && card::is_able_to_grave_as_cost(d, c)
        && card::check_activate_effect(d, c, true, true, false).is_some()
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(-100)
    effect::set_label(d, a.e, &[-100]);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel).nil();
    // Duel.SendtoGrave(sc,REASON_COST)
    duel::send_to_grave(d, sc, REASON_COST, None, None);
    // e:SetLabelObject(sc)
    effect::set_label_object(d, a.e, Value::Card(sc));
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // local te=e:GetLabelObject()
        let te = effect::get_label_object(d, a.e).as_effect().nil();
        // local tg=te:GetTarget()
        // return tg and tg(e,tp,eg,ep,ev,re,r,rp,0,chkc)
        return match effect::get_target(d, te) {
            Some(tg) => {
                let mut args = call_args(a);
                args.push(Value::Int(0));
                args.push(Value::Card(chkc));
                tg.call(d, &args).truthy()
            }
            None => false,
        };
    }
    // if chk==0 then local res=e:GetLabel()==-100 e:SetLabel(0) return res end
    if a.chk == Some(0) {
        let res = effect::get_label(d, a.e) == -100;
        effect::set_label(d, a.e, &[0]);
        return res;
    }
    // Duel.ClearTargetCard()
    duel::clear_target_card(d);
    // local sc=e:GetLabelObject()
    let sc = effect::get_label_object(d, a.e).as_card().nil();
    // local te,ceg,cep,cev,cre,cr,crp=sc:CheckActivateEffect(false,true,true)
    let (te, info) = card::check_activate_effect(d, sc, false, true, true).nil();
    // local tg=te:GetTarget()
    let tg = effect::get_target(d, te);
    // e:SetProperty(te:GetProperty())
    let (p1, p2) = effect::get_property(d, te);
    effect::set_property(d, a.e, p1, p2);
    // if tg then tg(e,tp,ceg,cep,cev,cre,cr,crp,1) end
    if let Some(tg) = tg {
        let mut args = vec![Value::Effect(a.e), Value::Int(a.tp as i64)];
        match info {
            Some(ev) => args.extend([
                ev.event_cards.clone().map_or(Value::Nil, Value::Group),
                Value::Int(ev.event_player as i64),
                Value::Int(ev.event_value as i64),
                Value::effect(ev.reason_effect),
                Value::Int(ev.reason as i64),
                Value::Int(ev.reason_player as i64),
            ]),
            None => args.extend([Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil]),
        }
        args.push(Value::Int(1));
        tg.call(d, &args);
    }
    // te:SetLabelObject(e:GetLabelObject())
    let lo = effect::get_label_object(d, a.e);
    effect::set_label_object(d, te, lo);
    // e:SetLabelObject(te)
    effect::set_label_object(d, a.e, Value::Effect(te));
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    true
}
/// The arguments `e,tp,eg,ep,ev,re,r,rp` of `a`, as a Lua call passes them.
fn call_args(a: &EffArgs) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ]
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local te=e:GetLabelObject()
    let te = effect::get_label_object(d, a.e);
    // if not te then return end
    let Some(te) = te.as_effect() else { return };
    // e:SetLabelObject(te:GetLabelObject())
    let lo = effect::get_label_object(d, te);
    effect::set_label_object(d, a.e, lo);
    // local op=te:GetOperation()
    // if op then op(e,tp,eg,ep,ev,re,r,rp) end
    if let Some(op) = effect::get_operation(d, te) {
        op.call(d, &call_args(a));
    }
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_WARRIOR) and c:IsAbleToDeck()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_WARRIOR) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && tdfilter(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() and Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None) && duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_DECK|LOCATION_EXTRA)
    //     and c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) && duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 && card::is_location(d, tc, LOCATION_DECK | LOCATION_EXTRA) && card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
