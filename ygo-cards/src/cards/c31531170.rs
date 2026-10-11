//! Harmonic Oscillation
use super::prelude::*;

pub const ID: u32 = 31531170;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_PZONE)==2 end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::get_field_group_count(d, a.tp, 0, LOCATION_PZONE) == 2)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:HasFlagEffect(id) or c:GetFlagEffectLabel(id)~=Duel.GetMatchingGroup(nil,c:GetControler(),LOCATION_PZONE,0,c):GetFirst():GetFlagEffectLabel(id)
    if !(card::get_flag_effect(d, c, ID) > 0) {
        return true;
    }
    let mine = card::get_flag_effect_label(d, c, ID).first().copied();
    let ctrl = card::get_controler(d, c);
    let g = duel::get_matching_group(d, None, ctrl, LOCATION_PZONE, 0, c.into());
    let first = group::get_first(d, &g).nil();
    let theirs = card::get_flag_effect_label(d, first, ID).first().copied();
    mine != theirs
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,0,LOCATION_PZONE,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), a.tp, 0, LOCATION_PZONE, 2, NO_EX);
    }
    // Duel.SetTargetCard(Duel.GetFieldGroup(tp,0,LOCATION_PZONE))
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_PZONE);
    duel::set_target_card(d, g);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg~=2 then return end
    if tg.len() != 2 {
        return;
    }
    // local fid=e:GetFieldID()
    let fid = effect::get_field_id(d, a.e);
    // for pc in tg:Iter() do
    for pc in group::iter(d, &tg) {
        // pc:ResetFlagEffect(id)
        card::reset_flag_effect(d, pc, ID);
        // pc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,fid,aux.Stringid(id,1))
        card::register_flag_effect(d, pc, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, fid as i64, aux::stringid(ID, 1));
    }
    // local left_scale_card=Duel.GetFieldCard(1-tp,LOCATION_PZONE,0)
    let left_scale_card = duel::get_field_card(d, 1 - a.tp, LOCATION_PZONE, 0);
    // local right_scale_card=Duel.GetFieldCard(1-tp,LOCATION_PZONE,1)
    let right_scale_card = duel::get_field_card(d, 1 - a.tp, LOCATION_PZONE, 1);
    // local e1,e2=Pendulum.CreateHarmonicOscillationEffect(e:GetHandler(),nil,nil,nil,10000000)
    let h = effect::get_handler(d, a.e).nil();
    let (e1, e2) = pendulum::create_harmonic_oscillation_effect(d, h, None, None, 0, Some(10000000));
    // left_scale_card:RegisterEffect(e1)
    card::register_effect(d, left_scale_card.nil(), e1, false);
    // right_scale_card:RegisterEffect(e2)
    card::register_effect(d, right_scale_card.nil(), e2, false);
}
