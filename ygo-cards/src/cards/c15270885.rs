//! Toon Goblin Attack Force
use super::prelude::*;

pub const ID: u32 = 15270885;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[15259703], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot attack the turn it was summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.atklimit)
    effect::set_operation(d, e1, Some(operation(atklimit)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy itself if "Toon World" is destroyed
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    // e4:SetCondition(s.sdescon)
    effect::set_condition(d, e4, Some(condition(sdescon)));
    // e4:SetOperation(s.sdesop)
    effect::set_operation(d, e4, Some(operation(sdesop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Can attack directly
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e5, EFFECT_DIRECT_ATTACK);
    // e5:SetCondition(s.dircon)
    effect::set_condition(d, e5, Some(condition(dircon)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Change itself to defense position
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e6:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e6, EVENT_PHASE | PHASE_BATTLE);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetCondition(s.poscon)
    effect::set_condition(d, e6, Some(condition(poscon)));
    // e6:SetOperation(s.posop)
    effect::set_operation(d, e6, Some(operation(posop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.atklimit(e,tp,eg,ep,ev,re,r,rp)`
fn atklimit(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, h, e1, false);
}
/// `s.sfilter(c)`
fn sfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsReason(REASON_DESTROY) and c:IsPreviousPosition(POS_FACEUP) and c:GetPreviousCodeOnField()==15259703 and c:IsPreviousLocation(LOCATION_ONFIELD)
    card::is_reason(d, c, REASON_DESTROY)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::get_previous_code_on_field(d, c).first().copied() == Some(15259703)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
/// `s.sdescon(e,tp,eg,ep,ev,re,r,rp)`
fn sdescon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.sfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &sfilter, 1, NO_EX)
}
/// `s.sdesop(e,tp,eg,ep,ev,re,r,rp)`
fn sdesop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
/// `s.dircon(e)`
fn dircon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,15259703),e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil)
    //     and not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_TOON),e:GetHandlerPlayer(),0,LOCATION_MZONE,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[15259703])),
        p,
        LOCATION_ONFIELD,
        0,
        1,
        NO_EX,
    ) && !duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_TOON)),
        p,
        0,
        LOCATION_MZONE,
        1,
        NO_EX,
    )
}
/// `s.poscon(e,tp,eg,ep,ev,re,r,rp)`
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetAttackedCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_attacked_count(d, h) > 0
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsAttackPos() then
    if card::is_attack_pos(d, c) {
        // Duel.ChangePosition(c,POS_FACEUP_DEFENSE)
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
    // Change itself to defense position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(3313)
    effect::set_description(d, e1, 3313);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_CHANGE_POSITION)
    effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_COPY_INHERIT+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_COPY_INHERIT + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetReset(RESETS_STANDARD_PHASE_END,3)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 3);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
