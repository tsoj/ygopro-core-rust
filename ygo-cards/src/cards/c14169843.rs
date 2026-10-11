//! Aromage Laurel
use super::prelude::*;

pub const ID: u32 = 14169843;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // become tuner
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_RECOVER)
    effect::set_code(d, e2, EVENT_RECOVER);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.tuncon)
    effect::set_condition(d, e2, Some(condition(tuncon)));
    // e2:SetTarget(s.tuntg)
    effect::set_target(d, e2, Some(cost_target(tuntg)));
    // e2:SetOperation(s.tunop)
    effect::set_operation(d, e2, Some(operation(tunop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // remove
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e3, CATEGORY_RECOVER);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetTarget(s.rectg)
    effect::set_target(d, e3, Some(cost_target(rectg)));
    // e3:SetOperation(s.recop)
    effect::set_operation(d, e3, Some(operation(recop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.spcon(e)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // return Duel.GetLP(tp)>Duel.GetLP(1-tp)
    duel::get_lp(d, tp) > duel::get_lp(d, 1 - tp)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        let h = effect::get_handler(d, a.e).nil();
        return card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.tuncon(e,tp,eg,ep,ev,re,r,rp)
fn tuncon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp
    a.ep == a.tp
}
// function s.tunfilter(c)
fn tunfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_PLANT) and not c:IsType(TYPE_TUNER)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_PLANT) && !card::is_type(d, c, TYPE_TUNER)
}
// function s.tuntg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tuntg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.tunfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && tunfilter(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SELF as u64);
    // Duel.SelectTarget(tp,s.tunfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&tunfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
// function s.tunop(e,tp,eg,ep,ev,re,r,rp)
fn tunop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_ADD_TYPE)
            effect::set_code(d, e1, EFFECT_ADD_TYPE);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // e1:SetValue(TYPE_TUNER)
            effect::set_value(d, e1, TYPE_TUNER as i64);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
// function s.rectg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(500)
    duel::set_target_param(d, 500);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,500)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 500);
    true
}
// function s.recop(e,tp,eg,ep,ev,re,r,rp)
fn recop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Recover(p,d,REASON_EFFECT)
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
