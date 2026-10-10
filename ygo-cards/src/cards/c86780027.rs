//! Solidarity
use super::prelude::*;

pub const ID: u32 = 86780027;

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
    // atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetLabel(0)
    effect::set_label(d, e2, &[0]);
    // e2:SetTarget(s.tg)
    effect::set_target(d, e2, Some(effect_card_fn(tg)));
    // e2:SetValue(800)
    effect::set_value(d, e2, 800);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // adjust
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_ADJUST)
    effect::set_code(d, e3, EVENT_ADJUST);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetOperation(s.adjustop)
    effect::set_operation(d, e3, Some(operation(adjustop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
}
/// `s.tg(e,c)`
fn tg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsRace(e:GetLabel())
    card::is_race(d, c, effect::get_label(d, e) as u64)
}
/// `s.adjustop(e,tp,eg,ep,ev,re,r,rp)`
fn adjustop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsMonster,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&card::is_monster), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local te=e:GetLabelObject()
    let te = effect::get_label_object(d, a.e).as_effect().nil();
    // if #g==0 then te:SetLabel(0)
    if g.is_empty() {
        effect::set_label(d, te, &[0]);
    } else {
        // local ct=g:GetClassCount(Card.GetOriginalRace)
        let ct = group::get_class_count(d, &g, &|d, c| card::get_original_race(d, c) as i64);
        // if ct==1 then te:SetLabel(g:GetFirst():GetOriginalRace())
        if ct == 1 {
            let first = group::get_first(d, &g).nil();
            let race = card::get_original_race(d, first);
            effect::set_label(d, te, &[race as i64]);
        } else {
            // else te:SetLabel(0) end
            effect::set_label(d, te, &[0]);
        }
    }
}
