//! Garoth, Lightsworn Warrior
use super::prelude::*;

pub const ID: u32 = 59019082;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_LIGHTSWORN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // discard deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCategory(CATEGORY_DECKDES+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DECKDES + CATEGORY_DRAW);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCondition(s.condtion)
    effect::set_condition(d, e1, Some(condition(condtion)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_DECK)
    card::is_previous_location(d, c, LOCATION_DECK)
}
/// `s.condtion(e,tp,eg,ep,ev,re,r,rp)`
fn condtion(d: &mut Duel, a: &EffArgs) -> bool {
    // if not re then return false end
    let Some(re) = a.re else {
        return false;
    };
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re);
    // return rp==tp and (r&REASON_EFFECT)~=0 and not rc:IsCode(id) and rc:IsSetCard(SET_LIGHTSWORN) and rc:IsMonster()
    //     and re:GetActivateLocation()==LOCATION_MZONE and eg:IsExists(s.cfilter,1,nil)
    if !(a.rp == a.tp && (a.r & REASON_EFFECT) != 0) {
        return false;
    }
    let rc = rc.nil();
    if card::is_code(d, rc, &[ID]) || !card::is_set_card(d, rc, &[SET_LIGHTSWORN as u16]) || !card::is_monster(d, rc) {
        return false;
    }
    if effect::get_activate_location(d, re) != LOCATION_MZONE {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &cfilter, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 2);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsSetCard(SET_LIGHTSWORN) and c:IsMonster()
    card::is_location(d, c, LOCATION_GRAVE) && card::is_set_card(d, c, &[SET_LIGHTSWORN as u16]) && card::is_monster(d, c)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetControler()~=tp or not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if card::get_controler(d, c) != tp || !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // Duel.DiscardDeck(tp,2,REASON_EFFECT)
    duel::discard_deck(d, tp, 2, REASON_EFFECT);
    // local dg=Duel.GetOperatedGroup()
    let dg = duel::get_operated_group(d);
    // local d=dg:FilterCount(s.filter,nil)
    let dcount = group::filter_count(d, &dg, &filter, NO_EX);
    // if d>0 then
    if dcount > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,d,REASON_EFFECT)
        duel::draw(d, tp, dcount as u32, REASON_EFFECT);
    }
}
