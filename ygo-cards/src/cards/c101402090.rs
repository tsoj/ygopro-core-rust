//! Angelechy Castellan
//! JP Name
use super::prelude::*;

pub const ID: u32 = 101402090;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ANGELECHY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.bantg)
    effect::set_target(d, e1, Some(cost_target(bantg)));
    // e1:SetOperation(s.banop)
    effect::set_operation(d, e1, Some(operation(banop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_MOVE)
    effect::set_code(d, e2, EVENT_MOVE);
    // e2:SetRange(LOCATION_STZONE)
    effect::set_range(d, e2, LOCATION_STZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.tgcon)
    effect::set_condition(d, e2, Some(condition(tgcon)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3a:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3a, EVENT_CHAINING);
    // e3a:SetRange(LOCATION_STZONE)
    effect::set_range(d, e3a, LOCATION_STZONE);
    // e3a:SetCondition(function(e) return e:GetHandler():IsContinuousSpell() end)
    effect::set_condition(d, e3a, Some(lua_fn(|d, a: Args| {
        let h = effect::get_handler(d, a.e(0)).nil();
        card::is_continuous_spell(d, h)
    })));
    // e3a:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) e:GetHandler():RegisterFlagEffect(id+(ep*100),RESET_EVENT|RESETS_STANDARD|RESET_CHAIN,0,1) end)
    effect::set_operation(d, e3a, Some(operation(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::register_flag_effect(d, h, ID + a.ep as u32 * 100, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, 0, 0);
    })));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local e3b=Effect.CreateEffect(c)
    let e3b = effect::create_effect(d, c);
    // e3b:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3b, EFFECT_TYPE_FIELD);
    // e3b:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3b, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3b:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3b, EFFECT_CANNOT_ACTIVATE);
    // e3b:SetRange(LOCATION_STZONE)
    effect::set_range(d, e3b, LOCATION_STZONE);
    // e3b:SetTargetRange(1,1)
    effect::set_target_range(d, e3b, 1, 1);
    // e3b:SetCondition(function(e) return e:GetHandler():IsContinuousSpell() end)
    effect::set_condition(d, e3b, Some(lua_fn(|d, a: Args| {
        let h = effect::get_handler(d, a.e(0)).nil();
        card::is_continuous_spell(d, h)
    })));
    // e3b:SetValue(function(e,re,rp) return e:GetHandler():HasFlagEffect(id+(rp*100)) end)
    effect::set_value_fn(d, e3b, lua_fn(|d, a: Args| {
        let h = effect::get_handler(d, a.e(0)).nil();
        card::has_flag_effect(d, h, ID + a.p(2) as u32 * 100, None)
    }));
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
}
// s.normalizeseq(seq)
fn normalizeseq(seq: i64) -> i64 {
    // if seq==5 then return 1 end
    if seq == 5 {
        return 1;
    }
    // if seq==6 then return 3 end
    if seq == 6 {
        return 3;
    }
    // return seq
    seq
}
// s.banfilter(c,seq)
fn banfilter(d: &mut Duel, c: CardId, seq: i64) -> bool {
    // if not c:IsAbleToRemove() then return false end
    if !card::is_able_to_remove(d, c, None, None, None) {
        return false;
    }
    // local dist=math.abs(seq-s.normalizeseq(c:GetSequence()))
    let dist = (seq - normalizeseq(card::get_sequence(d, c) as i64)).abs();
    // if c:IsLocation(LOCATION_MZONE) then return dist==1 end
    if card::is_location(d, c, LOCATION_MZONE) {
        return dist == 1;
    }
    // return dist==2
    dist == 2
}
// s.bantg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local seq=4-s.normalizeseq(e:GetHandler():GetSequence())
    let h = effect::get_handler(d, a.e).nil();
    let seq = 4 - normalizeseq(card::get_sequence(d, h) as i64);
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE|LOCATION_STZONE) and s.banfilter(chkc,seq) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE | LOCATION_STZONE) && banfilter(d, chkc, seq);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.banfilter,tp,0,LOCATION_MZONE|LOCATION_STZONE,1,nil,seq) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| banfilter(d, c, seq)), tp, 0, LOCATION_MZONE | LOCATION_STZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.banfilter,tp,0,LOCATION_MZONE|LOCATION_STZONE,1,1,nil,seq)
    let g = duel::select_target(d, tp, Some(&|d, c| banfilter(d, c, seq)), tp, 0, LOCATION_MZONE | LOCATION_STZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, tp, 0);
    true
}
// s.banop(e,tp,eg,ep,ev,re,r,rp)
fn banop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then Duel.Remove(tc,POS_FACEUP,REASON_EFFECT) end
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
// s.tgcon(e,tp,eg,ep,ev,re,r,rp)
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return eg:IsContains(c) and not c:IsPreviousLocation(LOCATION_SZONE)
    let eg = a.eg.clone().nil();
    group::is_contains(&eg, c) && !card::is_previous_location(d, c, LOCATION_SZONE)
}
// s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ANGELECHY) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_ANGELECHY as u16]) && card::is_able_to_grave(d, c, None)
}
// s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SendtoGrave(g,REASON_EFFECT) end
    if !g.is_empty() {
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
