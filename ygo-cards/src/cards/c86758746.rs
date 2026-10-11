//! Amazoness Secret Arts
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 86758746;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMAZONESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 "Amazoness" monster
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsSetCard,SET_AMAZONESS))
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_AMAZONESS as u16]))),
            ..Default::default()
        },
    );
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Allow Fusion material from the Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetOperation(s.fexop)
    effect::set_operation(d, e2, Some(operation(fexop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.fexop(e,tp,eg,ep,ev,re,r,rp)`
fn fexop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_EXTRA_FUSION_MATERIAL)
    effect::set_code(d, e1, EFFECT_EXTRA_FUSION_MATERIAL);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTargetRange(LOCATION_EXTRA,0)
    effect::set_target_range(d, e1, LOCATION_EXTRA, 0);
    // e1:SetTarget(function(e,c) return c:IsSetCard(SET_AMAZONESS) and c:IsAbleToGrave() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_AMAZONESS as u16]) && card::is_able_to_grave(d, c, None))));
    // e1:SetValue(s.matval)
    effect::set_value_fn(d, e1, lua_fn(matval));
    // e1:SetLabelObject({s.extrafil_replacement})
    effect::set_label_object(d, e1, Value::Multi(vec![Value::Fn(lua_fn(extrafil_replacement))]));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
}
/// `s.matval(e,c)`
fn matval(d: &mut Duel, a: Args) -> bool {
    // return c and c:IsSetCard(SET_AMAZONESS) and c:IsControler(e:GetHandlerPlayer())
    let Some(c) = a.card(1) else {
        return false;
    };
    let hp = effect::get_handler_player(d, a.e(0));
    card::is_set_card(d, c, &[SET_AMAZONESS as u16]) && card::is_controler(d, c, hp)
}
/// `s.extrafil_repl_filter(c)`
fn extrafil_repl_filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToGrave() and c:IsSetCard(SET_AMAZONESS)
    card::is_monster(d, c) && card::is_able_to_grave(d, c, None) && card::is_set_card(d, c, &[SET_AMAZONESS as u16])
}
/// `s.extrafil_replacement(e,tp,mg)`
fn extrafil_replacement(d: &mut Duel, a: Args) -> Value {
    // local g=Duel.GetMatchingGroup(s.extrafil_repl_filter,tp,LOCATION_EXTRA,0,nil)
    let tp = a.p(1);
    let g = duel::get_matching_group(d, Some(&extrafil_repl_filter), tp, LOCATION_EXTRA, 0, NO_EX);
    // return g,s.fcheck_replacement
    Value::Multi(vec![Value::Group(g), Value::Fn(lua_fn(fcheck_replacement))])
}
/// `s.fcheck_replacement(tp,sg,fc)`
fn fcheck_replacement(d: &mut Duel, a: Args) -> bool {
    // return sg:FilterCount(Card.IsLocation,nil,LOCATION_EXTRA)<=1
    let sg = a.g(1);
    group::filter_count(d, &sg, &|d, c| card::is_location(d, c, LOCATION_EXTRA), NO_EX) <= 1
}
