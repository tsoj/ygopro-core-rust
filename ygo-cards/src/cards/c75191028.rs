//! GMX - COMPREX
use super::prelude::*;

pub const ID: u32 = 75191028;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GMX],
    material_setcode: &[SET_GMX],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "GMX" monster + 2+ Dinosaur monsters
    // Fusion.AddProcMixRep(c,true,true,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DINOSAUR),2,99,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GMX))
    fusion::add_proc_mix_rep(
        d,
        c,
        true,
        true,
        fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| {
            card::is_race_ctx(d, x, RACE_DINOSAUR, Some(fc), st, Some(tp))
        })),
        2,
        99,
        vec![fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| {
            card::is_set_card_ctx(d, x, &[SET_GMX as u16], Some(fc), st, Some(tp))
        }))],
    );
    // Gains these effects based on the number of Dinosaur monsters used as Fusion Material for this card
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1a:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SPSUMMON_SUCCESS);
    // e1a:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e1a, Some(condition(e1a_cond)));
    // e1a:SetOperation(s.effop)
    effect::set_operation(d, e1a, Some(operation(effop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // Track the number of Dinosaur monsters used as Fusion Material for this card
    // local e1b=Effect.CreateEffect(c)
    let e1b = effect::create_effect(d, c);
    // e1b:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1b, EFFECT_TYPE_SINGLE);
    // e1b:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1b, EFFECT_MATERIAL_CHECK);
    // e1b:SetValue(function(e,c) e1a:SetLabel(c:GetMaterial():FilterCount(Card.IsRace,nil,RACE_DINOSAUR,c,SUMMON_TYPE_FUSION,e:GetHandlerPlayer())) end)
    effect::set_value_fn(
        d,
        e1b,
        effect_card_value(move |d, e, c| {
            let mat = card::get_material(d, c);
            let tp = effect::get_handler_player(d, e);
            let n = group::filter_count(
                d,
                &mat,
                &|d, x| card::is_race_ctx(d, x, RACE_DINOSAUR, Some(c), SUMMON_TYPE_FUSION as u64, Some(tp)),
                NO_EX,
            );
            effect::set_label(d, e1a, &[n as i64]);
            0
        }),
    );
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Once per turn, if you excavate a card(s) by a "GMX" card effect: You can destroy all other monsters on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_CUSTOM+1595137)
    effect::set_code(d, e2, EVENT_CUSTOM + 1595137);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsFusionSummoned() end` (e1a)
fn e1a_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local dino_mats_count=e:GetLabel()
    let dino_mats_count = effect::get_label(d, a.e);
    // if dino_mats_count>=3 then
    if dino_mats_count >= 3 {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1))
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
        // --● 3+: Your opponent cannot target this card with card effects
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
        effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
        // e1:SetValue(aux.tgoval)
        effect::set_value_fn(d, e1, aux::tgoval());
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // if dino_mats_count>=4 then
    if dino_mats_count >= 4 {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
        // --● 4+: Can make up to 3 attacks during each Battle Phase
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e2, EFFECT_EXTRA_ATTACK);
        // e2:SetValue(2)
        effect::set_value(d, e2, 2);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
    // if dino_mats_count>=5 then
    if dino_mats_count >= 5 {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,3))
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 3));
        // --● 5+: Each time your opponent Normal or Special Summons a monster(s), they lose 800 LP
        // local e3a=Effect.CreateEffect(c)
        let e3a = effect::create_effect(d, c);
        // e3a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e3a:SetCode(EVENT_SUMMON_SUCCESS)
        effect::set_code(d, e3a, EVENT_SUMMON_SUCCESS);
        // e3a:SetRange(LOCATION_MZONE)
        effect::set_range(d, e3a, LOCATION_MZONE);
        // e3a:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end)
        effect::set_condition(d, e3a, Some(condition(summon_cond)));
        // e3a:SetOperation(s.lpop)
        effect::set_operation(d, e3a, Some(operation(lpop)));
        // e3a:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3a, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e3a)
        card::register_effect(d, c, e3a, false);
        // local e3b=e3a:Clone()
        let e3b = effect::clone(d, e3a);
        // e3b:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, e3b, EVENT_SPSUMMON_SUCCESS);
        // c:RegisterEffect(e3b)
        card::register_effect(d, c, e3b, false);
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end`
fn summon_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, x| card::is_summon_player(d, x, 1 - a.tp), 1, NO_EX)
}
/// `s.lpop(e,tp,eg,ep,ev,re,r,rp)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsChainSolving() then
    if !duel::is_chain_solving(d) {
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // local opp=1-tp
        let opp = 1 - a.tp;
        // Duel.SetLP(opp,Duel.GetLP(opp)-800)
        let lp = duel::get_lp(d, opp) - 800;
        duel::set_lp(d, opp, lp);
    } else {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // Your opponent loses 800 LP at the end of the Chain Link
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetOperation(function(e,tp) Duel.Hint(HINT_CARD,0,id) local opp=1-tp return Duel.SetLP(opp,Duel.GetLP(opp)-800) end)
        effect::set_operation(
            d,
            e1,
            Some(operation(|d, a| {
                duel::hint(d, HINT_CARD as u8, 0, ID as u64);
                let opp = 1 - a.tp;
                let lp = duel::get_lp(d, opp) - 800;
                duel::set_lp(d, opp, lp);
            })),
        );
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_CHAIN)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // Reset "e1" at the end of the Chain Link
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
        // e2:SetOperation(function() e1:Reset() end)
        effect::set_operation(
            d,
            e2,
            Some(operation(move |d, _a| {
                effect::reset(d, e1);
            })),
        );
        // e2:SetReset(RESET_CHAIN)
        effect::set_reset(d, e2, RESET_CHAIN, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and r&REASON_EFFECT>0 and Chain.IsSetcode(0,SET_GMX)
    a.ep == a.tp && a.r & REASON_EFFECT > 0 && chain::is(d, 0, chain::Prop::SetCard, &[SET_GMX as i64]).truthy()
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, h.into());
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc = if card::is_relate_to_effect(d, c, a.e) { c.into() } else { NO_EX };
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,exc)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, exc);
    // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
