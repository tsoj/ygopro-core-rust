//! Taotie Dragon
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 82566662;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link summon procedure
    // Link.AddProcedure(c,s.matfilter,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, _sc, _st, _tp| matfilter(d, c))), 2, None, None, None, None);
    // Check materials used for its link summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    // e1:SetValue(s.valcheck)
    effect::set_value_fn(d, e1, lua_fn(valcheck));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gains various effects, based on card types used for its link summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.regcon)
    effect::set_condition(d, e2, Some(condition(regcon)));
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
}
/// `s.matfilter(c)`
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSummonLocation(LOCATION_EXTRA)
    card::is_summon_location(d, c, LOCATION_EXTRA)
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) -> Value {
    let (e, c) = (a.e(0), a.c(1));
    // local g=c:GetMaterial()
    // if not g then return end
    // (GetMaterial always returns a group, possibly empty)
    let g = card::get_material(d, c);
    // local typ=0
    let mut typ: u32 = 0;
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // typ=(typ|tc:GetType())
        typ |= card::get_type(d, tc);
    }
    // typ=(typ&TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ)
    typ &= TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ;
    // e:SetLabel(typ)
    effect::set_label(d, e, &[typ as i64]);
    Value::Nil
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned() and e:GetLabelObject():GetLabel()~=0
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_link_summoned(d, h) {
        return false;
    }
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::get_label(d, lo) != 0
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local typ=e:GetLabelObject():GetLabel()
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let typ = effect::get_label(d, lo) as u32;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if (typ&TYPE_FUSION)~=0 then
    if (typ & TYPE_FUSION) != 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetTargetRange(0,1)
        effect::set_target_range(d, e1, 0, 1);
        // e1:SetCondition(s.fuscond)
        effect::set_condition(d, e1, Some(lua_fn(fuscond)));
        // e1:SetValue(s.fuslimit)
        effect::set_value_fn(d, e1, lua_fn(fuslimit));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // c:RegisterFlagEffect(0,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1))
        card::register_flag_effect(d, c, 0, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
    }
    // if (typ&TYPE_SYNCHRO)~=0 then
    if (typ & TYPE_SYNCHRO) != 0 {
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
        // e2:SetRange(LOCATION_MZONE)
        effect::set_range(d, e2, LOCATION_MZONE);
        // e2:SetTargetRange(0,1)
        effect::set_target_range(d, e2, 0, 1);
        // e2:SetCondition(s.syncond)
        effect::set_condition(d, e2, Some(lua_fn(syncond)));
        // e2:SetValue(s.synlimit)
        effect::set_value_fn(d, e2, lua_fn(synlimit));
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
        // c:RegisterFlagEffect(0,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
        card::register_flag_effect(d, c, 0, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
    }
    // if (typ&TYPE_XYZ)~=0 then
    if (typ & TYPE_XYZ) != 0 {
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD);
        // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
        // e3:SetRange(LOCATION_MZONE)
        effect::set_range(d, e3, LOCATION_MZONE);
        // e3:SetTargetRange(0,1)
        effect::set_target_range(d, e3, 0, 1);
        // e3:SetCondition(s.xyzcond)
        effect::set_condition(d, e3, Some(lua_fn(xyzcond)));
        // e3:SetValue(s.xyzlimit)
        effect::set_value_fn(d, e3, lua_fn(xyzlimit));
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
        // c:RegisterFlagEffect(0,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,3))
        card::register_flag_effect(d, c, 0, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 3));
    }
}
/// `s.fuscond(e)`
fn fuscond(d: &mut Duel, _a: Args) -> bool {
    // return Duel.IsBattlePhase()
    duel::is_battle_phase(d, None)
}
/// `s.fuslimit(e,re,tp)`
fn fuslimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsMonsterEffect()
    effect::is_monster_effect(d, a.e(1))
}
/// `s.syncond(e)`
fn syncond(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetTurnPlayer()==e:GetHandlerPlayer() and Duel.IsMainPhase()
    let hp = effect::get_handler_player(d, a.e(0));
    duel::get_turn_player(d) == hp && duel::is_main_phase(d, None)
}
/// `s.synlimit(e,re,tp)`
fn synlimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsSpellTrapEffect()
    effect::is_spell_trap_effect(d, a.e(1))
}
/// `s.xyzcond(e)`
fn xyzcond(d: &mut Duel, a: Args) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let _ph = duel::get_current_phase(d);
    // return Duel.GetTurnPlayer()==e:GetHandlerPlayer() and (Duel.IsBattlePhase() or Duel.IsMainPhase())
    let hp = effect::get_handler_player(d, a.e(0));
    duel::get_turn_player(d) == hp && (duel::is_battle_phase(d, None) || duel::is_main_phase(d, None))
}
/// `s.xyzlimit(e,re,tp)`
fn xyzlimit(d: &mut Duel, a: Args) -> bool {
    // return re:GetActivateLocation()==LOCATION_GRAVE
    effect::get_activate_location(d, a.e(1)) == LOCATION_GRAVE
}
