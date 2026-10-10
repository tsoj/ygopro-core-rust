//! Celestia, Lightsworn Angel
use super::prelude::*;

pub const ID: u32 = 94381039;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_LIGHTSWORN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon success
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // tribute check
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e2, EFFECT_MATERIAL_CHECK);
    // e2:SetValue(s.valcheck)
    effect::set_value_fn(d, e2, effect_card_value(valcheck));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.valcheck(e,c)`: a value function that returns nothing (0)
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsSetCard,1,nil,SET_LIGHTSWORN) then
    let label = if group::is_exists(d, &g, &|d, x| card::is_set_card(d, x, &[SET_LIGHTSWORN as u16]), 1, NO_EX) {
        // e:GetLabelObject():SetLabel(1)
        1
    } else {
        // e:GetLabelObject():SetLabel(0)
        0
    };
    let lo = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, lo, &[label]);
    0
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned() and e:GetLabel()==1
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h) && effect::get_label(d, a.e) == 1
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeckAsCost(tp,4) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck_as_cost(d, a.tp, 4);
    }
    // Duel.DiscardDeck(tp,4,REASON_COST)
    duel::discard_deck(d, a.tp, 4, REASON_COST);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.TRUE,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_ONFIELD,1,2,nil)
    let g = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 2, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let tg = chain::get_target_cards(d, 0).as_group().cloned().nil();
    // local g=tg:Filter(Card.IsRelateToEffect,nil,e)
    let g = group::filter(d, &tg, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
