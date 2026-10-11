//! Libromancer Bonded
//! リブロマンサー・リライジング
use super::prelude::*;

pub const ID: u32 = 41085464;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[88106656, 45001322],
    listed_series: &[SET_LIBROMANCER],
    fit_monster: &[88106656, 45001322],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreater({handler=c,filter=s.ritualfil,stage2=s.stage2})
    ritual::add_proc_greater(
        d,
        c,
        ritual::RitualProc {
            filter: Some(std::sync::Arc::new(|d, cc| ritualfil(d, cc))),
            stage2: Some(std::sync::Arc::new(stage2)),
            ..Default::default()
        },
    );
}
/// `s.ritualfil(c)`
fn ritualfil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_LIBROMANCER) and c:IsRitualMonster()
    card::is_set_card(d, c, &[SET_LIBROMANCER as u16]) && card::is_ritual_monster(d, c)
}
/// `s.mfilter(c)`
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:GetPreviousCodeOnField()==45001322
    card::is_previous_location(d, c, LOCATION_MZONE) && card::get_previous_code_on_field(d, c).first().copied() == Some(45001322)
}
/// `s.stage2(mg,e,tp,eg,ep,ev,re,r,rp,tc)`
fn stage2(d: &mut Duel, mg: &CardSet, a: &EffArgs, tc: CardId) {
    // if tc:IsCode(88106656) and mg:IsExists(s.mfilter,1,nil) then
    if card::is_code(d, tc, &[88106656]) && group::is_exists(d, mg, &|d, cc| mfilter(d, cc), 1, NO_EX) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // --Cannot be destroyed by effects
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
        // --Cannot be banished by effects
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e2:SetCode(EFFECT_CANNOT_REMOVE)
        effect::set_code(d, e2, EFFECT_CANNOT_REMOVE);
        // e2:SetRange(LOCATION_MZONE)
        effect::set_range(d, e2, LOCATION_MZONE);
        // e2:SetTargetRange(1,1)
        effect::set_target_range(d, e2, 1, 1);
        // e2:SetTarget(s.rmlimit)
        effect::set_target(d, e2, Some(lua_fn(rmlimit)));
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e2,true)
        card::register_effect(d, tc, e2, true);
    }
}
/// `s.rmlimit(e,c,tp,r)`
fn rmlimit(d: &mut Duel, a: Args) -> bool {
    // return c==e:GetHandler() and r==REASON_EFFECT
    let c = a.c(1);
    let h = effect::get_handler(d, a.e(0)).nil();
    c == h && a.int(3) == REASON_EFFECT as i64
}
