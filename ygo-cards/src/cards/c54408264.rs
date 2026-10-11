//! Magikey Battle
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 54408264;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[99426088], listed_series: &[SET_MAGIKEY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsMonster() and (c:IsType(TYPE_NORMAL) or c:IsSetCard(SET_MAGIKEY))) or c:IsCode(99426088)) and c:IsAbleToDeck()
    ((card::is_monster(d, c) && (card::is_type(d, c, TYPE_NORMAL) || card::is_set_card(d, c, &[SET_MAGIKEY as u16]))) || card::is_code(d, c, &[99426088]))
        && card::is_able_to_deck(d, c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local ch=Duel.GetCurrentChain(true)-1
    let ch = duel::get_current_chain(d, true) - 1;
    // if tc:IsRelateToEffect(e) and Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0
    //     and ch>0 and Duel.GetChainInfo(ch,CHAININFO_TRIGGERING_PLAYER)~=tp then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && ch > 0
        && !matches!(duel::get_chain_info(d, ch as u8, &[CHAININFO_TRIGGERING_PLAYER]).remove(0), Value::Int(p) if p == a.tp as i64)
    {
        // --Unaffected
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_IMMUNE_EFFECT)
        effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
        // e1:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e1, LOCATION_MZONE, 0);
        // e1:SetTarget(s.etg)
        effect::set_target(d, e1, Some(effect_card_fn(etg)));
        // e1:SetValue(s.efilter)
        effect::set_value_fn(d, e1, lua_fn(efilter));
        // e1:SetLabelObject(Duel.GetChainInfo(ch,CHAININFO_TRIGGERING_EFFECT))
        let label = duel::get_chain_info(d, ch as u8, &[CHAININFO_TRIGGERING_EFFECT]).remove(0);
        effect::set_label_object(d, e1, label);
        // e1:SetReset(RESET_CHAIN)
        effect::set_reset(d, e1, RESET_CHAIN, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
// function s.etg(e,c)
fn etg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_MAGIKEY) or (c:IsType(TYPE_NORMAL) and not c:IsType(TYPE_TOKEN))
    card::is_set_card(d, c, &[SET_MAGIKEY as u16]) || (card::is_type(d, c, TYPE_NORMAL) && !card::is_type(d, c, TYPE_TOKEN))
}
// function s.efilter(e,re)
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return re==e:GetLabelObject()
    let label = effect::get_label_object(d, a.e(0));
    a.v(1).as_effect() == label.as_effect()
}
