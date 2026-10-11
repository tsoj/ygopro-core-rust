//! Prank-Kids Meow-Meow-Mu
use super::prelude::*;

pub const ID: u32 = 25725326;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PRANK_KIDS], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 1 Level 4 or lower "Prank-Kids" monster
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| matfilter(d, c, sc, st, tp))), 1, Some(1), None, None, None);
    // You can only Link Summon "Prank-Kids Meow-Meow-Mu" once per turn
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e0, EVENT_SPSUMMON_SUCCESS);
    // e0:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e0, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_link_summoned(d, h)
    })));
    // e0:SetOperation(s.regop)
    effect::set_operation(d, e0, Some(operation(regop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // If a "Prank-Kids" monster you control would Tribute itself to activate its effect during your opponent's turn, you can banish this card you control or from your GY instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_COST_REPLACE)
    effect::set_code(d, e1, EFFECT_COST_REPLACE);
    // e1:SetRange(LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_GRAVE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsAbleToRemoveAsCost() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_able_to_remove_as_cost(d, h, None)
    })));
    // e1:SetValue(s.repval)
    effect::set_value_fn(d, e1, lua_fn(repval));
    // e1:SetOperation(function(base) Duel.Remove(base:GetHandler(),POS_FACEUP,REASON_COST|REASON_REPLACE) end)
    effect::set_operation(d, e1, Some(lua_fn(|d, a: Args| {
        let base = a.e(0);
        let h = effect::get_handler(d, base).nil();
        duel::remove(d, h, POS_FACEUP as u8, REASON_COST | REASON_REPLACE, None, None);
    })));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.matfilter(c,lc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_PRANK_KIDS,lc,sumtype,tp) and c:IsLevelBelow(4)
    card::is_set_card_ctx(d, c, &[SET_PRANK_KIDS as u16], lc, sumtype, Some(tp)) && card::is_level_below(d, c, 4)
}
/// `s.regop(e,tp,...)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()  (the effect is created on the handler)
    let h = effect::get_handler(d, a.e).nil();
    // You cannot Link Summon "Prank-Kids Meow-Meow-Mu" for the rest of this turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c,sump,sumtype,sumpos,targetp,se) return c:IsCode(id) and sumtype&SUMMON_TYPE_LINK==SUMMON_TYPE_LINK end)
    effect::set_target(d, e1, Some(lua_fn(|d, a: Args| {
        let c = a.c(1);
        let sumtype = a.int(3) as u64;
        card::is_code(d, c, &[ID]) && sumtype & SUMMON_TYPE_LINK as u64 == SUMMON_TYPE_LINK as u64
    })));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.repval(base,extracon,e,tp,eg,ep,ev,re,r,rp,chk)`
fn repval(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let e = a.e(2);
    let tp = a.p(3);
    let c = effect::get_handler(d, e).nil();
    // return Duel.IsTurnPlayer(1-tp) and c:IsSetCard(SET_PRANK_KIDS) and c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    //     and extracon(base,e,tp,eg,ep,ev,re,r,rp)
    if !(duel::is_turn_player(d, 1 - tp)
        && card::is_set_card(d, c, &[SET_PRANK_KIDS as u16])
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp))
    {
        return false;
    }
    let extracon = a.v(1).as_fn().nil().clone();
    let args = [
        a.v(0).clone(),
        a.v(2).clone(),
        a.v(3).clone(),
        a.v(4).clone(),
        a.v(5).clone(),
        a.v(6).clone(),
        a.v(7).clone(),
        a.v(8).clone(),
        a.v(9).clone(),
    ];
    extracon.call(d, &args).truthy()
}
