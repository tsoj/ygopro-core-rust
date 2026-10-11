//! Infernity Suppression
use super::prelude::*;

pub const ID: u32 = 12541409;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INFERNITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate monster effect and inflict damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can be activated during the turn it was Set
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_SET_TURN);
    // e2:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e2:SetCondition(s.actcon)
    effect::set_condition(d, e2, Some(lua_fn(actcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsMonsterEffect() and Duel.IsChainDisablable(ev)
    a.rp == 1 - a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && duel::is_chain_disablable(d, a.ev as u8)
        // and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_INFERNITY),tp,LOCATION_MZONE,0,1,nil)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_INFERNITY as u16])),
            a.tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        )
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():HasLevel() then
    let rc = effect::get_handler(d, a.re.nil()).nil();
    if card::has_level(d, rc) {
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,re:GetHandler():GetLevel()*100)
        let rc = effect::get_handler(d, a.re.nil()).nil();
        let lvl = card::get_level(d, rc);
        duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (lvl * 100) as i32);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.NegateEffect(ev) and re:GetHandler():HasLevel() and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if duel::negate_effect(d, a.ev as u8) {
        let rc = effect::get_handler(d, a.re.nil()).nil();
        if card::has_level(d, rc) && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local dam=re:GetHandler():GetLevel()*100
            let rc = effect::get_handler(d, a.re.nil()).nil();
            let dam = card::get_level(d, rc) as i64 * 100;
            // if dam>0 then
            if dam > 0 {
                // Duel.Damage(1-tp,dam,REASON_EFFECT)
                duel::damage(d, 1 - tp, dam, REASON_EFFECT, false, None);
            }
        }
    }
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_HAND,0)==0
    let p = effect::get_handler_player(d, e);
    duel::get_field_group_count(d, p, LOCATION_HAND, 0) == 0
}
