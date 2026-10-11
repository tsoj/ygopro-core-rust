//! Cracking
use super::prelude::*;

pub const ID: u32 = 32835363;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.regcon)
    effect::set_condition(d, e2, Some(condition(regcon)));
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e3, EVENT_CUSTOM + ID);
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // local d1=false
    let mut d1 = false;
    // local d2=false
    let mut d2 = false;
    // local tc=eg:GetFirst()
    let eg = a.eg();
    let _ = group::get_first(d, &eg);
    // for tc in aux.Next(eg) do
    for tc in group::iter(d, &eg) {
        // if tc:IsPreviousLocation(LOCATION_MZONE) and tc:IsMonster() and tc:IsReason(REASON_EFFECT) then
        if card::is_previous_location(d, tc, LOCATION_MZONE) && card::is_monster(d, tc) && card::is_reason(d, tc, REASON_EFFECT) {
            // if tc:GetControler()==0 then d1=true
            if card::get_controler(d, tc) == 0 {
                d1 = true;
            // else d2=true end
            } else {
                d2 = true;
            }
        }
    }
    // local evt_p=PLAYER_NONE
    let evt_p: u32 = if d1 && d2 {
        // if d1 and d2 then evt_p=PLAYER_ALL
        PLAYER_ALL as u32
    } else if d1 {
        // elseif d1 then evt_p=0
        0
    } else if d2 {
        // elseif d2 then evt_p=1 end
        1
    } else {
        PLAYER_NONE as u32
    };
    // e:SetLabel(evt_p)
    effect::set_label(d, e, &[evt_p as i64]);
    // return evt_p~=PLAYER_NONE
    evt_p != PLAYER_NONE as u32
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,e:GetLabel(),0)
    let h = effect::get_handler(d, e).nil();
    let label = effect::get_label(d, e);
    duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(e), 0, tp, label as u8, 0);
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsRelateToEffect(e) end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_relate_to_effect(d, h, a.e);
    }
    // Duel.SetTargetParam(800)
    duel::set_target_param(d, 800);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,0,0,ep,800)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.ep, 800);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    let (e, tp, ep) = (a.e, a.tp, a.ep);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local d=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let dmg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if ep==PLAYER_ALL then
    if ep as u32 == PLAYER_ALL as u32 {
        // Duel.Damage(tp,d,REASON_EFFECT,true)
        duel::damage(d, tp, dmg, REASON_EFFECT, true, None);
        // Duel.Damage(1-tp,d,REASON_EFFECT,true)
        duel::damage(d, 1 - tp, dmg, REASON_EFFECT, true, None);
        // Duel.RDComplete()
        duel::rd_complete(d);
    } else {
        // Duel.Damage(ep,d,REASON_EFFECT)
        duel::damage(d, ep, dmg, REASON_EFFECT, false, None);
    }
}
