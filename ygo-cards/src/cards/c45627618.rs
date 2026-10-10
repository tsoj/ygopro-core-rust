//! Odd-Eyes Rebellion Dragon
use super::prelude::*;

pub const ID: u32 = 45627618;

pub const INFO: ScriptInfo = ScriptInfo { pendulum_level: Some(7), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON),7,2)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_DRAGON, sc, st, Some(tp)))),
        Some(7),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // pendulum summon
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // pendulum set
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
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e3, EFFECT_MATERIAL_CHECK);
    // e3:SetValue(s.valcheck)
    effect::set_value_fn(d, e3, lua_fn(valcheck));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // pendulum
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCondition(s.pencon)
    effect::set_condition(d, e4, Some(condition(pencon)));
    // e4:SetTarget(s.pentg)
    effect::set_target(d, e4, Some(cost_target(pentg)));
    // e4:SetOperation(s.penop)
    effect::set_operation(d, e4, Some(operation(penop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
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
        return duel::check_pendulum_zones(d, a.tp) && duel::is_existing_matching_card(d, Some(&pcfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.pcop(e,tp,eg,ep,ev,re,r,rp)`
fn pcop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // if not Duel.CheckPendulumZones(tp) then return end
    if !duel::check_pendulum_zones(d, a.tp) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local g=Duel.SelectMatchingCard(tp,s.pcfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&pcfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.MoveToField(g:GetFirst(),tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        let tc = group::get_first(d, &g).nil();
        duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned() and e:GetLabel()==1
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h) && effect::get_label(d, a.e) == 1
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsLevelBelow,7),tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_level_below(d, c, 7))), a.tp, 0, LOCATION_MZONE, NO_EX);
    let n = g.len();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,#g*1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (n * 1000) as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsLevelBelow,7),tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_level_below(d, c, 7))), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local ct=Duel.Destroy(g,REASON_EFFECT)
    let ct = duel::destroy(d, g, REASON_EFFECT, None, None);
    // if Duel.Damage(1-tp,ct*1000,REASON_EFFECT)~=0 then
    if duel::damage(d, 1 - a.tp, ct as i64 * 1000, REASON_EFFECT, false, None) != 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,3))
        effect::set_description(d, e1, aux::stringid(ID, 3));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetValue(2)
        effect::set_value(d, e1, 2);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(1);
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsType,1,nil,TYPE_XYZ,c,SUMMON_TYPE_XYZ) then
    if group::is_exists(d, &g, &|d, cc| card::is_type_ctx(d, cc, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, None), 1, NO_EX) {
        // e:GetLabelObject():SetLabel(1)
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, lo, &[1]);
    } else {
        // e:GetLabelObject():SetLabel(0)
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, lo, &[0]);
    }
}
/// `s.pencon(e,tp,eg,ep,ev,re,r,rp)`
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return (r&REASON_EFFECT+REASON_BATTLE)~=0 and c:IsPreviousLocation(LOCATION_MZONE) and c:IsFaceup()
    (a.r & (REASON_EFFECT + REASON_BATTLE)) != 0 && card::is_previous_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c)
}
/// `s.pentg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_PZONE,0)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_PZONE, 0) > 0;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_PZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_PZONE, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.penop(e,tp,eg,ep,ev,re,r,rp)`
fn penop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,LOCATION_PZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_PZONE, 0);
    // if Duel.Destroy(g,REASON_EFFECT)~=0 and e:GetHandler():IsRelateToEffect(e) then
    if duel::destroy(d, g, REASON_EFFECT, None, None) != 0 {
        let h = effect::get_handler(d, a.e).nil();
        if card::is_relate_to_effect(d, h, a.e) {
            // Duel.MoveToField(e:GetHandler(),tp,tp,LOCATION_PZONE,POS_FACEUP,true)
            duel::move_to_field(d, h, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
        }
    }
}
