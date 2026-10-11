//! World of Spirits
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 5414777;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ANCIENT_FAIRY_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change any monster that is Special Summoned in Attack Position to Defense Position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e2, CATEGORY_POSITION);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCondition(s.lightdragonsynchrocon)
    effect::set_condition(d, e2, Some(condition(lightdragonsynchrocon)));
    // e2:SetTarget(s.postg)
    effect::set_target(d, e2, Some(cost_target(postg)));
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy all the turn player's Attack Position monsters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.lightdragonsynchrocon)
    effect::set_condition(d, e3, Some(condition(lightdragonsynchrocon)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:ListsCode(CARD_ANCIENT_FAIRY_DRAGON) and c:IsAbleToHand()
    card::is_monster(d, c) && card::lists_code(d, c, &[CARD_ANCIENT_FAIRY_DRAGON]) && card::is_able_to_hand(d, c, None)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if g.len() > 0 {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
// function s.lightdragonsynchroconfilter(c)
fn lightdragonsynchroconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(7) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_DRAGON) and c:IsType(TYPE_SYNCHRO) and c:IsFaceup()
    card::is_level_above(d, c, 7)
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_DRAGON)
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_faceup(d, c)
}
// function s.lightdragonsynchrocon(e,tp,eg,ev,ep,re,r,rp)
fn lightdragonsynchrocon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.lightdragonsynchroconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&lightdragonsynchroconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.postg(e,tp,eg,ep,ev,re,r,rp,chk)
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=eg:Filter(Card.IsPosition,nil,POS_ATTACK)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| card::is_position(d, c, POS_ATTACK), NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return g.len() > 0;
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, a.tp, 0);
    true
}
// function s.posop(e,tp,eg,ep,ev,re,r,rp)
fn posop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e):Filter(Card.IsPosition,nil,POS_ATTACK)
    let targets = duel::get_target_cards(d, a.e).unwrap_or_default();
    let g = group::filter(d, &targets, &|d, c| card::is_position(d, c, POS_ATTACK), NO_EX);
    // if #g>0 then
    if g.len() > 0 {
        // Duel.ChangePosition(g,POS_FACEUP_DEFENSE,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, g, POS_FACEUP_DEFENSE as u8, Some(POS_FACEDOWN_DEFENSE as u8), None, None, false);
    }
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(Card.IsPosition,Duel.GetTurnPlayer(),LOCATION_MZONE,0,nil,POS_ATTACK)
    let turn = duel::get_turn_player(d);
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_position(d, c, POS_ATTACK)), turn, LOCATION_MZONE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, _a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsPosition,Duel.GetTurnPlayer(),LOCATION_MZONE,0,nil,POS_ATTACK)
    let turn = duel::get_turn_player(d);
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_position(d, c, POS_ATTACK)), turn, LOCATION_MZONE, 0, NO_EX);
    // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
    if g.len() > 0 {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
