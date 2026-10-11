//! Destiny HERO - Drawhand
use super::prelude::*;

pub const ID: u32 = 48150362;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Both players draw 1
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.drcon)
    effect::set_condition(d, e1, Some(condition(drcon)));
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Register the fact it was sent to GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetOperation(s.spreg)
    effect::set_operation(d, e2, Some(operation(spreg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Special summon itself from GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.drcon(e,tp,eg,ep,ev,re,r,rp)
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re and re:IsMonsterEffect() and re:GetHandler():IsSetCard(SET_HERO)
    let re = match a.re {
        Some(re) => re,
        None => return false,
    };
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_set_card(d, h, &[SET_HERO as u16])
}
// s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and Duel.IsPlayerCanDraw(1-tp,1) end
    if a.chk == Some(0) {
        if !duel::is_player_can_draw(d, a.tp, 1) {
            return false;
        }
        return duel::is_player_can_draw(d, 1 - a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_ALL,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 1);
    true
}
// s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // Duel.Draw(tp,1,REASON_EFFECT)
    duel::draw(d, a.tp, 1, REASON_EFFECT);
    // Duel.Draw(1-tp,1,REASON_EFFECT)
    duel::draw(d, 1 - a.tp, 1, REASON_EFFECT);
}
// s.spreg(e,tp,eg,ep,ev,re,r,rp)
fn spreg(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.IsPhase(PHASE_STANDBY) then
    if duel::is_phase(d, PHASE_STANDBY) {
        // e:SetLabel(Duel.GetTurnCount())
        let turn = duel::get_turn_count(d, None);
        effect::set_label(d, a.e, &[turn as i64]);
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY,0,2)
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY, 0, 2, 0, 0);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY,0,1)
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY, 0, 1, 0, 0);
    }
}
// s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetLabelObject():GetLabel()~=Duel.GetTurnCount() and e:GetHandler():GetFlagEffect(id)>0
    let label_obj = effect::get_label_object(d, a.e).as_effect().nil();
    let label = effect::get_label(d, label_obj);
    let turn = duel::get_turn_count(d, None);
    if label == turn as i64 {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, c, ID) > 0
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // c:ResetFlagEffect(id)
    card::reset_flag_effect(d, c, ID);
    true
}
// s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,1,tp,tp,false,false,POS_FACEUP)~=0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, CardSet::from_card(c), 1, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
        // --Banish it if it leaves the field
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
    }
}
