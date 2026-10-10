//! Blue-Eyes Twin Burst Dragon
use super::prelude::*;

pub const ID: u32 = 2129638;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.material_setcode=SET_BLUE_EYES
    material_setcode: &[SET_BLUE_EYES],
    // s.listed_names={CARD_BLUEEYES_W_DRAGON}
    listed_names: &[CARD_BLUEEYES_W_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,CARD_BLUEEYES_W_DRAGON,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::Code(CARD_BLUEEYES_W_DRAGON), 2)]);
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.splimit,nil,nil,nil,false)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(Some(lua_fn(splimit))),
            cannot_be_lizard: Some(false),
            ..Default::default()
        },
    );
    // indes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // attack twice
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
    effect::set_code(d, e4, EFFECT_EXTRA_ATTACK_MONSTER);
    // e4:SetValue(1)
    effect::set_value(d, e4, 1);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // remove
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e7, aux::stringid(ID, 0));
    // e7:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e7, CATEGORY_REMOVE);
    // e7:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e7, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e7:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e7, EVENT_DAMAGE_STEP_END);
    // e7:SetCondition(s.rmcon)
    effect::set_condition(d, e7, Some(condition(rmcon)));
    // e7:SetTarget(s.rmtg)
    effect::set_target(d, e7, Some(cost_target(rmtg)));
    // e7:SetOperation(s.rmop)
    effect::set_operation(d, e7, Some(operation(rmop)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(_d: &mut Duel, a: Args) -> bool {
    // return (st&SUMMON_TYPE_FUSION)==SUMMON_TYPE_FUSION
    (a.int(3) as u32 & SUMMON_TYPE_FUSION) == SUMMON_TYPE_FUSION
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(Card.IsAbleToGraveAsCost,tp,LOCATION_MZONE,0,nil)
    duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_MZONE, 0, NO_EX)
}
/// `s.contactop(g)`
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, _c: CardId) {
    // Duel.SendtoGrave(g,REASON_COST|REASON_MATERIAL)
    duel::send_to_grave(d, g, REASON_COST | REASON_MATERIAL, None, None);
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // e:SetLabelObject(bc)
    effect::set_label_object(d, a.e, bc.map_or(Value::Nil, Value::Card));
    // return c==Duel.GetAttacker() and bc and c:IsStatus(STATUS_OPPO_BATTLE) and bc:IsOnField() and bc:IsRelateToBattle()
    if duel::get_attacker(d) != Some(c) {
        return false;
    }
    let Some(bc) = bc else { return false };
    card::is_status(d, c, STATUS_OPPO_BATTLE) && card::is_on_field(d, bc) && card::is_relate_to_battle(d, bc)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetLabelObject():IsAbleToRemove() end
    if a.chk == Some(0) {
        let bc = effect::get_label_object(d, a.e).as_card().nil();
        return card::is_able_to_remove(d, bc, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,e:GetLabelObject(),1,0,0)
    let bc = effect::get_label_object(d, a.e).as_card();
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, bc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetLabelObject()
    let bc = effect::get_label_object(d, a.e).as_card().nil();
    // if bc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, bc) {
        // Duel.Remove(bc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, bc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
