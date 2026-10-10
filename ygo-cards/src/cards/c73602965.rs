//! Arias the Labrynth Butler
use super::prelude::*;

pub const ID: u32 = 73602965;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LABRYNTH], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Labrynth" monster, or Set 1 Normal Trap, from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND|LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(lua_fn(|d, _a: Args| duel::is_main_phase(d, None))));
    // e1:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    // e1:SetTarget(s.spsettarget)
    effect::set_target(d, e1, Some(cost_target(spsettarget)));
    // e1:SetOperation(s.spsetop)
    effect::set_operation(d, e1, Some(operation(spsetop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.selfspcon)
    effect::set_condition(d, e2, Some(condition(selfspcon)));
    // e2:SetTarget(s.selfsptg)
    effect::set_target(d, e2, Some(cost_target(selfsptg)));
    // e2:SetOperation(s.selfspop)
    effect::set_operation(d, e2, Some(operation(selfspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spsetfilter(c,e,tp,ft)
fn spsetfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return (ft>0 and c:IsSetCard(SET_LABRYNTH) and c:IsCanBeSpecialSummoned(e,0,tp,false,false))
    //     or (c:IsNormalTrap() and c:IsSSetable())
    (ft > 0 && card::is_set_card(d, c, &[SET_LABRYNTH as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
        || (card::is_normal_trap(d, c) && card::is_s_setable(d, c, false, None))
}
// function s.spsettarget(e,tp,eg,ep,ev,re,r,rp,chk)
fn spsettarget(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ft=Duel.GetMZoneCount(tp,c)
    let ft = duel::get_m_zone_count(d, a.tp, c.into(), None, None, None).0;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spsetfilter,tp,LOCATION_HAND,0,1,c,e,tp,ft) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| spsetfilter(d, x, a.e, a.tp, ft)), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
// function s.spsetop(e,tp,eg,ep,ev,re,r,rp)
fn spsetop(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,s.spsetfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp,ft):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spsetfilter(d, x, a.e, a.tp, ft)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if not sc then return end
    let Some(sc) = sc else { return };
    // if sc:IsMonster() then
    if card::is_monster(d, sc) {
        // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(sc), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    // elseif sc:IsTrap() and Duel.SSet(tp,sc)>0 then
    } else if card::is_trap(d, sc) && duel::s_set(d, a.tp, CardSet::from_card(sc), None, true) > 0 {
        // It can be activated this turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,3))
        effect::set_description(d, e1, aux::stringid(ID, 3));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
        effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
        // e1:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
        effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
    }
}
// function s.selfspcon(e,tp,eg,ep,ev,re,r,rp)
fn selfspcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ch=ev-1
    let ch = a.ev as i64 - 1;
    // if ch==0 or ep==tp then return false end
    if ch == 0 || a.ep == a.tp {
        return false;
    }
    // local ch_player,ch_eff=Duel.GetChainInfo(ch,CHAININFO_TRIGGERING_PLAYER,CHAININFO_TRIGGERING_EFFECT)
    let info = duel::get_chain_info(d, ch as u8, &[CHAININFO_TRIGGERING_PLAYER, CHAININFO_TRIGGERING_EFFECT]);
    let ch_player = info[0].int();
    let ch_eff = info[1].as_effect().nil();
    // local ch_c=ch_eff:GetHandler()
    let ch_c = effect::get_handler(d, ch_eff).nil();
    // return ch_player==tp and ((ch_c:IsSetCard(SET_LABRYNTH) and not ch_c:IsCode(id))
    //     or (ch_c:GetOriginalType()==TYPE_TRAP and ch_eff:IsTrapEffect()))
    ch_player == a.tp as i64
        && ((card::is_set_card(d, ch_c, &[SET_LABRYNTH as u16]) && !card::is_code(d, ch_c, &[ID]))
            || (card::get_original_type(d, ch_c) == TYPE_TRAP && effect::is_trap_effect(d, ch_eff)))
}
// function s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, LOCATION_GRAVE as i32);
    true
}
// function s.selfspop(e,tp,eg,ep,ev,re,r,rp)
fn selfspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
