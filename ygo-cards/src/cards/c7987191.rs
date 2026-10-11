//! Borreload Riot Dragon
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 7987191;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_names={20071842}
    listed_names: &[20071842],
    // s.listed_series={SET_ROKKET,SET_BORREL}
    listed_series: &[SET_ROKKET, SET_BORREL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Disable SpSummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e1, EVENT_SPSUMMON);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy/Add to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp~=ep and Duel.GetCurrentChain()==0
    a.tp != a.ep && duel::get_current_chain(d, false) == 0
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_ROKKET),tp,LOCATION_MZONE,0,nil)
    let mut g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x) && card::is_set_card(d, x, &[SET_ROKKET as u16])), tp, LOCATION_MZONE, 0, NO_EX);
    // g:AddCard(e:GetHandler())
    group::add_card(&mut g, &CardSet::from_card(c));
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // g:Merge(eg)
    group::add_card(&mut g, &a.eg());
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,0,0)
    let eg = a.eg();
    let n_eg = eg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(eg), n_eg, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n_g = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n_g, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp,chk)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, a.eg());
    // Duel.Destroy(eg,REASON_EFFECT)
    duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_ROKKET),tp,LOCATION_MZONE,0,nil)
    let mut g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x) && card::is_set_card(d, x, &[SET_ROKKET as u16])), tp, LOCATION_MZONE, 0, NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then g:AddCard(c) end
    if card::is_relate_to_effect(d, c, a.e) {
        group::add_card(&mut g, &CardSet::from_card(c));
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local sg=g:Select(tp,1,1,nil)
    let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.HintSelection(sg)
        duel::hint_selection(d, &sg, true);
        // Duel.Destroy(sg,REASON_EFFECT)
        duel::destroy(d, sg, REASON_EFFECT, None, None);
    }
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_ROKKET) or c:IsSetCard(SET_BORREL)) and c:IsMonster() and c:IsAbleToHand()
    (card::is_set_card(d, c, &[SET_ROKKET as u16]) || card::is_set_card(d, c, &[SET_BORREL as u16])) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDestructable,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,nil)
    //     and Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_destructable(d, x, None)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_HAND|LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, tp, (LOCATION_HAND | LOCATION_ONFIELD) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, tp, LOCATION_GRAVE as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsDestructable,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| card::is_destructable(d, x, None)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>0 and tc and tc:IsRelateToEffect(e) then
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) {
                // Duel.SendtoHand(tc,nil,REASON_EFFECT)
                duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,tc)
                duel::confirm_cards(d, 1 - tp, tc);
            }
        }
    }
}
