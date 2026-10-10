//! Life Stream Dragon
use super::prelude::*;

pub const ID: u32 = 25165047;

pub const INFO: ScriptInfo = ScriptInfo { material: &[2403771], listed_names: &[2403771], synchro_nt_required: Some(1), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon Procedure
    // Synchro.AddProcedure(c,nil,1,1,aux.FilterSummonCode(2403771),1,1)
    synchro::add_procedure(d, c, None, 1, 1, Some(aux::filter_summon_code(&[2403771])), 1, 1, None, None, None, None, None);
    // Your LP becomes 4000
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function (e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_synchro_summoned(d, h)
    })));
    // e1:SetOperation(s.lpop)
    effect::set_operation(d, e1, Some(operation(lpop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Take no effect damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_DAMAGE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetValue(s.damval)
    effect::set_value_fn(d, e2, lua_fn(damval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e3, EFFECT_NO_EFFECT_DAMAGE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destruction replacement for itself
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    // e4:SetTarget(s.desreptg)
    effect::set_target(d, e4, Some(cost_target(desreptg)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Multiple tuner
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e5, EFFECT_MATERIAL_CHECK);
    // e5:SetValue(s.valcheck)
    effect::set_value_fn(d, e5, lua_fn(valcheck));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
fn lpop(d: &mut Duel, a: &EffArgs) {
    // Duel.SetLP(tp,4000)
    duel::set_lp(d, a.tp, 4000);
}
/// `s.damval(e,re,val,r,rp,rc)`
fn damval(d: &mut Duel, a: Args) -> i64 {
    let _ = d;
    // if r&REASON_EFFECT~=0 then return 0 end
    if a.int(3) & REASON_EFFECT as i64 != 0 {
        return 0;
    }
    // return val
    a.int(2)
}
fn repfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsEquipSpell() and c:IsAbleToRemoveAsCost()
    card::is_equip_spell(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE)
    //   and Duel.IsExistingMatchingCard(s.repfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE) && duel::is_existing_matching_card(d, Some(&repfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.repfilter,tp,LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&repfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // Duel.Remove(g,POS_FACEUP,REASON_COST)
        duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) {
    // local c=e:GetHandler() (the card the material check is for)
    let c = a.card(1).nil();
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsType,2,nil,TYPE_TUNER) then
    if group::is_exists(d, &g, &|d, x| card::is_type(d, x, TYPE_TUNER), 2, NO_EX) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
        // e1:SetCode(EFFECT_MULTIPLE_TUNERS)
        effect::set_code(d, e1, EFFECT_MULTIPLE_TUNERS);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~(RESET_TOFIELD)|RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD) | RESET_PHASE | PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
