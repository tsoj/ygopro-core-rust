//! S-Force Chase
//! Ｓ－Ｆｏｒｃｅ チェイス
use super::prelude::*;

pub const ID: u32 = 55049722;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_S_FORCE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target face-up cards your opponent controls, up to the number of "S-Force" monsters you control with different names; return them to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If an "S-Force" monster you control would banish a card(s) from your hand to activate its effect, you can banish this card from your GY instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
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
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_able_to_remove_as_cost(d, h, None)
    })));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, lua_fn(repval));
    // e2:SetOperation(function(base) Duel.Remove(base:GetHandler(),POS_FACEUP,REASON_COST) end)
    effect::set_operation(d, e2, Some(lua_fn(|d, a: Args| {
        let base = a.e(0);
        let h = effect::get_handler(d, base).nil();
        duel::remove(d, h, POS_FACEUP as u8, REASON_COST, None, None);
    })));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_S_FORCE)`
fn faceup_sforce(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_S_FORCE as u16])
}
/// `aux.FaceupFilter(Card.IsAbleToHand)`
fn faceup_able_to_hand(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
/// `Card.GetCode` as a value (its first code)
fn code_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_code(d, c)[0] as i64
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) and chkc:IsFaceup() and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp) && card::is_faceup(d, chkc) && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_S_FORCE),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAbleToHand),tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&faceup_sforce), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&faceup_able_to_hand), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local ct=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_S_FORCE),tp,LOCATION_MZONE,0,nil):GetClassCount(Card.GetCode)
    let mg = duel::get_matching_group(d, Some(&faceup_sforce), tp, LOCATION_MZONE, 0, NO_EX);
    let ct = group::get_class_count(d, &mg, &code_value);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAbleToHand),tp,0,LOCATION_ONFIELD,1,ct,nil)
    let g = duel::select_target(d, tp, Some(&faceup_able_to_hand), tp, 0, LOCATION_ONFIELD, 1, ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then Duel.SendtoHand(g,nil,REASON_EFFECT) end
    if !g.is_empty() {
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
/// `s.repval(base,extracon,e,tp,eg,ep,ev,re,r,rp,chk)`
fn repval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(2);
    let tp = a.p(3);
    let extracon = a.v(1).as_fn().nil().clone();
    // extracon(base,e,tp,e,tp,eg,ep,ev,re,r,rp,chk), as written in the Lua
    let call = |d: &mut Duel| {
        let args: Vec<Value> = [0, 2, 3, 2, 3, 4, 5, 6, 7, 8, 9, 10].iter().map(|&i| a.v(i).clone()).collect();
        extracon.call(d, &args).truthy()
    };
    if matches!(a.v(10), Value::Int(0)) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE) and c:IsSetCard(SET_S_FORCE) and extracon(...)
        return card::is_faceup(d, c)
            && card::is_controler(d, c, tp)
            && card::is_location(d, c, LOCATION_MZONE)
            && card::is_set_card(d, c, &[SET_S_FORCE as u16])
            && call(d);
    }
    // return Chain.IsTriggeringControler(0,tp) and Chain.IsTriggeringLocation(0,LOCATION_MZONE) and Chain.IsTriggeringPosition(0,POS_FACEUP)
    //     and Chain.IsTriggeringSetcode(0,SET_S_FORCE) and extracon(...)
    chain::is_triggering_player(d, 0, tp).truthy()
        && chain::is_triggering(d, 0, chain::Prop::Location, &[LOCATION_MZONE as i64]).truthy()
        && chain::is_triggering(d, 0, chain::Prop::Position, &[POS_FACEUP as i64]).truthy()
        && chain::is_triggering(d, 0, chain::Prop::SetCard, &[SET_S_FORCE as i64]).truthy()
        && call(d)
}
