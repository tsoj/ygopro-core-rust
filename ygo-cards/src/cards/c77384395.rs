//! Pazuzule
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 77384395;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetTarget(s.sctg)
    effect::set_target(d, e1, Some(cost_target(sctg)));
    // e1:SetOperation(s.scop)
    effect::set_operation(d, e1, Some(operation(scop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_DISABLE_SPSUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_DISABLE_SPSUMMON);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_RANGE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(function(_,c)return c:IsPendulumSummoned() end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_pendulum_summoned(d, c))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.scfilter(c,sc)`: `return c:GetOriginalLevel()>0 and sc~=c:GetOriginalLevel()`
fn scfilter(d: &mut Duel, c: CardId, sc: u32) -> bool {
    let lv = card::get_original_level(d, c);
    lv > 0 && sc != lv
}
/// `s.sctg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sctg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_PZONE) and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_PZONE) && chkc != c;
    }
    let sc = card::get_scale(d, c);
    // if chk==0 then return Duel.IsExistingTarget(s.scfilter,tp,LOCATION_PZONE,0,1,c,c:GetScale()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| scfilter(d, x, sc)), tp, LOCATION_PZONE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.scfilter,tp,LOCATION_PZONE,0,1,1,c,c:GetScale())
    let _ = duel::select_target(d, tp, Some(&|d, x| scfilter(d, x, sc)), tp, LOCATION_PZONE, 0, 1, 1, false, c.into());
    true
}
/// `s.scop(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn scop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, e) {
            // local lv=tc:GetOriginalLevel()
            let lv = card::get_original_level(d, tc);
            // if lv~=c:GetLeftScale() then
            if lv != card::get_left_scale(d, c) {
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_CHANGE_LSCALE)
                effect::set_code(d, e1, EFFECT_CHANGE_LSCALE);
                // e1:SetValue(lv)
                effect::set_value(d, e1, lv as i64);
                // e1:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                // c:RegisterEffect(e1)
                card::register_effect(d, c, e1, false);
                // local e2=e1:Clone()
                let e2 = effect::clone(d, e1);
                // e2:SetCode(EFFECT_CHANGE_RSCALE)
                effect::set_code(d, e2, EFFECT_CHANGE_RSCALE);
                // e2:SetValue(lv)
                effect::set_value(d, e2, lv as i64);
                // c:RegisterEffect(e2)
                card::register_effect(d, c, e2, false);
            }
        }
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(_,_,_,st)return SUMMON_TYPE_PENDULUM~=(st&SUMMON_TYPE_PENDULUM)end)
    effect::set_target(d, e1, Some(lua_fn(|_d, a: Args| {
        let st = a.int(3) as u64;
        SUMMON_TYPE_PENDULUM as u64 != (st & SUMMON_TYPE_PENDULUM as u64)
    })));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
