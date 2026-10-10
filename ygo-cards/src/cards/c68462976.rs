//! Secret Village of the Spellcasters
use super::prelude::*;

pub const ID: u32 = 68462976;

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
    // adjust
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_ADJUST)
    effect::set_code(d, e2, EVENT_ADJUST);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetOperation(s.adjustop)
    effect::set_operation(d, e2, Some(operation(adjustop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // cannot activate
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(1,1)
    effect::set_target_range(d, e3, 1, 1);
    // e3:SetLabel(0)
    effect::set_label(d, e3, &[0]);
    // e3:SetValue(s.actlimit)
    effect::set_value_fn(d, e3, lua_fn(actlimit));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // e2:SetLabelObject(e3)
    effect::set_label_object(d, e2, Value::Effect(e3));
}
/// `s.actlimit(e,te,tp)`
fn actlimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    let tp = a.p(2);
    // if not te:IsHasType(EFFECT_TYPE_ACTIVATE) or not te:IsSpellEffect() then return false end
    if !effect::is_has_type(d, te, EFFECT_TYPE_ACTIVATE) || !effect::is_spell_effect(d, te) {
        return false;
    }
    // if tp==e:GetHandlerPlayer() then return e:GetLabel()==1
    if tp == effect::get_handler_player(d, e) {
        effect::get_label(d, e) == 1
    }
    // else return e:GetLabel()==2 end
    else {
        effect::get_label(d, e) == 2
    }
}
/// `s.adjustop(e,tp,eg,ep,ev,re,r,rp)`
fn adjustop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local b1=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_SPELLCASTER),tp,LOCATION_MZONE,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_SPELLCASTER)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_SPELLCASTER),tp,0,LOCATION_MZONE,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_SPELLCASTER)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local te=e:GetLabelObject()
    let te = effect::get_label_object(d, a.e).as_effect().nil();
    // if not b1 then te:SetLabel(1)
    if !b1 {
        effect::set_label(d, te, &[1]);
    }
    // elseif b1 and not b2 then te:SetLabel(2)
    else if !b2 {
        effect::set_label(d, te, &[2]);
    }
    // else te:SetLabel(0) end
    else {
        effect::set_label(d, te, &[0]);
    }
}
