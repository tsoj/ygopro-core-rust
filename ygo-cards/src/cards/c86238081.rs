//! Odd-Eyes Raging Dragon
use super::prelude::*;

pub const ID: u32 = 86238081;
pub const INFO: ScriptInfo = ScriptInfo {
    pendulum_level: Some(7),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz summon procedure
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON),7,2)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_DRAGON, sc, st, Some(tp)))),
        Some(7),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // Enable pendulum summon
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // Place 1 pendulum monster from deck into pendulum zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.pctg)
    effect::set_target(d, e1, Some(cost_target(pctg)));
    // e1:SetOperation(s.pcop)
    effect::set_operation(d, e1, Some(operation(pcop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Check materials used for its Xyz summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.regcon)
    effect::set_condition(d, e2, Some(condition(regcon)));
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e3, EFFECT_MATERIAL_CHECK);
    // e3:SetValue(s.valcheck)
    effect::set_value_fn(d, e3, effect_card_value(valcheck));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Can make a second attack
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e4, EFFECT_EXTRA_ATTACK);
    // e4:SetValue(1)
    effect::set_value(d, e4, 1);
    // e4:SetCondition(s.effcon)
    effect::set_condition(d, e4, Some(condition(effcon)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Destroy as many of opponent's cards as possible, gains 200 ATK per card
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e5, CATEGORY_DESTROY);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetCondition(s.effcon)
    effect::set_condition(d, e5, Some(condition(effcon)));
    // e5:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e5, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e5:SetTarget(s.destg)
    effect::set_target(d, e5, Some(cost_target(destg)));
    // e5:SetOperation(s.desop)
    effect::set_operation(d, e5, Some(operation(desop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Place itself into pendulum zone
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e6, aux::stringid(ID, 2));
    // e6:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e6:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e6, EVENT_DESTROYED);
    // e6:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e6, EFFECT_FLAG_DELAY, 0);
    // e6:SetCondition(s.pencon)
    effect::set_condition(d, e6, Some(condition(pencon)));
    // e6:SetTarget(s.pentg)
    effect::set_target(d, e6, Some(cost_target(pentg)));
    // e6:SetOperation(s.penop)
    effect::set_operation(d, e6, Some(operation(penop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.pcfilter(c)`
fn pcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_PENDULUM) and not c:IsForbidden()
    card::is_type(d, c, TYPE_PENDULUM) && !card::is_forbidden(d, c)
}
/// `s.pctg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pctg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckPendulumZones(tp)
    //     and Duel.IsExistingMatchingCard(s.pcfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp)
            && duel::is_existing_matching_card(d, Some(&pcfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.pcop(e,tp,eg,ep,ev,re,r,rp)`
fn pcop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if not Duel.CheckPendulumZones(tp) then return end
    if !duel::check_pendulum_zones(d, tp) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local g=Duel.SelectMatchingCard(tp,s.pcfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&pcfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.MoveToField(g:GetFirst(),tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        let tc = group::get_first(d, &g).nil();
        duel::move_to_field(d, tc, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned() and e:GetLabel()==1
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h) && effect::get_label(d, a.e) == 1
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
    // c:RegisterFlagEffect(0,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,3))
    card::register_flag_effect(
        d,
        c,
        0,
        RESET_EVENT | RESETS_STANDARD,
        EFFECT_FLAG_CLIENT_HINT,
        1,
        0,
        aux::stringid(ID, 3),
    );
}
/// `s.effcon(e)`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) > 0
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD) > 0;
    }
    // local sg=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let sg = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local sg=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let sg = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD);
    // local ct=Duel.Destroy(sg,REASON_EFFECT)
    let ct = duel::destroy(d, sg, REASON_EFFECT, None, None);
    // if ct>0 and c:IsFaceup() and c:IsRelateToEffect(e) then
    if ct > 0 && card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(ct*200)
        effect::set_value(d, e1, ct as i64 * 200);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsType,1,nil,TYPE_XYZ,c,SUMMON_TYPE_XYZ) then
    let lo = effect::get_label_object(d, e).as_effect().nil();
    if group::is_exists(d, &g, &|d, x| card::is_type_ctx(d, x, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, None), 1, NO_EX) {
        // e:GetLabelObject():SetLabel(1)
        effect::set_label(d, lo, &[1]);
    } else {
        // e:GetLabelObject():SetLabel(0)
        effect::set_label(d, lo, &[0]);
    }
    // (a value function that falls off the end is read as 0 by the core)
    0
}
/// `s.pencon(e,tp,eg,ep,ev,re,r,rp)`
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsFaceup()
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c)
}
/// `s.pentg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckPendulumZones(tp) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp);
    }
    true
}
/// `s.penop(e,tp,eg,ep,ev,re,r,rp)`
fn penop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.CheckPendulumZones(tp) then return false end
    if !duel::check_pendulum_zones(d, tp) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
