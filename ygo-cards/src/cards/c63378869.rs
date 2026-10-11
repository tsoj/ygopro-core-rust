//! Aiza the Dragoness of Deranged Devotion
//! (Lua: 狂愛の竜娘アイザ)
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 63378869;
const COUNTER_DERANGED: u32 = 0x1209;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_DERANGED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Place 1 Deranged Counter on opponent monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.cttg)
    effect::set_target(d, e1, Some(cost_target(cttg)));
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy monster with Deranged Counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e2, EVENT_BATTLE_START);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsCanAddCounter,tp,0,LOCATION_MZONE,1,nil,COUNTER_DERANGED,1) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_DERANGED as u16, 1, false, 0)),
            a.tp,
            0,
            LOCATION_MZONE,
            1,
            NO_EX,
        );
    }
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local tc=Duel.SelectMatchingCard(tp,Card.IsCanAddCounter,tp,0,LOCATION_MZONE,1,1,nil,COUNTER_DERANGED,1):GetFirst()
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_DERANGED as u16, 1, false, 0)),
        tp,
        0,
        LOCATION_MZONE,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let tc = group::get_first(d, &g);
    // if tc and tc:AddCounter(COUNTER_DERANGED,1) and not tc:HasFlagEffect(id) then
    let Some(tc) = tc else {
        return;
    };
    if !card::add_counter(d, tc, COUNTER_DERANGED as u16, 1, false) || card::has_flag_effect(d, tc, ID, None) {
        return;
    }
    // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
    card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
    // Cannot be used as material for a Fusion, Synchro, Xyz, or Link Summon
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_MATERIAL);
    // e1:SetCondition(function(_e) return _e:GetHandler():HasCounter(COUNTER_DERANGED) end)
    effect::set_condition(d, e1, Some(condition(mat_cond)));
    // e1:SetValue(aux.cannotmatfilter(SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK))
    effect::set_value_fn(
        d,
        e1,
        aux::cannotmatfilter(&[SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_XYZ, SUMMON_TYPE_LINK]),
    );
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
}
/// `function(_e) return _e:GetHandler():HasCounter(COUNTER_DERANGED) end` of the material lock
fn mat_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::has_counter(d, h, COUNTER_DERANGED as u16)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    // return bc and bc:HasCounter(COUNTER_DERANGED) and bc:IsControler(1-tp)
    match card::get_battle_target(d, h) {
        Some(bc) => card::has_counter(d, bc, COUNTER_DERANGED as u16) && card::is_controler(d, bc, 1 - a.tp),
        None => false,
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c).nil();
    // if chk==0 then return bc:IsRelateToBattle() end
    if a.chk == Some(0) {
        return card::is_relate_to_battle(d, bc);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,bc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(bc)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,1,PLAYER_ALL,bc:GetBaseAttack())
    let atk = card::get_base_attack(d, bc);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, PLAYER_ALL as u8, atk);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,c,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// The `function(ag) Duel.Destroy(ag,REASON_EFFECT) end` of the delayed operation.
fn destroy_oper(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::destroy(d, ag.clone(), REASON_EFFECT, None, None);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c).nil();
    // if bc:IsRelateToBattle() and bc:IsControler(1-tp) and Duel.Destroy(bc,REASON_EFFECT)>0 then
    if card::is_relate_to_battle(d, bc)
        && card::is_controler(d, bc, 1 - tp)
        && duel::destroy(d, bc, REASON_EFFECT, None, None) > 0
    {
        // local dam=bc:GetBaseAttack()
        let dam = card::get_base_attack(d, bc);
        // if dam>0 then
        if dam > 0 {
            // Duel.Damage(tp,dam,REASON_EFFECT,true)
            duel::damage(d, tp, dam as i64, REASON_EFFECT, true, None);
            // Duel.Damage(1-tp,dam,REASON_EFFECT,true)
            duel::damage(d, 1 - tp, dam as i64, REASON_EFFECT, true, None);
            // Duel.RDComplete()
            duel::rd_complete(d);
        }
    }
    // if c:IsRelateToBattle() then
    if card::is_relate_to_battle(d, c) {
        // aux.DelayedOperation(c,PHASE_BATTLE,id,e,tp,function(ag) Duel.Destroy(ag,REASON_EFFECT) end,nil,nil,1,aux.Stringid(id,3))
        aux::delayed_operation(
            d,
            c,
            PHASE_BATTLE,
            ID,
            a.e,
            tp,
            Some(std::sync::Arc::new(destroy_oper)),
            None,
            None,
            Some(1),
            Some(aux::stringid(ID, 3)),
            None,
        );
    }
}
