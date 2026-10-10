//! Mimikuril
use super::prelude::*;

pub const ID: u32 = 74879881;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // excavate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,1) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 1) && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not Duel.IsPlayerCanDiscardDeck(tp,1) then return end
    if !duel::is_player_can_discard_deck(d, a.tp, 1) {
        return;
    }
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, a.tp, 1);
    // local tc=Duel.GetDecktopGroup(tp,1):GetFirst()
    let decktop = duel::get_decktop_group(d, a.tp, 1);
    let tc = group::get_first(d, &decktop).nil();
    // if tc:IsMonster() and tc:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    if card::is_monster(d, tc)
        && card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, None, None, None)
        && duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None)
    {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // local fid=c:GetFieldID()
        let fid = card::get_field_id(d, c);
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e2:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
        // e2:SetCountLimit(1)
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        // e2:SetLabel(fid)
        effect::set_label(d, e2, &[fid as i64]);
        // e2:SetLabelObject(tc)
        effect::set_label_object(d, e2, Value::Card(tc));
        // e2:SetCondition(s.tdcon)
        effect::set_condition(d, e2, Some(condition(tdcon)));
        // e2:SetOperation(s.tdop)
        effect::set_operation(d, e2, Some(operation(tdop)));
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
        // if c:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, c, a.e) {
            // Duel.SendtoDeck(c,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
            duel::send_to_deck(d, CardSet::from_card(c), None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    } else {
        // Duel.MoveSequence(tc,1)
        duel::move_sequence(d, tc, 1, None);
    }
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffectLabel(id)~=e:GetLabel() then
    let label = card::get_flag_effect_label(d, tc, ID).first().copied();
    if label != Some(effect::get_label(d, a.e)) {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // else return true end
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // Duel.SendtoDeck(e:GetLabelObject(),nil,SEQ_DECKBOTTOM,REASON_EFFECT)
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
}
