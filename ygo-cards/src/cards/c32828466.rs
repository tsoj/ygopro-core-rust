//! Wake Up Your Elemental HERO
use super::prelude::*;

pub const ID: u32 = 32828466;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ELEMENTAL_HERO],
    material_setcode: &[SET_HERO, SET_ELEMENTAL_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixRep(c,true,true,aux.FilterBoolFunctionEx(Card.IsRace,RACE_WARRIOR),1,99,s.mfilter)
    fusion::add_proc_mix_rep(
        d,
        c,
        true,
        true,
        fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_race_ctx(d, x, RACE_WARRIOR, Some(fc), st, Some(tp)))),
        1,
        99,
        vec![fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| mfilter(d, x, fc, st, tp)))],
    );
    // Must be Fusion Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.fuslimit)
    effect::set_value_fn(d, e1, aux::fuslimit());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Fusion Materials check
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e2, EFFECT_MATERIAL_CHECK);
    // e2:SetValue(s.matcheck)
    effect::set_value_fn(d, e2, effect_card_value(|d, _e, c| {
        matcheck(d, c);
        0
    }));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy and inflict damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_BATTLED)
    effect::set_code(d, e3, EVENT_BATTLED);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon 1 Warrior from your hand/Deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetCondition(s.spcon)
    effect::set_condition(d, e4, Some(condition(spcon)));
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.mfilter(c,fc,sumtype,tp)`
fn mfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_ELEMENTAL_HERO,fc,sumtype,tp) and c:IsType(TYPE_FUSION,fc,sumtype,tp)
    card::is_set_card_ctx(d, c, &[SET_ELEMENTAL_HERO as u16], Some(fc), sumtype, Some(tp))
        && card::is_type_ctx(d, c, TYPE_FUSION, Some(fc), sumtype, Some(tp))
}
/// `s.matcheck(e,c)`
fn matcheck(d: &mut Duel, c: CardId) {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // Increase ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(#g*300)
    effect::set_value(d, e1, (g.len() * 300) as i64);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE&~RESET_TOFIELD)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Multiple attacks
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
    effect::set_code(d, e2, EFFECT_EXTRA_ATTACK_MONSTER);
    // e2:SetValue(g:FilterCount(Card.IsType,nil,TYPE_FUSION)-1)
    let fusions = group::filter_count(d, &g, &|d, x| card::is_type(d, x, TYPE_FUSION), NO_EX) as i64;
    effect::set_value(d, e2, fusions - 1);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE&~RESET_TOFIELD)
    effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    // return bc and bc:IsRelateToBattle()
    match card::get_battle_target(d, h) {
        Some(bc) => card::is_relate_to_battle(d, bc),
        None => false,
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, bc.map(CardSet::from_card), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,bc:GetBaseAttack())
    let atk = card::get_base_attack(d, bc.nil());
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    // if bc and bc:IsRelateToBattle() and Duel.Destroy(bc,REASON_EFFECT)>0 then
    if let Some(bc) = card::get_battle_target(d, h) {
        if card::is_relate_to_battle(d, bc) && duel::destroy(d, bc, REASON_EFFECT, None, None) > 0 {
            // Duel.Damage(1-tp,bc:GetBaseAttack(),REASON_EFFECT)
            let atk = card::get_base_attack(d, bc);
            duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
        }
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsFusionSummoned()
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_fusion_summoned(d, c)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.spfilter(c,e,sp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, sp: u8) -> bool {
    // return c:IsRace(RACE_WARRIOR) and c:IsCanBeSpecialSummoned(e,0,sp,false,false)
    card::is_race(d, c, RACE_WARRIOR) && card::is_can_be_special_summoned(d, c, e, 0, sp, false, false, None, None, None)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK|LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
