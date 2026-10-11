//! Emissary from the House of Wax
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 87462901;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let att = duel::get_attacker(d).nil();
            card::is_controler(d, att, 1 - a.tp)
        })),
    );
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Neither monster can be destroyed by battle
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(s.indestg)
    effect::set_target(d, e2, Some(effect_card_fn(indestg)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Apply effects at the end of the opponent's Battle Phase
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e3, CATEGORY_DISABLE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_BATTLE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e3, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e3:SetTarget(s.efftg)
    effect::set_target(d, e3, Some(cost_target(efftg)));
    // e3:SetOperation(s.effop)
    effect::set_operation(d, e3, Some(operation(effop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, LOCATION_HAND as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.indestg(e,c)
fn indestg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local handler=e:GetHandler()
    let handler = effect::get_handler(d, e).nil();
    // return c==handler or c==handler:GetBattleTarget()
    c == handler || card::get_battle_target(d, handler) == Some(c)
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttackPos() and not c:HasFlagEffect(id)
    //   and not (c:IsHasEffect(EFFECT_CANNOT_CHANGE_POSITION) and c:IsHasEffect(EFFECT_CANNOT_BE_MATERIAL) and not c:IsNegatableMonster())
    card::is_attack_pos(d, c)
        && !card::has_flag_effect(d, c, ID, None)
        && !(!card::is_has_effect(d, c, EFFECT_CANNOT_CHANGE_POSITION, None).is_empty()
            && !card::is_has_effect(d, c, EFFECT_CANNOT_BE_MATERIAL, None).is_empty()
            && !card::is_negatable_monster(d, c))
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil):Match(aux.NOT(Card.IsImmuneToEffect),nil,e)
    let mut g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE, NO_EX);
    let e = a.e;
    group::match_(d, &mut g, &|d, c| !card::is_immune_to_effect(d, c, e), NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
        // Cannot change its battle position
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CANNOT_CHANGE_POSITION)
        effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
        // e1:SetCondition(function(_e) return _e:GetHandler():HasFlagEffect(id) end)
        effect::set_condition(
            d,
            e1,
            Some(condition(|d, a| {
                let h = effect::get_handler(d, a.e).nil();
                card::has_flag_effect(d, h, ID, None)
            })),
        );
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // Cannot be used as material for a Fusion, Synchro, Xyz, or Link Summon
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_CANNOT_BE_MATERIAL)
        effect::set_code(d, e2, EFFECT_CANNOT_BE_MATERIAL);
        // e2:SetValue(aux.cannotmatfilter(SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK))
        effect::set_value_fn(d, e2, aux::cannotmatfilter(&[SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_XYZ, SUMMON_TYPE_LINK]));
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        // Negate its effects
        // local e3=e1:Clone()
        let e3 = effect::clone(d, e1);
        // e3:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e3, EFFECT_DISABLE);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
        // local e4=e1:Clone()
        let e4 = effect::clone(d, e1);
        // e4:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e4, EFFECT_DISABLE_EFFECT);
        // e4:SetValue(RESET_TURN_SET)
        effect::set_value(d, e4, RESET_TURN_SET as i64);
        // tc:RegisterEffect(e4)
        card::register_effect(d, tc, e4, false);
    }
    // Reset the flags at the start of the opponent's next Battle Phase
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_PHASE|PHASE_BATTLE_START)
    effect::set_code(d, e5, EVENT_PHASE | PHASE_BATTLE_START);
    // e5:SetCondition(function() return Duel.IsTurnPlayer(1-tp) end)
    let tp = a.tp;
    effect::set_condition(d, e5, Some(condition(move |d, _a| duel::is_turn_player(d, 1 - tp))));
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetOperation(s.reset)
    effect::set_operation(d, e5, Some(operation(reset)));
    // e5:SetReset(RESET_PHASE|PHASE_BATTLE_START|RESET_OPPO_TURN)
    effect::set_reset(d, e5, RESET_PHASE | PHASE_BATTLE_START | RESET_OPPO_TURN, 1);
    // Duel.RegisterEffect(e5,tp)
    duel::register_effect(d, e5, tp);
}
// function s.reset(e)
fn reset(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.HasFlagEffect,0,LOCATION_MZONE,LOCATION_MZONE,nil,id)
    let g = duel::get_matching_group(d, Some(&|d, c| card::has_flag_effect(d, c, ID, None)), 0, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // tc:ResetFlagEffect(id)
        card::reset_flag_effect(d, tc, ID);
    }
    // e:Reset()
    effect::reset(d, a.e);
}
