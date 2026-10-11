//! Threshold Borg
use super::prelude::*;

pub const ID: u32 = 31944175;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(-500)
    effect::set_value(d, e2, -500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_MZONE,0)==0 and
    //     Duel.IsExistingMatchingCard(Card.IsRace,c:GetControler(),LOCATION_HAND,0,1,c,RACE_CYBERSE)
    let hp = effect::get_handler_player(d, a.e(0));
    duel::get_field_group_count(d, hp, LOCATION_MZONE, 0) == 0 && {
        let tp = card::get_controler(d, c);
        duel::is_existing_matching_card(d, Some(&|d, cc| card::is_race(d, cc, RACE_CYBERSE)), tp, LOCATION_HAND, 0, 1, c.into())
    }
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp,c)
fn spop(d: &mut Duel, a: Args) -> Value {
    let tp = a.p(1);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsRace,tp,LOCATION_HAND,0,1,1,c,RACE_CYBERSE)
    let ex = a.card(8).map_or(NO_EX, |c| c.into());
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| card::is_race(d, cc, RACE_CYBERSE)), tp, LOCATION_HAND, 0, 1, 1, false, ex).nil();
    // Duel.SendtoGrave(g,REASON_DISCARD|REASON_COST)
    duel::send_to_grave(d, g, REASON_DISCARD | REASON_COST, None, None);
    Value::Nil
}
