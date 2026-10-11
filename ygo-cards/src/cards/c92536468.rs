//! D/D/D Rebel King Leonidas
use super::prelude::*;

pub const ID: u32 = 92536468;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum Summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Destroy this card and reverse damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e1, EVENT_DAMAGE);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCondition(s.revcon1)
    effect::set_condition(d, e1, Some(condition(revcon1)));
    // e1:SetTarget(s.revtg)
    effect::set_target(d, e1, Some(cost_target(revtg)));
    // e1:SetOperation(s.revop)
    effect::set_operation(d, e1, Some(operation(revop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Workaround to have e1 trigger with "D/D/D Abyss King Gilgamesh"
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    // e2:SetCondition(s.revcon2)
    effect::set_condition(d, e2, Some(condition(revcon2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_DAMAGE)
        effect::set_code(d, ge1, EVENT_DAMAGE);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
    // Special Summon this card from your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_RECOVER)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_RECOVER);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e3, EVENT_DAMAGE);
    // e3:SetRange(LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_HAND);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // You take no effect damage
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e4, EFFECT_CHANGE_DAMAGE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // e4:SetValue(s.damval)
    effect::set_value_fn(d, e4, lua_fn(damval));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e5, EFFECT_NO_EFFECT_DAMAGE);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.revcon1(e,tp,eg,ep,ev,re,r,rp)`
fn revcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // if not (ep==tp and (r&REASON_EFFECT)~=0) then return false end
    if !(a.ep == a.tp && (a.r & REASON_EFFECT) != 0) {
        return false;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_CHAIN,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, 0, 0);
    // if Duel.GetFlagEffect(tp,id)>0 then Duel.ResetFlagEffect(tp,id) end
    if duel::get_flag_effect(d, a.tp, ID) > 0 {
        duel::reset_flag_effect(d, a.tp, ID);
    }
    // return true
    true
}
/// `s.revcon2(e,tp,eg,ep,ev,re,r,rp)`
fn revcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetFlagEffect(id)>0 then
    if card::get_flag_effect(d, c, ID) > 0 {
        // c:ResetFlagEffect(id)
        card::reset_flag_effect(d, c, ID);
        // return false
        return false;
    }
    // if Duel.GetFlagEffect(tp,id)==0 then return false end
    if duel::get_flag_effect(d, a.tp, ID) == 0 {
        return false;
    }
    // Duel.ResetFlagEffect(tp,id)
    duel::reset_flag_effect(d, a.tp, ID);
    // return Duel.CheckEvent(EVENT_DAMAGE)
    duel::check_event(d, EVENT_DAMAGE).is_some()
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if (r&REASON_EFFECT)~=0 then
    if (a.r & REASON_EFFECT) != 0 {
        // Duel.RegisterFlagEffect(ep,id,RESET_CHAIN,0,1)
        duel::register_flag_effect(d, a.ep, ID, RESET_CHAIN, 0, 1, 0);
    }
}
/// `s.revtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn revtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.revop(e,tp,eg,ep,ev,re,r,rp)`
fn revop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.Destroy(c,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::destroy(d, c, REASON_EFFECT, None, None) > 0 {
        // Any effect that would inflict damage increases LP instead
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_REVERSE_DAMAGE)
        effect::set_code(d, e1, EFFECT_REVERSE_DAMAGE);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetTargetRange(1,1)
        effect::set_target_range(d, e1, 1, 1);
        // e1:SetValue(s.revval)
        effect::set_value_fn(d, e1, lua_fn(revval));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
/// `s.revval(e,re,r,rp,rc)`
fn revval(_d: &mut Duel, a: Args) -> bool {
    // return (r&REASON_EFFECT)~=0
    (a.int(2) as u32 & REASON_EFFECT) != 0
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and (r&REASON_EFFECT)~=0
    a.ep == a.tp && (a.r & REASON_EFFECT) != 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,ev)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, a.ev as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Duel.Recover(tp,ev,REASON_EFFECT)
        duel::recover(d, a.tp, a.ev as i64, REASON_EFFECT, false, None);
    }
}
/// `s.damval(e,re,val,r,rp,rc)`
fn damval(_d: &mut Duel, a: Args) -> i64 {
    // if (r&REASON_EFFECT)~=0 then return 0 end
    if (a.int(3) as u32 & REASON_EFFECT) != 0 {
        return 0;
    }
    // return val
    a.int(2)
}
