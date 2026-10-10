//! PSY-Framelord Zeta
use super::prelude::*;

pub const ID: u32 = 37192109;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PSY_FRAME], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // remove
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_END_PHASE));
    // e1:SetHintTiming(TIMING_SPSUMMON)
    effect::set_hint_timing(d, e1, TIMING_SPSUMMON, None);
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEUP_ATTACK) and c:IsAbleToRemove() and c:IsSpecialSummoned()
    card::is_position(d, c, POS_FACEUP_ATTACK) && card::is_able_to_remove(d, c, None, None, None) && card::is_special_summoned(d, c)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.rmfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && rmfilter(d, chkc);
    }
    // if chk==0 then return e:GetHandler():IsAbleToRemove()
    //     and Duel.IsExistingTarget(s.rmfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_able_to_remove(d, h, None, None, None) && duel::is_existing_target(d, Some(&rmfilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmfilter,tp,0,LOCATION_MZONE,1,1,nil)
    let mut g = duel::select_target(d, tp, Some(&rmfilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // g:AddCard(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    group::add_card(&mut g, &CardSet::from_card(h));
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 2, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not c:IsRelateToEffect(e) or not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) || !card::is_relate_to_effect(d, tc.nil(), a.e) {
        return;
    }
    // local g=Group.FromCards(c,tc)
    let g = CardSet::from_iter([c, tc.nil()]);
    // if Duel.Remove(g,0,REASON_EFFECT|REASON_TEMPORARY)~=0 then
    if duel::remove(d, g, 0, REASON_EFFECT | REASON_TEMPORARY, None, None) != 0 {
        // local fid=c:GetFieldID()
        let fid = card::get_field_id(d, c) as i64;
        // local rct=1
        let mut rct: u16 = 1;
        // if Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_STANDBY) then rct=2 end
        if duel::is_turn_player(d, tp) && duel::is_phase(d, PHASE_STANDBY) {
            rct = 2;
        }
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // local oc=og:GetFirst()
        let _ = group::get_first(d, &og);
        // for oc in aux.Next(og) do
        for oc in group::iter(d, &og) {
            // if oc:IsControler(tp) then
            if card::is_controler(d, oc, tp) {
                // oc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,0,rct,fid)
                card::register_flag_effect(d, oc, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 0, rct, fid, 0);
            } else {
                // oc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_OPPO_TURN,0,rct,fid)
                card::register_flag_effect(d, oc, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_OPPO_TURN, 0, rct, fid, 0);
            }
        }
        // og:KeepAlive() (groups are values in Rust)
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
        effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetLabel(fid)
        effect::set_label(d, e1, &[fid]);
        // e1:SetLabelObject(og)
        effect::set_label_object(d, e1, Value::Group(og));
        // e1:SetCondition(s.retcon)
        effect::set_condition(d, e1, Some(condition(retcon)));
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // if Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_STANDBY) then
        if duel::is_turn_player(d, tp) && duel::is_phase(d, PHASE_STANDBY) {
            // e1:SetReset(RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,2)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 2);
            // e1:SetValue(Duel.GetTurnCount())
            let tc_count = duel::get_turn_count(d, None) as i64;
            effect::set_value(d, e1, tc_count);
        } else {
            // e1:SetReset(RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 1);
            // e1:SetValue(0)
            effect::set_value(d, e1, 0);
        }
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.retfilter(c,fid)`
fn retfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if Duel.IsTurnPlayer(1-tp) or Duel.GetTurnCount()==e:GetValue() then return false end
    if duel::is_turn_player(d, 1 - a.tp) {
        return false;
    }
    let turn_count = duel::get_turn_count(d, None) as i64;
    if turn_count == effect::get_value(d, a.e).int() {
        return false;
    }
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let fid = effect::get_label(d, a.e);
    // if not g:IsExists(s.retfilter,1,nil,e:GetLabel()) then
    if !group::is_exists(d, &g, &|d, c| retfilter(d, c, fid), 1, NO_EX) {
        // g:DeleteGroup() (nothing in Rust)
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // else return true end
    true
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let fid = effect::get_label(d, a.e);
    // local sg=g:Filter(s.retfilter,nil,e:GetLabel())
    let mut sg = group::filter(d, &g, &|d, c| retfilter(d, c, fid), NO_EX);
    // g:DeleteGroup() (nothing in Rust)
    // if #sg>1 and sg:GetClassCount(Card.GetPreviousControler)==1 then
    if sg.len() > 1 && group::get_class_count(d, &sg, &|d, c| card::get_previous_controler(d, c) as i64) == 1 {
        // local ft=Duel.GetLocationCount(sg:GetFirst():GetPreviousControler(),LOCATION_MZONE)
        let first = group::get_first(d, &sg).nil();
        let prev_controler = card::get_previous_controler(d, first);
        let ft = duel::get_location_count(d, prev_controler, LOCATION_MZONE as u8, None, None, None).0;
        // if ft==1 then
        if ft == 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
            duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
            // local tc=sg:Select(tp,1,1,nil):GetFirst()
            let selected = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
            let tc = group::get_first(d, &selected).nil();
            // Duel.ReturnToField(tc)
            duel::return_to_field(d, tc, None, None);
            // sg:RemoveCard(tc)
            group::remove_card(&mut sg, &CardSet::from_card(tc));
        }
    }
    // local tc=sg:GetFirst()
    let _ = group::get_first(d, &sg);
    // for tc in aux.Next(sg) do
    for tc in group::iter(d, &sg) {
        // Duel.ReturnToField(tc)
        duel::return_to_field(d, tc, None, None);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PSY_FRAME) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_PSY_FRAME as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && thfilter(d, chkc) && chkc != effect::get_handler(d, a.e).nil();
    }
    // if chk==0 then return e:GetHandler():IsAbleToExtra()
    //     and Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,e:GetHandler()) end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_extra(d, h) && duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,e:GetHandler())
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)~=0
    //     and c:IsLocation(LOCATION_EXTRA) and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_deck(d, CardSet::from_card(c), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) != 0
        && card::is_location(d, c, LOCATION_EXTRA)
        && card::is_relate_to_effect(d, tc.nil(), a.e)
    {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tc.nil()), None, REASON_EFFECT, None);
    }
}
