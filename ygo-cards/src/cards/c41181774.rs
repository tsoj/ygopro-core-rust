//! Earthbound Immortal Wiraqocha Rasca
use super::prelude::*;

pub const ID: u32 = 41181774;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EARTHBOUND_IMMORTAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // There can only be 1 "Earthbound Immortal" on the field
    // c:SetUniqueOnField(1,1,aux.FilterBoolFunction(Card.IsSetCard,SET_EARTHBOUND_IMMORTAL),LOCATION_MZONE)
    card::set_unique_on_field(d, c, 1, 1, card::UniqueCode::Function(lua_fn(unique_filter)), Some(LOCATION_MZONE));
    // Destroy it if no face-up Field Spell is on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e1, EFFECT_SELF_DESTROY);
    // e1:SetCondition(s.sdcon)
    effect::set_condition(d, e1, Some(condition(sdcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be targeted for attacks
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(aux.imval2)
    effect::set_value_fn(d, e2, aux::imval2());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Can attack directly
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e3, EFFECT_DIRECT_ATTACK);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Shuffle cards into the Deck and make the opponent discard the same number
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_TODECK+CATEGORY_HANDES)
    effect::set_category(d, e4, CATEGORY_TODECK + CATEGORY_HANDES);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
    // e4:SetTarget(s.hdtg)
    effect::set_target(d, e4, Some(cost_target(hdtg)));
    // e4:SetOperation(s.hdop)
    effect::set_operation(d, e4, Some(operation(hdop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `aux.FilterBoolFunction(Card.IsSetCard,SET_EARTHBOUND_IMMORTAL)`, read as `a.c(0)`
fn unique_filter(d: &mut Duel, a: Args) -> bool {
    card::is_set_card(d, a.c(0), &[SET_EARTHBOUND_IMMORTAL as u16])
}
/// `s.sdcon(e)`
fn sdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_BATTLE_DESTROYED) then return false end
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // return not Duel.IsExistingMatchingCard(Card.IsFaceup,0,LOCATION_FZONE,LOCATION_FZONE,1,nil)
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
/// `s.hdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn hdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and chkc:IsAbleToDeck() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, a.tp) && card::is_able_to_deck(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
    let mut ct = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND);
    // if ct==0 then return end
    if ct == 0 {
        return false;
    }
    // if ct>3 then ct=3 end
    if ct > 3 {
        ct = 3;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToDeck,tp,LOCATION_ONFIELD,0,1,ct,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_ONFIELD, 0, 1, ct as u16, false, h.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
/// `s.hdop(e,tp,eg,ep,ev,re,r,rp)`
fn hdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    // if not g then return end
    let Some(Value::Group(g)) = info.into_iter().next() else { return };
    // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, sg.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    // local ct=sg:FilterCount(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)
    let ct = group::filter_count(d, &sg, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local dg=Duel.GetFieldGroup(tp,0,LOCATION_HAND):RandomSelect(tp,ct)
    let hg = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    let dg = group::random_select(d, &hg, a.tp, ct);
    // local dt=Duel.SendtoGrave(dg,REASON_EFFECT|REASON_DISCARD)
    let dt = duel::send_to_grave(d, dg, REASON_EFFECT | REASON_DISCARD, None, None);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if dt~=0 and c:IsRelateToEffect(e) and c:IsFaceup() then
    if dt != 0 && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(dt*1000)
        effect::set_value(d, e1, dt as i64 * 1000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
