//! Black Luster Soldier - Envoy of the Evening Twilight
use super::prelude::*;

pub const ID: u32 = 77498348;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLACK_LUSTER_SOLDIER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Special Summon limit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Also treated as a LIGHT monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_ADD_ATTRIBUTE)
    effect::set_code(d, e2, EFFECT_ADD_ATTRIBUTE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(ATTRIBUTE_LIGHT)
    effect::set_value(d, e2, ATTRIBUTE_LIGHT as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Special Summon procedure
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e3, EFFECT_SPSUMMON_PROC);
    // e3:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetRange(LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_HAND);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(lua_fn(spcon)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(lua_fn(spop)));
    // e3:SetLabel(ATTRIBUTE_LIGHT)
    effect::set_label(d, e3, &[ATTRIBUTE_LIGHT as i64]);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetLabel(ATTRIBUTE_DARK)
    effect::set_label(d, e4, &[ATTRIBUTE_DARK as i64]);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // --Banish a monster and 1 random card from the opponent's hand
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e5, aux::stringid(ID, 2));
    // e5:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e5, CATEGORY_REMOVE);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e5:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
    // e5:SetCondition(function(e) return e:GetHandler():GetSummonType()==SUMMON_TYPE_SPECIAL+1 end)
    effect::set_condition(d, e5, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::get_summon_type(d, h) == SUMMON_TYPE_SPECIAL + 1
    })));
    // e5:SetCost(s.rmcost)
    effect::set_cost(d, e5, Some(cost_target(rmcost)));
    // e5:SetTarget(s.rmtg)
    effect::set_target(d, e5, Some(cost_target(rmtg)));
    // e5:SetOperation(s.rmop)
    effect::set_operation(d, e5, Some(operation(rmop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // e3:SetLabelObject(e5)
    effect::set_label_object(d, e3, Value::Effect(e5));
    // e4:SetLabelObject(e5)
    effect::set_label_object(d, e4, Value::Effect(e5));
}
/// `s.spfilter(c,att)`
fn spfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsAttribute(att) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, att) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let e = a.e(0);
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local ct=Duel.GetMatchingGroupCount(Card.IsAttribute,tp,LOCATION_GRAVE,0,nil,ATTRIBUTE_LIGHT)
    let ct = duel::get_matching_group(d, Some(&|d, x| card::is_attribute(d, x, ATTRIBUTE_LIGHT)), tp, LOCATION_GRAVE, 0, NO_EX).len() as u32;
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and ct>0 and ct==Duel.GetMatchingGroupCount(Card.IsAttribute,tp,LOCATION_GRAVE,0,nil,ATTRIBUTE_DARK)
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,ct,nil,e:GetLabel())
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && ct > 0
        && ct == duel::get_matching_group(d, Some(&|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK)), tp, LOCATION_GRAVE, 0, NO_EX).len() as u32
        && {
            let lbl = effect::get_label(d, e) as u32;
            duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, lbl)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, ct, NO_EX)
        }
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let (e, tp) = (a.e(0), a.p(1));
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,e:GetLabel())
    let lbl = effect::get_label(d, e);
    let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, lbl as u32)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // e:GetLabelObject():SetLabel(e:GetLabel())
    let e5 = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, e5, &[lbl]);
}
/// `s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsPhase(PHASE_MAIN1) end
    if a.chk == Some(0) {
        return duel::is_phase(d, PHASE_MAIN1);
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BP)
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    true
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()==ATTRIBUTE_LIGHT then
        if effect::get_label(d, e) == ATTRIBUTE_LIGHT as i64 {
            // return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
            return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
        }
        // return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_HAND,1,nil,tp,POS_FACEDOWN)
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)), tp, 0, LOCATION_HAND, 1, NO_EX);
    }
    // if e:GetLabel()==ATTRIBUTE_LIGHT then
    if effect::get_label(d, e) == ATTRIBUTE_LIGHT as i64 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_HAND as i32);
        // e:SetProperty(0)
        effect::set_property(d, e, 0, 0);
    }
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if e:GetLabel()==ATTRIBUTE_LIGHT then
    if effect::get_label(d, e) == ATTRIBUTE_LIGHT as i64 {
        // local tc=Duel.GetFirstTarget()
        // if tc:IsRelateToEffect(e) then
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, e) {
            // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, CardSet::from_card(tc), POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    } else {
        // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_HAND,nil,tp,POS_FACEDOWN)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)), tp, 0, LOCATION_HAND, NO_EX);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // local rg=g:RandomSelect(tp,1)
        let rg = group::random_select(d, &g, tp, 1);
        // local tc=rg:GetFirst()
        let tc = group::get_first(d, &rg).nil();
        // Duel.Remove(tc,POS_FACEDOWN,REASON_EFFECT)
        duel::remove(d, CardSet::from_card(tc), POS_FACEDOWN as u8, REASON_EFFECT, None, None);
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetReset(RESET_PHASE|PHASE_END,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
        // e1:SetCondition(s.retcon)
        effect::set_condition(d, e1, Some(condition(retcon)));
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e).as_card().nil();
    // if tc:GetFlagEffect(id)==0 then
    if card::get_flag_effect(d, tc, ID) == 0 {
        // e:Reset()
        effect::reset(d, e);
        // return false
        false
    } else {
        // return Duel.IsTurnPlayer(1-tp)
        duel::is_turn_player(d, 1 - tp)
    }
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e).as_card().nil();
    // Duel.SendtoHand(tc,1-tp,REASON_EFFECT)
    duel::send_to_hand(d, CardSet::from_card(tc), Some(1 - tp), REASON_EFFECT, None);
}
