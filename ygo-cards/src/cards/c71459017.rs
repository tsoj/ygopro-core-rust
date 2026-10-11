//! Hinezumi Hanabi
use super::prelude::*;

pub const ID: u32 = 71459017;
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x203)
    card::enable_counter_permit(d, c, 0x203, None, None);
    // Cannot be used as material for a Fusion/Synchro/Xyz/Link Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_MATERIAL);
    // e1:SetValue(aux.cannotmatfilter(SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK))
    effect::set_value_fn(d, e1, aux::cannotmatfilter(&[SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_XYZ, SUMMON_TYPE_LINK]));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place counters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e2, CATEGORY_COUNTER);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetTarget(s.cttg)
    effect::set_target(d, e2, Some(cost_target(cttg)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Give control to your opponent
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e4, CATEGORY_CONTROL);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.ctlcon)
    effect::set_condition(d, e4, Some(condition(ctlcon)));
    // e4:SetTarget(s.ctltg)
    effect::set_target(d, e4, Some(cost_target(ctltg)));
    // e4:SetOperation(s.ctlop)
    effect::set_operation(d, e4, Some(operation(ctlop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Remove counters
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e5, aux::stringid(ID, 2));
    // e5:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE+CATEGORY_DICE)
    effect::set_category(d, e5, CATEGORY_DESTROY + CATEGORY_DAMAGE + CATEGORY_DICE);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e5:SetCode(EVENT_CONTROL_CHANGED)
    effect::set_code(d, e5, EVENT_CONTROL_CHANGED);
    // e5:SetCondition(s.descon)
    effect::set_condition(d, e5, Some(condition(descon)));
    // e5:SetTarget(s.destg)
    effect::set_target(d, e5, Some(cost_target(destg)));
    // e5:SetOperation(s.desop)
    effect::set_operation(d, e5, Some(operation(desop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,3,0,0x203)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 3, 0, 0x203);
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // e:GetHandler():AddCounter(0x203,6)
        card::add_counter(d, h, 0x203, 6, false);
    }
}
/// `s.ctlcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctlcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.ctltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ctltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsControlerCanBeChanged() end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_controler_can_be_changed(d, h, false, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.ctlop(e,tp,eg,ep,ev,re,r,rp)`
fn ctlop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // Duel.GetControl(c,1-tp)
        duel::get_control(d, c, 1 - a.tp, 0, 0, None, None);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetCounter(0x203)>0
    card::get_counter(d, effect::get_handler(d, a.e).nil(), 0x203) > 0
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsCanRemoveCounter(tp,0x203,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, effect::get_handler(d, a.e).nil(), a.tp, 0x203, 1, REASON_EFFECT);
    }
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsCanRemoveCounter(tp,0x203,1,REASON_EFFECT) then return end
    if !card::is_can_remove_counter(d, c, a.tp, 0x203, 1, REASON_EFFECT) {
        return;
    }
    // local ct=math.min(Duel.TossDice(tp,1),c:GetCounter(0x203))
    let dice = duel::toss_dice(d, a.tp, 1, 0)[0];
    let ct = std::cmp::min(dice, card::get_counter(d, c, 0x203));
    // c:RemoveCounter(tp,0x203,ct,REASON_EFFECT)
    card::remove_counter(d, c, a.tp, 0x203, ct as u16, REASON_EFFECT);
    // if c:GetCounter(0x203)==0 and Duel.Destroy(c,REASON_EFFECT)>0 then
    if card::get_counter(d, c, 0x203) == 0 && duel::destroy(d, c, REASON_EFFECT, None, None) > 0 {
        // Duel.Damage(tp,2000,REASON_EFFECT)
        duel::damage(d, a.tp, 2000, REASON_EFFECT, false, None);
    }
}
