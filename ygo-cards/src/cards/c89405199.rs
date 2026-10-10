//! Greed
use super::prelude::*;

pub const ID: u32 = 89405199;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // reg
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_DRAW);
    effect::set_condition(d, e2, Some(condition(drcon)));
    effect::set_operation(d, e2, Some(operation(drop_op)));
    card::register_effect(d, c, e2, false);
    // damage
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(damcon)));
    effect::set_target(d, e3, Some(cost_target(damtg)));
    effect::set_operation(d, e3, Some(operation(damop)));
    card::register_effect(d, c, e3, false);
}
fn drcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_EFFECT)~=0
    a.r & REASON_EFFECT != 0
}
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local flag=(ep==0 and id or id+1)
    let flag = if a.ep == 0 { ID } else { ID + 1 };
    // local ct=c:GetFlagEffectLabel(flag)
    let ct = card::get_flag_effect_label(d, c, flag).first().copied();
    if let Some(ct) = ct {
        // c:SetFlagEffectLabel(flag,ct+ev)
        card::set_flag_effect_label(d, c, flag, (ct + a.ev as i64) as u32);
    } else {
        // c:RegisterFlagEffect(flag,RESETS_STANDARD_PHASE_END,0,1,ev)
        card::register_flag_effect(d, c, flag, RESETS_STANDARD_PHASE_END, 0, 1, a.ev as i64, 0);
    }
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:GetFlagEffect(id)>0 or c:GetFlagEffect(id+1)>0
    card::get_flag_effect(d, c, ID) > 0 || card::get_flag_effect(d, c, ID + 1) > 0
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct1=c:GetFlagEffectLabel(id)
    let ct1 = card::get_flag_effect_label(d, c, ID).first().copied();
    // local ct2=c:GetFlagEffectLabel(id+1)
    let ct2 = card::get_flag_effect_label(d, c, ID + 1).first().copied();
    match (ct1, ct2) {
        // if ct1 and ct2 then
        (Some(_), Some(_)) => {
            // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,PLAYER_ALL,0)
            duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, 0);
        }
        // elseif ct1 then
        (Some(ct1), None) => {
            // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,0,ct1*500)
            duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 0, (ct1 * 500) as i32);
        }
        // elseif ct2 then
        (None, Some(ct2)) => {
            // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1,ct2*500)
            duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1, (ct2 * 500) as i32);
        }
        _ => {}
    }
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local ct1=c:GetFlagEffectLabel(id+tp)
    let ct1 = card::get_flag_effect_label(d, c, ID + a.tp as u32).first().copied();
    // local ct2=c:GetFlagEffectLabel(id+1-tp)
    let ct2 = card::get_flag_effect_label(d, c, ID + 1 - a.tp as u32).first().copied();
    // if ct1 then Duel.Damage(tp,ct1*500,REASON_EFFECT,true) end
    if let Some(ct1) = ct1 {
        duel::damage(d, a.tp, ct1 * 500, REASON_EFFECT, true, None);
    }
    // if ct2 then Duel.Damage(1-tp,ct2*500,REASON_EFFECT,true) end
    if let Some(ct2) = ct2 {
        duel::damage(d, 1 - a.tp, ct2 * 500, REASON_EFFECT, true, None);
    }
    // Duel.RDComplete()
    duel::rd_complete(d);
}
