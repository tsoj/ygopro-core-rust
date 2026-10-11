//! Blooming of the Darkest Rose
use super::prelude::*;

pub const ID: u32 = 99092624;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_BLACK_ROSE_DRAGON, TOKEN_ROSE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // to deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_REMOVE+CATEGORY_TODECK+CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_TODECK + CATEGORY_LEAVE_GRAVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `ct` as computed in `s.target` and `s.activate`.
fn rose_count(d: &mut Duel, tp: u8) -> i32 {
    // local ct=Duel.GetMatchingGroupCount(Card.IsSequence,tp,LOCATION_SZONE,LOCATION_SZONE,nil,5)
    let mut ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_sequence(d, c, &[5])), tp, LOCATION_SZONE, LOCATION_SZONE, NO_EX) as i32;
    // ct=ct+Duel.GetMatchingGroupCount(Card.IsType,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,TYPE_FIELD)
    ct += duel::get_matching_group_count(d, Some(&|d, c| card::is_type(d, c, TYPE_FIELD)), tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX) as i32;
    ct
}
/// The token data of `TOKEN_ROSE` in `Duel.IsPlayerCanSpecialSummonMonster`.
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_ROSE as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(800),
        defense: Some(800),
        level: Some(2),
        race: Some(RACE_PLANT),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_ROSE,SET_ROSE,TYPES_TOKEN,800,800,2,RACE_PLANT,ATTRIBUTE_DARK,POS_FACEUP_DEFENSE,p)`
fn can_summon_token(d: &mut Duel, tp: u8, p: u8) -> bool {
    duel::is_player_can_special_summon_monster(d, tp, TOKEN_ROSE, token_data(), Some(POS_FACEUP_DEFENSE as u8), Some(p), 0)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let ct = rose_count(d, a.tp);
        // if ct==0 then return false end
        if ct == 0 {
            return false;
        }
        // for p=0,1 do
        for p in 0..2u8 {
            // if Duel.GetLocationCount(p,LOCATION_MZONE)>0 and Duel.IsPlayerCanSpecialSummonMonster(...) then return true end
            if duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon_token(d, a.tp, p) {
                return true;
            }
        }
        return false;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,0,PLAYER_ALL,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, PLAYER_ALL as u8, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,0,PLAYER_ALL,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 0, PLAYER_ALL as u8, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let mut ct = rose_count(d, a.tp);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    // repeat
    loop {
        // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsPlayerCanSpecialSummonMonster(...,tp)
        let b1 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon_token(d, a.tp, a.tp);
        // local b2=Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and Duel.IsPlayerCanSpecialSummonMonster(...,1-tp)
        let b2 = duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon_token(d, a.tp, 1 - a.tp);
        // if not (b1 or b2) then break end
        if !(b1 || b2) {
            break;
        }
        // local op=0
        // if b1 and b2 then op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
        // elseif b1 then op=Duel.SelectOption(tp,aux.Stringid(id,1))
        // else op=Duel.SelectOption(tp,aux.Stringid(id,2))+1 end
        let op = if b1 && b2 {
            duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)])
        } else if b1 {
            duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)])
        } else {
            duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]) + 1
        };
        // local p=tp
        // if op>0 then p=1-tp end
        let p = if op > 0 { 1 - a.tp } else { a.tp };
        // local token=Duel.CreateToken(tp,TOKEN_ROSE)
        let token = duel::create_token(d, a.tp, TOKEN_ROSE).nil();
        // Duel.SpecialSummonStep(token,0,tp,p,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, token, 0, a.tp, p, false, false, POS_FACEUP_DEFENSE as u8, None);
        // ct=ct-1
        ct -= 1;
        // until ct==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,0))
        if ct == 0 || !duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
            break;
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToRemove() and (c:IsCode(CARD_BLACK_ROSE_DRAGON) or c:IsRace(RACE_PLANT))
    card::is_faceup(d, c)
        && card::is_able_to_remove(d, c, None, Some(POS_FACEUP as u8), Some(REASON_EFFECT))
        && (card::is_code(d, c, &[CARD_BLACK_ROSE_DRAGON]) || card::is_race(d, c, RACE_PLANT))
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.rmfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && rmfilter(d, chkc);
    }
    // if chk==0 then return e:GetHandler():IsAbleToDeck() and Duel.IsExistingTarget(s.rmfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_able_to_deck(d, h) && duel::is_existing_target(d, Some(&rmfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&rmfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,0,REASON_EFFECT|REASON_TEMPORARY)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::remove(d, tc, 0, REASON_EFFECT | REASON_TEMPORARY, None, None) > 0 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
        effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // if Duel.IsPhase(PHASE_STANDBY) then
        if duel::is_phase(d, PHASE_STANDBY) {
            // e1:SetLabel(Duel.GetTurnCount())
            let turn = duel::get_turn_count(d, None) as i64;
            effect::set_label(d, e1, &[turn]);
            // e1:SetCondition(s.retcon)
            effect::set_condition(d, e1, Some(condition(retcon)));
            // e1:SetReset(RESET_PHASE|PHASE_STANDBY,2)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY, 2);
        } else {
            // e1:SetReset(RESET_PHASE|PHASE_STANDBY)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY, 1);
        }
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // if c:IsRelateToEffect(e) then Duel.SendtoDeck(c,nil,SEQ_DECKBOTTOM,REASON_EFFECT) end
        if card::is_relate_to_effect(d, c, a.e) {
            duel::send_to_deck(d, c, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
        }
    }
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()~=e:GetLabel()
    duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e)
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.ReturnToField(tc)
    duel::return_to_field(d, tc, None, None);
}
