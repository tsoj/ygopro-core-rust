//! GMX - ALLOS
use super::prelude::*;

pub const ID: u32 = 56311997;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GMX],
    material_setcode: &[SET_GMX],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "GMX" monster + 1 Dinosaur monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GMX),aux.FilterBoolFunctionEx(Card.IsRace,RACE_DINOSAUR))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| {
                card::is_set_card_ctx(d, x, &[SET_GMX as u16], Some(fc), st, Some(tp))
            })),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| {
                card::is_race_ctx(d, x, RACE_DINOSAUR, Some(fc), st, Some(tp))
            })),
        ],
    );
    // Each time your opponent Normal or Special Summons a monster(s), gain 200 LP
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end)
    effect::set_condition(d, e1a, Some(condition(e1_cond)));
    // e1a:SetOperation(s.lpop)
    effect::set_operation(d, e1a, Some(operation(lpop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // You can target 1 "GMX" monster or 1 Dinosaur monster in your GY or banishment; Special Summon it.
    // This card cannot attack the turn this effect is activated
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // At the start of the Damage Step, if this card battles an opponent's monster: You can destroy that monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e3, EVENT_BATTLE_START);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end` (e1a)
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, x| card::is_summon_player(d, x, 1 - a.tp), 1, NO_EX)
}
/// `s.lpop(e,tp,eg,ep,ev,re,r,rp)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsChainSolving() then
    if !duel::is_chain_solving(d) {
        // Duel.Recover(tp,200,REASON_EFFECT)
        duel::recover(d, a.tp, 200, REASON_EFFECT, false, None);
    } else {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // Gain 200 LP at the end of the Chain Link
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) return Duel.Recover(tp,200,REASON_EFFECT) end)
        effect::set_operation(
            d,
            e1,
            Some(operation(|d, a| {
                duel::recover(d, a.tp, 200, REASON_EFFECT, false, None);
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
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetAttackAnnouncedCount()==0 end
    if a.chk == Some(0) {
        return card::get_attack_announced_count(d, c) == 0;
    }
    // This card cannot attack the turn this effect is activated
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(3206)
    effect::set_description(d, e1, 3206);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsSetCard(SET_GMX) or c:IsRace(RACE_DINOSAUR)) and c:IsFaceup() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    (card::is_set_card(d, c, &[SET_GMX as u16]) || card::is_race(d, c, RACE_DINOSAUR as u64))
        && card::is_faceup(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp)
            && card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED)
            && spfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(
                d,
                Some(&|d, c| spfilter(d, c, a.e, tp)),
                tp,
                LOCATION_GRAVE | LOCATION_REMOVED,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_target(
        d,
        tp,
        Some(&|d, c| spfilter(d, c, a.e, tp)),
        tp,
        LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        1,
        1,
        false,
        NO_EX,
    );
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetBattleTarget()
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    // if chk==0 then return bc and bc:IsControler(1-tp) end
    if a.chk == Some(0) {
        return bc.is_some_and(|bc| card::is_controler(d, bc, 1 - a.tp));
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,bc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, bc.map(CardSet::from_card), 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetHandler():GetBattleTarget()
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c).nil();
    // if bc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, bc) {
        // Duel.Destroy(bc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(bc), REASON_EFFECT, None, None);
    }
}
