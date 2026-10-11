//! Rebirth Judgment
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 45064756;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Change race
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(tg)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local rc=Duel.AnnounceRace(tp,1,RACE_ALL)
    let rc = duel::announce_race(d, a.tp, 1, RACE_ALL);
    // e:SetLabel(rc)
    effect::set_label(d, a.e, &[rc as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local race=e:GetLabel()
    let race = effect::get_label(d, e);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_RACE)
    effect::set_code(d, e1, EFFECT_CHANGE_RACE);
    // e1:SetTargetRange(LOCATION_GRAVE,LOCATION_GRAVE)
    effect::set_target_range(d, e1, LOCATION_GRAVE, LOCATION_GRAVE);
    // e1:SetTarget(s.rctg)
    effect::set_target(d, e1, Some(effect_card_fn(rctg)));
    // e1:SetValue(race)
    effect::set_value(d, e1, race);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // --id chk
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(4064256)
    effect::set_code(d, e2, 4064256);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetValue(s.val(race))
    effect::set_value_fn(d, e2, effect_card_value(move |_d, _e, _c| race));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rctg(e,c)`: Necro Valley check
fn rctg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // if c:GetFlagEffect(1)==0 then
    if card::get_flag_effect(d, c, 1) == 0 {
        // c:RegisterFlagEffect(1,0,0,0)
        card::register_flag_effect(d, c, 1, 0, 0, 0, 0, 0);
        // local eff
        // if c:IsLocation(LOCATION_MZONE) then
        //     eff={Duel.GetPlayerEffect(c:GetControler(),EFFECT_NECRO_VALLEY)}
        // else
        //     eff={c:GetCardEffect(EFFECT_NECRO_VALLEY)}
        // end
        let eff: Vec<EffectId> = if card::is_location(d, c, LOCATION_MZONE) {
            let p = card::get_controler(d, c);
            duel::get_player_effect(d, p, EFFECT_NECRO_VALLEY)
        } else {
            card::get_card_effect(d, c, EFFECT_NECRO_VALLEY)
        };
        // c:ResetFlagEffect(1)
        card::reset_flag_effect(d, c, 1);
        // for _,te in ipairs(eff) do
        for te in eff {
            // local op=te:GetOperation()
            // if not op or op(e,c) then return false end
            match effect::get_operation(d, te) {
                None => return false,
                Some(op) => {
                    if op.call(d, &[Value::Effect(e), Value::Card(c)]).truthy() {
                        return false;
                    }
                }
            }
        }
    }
    // return true
    true
}
