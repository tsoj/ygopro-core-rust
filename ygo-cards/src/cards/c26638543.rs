//! Metrognome
use super::prelude::*;

pub const ID: u32 = 26638543;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // scale
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.sctg)
    effect::set_target(d, e1, Some(cost_target(sctg)));
    // e1:SetOperation(s.scop)
    effect::set_operation(d, e1, Some(operation(scop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk/def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.con)
    effect::set_condition(d, e2, Some(condition(con)));
    // e2:SetValue(s.val)
    effect::set_value_fn(d, e2, effect_card_value(val));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // direct attack
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e4, EFFECT_DIRECT_ATTACK);
    // e4:SetCondition(s.con)
    effect::set_condition(d, e4, Some(condition(con)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // destroy
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e5, CATEGORY_DESTROY);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e5:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e5, EVENT_BATTLE_DAMAGE);
    // e5:SetCondition(s.descon)
    effect::set_condition(d, e5, Some(condition(descon)));
    // e5:SetTarget(s.destg)
    effect::set_target(d, e5, Some(cost_target(destg)));
    // e5:SetOperation(s.desop)
    effect::set_operation(d, e5, Some(operation(desop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.scfilter(c,pc)`
fn scfilter(d: &mut Duel, c: CardId, pc: CardId) -> bool {
    // return c:GetLeftScale()~=pc:GetLeftScale()
    card::get_left_scale(d, c) != card::get_left_scale(d, pc)
}
/// `s.sctg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sctg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_PZONE) and s.scfilter(chkc,c) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_PZONE) && scfilter(d, chkc, c) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.scfilter,tp,LOCATION_PZONE,LOCATION_PZONE,1,c,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(
            d,
            Some(&|d, x| scfilter(d, x, c)),
            a.tp,
            LOCATION_PZONE,
            LOCATION_PZONE,
            1,
            c.into(),
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.scfilter,tp,LOCATION_PZONE,LOCATION_PZONE,1,1,c,c)
    duel::select_target(
        d,
        a.tp,
        Some(&|d, x| scfilter(d, x, c)),
        a.tp,
        LOCATION_PZONE,
        LOCATION_PZONE,
        1,
        1,
        false,
        c.into(),
    );
    true
}
/// `s.scop(e,tp,eg,ep,ev,re,r,rp)`
fn scop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LSCALE)
        effect::set_code(d, e1, EFFECT_CHANGE_LSCALE);
        // e1:SetValue(tc:GetLeftScale())
        let lscale = card::get_left_scale(d, tc);
        effect::set_value(d, e1, lscale as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_CHANGE_RSCALE)
        effect::set_code(d, e2, EFFECT_CHANGE_RSCALE);
        // e2:SetValue(tc:GetRightScale())
        let rscale = card::get_right_scale(d, tc);
        effect::set_value(d, e2, rscale as i64);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
/// `s.con(e)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandler():GetControler()
    let h = effect::get_handler(d, a.e).nil();
    let tp = card::get_controler(d, h);
    // local tc1=Duel.GetFieldCard(tp,LOCATION_PZONE,0)
    let tc1 = duel::get_field_card(d, tp, LOCATION_PZONE, 0);
    // local tc2=Duel.GetFieldCard(tp,LOCATION_PZONE,1)
    let tc2 = duel::get_field_card(d, tp, LOCATION_PZONE, 1);
    // if not tc1 or not tc2 then return false end
    let (Some(tc1), Some(tc2)) = (tc1, tc2) else {
        return false;
    };
    // return tc1:GetLeftScale()==tc2:GetRightScale()
    card::get_left_scale(d, tc1) == card::get_right_scale(d, tc2)
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local tc=Duel.GetFieldCard(c:GetControler(),LOCATION_PZONE,0)
    let p = card::get_controler(d, c);
    let tc = duel::get_field_card(d, p, LOCATION_PZONE, 0);
    // return tc:GetLeftScale()*100
    card::get_left_scale(d, tc.nil()) as i64 * 100
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp and Duel.GetAttackTarget()==nil
    a.ep != a.tp && duel::get_attack_target(d).is_none()
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_PZONE,LOCATION_PZONE)
    let g = duel::get_field_group(d, a.tp, LOCATION_PZONE, LOCATION_PZONE);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,LOCATION_PZONE,LOCATION_PZONE)
    let g = duel::get_field_group(d, a.tp, LOCATION_PZONE, LOCATION_PZONE);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
