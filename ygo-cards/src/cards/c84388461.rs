//! Zefrasaber, Swordmaster of the Nekroz
use super::prelude::*;

pub const ID: u32 = 84388461;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NEKROZ, SET_ZEFRA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // --You cannot Pendulum Summon monsters, except "Nekroz" and "Zefra" monsters, this effect cannot be negated
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE)
    effect::set_property(d, e0, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE, 0);
    // e0:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e0, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e0:SetRange(LOCATION_PZONE)
    effect::set_range(d, e0, LOCATION_PZONE);
    // e0:SetTargetRange(1,0)
    effect::set_target_range(d, e0, 1, 0);
    // e0:SetTarget(function(e,c,sump,sumtype,sumpos,targetp) return (sumtype&SUMMON_TYPE_PENDULUM)==SUMMON_TYPE_PENDULUM and not c:IsSetCard({SET_NEKROZ,SET_ZEFRA}) end)
    effect::set_target(
        d,
        e0,
        Some(lua_fn(|d, a: Args| {
            let sumtype = a.int(3) as u32;
            let c = a.c(1);
            (sumtype & SUMMON_TYPE_PENDULUM) == SUMMON_TYPE_PENDULUM && !card::is_set_card(d, c, &[SET_NEKROZ as u16, SET_ZEFRA as u16])
        })),
    );
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // local ritual_target_params={handler=c,lvtype=RITPROC_EQUAL,filter=function(ritual_c) return ritual_c:IsSetCard(SET_NEKROZ) and ritual_c~=c end,forcedselection=s.forcedselection}
    let ritual_target_params = ritual::RitualProc {
        lvtype: RITPROC_EQUAL,
        filter: Some(std::sync::Arc::new(move |d: &mut Duel, ritual_c: CardId| card::is_set_card(d, ritual_c, &[SET_NEKROZ as u16]) && ritual_c != c)),
        forcedselection: Some(std::sync::Arc::new(forcedselection)),
        ..Default::default()
    };
    // local ritual_operation_params={handler=c,lvtype=RITPROC_EQUAL,filter=function(ritual_c) return ritual_c:IsSetCard(SET_NEKROZ) end}
    let ritual_operation_params = ritual::RitualProc {
        lvtype: RITPROC_EQUAL,
        filter: Some(std::sync::Arc::new(|d: &mut Duel, ritual_c: CardId| card::is_set_card(d, ritual_c, &[SET_NEKROZ as u16]))),
        ..Default::default()
    };
    // --Tribute monsters from your hand or field, then Ritual Summon 1 "Nekroz" Ritual Monster from your hand whose Level exactly equals the total Levels of those monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND|LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    // e1:SetTarget(Ritual.Target(ritual_target_params))
    let rit_tg = ritual::target(d, ritual_target_params);
    effect::set_target(d, e1, Some(rit_tg));
    // e1:SetOperation(Ritual.Operation(ritual_operation_params))
    let rit_op = ritual::operation(d, ritual_operation_params);
    effect::set_operation(d, e1, Some(rit_op));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.forcedselection(e,tp,g,sc)`: `return not g:IsContains(c), g:IsContains(c)` with `c=e:GetHandler()`
fn forcedselection(d: &mut Duel, e: EffectId, _tp: u8, g: &CardSet, _sc: CardId) -> (bool, bool) {
    let c = effect::get_handler(d, e).nil();
    (!g.contains(c), g.contains(c))
}
