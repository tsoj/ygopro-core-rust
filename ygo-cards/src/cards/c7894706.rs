//! The Chaotic Phantasmal Sacred Beasts
//! 混沌の三幻魔
use super::prelude::*;

pub const ID: u32 = 7894706;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 3 Level 10 monsters that cannot be Normal Summoned/Set
    // Fusion.AddProcFunRep(c,s.matfilter,3,true)
    fusion::add_proc_fun_rep(d, c, fusion::fus_mat(|d, cc, _fc, _st, _tp| matfilter(d, cc)), 3, true);
    // Must be either Fusion Summoned, or Special Summoned (from your Extra Deck) by sending the above cards you control to the GY
    // c:AddMustBeFusionSummoned()
    card::add_must_be_fusion_summoned(d, c);
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,false,nil,1)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(|d, tp| contactfil(d, tp))),
            material_operation: Some(std::sync::Arc::new(|d, g, _tp, _c| contactop(d, g))),
            sumcon: None,
            condition: None,
            sumtype: Some(1),
            ..Default::default()
        },
    );
    // You can only Special Summon "Phantasmal Sacred Beasts of Chaos" once per turn this way, no matter which method you use
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE | EFFECT_TYPE_CONTINUOUS);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e0, EVENT_SPSUMMON_SUCCESS);
    // e0:SetCondition(s.regcon)
    effect::set_condition(d, e0, Some(condition(regcon)));
    // e0:SetOperation(s.regop)
    effect::set_operation(d, e0, Some(operation(regop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // The first two times this card would be destroyed by card effect each turn, it is not destroyed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(2)
    effect::set_count_limit(d, e1, 2, 0, 0, 0);
    // e1:SetValue(function(e,re,r) return (r&REASON_EFFECT)>0 end)
    effect::set_value_fn(d, e1, lua_fn(|_d, a| (a.int(2) as u32 & REASON_EFFECT) > 0));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per Chain and thrice per turn (Quick Effect): You can target 1 face-up monster your opponent controls; negate its effects (until the end of this turn), then you can gain LP equal to half its ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE+CATEGORY_RECOVER)
    effect::set_category(d, e2, CATEGORY_DISABLE | CATEGORY_RECOVER);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET | EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(3)
    effect::set_count_limit(d, e2, 3, 0, 0, 0);
    // e2:SetCost(Cost.SoftOncePerChain(id))
    effect::set_cost(d, e2, Some(cost::soft_once_per_chain(ID, None)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c)`
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(10) and not c:IsSummonableCard()
    card::is_level(d, c, &[10]) && !card::is_summonable_card(d, c)
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(Card.IsAbleToGraveAsCost,tp,LOCATION_MZONE,0,nil)
    duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_MZONE, 0, NO_EX)
}
/// `s.contactop(g)`
fn contactop(d: &mut Duel, g: &CardSet) {
    // Duel.SendtoGrave(g,REASON_COST|REASON_MATERIAL)
    duel::send_to_grave(d, g.clone(), REASON_COST | REASON_MATERIAL, None, None);
}
/// `s.regcon(e)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFusionSummoned() or c:IsSummonType(SUMMON_TYPE_SPECIAL+1)
    card::is_fusion_summoned(d, c) || card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL + 1])
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // You can only Special Summon "Phantasmal Sacred Beasts of Chaos" once per turn this way, no matter which method you use
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
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
        Some(lua_fn(|d, a| {
            let c = a.c(1);
            let sumtype = a.int(3);
            card::is_original_code(d, c, &[ID])
                && ((sumtype & SUMMON_TYPE_FUSION as i64) == SUMMON_TYPE_FUSION as i64
                    || (sumtype & (SUMMON_TYPE_SPECIAL as i64 + 1)) == SUMMON_TYPE_SPECIAL as i64 + 1)
        })),
    );
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsNegatableMonster() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_negatable_monster(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_negatable_monster(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    // local tc=Duel.SelectTarget(tp,Card.IsNegatableMonster,tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
    let sel = duel::select_target(d, tp, Some(&|d, c| card::is_negatable_monster(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(tc)), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,math.floor(tc:GetAttack()/2))
    let atk = card::get_attack(d, tc);
    duel::set_possible_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, atk.div_euclid(2) as i32);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsCanBeDisabledByEffect(e) then
    if card::is_relate_to_effect(d, tc.nil(), e) && card::is_faceup(d, tc.nil()) && card::is_can_be_disabled_by_effect(d, tc.nil(), e, true) {
        // --Negate its effects (until the end of this turn)
        // tc:NegateEffects(e:GetHandler(),RESETS_STANDARD_PHASE_END)
        let h = effect::get_handler(d, e).nil();
        card::negate_effects(d, tc.nil(), h, Some(RESETS_STANDARD_PHASE_END), false, None);
        // Duel.AdjustInstantly(tc)
        duel::adjust_instantly(d, tc);
        // local lp=math.floor(tc:GetAttack()/2)
        let lp = card::get_attack(d, tc.nil()).div_euclid(2);
        // if lp>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if lp > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
            let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Recover(p,lp,REASON_EFFECT)
            duel::recover(d, p, lp as i64, REASON_EFFECT, false, None);
        }
    }
}
