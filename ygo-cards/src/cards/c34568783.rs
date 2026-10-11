//! R-Genex Undine
use super::prelude::*;

pub const ID: u32 = 34568783;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GENEX], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 "Genex" monster from your GY and gain its Attribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(s.attrcost)
    effect::set_cost(d, e1, Some(cost_target(attrcost)));
    // e1:SetOperation(s.attrop)
    effect::set_operation(d, e1, Some(operation(attrop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 2 "Genex" monsters from your GY to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.attrcfilter(c,attr)
fn attrcfilter(d: &mut Duel, c: CardId, attr: u32) -> bool {
    // return c:IsSetCard(SET_GENEX) and c:IsMonster() and c:IsAbleToRemoveAsCost() and attr&c:GetAttribute()==0
    card::is_set_card(d, c, &[SET_GENEX as u16])
        && card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && attr & card::get_attribute(d, c) == 0
}
// function s.attrcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn attrcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local attr=e:GetHandler():GetAttribute()
    let c = effect::get_handler(d, a.e).nil();
    let attr = card::get_attribute(d, c);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.attrcfilter,tp,LOCATION_GRAVE,0,1,nil,attr) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| attrcfilter(d, x, attr)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.attrcfilter,tp,LOCATION_GRAVE,0,1,1,nil,attr):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| attrcfilter(d, x, attr)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g).nil();
    // Duel.Remove(sc,POS_FACEUP,REASON_COST)
    duel::remove(d, CardSet::from_card(sc), POS_FACEUP as u8, REASON_COST, None, None);
    // local cd=e:GetChainData()
    // cd.attribute=sc:GetAttribute()
    let attribute = card::get_attribute(d, sc);
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(attribute as i64));
    // cd.banished_tuner=sc:IsType(TYPE_TUNER)
    let banished_tuner = card::is_type(d, sc, TYPE_TUNER);
    chain::set_effect_data(d, a.e, None, chain::field::CARD + 1, Value::Bool(banished_tuner));
    true
}
// function s.attrop(e,tp,eg,ep,ev,re,r,rp)
fn attrop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsFaceup() and c:IsRelateToEffect(e)) then return end
    if !(card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e)) {
        return;
    }
    // local cd=e:GetChainData()
    // if c:IsAttribute(cd.attribute) then return end
    let attribute = chain::effect_data(d, a.e, None, chain::field::CARD).int() as u32;
    if card::is_attribute(d, c, attribute) {
        return;
    }
    // This card gains that monster's Attribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_ADD_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_ADD_ATTRIBUTE);
    // e1:SetValue(cd.attribute)
    effect::set_value(d, e1, attribute as i64);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // if not cd.banished_tuner then return end
    let banished_tuner = chain::effect_data(d, a.e, None, chain::field::CARD + 1).truthy();
    if !banished_tuner {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Can be treated as a Tuner this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetCode(EFFECT_CAN_BE_TUNER)
    effect::set_code(d, e2, EFFECT_CAN_BE_TUNER);
    // e2:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.thcfilter(c)
fn thcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GENEX) and c:IsType(TYPE_SYNCHRO) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_GENEX as u16]) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_faceup(d, c)
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.thcfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&thcfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.thfilter(c,e)
fn thfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsSetCard(SET_GENEX) and c:IsMonster() and c:IsAbleToHand() and c:IsCanBeEffectTarget(e)
    card::is_set_card(d, c, &[SET_GENEX as u16])
        && card::is_monster(d, c)
        && card::is_able_to_hand(d, c, None)
        && card::is_can_be_effect_target(d, c, Some(e))
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() and c:IsCanBeEffectTarget(e)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,c,e) end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None)
            && card::is_can_be_effect_target(d, c, Some(a.e))
            && duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, a.e)), a.tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,c,e)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| thfilter(d, x, a.e)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // Duel.SetTargetCard(g+c)
    let mut gc = g.clone();
    group::add_card(&mut gc, &CardSet::from_card(c));
    duel::set_target_card(d, gc);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 2, a.tp, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Banish any card sent to your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_RANGE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_TO_GRAVE_REDIRECT)
    effect::set_code(d, e1, EFFECT_TO_GRAVE_REDIRECT);
    // e1:SetTargetRange(0xff,0)
    effect::set_target_range(d, e1, 0xff, 0);
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(effect_card_fn(rmtg)));
    // e1:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e1, LOCATION_REMOVED as i64);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,3))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
}
// function s.rmtg(e,c)
fn rmtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // return c:GetOwner()==tp and Duel.IsPlayerCanRemove(tp,c)
    card::get_owner(d, c) == tp && duel::is_player_can_remove(d, tp, Some((c, None)))
}
