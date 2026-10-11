//! Archfiend's Advent
//! デーモンの光来
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 53008933;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SHINING_SARCOPHAGUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can be Normal Summoned without Tributing if you control "Shining Sarcophagus"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.notribcon)
    effect::set_condition(d, e1, Some(lua_fn(notribcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Take control of 1 of your opponent's monsters until the End Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e2, CATEGORY_CONTROL);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.ctrltg)
    effect::set_target(d, e2, Some(cost_target(ctrltg)));
    // e2:SetOperation(s.ctrlop)
    effect::set_operation(d, e2, Some(operation(ctrlop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Other monsters you control gain 500 ATK during your turn
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e4, EFFECT_UPDATE_ATTACK);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e4, LOCATION_MZONE, 0);
    // e4:SetCondition(function(e) return Duel.IsTurnPlayer(e:GetHandlerPlayer()) end)
    effect::set_condition(d, e4, Some(condition(|d, a| duel::is_turn_player(d, effect::get_handler_player(d, a.e)))));
    // e4:SetTarget(function(e,c) return c~=e:GetHandler() end)
    effect::set_target(d, e4, Some(effect_card_fn(|d, e, c| c != effect::get_handler(d, e).nil())));
    // e4:SetValue(500)
    effect::set_value(d, e4, 500);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.notribcon(e,c)`
fn notribcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_SHINING_SARCOPHAGUS),tp,LOCATION_ONFIELD,0,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_SHINING_SARCOPHAGUS]))), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.ctrltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn ctrltg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsControlerCanBeChanged() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - tp)
            && card::is_controler_can_be_changed(d, chkc, false, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, tp, 0);
    // local cd=e:GetChainData()
    // cd.controlled_sarcophagus=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_SHINING_SARCOPHAGUS),tp,LOCATION_ONFIELD,0,1,nil)
    let b = duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_SHINING_SARCOPHAGUS]))), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    chain::set_effect_data(d, e, None, chain::field::CARD, Value::Bool(b));
    true
}
/// `s.ctrlop(e,tp,eg,ep,ev,re,r,rp)`
fn ctrlop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local cd=e:GetChainData()
    // if tc:IsRelateToEffect(e) and Duel.GetControl(tc,tp,PHASE_END,1) and not cd.controlled_sarcophagus then
    if card::is_relate_to_effect(d, tc, e) {
        // Duel.GetControl always returns a number (true in Lua), so the call is made and the test is true
        duel::get_control(d, tc, tp, PHASE_END as u16, 1, None, None);
        let controlled = chain::effect_data(d, e, None, chain::field::CARD).truthy();
        if !controlled {
            // Cannot attack while you control it
            // local e1=Effect.CreateEffect(e:GetHandler())
            let handler = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, handler);
            // e1:SetDescription(3206)
            effect::set_description(d, e1, 3206);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_CONTROL)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CONTROL, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
