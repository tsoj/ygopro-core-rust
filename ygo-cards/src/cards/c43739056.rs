//! Amanokujaki
use super::prelude::*;

pub const ID: u32 = 43739056;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // give control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e2, CATEGORY_CONTROL);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    let re = a.e(1);
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // return rc~=c and re:IsMonsterEffect() and rc:IsOnField() and rc:IsSpecialSummoned() and rc:IsAttribute(c:GetAttribute())
    if rc == c || !effect::is_monster_effect(d, re) {
        return false;
    }
    let att = card::get_attribute(d, c);
    card::is_on_field(d, rc) && card::is_special_summoned(d, rc) && card::is_attribute(d, rc, att)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let _ph = duel::get_current_phase(d);
    // return Duel.GetTurnPlayer()==e:GetHandler():GetOwner() and Duel.IsMainPhase()
    let c = effect::get_handler(d, a.e).nil();
    duel::get_turn_player(d) == card::get_owner(d, c) && duel::is_main_phase(d, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsAbleToChangeControler() end
    if a.chk == Some(0) {
        return card::is_able_to_change_controler(d, c);
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.GetControl(c,1-tp) and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if card::is_relate_to_effect(d, c, e) {
        // Duel.GetControl returns a number, which is true in Lua whatever its value
        let _ = duel::get_control(d, c, 1 - tp, 0, 0, None, None);
        if duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
            // local att=c:AnnounceAnotherAttribute(tp)
            let att = card::announce_another_attribute(d, c, tp);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
            effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
            // e1:SetValue(att)
            effect::set_value(d, e1, att as i64);
            // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END,2)
            effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 2);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
