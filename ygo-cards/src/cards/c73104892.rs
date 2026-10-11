//! Maple Maiden
use super::prelude::*;

pub const ID: u32 = 73104892;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon Procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Change Attribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.atttg)
    effect::set_target(d, e1, Some(cost_target(atttg)));
    // e1:SetOperation(s.attop)
    effect::set_operation(d, e1, Some(operation(attop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make the opponent send 1 card to GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.gytg)
    effect::set_target(d, e2, Some(cost_target(gytg)));
    // e2:SetOperation(s.gyop)
    effect::set_operation(d, e2, Some(operation(gyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTRIBUTE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTRIBUTE as u64);
    // local rc=Duel.AnnounceAttribute(tp,1,ATTRIBUTE_ALL)
    let rc = duel::announce_attribute(d, tp, 1, ATTRIBUTE_ALL);
    // Duel.SetTargetParam(rc)
    duel::set_target_param(d, rc);
    true
}
/// `s.attop(e,tp,eg,ep,ev,re,r,rp)`
fn attop(d: &mut Duel, a: &EffArgs) {
    let (e, _tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local rc=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let rc = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
    // e1:SetValue(rc)
    effect::set_value(d, e1, rc);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.optadd(loc,seq,player)` inside `s.group`: `if not player then player=tp end`, then `g:AddCard` of the field card
fn optadd(d: &mut Duel, g: &mut CardSet, tp: u8, loc: u32, seq: i64, player: Option<u8>) {
    let player = player.unwrap_or(tp);
    if let Some(c) = duel::get_field_card(d, player, loc, seq as u32) {
        group::add_card(g, &CardSet::from_card(c));
    }
}
/// `s.group(seq,tp)`
fn group_of(d: &mut Duel, seq: i64, tp: u8) -> CardSet {
    // local g=Group.CreateGroup()
    let mut g = CardSet::new();
    // if seq+1<=4 then optadd(LOCATION_MZONE,seq+1) end
    if seq + 1 <= 4 {
        optadd(d, &mut g, tp, LOCATION_MZONE, seq + 1, None);
    }
    // if seq-1>=0 then optadd(LOCATION_MZONE,seq-1) end
    if seq - 1 >= 0 {
        optadd(d, &mut g, tp, LOCATION_MZONE, seq - 1, None);
    }
    // if seq<5 then
    if seq < 5 {
        // optadd(LOCATION_SZONE,seq)
        optadd(d, &mut g, tp, LOCATION_SZONE, seq, None);
        // if seq==1 then
        if seq == 1 {
            // optadd(LOCATION_MZONE,5)
            optadd(d, &mut g, tp, LOCATION_MZONE, 5, None);
            // optadd(LOCATION_MZONE,6,1-tp)
            optadd(d, &mut g, tp, LOCATION_MZONE, 6, Some(1 - tp));
        }
        // if seq==3 then
        if seq == 3 {
            // optadd(LOCATION_MZONE,6)
            optadd(d, &mut g, tp, LOCATION_MZONE, 6, None);
            // optadd(LOCATION_MZONE,5,1-tp)
            optadd(d, &mut g, tp, LOCATION_MZONE, 5, Some(1 - tp));
        }
    // elseif seq==5 then
    } else if seq == 5 {
        // optadd(LOCATION_MZONE,1)
        optadd(d, &mut g, tp, LOCATION_MZONE, 1, None);
        // optadd(LOCATION_MZONE,3,1-tp)
        optadd(d, &mut g, tp, LOCATION_MZONE, 3, Some(1 - tp));
    // elseif seq==6 then
    } else if seq == 6 {
        // optadd(LOCATION_MZONE,3)
        optadd(d, &mut g, tp, LOCATION_MZONE, 3, None);
        // optadd(LOCATION_MZONE,1,1-tp)
        optadd(d, &mut g, tp, LOCATION_MZONE, 1, Some(1 - tp));
    }
    // return g
    g
}
/// `s.gyfilter(c,tp)`
fn gyfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return #(s.group(c:GetSequence(),1-tp))>0
    let seq = card::get_sequence(d, c) as i64;
    !group_of(d, seq, 1 - tp).is_empty()
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MMZONE) and s.gyfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MMZONE) && gyfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.gyfilter,tp,0,LOCATION_MMZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| gyfilter(d, c, tp)), tp, 0, LOCATION_MMZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.gyfilter,tp,0,LOCATION_MMZONE,1,1,nil,tp)
    duel::select_target(d, tp, Some(&|d, c| gyfilter(d, c, tp)), tp, 0, LOCATION_MMZONE, 1, 1, false, NO_EX);
    true
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) {
        // local g=s.group(tc:GetSequence(),1-tp)
        let seq = card::get_sequence(d, tc) as i64;
        let g = group_of(d, seq, 1 - tp);
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_TOGRAVE as u64);
            // local sg=g:Select(1-tp,1,1,nil)
            let sg = group::select(d, &g, 1 - tp, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(sg,true)
            duel::hint_selection(d, sg.clone(), true);
            // Duel.SendtoGrave(sg,REASON_RULE,PLAYER_NONE,1-tp)
            duel::send_to_grave(d, sg, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - tp));
        }
    }
}
