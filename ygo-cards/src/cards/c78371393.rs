//! Yubel
use super::prelude::*;

pub const ID: u32 = 78371393;

// s.listed_names={4779091} --"Yubel - Terror Incarnate"
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[4779091], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You take no battle damage from battles involving this card
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Register that this card was in face-up Attack Position when it was targeted for an attack
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3a:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e3a, EVENT_BE_BATTLE_TARGET);
    // e3a:SetOperation(s.regop)
    effect::set_operation(d, e3a, Some(operation(regop)));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // Inflict damage to your opponent equal to the attacking monster's ATK
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EVENT_BATTLE_CONFIRM)
    effect::set_code(d, e3, EVENT_BATTLE_CONFIRM);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Tribute 1 other monster or destroy this card
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_RELEASE+CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_RELEASE + CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e4, Some(condition(turn_player_cond)));
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Special Summon 1 "Yubel - Terror Incarnate" from your hand, Deck, or GY
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e5, aux::stringid(ID, 2));
    // e5:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e5:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e5, EVENT_DESTROYED);
    // e5:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re~=e4 end)
    effect::set_condition(d, e5, Some(condition(move |_d, a| a.re != Some(e4))));
    // e5:SetTarget(s.sptg)
    effect::set_target(d, e5, Some(cost_target(sptg)));
    // e5:SetOperation(s.spop)
    effect::set_operation(d, e5, Some(operation(spop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.regop(e)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c==Duel.GetAttackTarget() and c:IsPosition(POS_FACEUP_ATTACK) then
    if duel::get_attack_target(d) == Some(c) && card::is_position(d, c, POS_FACEUP_ATTACK) {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE,0,1)
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
    } else {
        // c:ResetFlagEffect(id)
        card::reset_flag_effect(d, c, ID);
    }
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:HasFlagEffect(id) and c==Duel.GetAttackTarget()
    card::has_flag_effect(d, c, ID, None) && duel::get_attack_target(d) == Some(c)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,Duel.GetAttacker():GetAttack())
    let at = duel::get_attacker(d).nil();
    let atk = card::get_attack(d, at);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local ac=Duel.GetAttacker()
    let ac = duel::get_attacker(d).nil();
    // if ac:IsRelateToBattle() then
    if card::is_relate_to_battle(d, ac) {
        // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
        let p = chain::get_target_player(d, 0).int() as u8;
        // Duel.Damage(p,ac:GetAttack(),REASON_EFFECT)
        let dmg = card::get_attack(d, ac);
        duel::damage(d, p, dmg as i64, REASON_EFFECT, false, None);
    }
}
/// `function(e,tp) return Duel.IsTurnPlayer(tp) end`
fn turn_player_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not Duel.CheckReleaseGroup(tp,Card.IsReleasableByEffect,1,c) then
    if !duel::check_release_group(
        d,
        a.tp,
        Some(&|d, c| card::is_releasable_by_effect(d, c, None, None)),
        1,
        duel::ReleaseArgs::default(),
        c.into(),
        false,
    ) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, a.tp, 0);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_RELEASE,nil,1,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_RELEASE, None, 1, a.tp, LOCATION_MZONE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,c,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local b1=Duel.CheckReleaseGroup(tp,Card.IsReleasableByEffect,1,c)
    let b1 = duel::check_release_group(
        d,
        tp,
        Some(&|d, c| card::is_releasable_by_effect(d, c, None, None)),
        1,
        duel::ReleaseArgs::default(),
        c.into(),
        false,
    );
    // local b2=true
    let b2 = true;
    // --Tribute 1 other monster or destroy this card
    // local op=b1 and Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,3)},
    //     {b2,aux.Stringid(id,4)}) or 2
    let op = if b1 {
        duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]).unwrap_or(2)
    } else {
        2
    };
    if op == 1 {
        // local g=Duel.SelectReleaseGroup(tp,Card.IsReleasableByEffect,1,1,c)
        let g = duel::select_release_group(
            d,
            tp,
            Some(&|d, c| card::is_releasable_by_effect(d, c, None, None)),
            1,
            1,
            duel::ReleaseArgs::default(),
            c.into(),
            false,
        );
        // if #g==0 then return end
        let Some(g) = g else {
            return;
        };
        if g.is_empty() {
            return;
        }
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Release(g,REASON_EFFECT)
        duel::release(d, g, REASON_EFFECT, None);
    } else if op == 2 {
        // Duel.Destroy(c,REASON_EFFECT)
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[4779091]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, true, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| spfilter(d, c, e, tp)),
                tp,
                LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))),
        tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,true,true,POS_FACEUP)>0 then
    if let Some(sc) = group::get_first(d, &g) {
        if duel::special_summon(d, sc, 0, tp, tp, true, true, POS_FACEUP as u8, None) > 0 {
            // sc:CompleteProcedure()
            card::complete_procedure(d, sc);
        }
    }
}
