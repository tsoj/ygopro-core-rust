//! PenduLuMoon
use super::prelude::*;

pub const ID: u32 = 66150724;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PENDULUM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum properties
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Add 1 "Pendulum" Pendulum monster from the Extra Deck to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add up to 2 Pendulum monsters from the Extra Deck to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg2)
    effect::set_target(d, e2, Some(cost_target(thtg2)));
    // e2:SetOperation(s.thop2)
    effect::set_operation(d, e2, Some(operation(thop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_PENDULUM) and c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_PENDULUM as u16]) && card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_EXTRA as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_EXTRA,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_HAND) then
    let Some(tc) = tc else { return };
    if duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None) > 0 && card::is_location(d, tc, LOCATION_HAND) {
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(tc));
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if not c:IsRelateToEffect(e) then return end
        if !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(c,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(c), REASON_EFFECT, None, None);
    }
}
/// `s.thfilter2(c,lsc,rsc)`
fn thfilter2(d: &mut Duel, c: CardId, lsc: u32, rsc: u32) -> bool {
    // return c:IsFaceup() and c:HasLevel() and c:IsType(TYPE_PENDULUM) and c:GetLevel()>lsc and c:GetLevel()<rsc and c:IsAbleToHand()
    card::is_faceup(d, c)
        && card::has_level(d, c)
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::get_level(d, c) > lsc
        && card::get_level(d, c) < rsc
        && card::is_able_to_hand(d, c, None)
}
/// The two Pendulum Scales of `tp`, the lower one first: `(lsc, rsc)`, or `None` when a Pendulum Zone is empty.
fn scales(d: &mut Duel, tp: u8) -> Option<(u32, u32)> {
    // local lsc=Duel.GetFieldCard(tp,LOCATION_PZONE,0)
    // local rsc=Duel.GetFieldCard(tp,LOCATION_PZONE,1)
    // if not (lsc and rsc) then return false end
    let lsc = duel::get_field_card(d, tp, LOCATION_PZONE, 0)?;
    let rsc = duel::get_field_card(d, tp, LOCATION_PZONE, 1)?;
    // lsc=lsc:GetLeftScale()
    // rsc=rsc:GetRightScale()
    let mut l = card::get_left_scale(d, lsc);
    let mut r = card::get_right_scale(d, rsc);
    // if lsc>rsc then lsc,rsc=rsc,lsc end
    if l > r {
        std::mem::swap(&mut l, &mut r);
    }
    Some((l, r))
}
/// `s.thtg2(e,tp,...,chk)`
fn thtg2(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local lsc=...; if not (lsc and rsc) then return false end
        let Some((lsc, rsc)) = scales(d, a.tp) else { return false };
        // return Duel.IsExistingMatchingCard(s.thfilter2,tp,LOCATION_EXTRA,0,1,nil,lsc,rsc)
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter2(d, c, lsc, rsc)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.thop2(e,tp,...)`
fn thop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Cannot activate monster effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(function(_,re) return re:IsMonsterEffect() end)
    effect::set_value_fn(d, e1, lua_fn(|d, a: Args| effect::is_monster_effect(d, a.e(1))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Pendulum effects are negated
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e2, EFFECT_DISABLE);
    // e2:SetTargetRange(LOCATION_PZONE,0)
    effect::set_target_range(d, e2, LOCATION_PZONE, 0);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // Reset restrictions if the player Pendulum Summons
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetLabelObject({e1,e2})
    effect::set_label_object(d, e3, Value::Multi(vec![Value::Effect(e1), Value::Effect(e2)]));
    // e3:SetOperation(s.checkop)
    effect::set_operation(d, e3, Some(operation(checkop)));
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
    // Add cards to the hand
    // (the scales as the Pendulum Zones are now, see thtg2)
    let Some((lsc, rsc)) = scales(d, tp) else { return };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.thfilter2,tp,LOCATION_EXTRA,0,1,2,nil,lsc,rsc)
    let sg = duel::select_matching_card(d, tp, Some(&|d, c| thfilter2(d, c, lsc, rsc)), tp, LOCATION_EXTRA, 0, 1, 2, false, NO_EX).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg);
    }
}
/// `s.psfilter(c,tp)`
fn psfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(tp) and c:IsPendulumSummoned()
    card::is_summon_player(d, c, tp) && card::is_pendulum_summoned(d, c)
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local ef=e:GetLabelObject()
    let ef = effect::get_label_object(d, a.e);
    // if ef and eg and eg:IsExists(s.psfilter,1,nil,tp) then
    if let (Value::Multi(v), Some(eg)) = (ef, a.eg.clone()) {
        let tp = a.tp;
        if group::is_exists(d, &eg, &|d, c| psfilter(d, c, tp), 1, NO_EX) {
            // ef[1]:Reset()
            // ef[2]:Reset()
            effect::reset(d, v[0].as_effect().nil());
            effect::reset(d, v[1].as_effect().nil());
            // e:Reset()
            effect::reset(d, a.e);
        }
    }
}
