//! Gunkan Suship Ikura-class Dreadnought
use super::prelude::*;

pub const ID: u32 = 75215744;
pub const CARD_SUSHIP_IKURA: u32 = 61027400;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GUNKAN], listed_names: &[CARD_SUSHIP_SHARI, CARD_SUSHIP_IKURA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz summon
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Destroy 1 card your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gains effects based on material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetLabel(0)
    effect::set_label(d, e2, &[0]);
    // e2:SetCondition(s.regcon)
    effect::set_condition(d, e2, Some(condition(regcon)));
    // e2:SetTarget(s.regtg)
    effect::set_target(d, e2, Some(cost_target(regtg)));
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e3, EFFECT_MATERIAL_CHECK);
    // e3:SetValue(s.valcheck)
    effect::set_value_fn(d, e3, lua_fn(valcheck));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.descon(e,tp,eg,ep,...)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // if ep==tp then return false end
    if a.ep == a.tp {
        return false;
    }
    // local rc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let rc = group::get_first(d, &eg).nil();
    // return rc:IsControler(tp) and rc:IsSetCard(SET_GUNKAN) and rc:IsSummonLocation(LOCATION_EXTRA)
    card::is_controler(d, rc, a.tp) && card::is_set_card(d, rc, &[SET_GUNKAN as u16]) && card::is_summon_location(d, rc, LOCATION_EXTRA)
}
/// `s.destg(e,tp,...,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
/// `s.regcon(e,tp,...)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned() and e:GetLabel()>0
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h) && effect::get_label(d, a.e) > 0
}
/// `s.regtg(e,tp,...,chk)`
fn regtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local effs=e:GetLabel()
    let effs = effect::get_label(d, a.e);
    // if chk==0 then return ((effs&1)>0 and Duel.IsPlayerCanDraw(tp,1)) or (effs&2)>0 end
    if a.chk == Some(0) {
        return ((effs & 1) > 0 && duel::is_player_can_draw(d, a.tp, 1)) || (effs & 2) > 0;
    }
    // if (effs&1)>0 then
    if (effs & 1) > 0 {
        // e:SetCategory(CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_DRAW);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    } else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    }
    true
}
/// `s.regop(e,tp,...)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local effs=e:GetLabel()
    let effs = effect::get_label(d, a.e);
    // --"Gukan Suship Shari": Draw 1 card
    // if (effs&1)>0 and Duel.IsPlayerCanDraw(tp,1) then
    if (effs & 1) > 0 && duel::is_player_can_draw(d, a.tp, 1) {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
    // --"Gukan Suship Ikura": This card can make a second attack during each Battle Phase.
    // if (effs&2)>0 then
    if (effs & 2) > 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3201)
        effect::set_description(d, e1, 3201);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) -> Value {
    let (e, c) = (a.e(0), a.c(1));
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local effs=0
    let mut effs: i64 = 0;
    // if g:IsExists(Card.IsCode,1,nil,CARD_SUSHIP_SHARI) then effs=effs|1 end
    if group::is_exists(d, &g, &|d, x| card::is_code(d, x, &[CARD_SUSHIP_SHARI]), 1, NO_EX) {
        effs |= 1;
    }
    // if g:IsExists(Card.IsCode,1,nil,CARD_SUSHIP_IKURA) then effs=effs|2 end
    if group::is_exists(d, &g, &|d, x| card::is_code(d, x, &[CARD_SUSHIP_IKURA]), 1, NO_EX) {
        effs |= 2;
    }
    // e:GetLabelObject():SetLabel(effs)
    let e2 = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, e2, &[effs]);
    Value::Nil
}
