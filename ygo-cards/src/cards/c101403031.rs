//! The Sun God's Steelcage - Lava Golem
//! Scripted by Eerie Code
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 101403031;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_SUN_GOD],
    material_setcode: &[SET_SUN_GOD],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Sun God" monster you control + 2 face-up monsters on the field
    // Fusion.AddProcMixN(c,false,false,s.matfilter,1,aux.AND(Card.IsFaceup,Card.IsOnField),2)
    fusion::add_proc_mix_n(
        d,
        c,
        false,
        false,
        vec![
            (fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter(d, c, fc, st, tp))), 1),
            (fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| card::is_faceup(d, c) && card::is_on_field(d, c))), 2),
        ],
    );
    // Cannot be used as Fusion Material
    // local e0a=Effect.CreateEffect(c)
    let e0a = effect::create_effect(d, c);
    // e0a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0a, EFFECT_TYPE_SINGLE);
    // e0a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0a, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0a:SetCode(EFFECT_CANNOT_BE_FUSION_MATERIAL)
    effect::set_code(d, e0a, EFFECT_CANNOT_BE_FUSION_MATERIAL);
    // e0a:SetValue(1)
    effect::set_value(d, e0a, 1);
    // c:RegisterEffect(e0a)
    card::register_effect(d, c, e0a, false);
    // Must be either Fusion Summoned, or Special Summoned (from your Extra Deck) to your opponent's Main Monster Zone by sending the above cards from either field to the GY, but if you Special Summon this card with the second method, you can only Special Summon once for the rest of this turn
    // c:AddMustBeFusionSummoned()
    card::add_must_be_fusion_summoned(d, c);
    // Fusion.AddContactProc({handler=c,materialFilter=s.contactfil,materialOperation=s.contactop,sumtype=1,desc=aux.Stringid(id,0),summonToPlayer=1,summonToZones=ZONES_MMZ})
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(Arc::new(|d: &mut Duel, tp: u8| contactfil(d, tp))),
            material_operation: Some(Arc::new(|d: &mut Duel, g: &CardSet, tp: u8, c: CardId| contactop(d, g, tp, c))),
            sumtype: Some(1),
            desc: Some(aux::stringid(ID, 0)),
            summon_to_player: Some(1),
            summon_to_zones: Some(ZONES_MMZ),
            ..Default::default()
        },
    );
    // You can only Special Summon "The Sun God's Steelcage - Lava Golem" once per turn this way, no matter which method you use
    // local e0c=Effect.CreateEffect(c)
    let e0c = effect::create_effect(d, c);
    // e0c:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0c, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e0c:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0c, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0c:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e0c, EVENT_SPSUMMON_SUCCESS);
    // e0c:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) local c=e:GetHandler() return c:IsFusionSummoned() or c:IsSummonType(SUMMON_TYPE_SPECIAL+1) end)
    effect::set_condition(
        d,
        e0c,
        Some(condition(|d, a| {
            let c = effect::get_handler(d, a.e).nil();
            card::is_fusion_summoned(d, c) || card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL + 1])
        })),
    );
    // e0c:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e0c, Some(operation(op_e0c)));
    // c:RegisterEffect(e0c)
    card::register_effect(d, c, e0c, false);
    // Once per turn, during your Standby Phase: Take 1000 damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_PHASE+PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_STANDBY);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.matfilter(c,fc,sumtype,tp)
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_SUN_GOD,fc,sumtype,tp) and c:IsControler(tp) and c:IsOnField()
    card::is_set_card_ctx(d, c, &[SET_SUN_GOD as u16], Some(fc), sumtype, Some(tp)) && card::is_controler(d, c, tp) && card::is_on_field(d, c)
}
// function s.contactfil(tp)
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(Card.IsAbleToGraveAsCost,tp,LOCATION_MZONE,LOCATION_MZONE,nil,tp)
    duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX)
}
// function s.contactop(g,tp,c)
fn contactop(d: &mut Duel, g: &CardSet, tp: u8, c: CardId) {
    // Duel.SendtoGrave(g,REASON_COST|REASON_MATERIAL)
    duel::send_to_grave(d, g.clone(), REASON_COST | REASON_MATERIAL, None, None);
    // local prev_sp_activity_count=Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)
    let prev_sp_activity_count = duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON])[0];
    // But if you Special Summon this card with the second method, you can only Special Summon once for the rest of this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c,tp) local current_sp_activity_count=Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON) return current_sp_activity_count>prev_sp_activity_count+1 end)
    effect::set_target(
        d,
        e1,
        Some(lua_fn(move |d: &mut Duel, a: Args| {
            let current = duel::get_activity_count(d, a.p(2), &[ACTIVITY_SPSUMMON])[0];
            current > prev_sp_activity_count + 1
        })),
    );
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
// function(e,tp,eg,ep,ev,re,r,rp) (the operation of e0c)
fn op_e0c(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local summon_player=c:GetSummonPlayer()
    let summon_player = card::get_summon_player(d, c);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c,sump,sumtype) return c:IsOriginalCode(id) and (sumtype&SUMMON_TYPE_FUSION==SUMMON_TYPE_FUSION or sumtype&SUMMON_TYPE_SPECIAL+1==SUMMON_TYPE_SPECIAL+1) end)
    effect::set_target(
        d,
        e1,
        Some(lua_fn(|d: &mut Duel, a: Args| {
            let c = a.c(1);
            let sumtype = a.int(3) as u64;
            card::is_original_code(d, c, &[ID])
                && ((sumtype & SUMMON_TYPE_FUSION as u64) == SUMMON_TYPE_FUSION as u64
                    || (sumtype & (SUMMON_TYPE_SPECIAL + 1) as u64) == (SUMMON_TYPE_SPECIAL + 1) as u64)
        })),
    );
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,summon_player)
    duel::register_effect(d, e1, summon_player);
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1000)
    duel::set_target_param(d, 1000);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, 1000);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
