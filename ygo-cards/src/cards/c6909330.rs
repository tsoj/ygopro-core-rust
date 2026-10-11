//! Soul Binding Gate
use super::prelude::*;

pub const ID: u32 = 6909330;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[62499965],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    //     local ge1=Effect.CreateEffect(c)
    //     ...
    // end)
    aux::global_check(d, ID, move |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
        effect::set_type(d, ge1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
        // ge1:SetCode(EVENT_DESTROYED)
        effect::set_code(d, ge1, EVENT_DESTROYED);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
    // Destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    // for tc in aux.Next(eg) do
    for tc in group::iter(d, &eg) {
        // if tc:GetPreviousTypeOnField() & (TYPE_SPELL+TYPE_TRAP)~=0 and tc:IsPreviousPosition(POS_FACEDOWN)
        //     and tc:IsPreviousLocation(LOCATION_ONFIELD) and tc:IsReason(REASON_EFFECT) then
        if (card::get_previous_type_on_field(d, tc) & (TYPE_SPELL + TYPE_TRAP)) != 0
            && card::is_previous_position(d, tc, POS_FACEDOWN)
            && card::is_previous_location(d, tc, LOCATION_ONFIELD)
            && card::is_reason(d, tc, REASON_EFFECT)
        {
            // Duel.RegisterFlagEffect(tc:GetPreviousControler(),id,RESET_PHASE|PHASE_END,0,1)
            let prev = card::get_previous_controler(d, tc);
            duel::register_flag_effect(d, prev, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFlagEffect(tp,id)>0 and Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    duel::get_flag_effect(d, a.tp, ID) > 0 && duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg);
    // return Duel.IsMainPhase() and #eg==1 and tc:IsFaceup() and tc:GetAttack()<Duel.GetLP(tp)
    //     and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,62499965)
    if !duel::is_main_phase(d, None) || eg.len() != 1 {
        return false;
    }
    let tc = tc.nil();
    card::is_faceup(d, tc)
        && card::get_attack(d, tc) < duel::get_lp(d, a.tp)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_code(d, c, &[62499965])),
            a.tp,
            LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        )
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,PLAYER_ALL,800)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, 800);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.Destroy(eg,REASON_EFFECT)>0 and Duel.Damage(tp,800,REASON_EFFECT)>0
    //     and Duel.GetLP(tp)>0 then
    if duel::destroy(d, a.eg(), REASON_EFFECT, None, None) > 0
        && duel::damage(d, tp, 800, REASON_EFFECT, false, None) > 0
        && duel::get_lp(d, tp) > 0
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Damage(1-tp,800,REASON_EFFECT)
        duel::damage(d, 1 - tp, 800, REASON_EFFECT, false, None);
    }
}
