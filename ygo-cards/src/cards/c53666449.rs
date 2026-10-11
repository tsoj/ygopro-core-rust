//! Minerva, Scholar of the Sky
use super::prelude::*;

pub const ID: u32 = 53666449;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SANCTUARY_SKY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EVENT_CHAINING)
    effect::set_code(d, e0, EVENT_CHAINING);
    // e0:SetRange(LOCATION_MZONE)
    effect::set_range(d, e0, LOCATION_MZONE);
    // e0:SetOperation(aux.chainreg)
    effect::set_operation(d, e0, Some(aux::chainreg()));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsActiveType(TYPE_COUNTER) and e:GetHandler():GetFlagEffect(1)>0
    let h = effect::get_handler(d, a.e).nil();
    effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE)
        && effect::is_active_type(d, a.re.nil(), TYPE_COUNTER)
        && card::get_flag_effect(d, h, 1) > 0
}
/// `s.thfilter(c,code)`
fn thfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsType(TYPE_COUNTER) and not c:IsCode(code) and c:IsAbleToHand()
    card::is_type(d, c, TYPE_COUNTER) && !card::is_code(d, c, &[code]) && card::is_able_to_hand(d, c, None)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(500)
    effect::set_value(d, e1, 500);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local rc=re:GetHandler()
    // if not rc then return end
    let Some(rc) = effect::get_handler(d, a.re.nil()) else {
        return;
    };
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,nil,rc:GetCode())
    // (the extra argument of thfilter is the first code: rc:GetCode() passes all of them, thfilter takes one)
    let code = card::get_code(d, rc)[0];
    let g = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(move |d: &mut Duel, x: CardId| thfilter(d, x, code))),
        a.tp,
        LOCATION_GRAVE,
        0,
        NO_EX,
    );
    // if Duel.IsEnvironment(CARD_SANCTUARY_SKY) and #g>0 then
    if duel::is_environment(d, CARD_SANCTUARY_SKY, None, None) && !g.is_empty() {
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
    }
}
