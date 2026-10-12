//! Reindsurm Carnation
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 101403056;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_REINDSURM], ..ScriptInfo::DEFAULT };

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
    // While you control a "Reindsurm" monster whose original Level is 8 or higher or your opponent has a Dragon monster in their field or GY, the activated effects of your "Reindsurm" monsters cannot be negated
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_DISEFFECT)
    effect::set_code(d, e1, EFFECT_CANNOT_DISEFFECT);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCondition(s.cannotdiscon)
    effect::set_condition(d, e1, Some(lua_fn(cannotdiscon)));
    // e1:SetValue(s.cannotdisfilter)
    effect::set_value_fn(d, e1, lua_fn(cannotdisfilter));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per turn: You can send 1 "Reindsurm" monster or Dragon monster from your hand or face-up field to the GY; add 1 Insect monster from your Deck to your hand with the same original Level, but a different original name
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(s.thcost)
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cannotdisconfilter(c,tp)
fn cannotdisconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsSetCard(SET_REINDSURM) and c:GetOriginalLevel()>=8 and c:IsFaceup() and c:IsControler(tp))
    //     or (c:IsRace(RACE_DRAGON) and c:IsFaceup() and c:IsControler(1-tp))
    (card::is_set_card(d, c, &[SET_REINDSURM as u16]) && card::get_original_level(d, c) >= 8 && card::is_faceup(d, c) && card::is_controler(d, c, tp))
        || (card::is_race(d, c, RACE_DRAGON) && card::is_faceup(d, c) && card::is_controler(d, c, 1 - tp))
}
// function s.cannotdiscon(e)
fn cannotdiscon(d: &mut Duel, a: Args) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // return Duel.IsExistingMatchingCard(s.cannotdisconfilter,tp,LOCATION_MZONE,LOCATION_MZONE|LOCATION_GRAVE,1,nil,tp)
    duel::is_existing_matching_card(d, Some(&|d, c| cannotdisconfilter(d, c, tp)), tp, LOCATION_MZONE, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX)
}
// function s.cannotdisfilter(e,ct)
fn cannotdisfilter(d: &mut Duel, a: Args) -> bool {
    // return Chain.IsTriggeringPlayer(ct,e:GetHandlerPlayer())
    //     and Chain.IsTriggeringSetcode(ct,SET_REINDSURM)
    //     and Chain.IsTriggeringType(ct,TYPE_MONSTER)
    let ct = a.int(1) as u8;
    let p = effect::get_handler_player(d, a.e(0));
    chain::is_triggering_player(d, ct, p).truthy()
        && chain::is_triggering(d, ct, chain::Prop::SetCard, &[SET_REINDSURM as i64]).truthy()
        && chain::is_triggering(d, ct, chain::Prop::Type, &[TYPE_MONSTER as i64]).truthy()
}
// function s.thcostfilter(c,tp)
fn thcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsSetCard(SET_REINDSURM) or c:IsRace(RACE_DRAGON)) and c:IsMonster() and (c:IsFaceup() or c:IsLocation(LOCATION_HAND)) and c:IsAbleToGraveAsCost()
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,c:GetOriginalLevel(),c:GetOriginalCodeRule())
    (card::is_set_card(d, c, &[SET_REINDSURM as u16]) || card::is_race(d, c, RACE_DRAGON))
        && card::is_monster(d, c)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && card::is_able_to_grave_as_cost(d, c) && {
        let lv = card::get_original_level(d, c) as u32;
        // the filter gets every code of GetOriginalCodeRule, but s.thfilter takes only the first
        let name = card::get_original_code_rule(d, c)[0];
        duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, lv, name)), tp, LOCATION_DECK, 0, 1, NO_EX)
    }
}
// function s.thfilter(c,cost_level,cost_name)
fn thfilter(d: &mut Duel, c: CardId, cost_level: u32, cost_name: u32) -> bool {
    // return c:IsRace(RACE_INSECT) and c:IsOriginalLevel(cost_level) and not c:IsOriginalCodeRule(cost_name) and c:IsAbleToHand()
    card::is_race(d, c, RACE_INSECT) && card::is_original_level(d, c, cost_level) && !card::is_original_code_rule(d, c, &[cost_name]) && card::is_able_to_hand(d, c, None)
}
// function s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thcostfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thcostfilter(d, c, a.tp)), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.thcostfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thcostfilter(d, c, a.tp)), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g).nil();
    // Duel.SendtoGrave(sc,REASON_COST)
    duel::send_to_grave(d, CardSet::from_card(sc), REASON_COST, None, None);
    // local cd=e:GetChainData()
    // cd.cost_level=sc:GetOriginalLevel()
    let level = card::get_original_level(d, sc) as i64;
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(level));
    // cd.cost_name=sc:GetOriginalCodeRule()
    let name = card::get_original_code_rule(d, sc)[0] as i64;
    chain::set_effect_data(d, a.e, None, chain::field::CARD + 1, Value::Int(name));
    true
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local cd=e:GetChainData()
    // local cost_level=cd.cost_level
    let cost_level = chain::effect_data(d, a.e, None, chain::field::CARD).int() as u32;
    // local cost_name=cd.cost_name
    let cost_name = chain::effect_data(d, a.e, None, chain::field::CARD + 1).int() as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,cost_level,cost_name)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, cost_level, cost_name)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
