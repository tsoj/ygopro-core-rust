//! Red Dragon Ninja
use super::prelude::*;

pub const ID: u32 = 58165765;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NINJA, SET_NINJITSU_ART],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // todeck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    // e2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    // c:RegisterEffect(e2)
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    // c:RegisterEffect(e3)
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_NINJA,SET_NINJITSU_ART}) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_set_card(d, c, &[SET_NINJA as u16, SET_NINJITSU_ART as u16])
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsAbleToDeck()
    card::is_facedown(d, c) && card::is_able_to_deck(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g.clone()), 1, 0, 0);
    // Duel.SetChainLimit(s.limit(g:GetFirst()))
    let first = group::get_first(d, &g);
    duel::set_chain_limit(d, limit(first));
    true
}
/// `s.limit(c)`: `function (e,lp,tp) return e:GetHandler()~=c end`
fn limit(c: Option<CardId>) -> LuaFn {
    lua_fn(move |d: &mut Duel, a: Args| {
        let h = effect::get_handler(d, a.e(0));
        h != c
    })
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFacedown() then
    if card::is_relate_to_effect(d, tc, e) && card::is_facedown(d, tc) {
        // Duel.ConfirmCards(tp,tc)
        duel::confirm_cards(d, tp, CardSet::from_card(tc));
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if tc:IsAbleToDeck() then
        if card::is_able_to_deck(d, tc) {
            // local opt=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
            let opt = duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]);
            // if opt==0 then
            if opt == 0 {
                // Duel.SendtoDeck(tc,nil,SEQ_DECKTOP,REASON_EFFECT)
                duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
            } else {
                // Duel.SendtoDeck(tc,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
                duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
            }
        }
    }
}
