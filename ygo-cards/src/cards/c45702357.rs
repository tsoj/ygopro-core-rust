//! Reversible Beetle
use super::prelude::*;

pub const ID: u32 = 45702357;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Shuffle this card, also any face-up monsters in its column, into the Deck(s)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change this card, also any monsters in its column, to face-down Defense Position
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e2a, CATEGORY_POSITION + CATEGORY_SET);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    // e2a:SetTarget(s.postg)
    effect::set_target(d, e2a, Some(cost_target(postg)));
    // e2a:SetOperation(s.posop)
    effect::set_operation(d, e2a, Some(operation(posop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,PLAYER_ALL,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, PLAYER_ALL as u8, LOCATION_MZONE as i32);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local colg=c:GetColumnGroup():Match(aux.AND(Card.IsMonster,Card.IsFaceup),nil)
    let mut colg = card::get_column_group(d, c, 0, 0);
    group::match_(d, &mut colg, &|d, x| card::is_monster(d, x) && card::is_faceup(d, x), NO_EX);
    // if not c:IsStatus(STATUS_BATTLE_DESTROYED) then colg:AddCard(c) end
    if !card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        group::add_card(&mut colg, &CardSet::from_card(c));
    }
    // if #colg>0 then
    if !colg.is_empty() {
        // Duel.SendtoDeck(colg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, colg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
// function s.postg(e,tp,eg,ep,ev,re,r,rp,chk)
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,nil,1,PLAYER_ALL,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, PLAYER_ALL as u8, POS_FACEDOWN_DEFENSE as i32);
    true
}
// function s.posop(e,tp,eg,ep,ev,re,r,rp)
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local colg=c:GetColumnGroup():Match(aux.AND(Card.IsMonster,Card.IsFaceup),nil)+c
    let mut colg = card::get_column_group(d, c, 0, 0);
    group::match_(d, &mut colg, &|d, x| card::is_monster(d, x) && card::is_faceup(d, x), NO_EX);
    group::add_card(&mut colg, &CardSet::from_card(c));
    // if #colg>0 then
    if !colg.is_empty() {
        // Duel.ChangePosition(colg,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, colg, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
