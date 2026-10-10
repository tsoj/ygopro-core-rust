//! Crystal Beast Sapphire Pegasus
use super::prelude::*;

pub const ID: u32 = 7093411;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // send replace
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_TO_GRAVE_REDIRECT_CB)
    effect::set_code(d, e1, EFFECT_TO_GRAVE_REDIRECT_CB);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCondition(s.repcon)
    effect::set_condition(d, e1, Some(lua_fn(repcon)));
    // e1:SetOperation(s.repop)
    effect::set_operation(d, e1, Some(lua_fn(repop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // place
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.repcon(e)`
fn repcon(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    // return c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsReason(REASON_DESTROY)
    card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_reason(d, c, REASON_DESTROY)
}
/// `s.repop(e,tp,...)`
fn repop(d: &mut Duel, a: Args) {
    let (e, tp) = (a.e(0), a.p(1));
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCode(EFFECT_CHANGE_TYPE)
    effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
    // e1:SetValue(TYPE_SPELL+TYPE_CONTINUOUS)
    effect::set_value(d, e1, (TYPE_SPELL + TYPE_CONTINUOUS) as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.RaiseEvent(c,EVENT_CUSTOM+CARD_CRYSTAL_TREE,e,0,tp,0,0)
    duel::raise_event(d, c, EVENT_CUSTOM + CARD_CRYSTAL_TREE, Some(e), 0, tp, 0, 0);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CRYSTAL_BEAST) and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]) && !card::is_forbidden(d, c)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_HAND,0,1,nil)
    //     and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK | LOCATION_GRAVE | LOCATION_HAND, 0, 1, NO_EX)
            && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    }
    true
}
/// `s.operation(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(filter)),
        tp,
        LOCATION_DECK | LOCATION_GRAVE | LOCATION_HAND,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // local tc=g:GetFirst()
    // if tc then
    if let Some(tc) = group::get_first(d, &g) {
        // Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetCode(EFFECT_CHANGE_TYPE)
        effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
        // e1:SetValue(TYPE_SPELL+TYPE_CONTINUOUS)
        effect::set_value(d, e1, (TYPE_SPELL + TYPE_CONTINUOUS) as i64);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // Duel.RaiseEvent(tc,EVENT_CUSTOM+CARD_CRYSTAL_TREE,e,0,tp,0,0)
        duel::raise_event(d, tc, EVENT_CUSTOM + CARD_CRYSTAL_TREE, Some(e), 0, tp, 0, 0);
    }
}
