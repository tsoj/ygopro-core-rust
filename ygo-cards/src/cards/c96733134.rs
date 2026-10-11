//! Supreme King Dragon Odd-Eyes
use super::prelude::*;

pub const ID: u32 = 96733134;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUPREME_KING_DRAGON, SET_SUPREME_KING_GATE], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Destroy this card, and if you do, add 1 Pendulum Monster with 1500 or less ATK from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCost(s.tributecost(1))
    effect::set_cost(d, e1, Some(tributecost(1)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCost(s.tributecost(2))
    effect::set_cost(d, e2, Some(tributecost(2)));
    // e2:SetTarget(s.selfsptg)
    effect::set_target(d, e2, Some(cost_target(selfsptg)));
    // e2:SetOperation(s.selfspop)
    effect::set_operation(d, e2, Some(operation(selfspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If your Pendulum Monster battles an opponent's monster, any battle damage it inflicts to your opponent is doubled
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(function(e,c) local bc=c:GetBattleTarget() return c:IsPendulumMonster() and bc and bc:IsControler(1-e:GetHandlerPlayer()) end)
    effect::set_target(d, e3, Some(effect_card_fn(e3_target)));
    // e3:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e3, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon up to 2 "Supreme King Dragon" and/or "Supreme King Gate" Pendulum Monsters from your face-up Extra Deck in Defense Position, except "Supreme King Dragon Odd-Eyes"
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetHintTiming(0,TIMING_BATTLE_START|TIMING_BATTLE_END)
    effect::set_hint_timing(d, e4, 0, Some(TIMING_BATTLE_START | TIMING_BATTLE_END));
    // e4:SetCondition(function() return Duel.IsBattlePhase() end)
    effect::set_condition(d, e4, Some(condition(e4_cond)));
    // e4:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e4, Some(cost::self_tribute()));
    // e4:SetTarget(s.exsptg)
    effect::set_target(d, e4, Some(cost_target(exsptg)));
    // e4:SetOperation(s.exspop)
    effect::set_operation(d, e4, Some(operation(exspop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// s.tributecost(count): the cost function returned by the script
fn tributecost(count: u32) -> LuaFn {
    cost_target(move |d, a| tributecost_fn(d, a, count))
}
// function(e,tp,eg,ep,ev,re,r,rp,chk) inside s.tributecost(count)
fn tributecost_fn(d: &mut Duel, a: &EffArgs, count: u32) -> bool {
    let tp = a.tp;
    // local spcheck=count==2 and aux.ReleaseCheckMMZ or nil
    let spcheck: Option<&dyn Fn(&mut Duel, &CardSet, u8, &CardSet) -> (bool, bool)> = if count == 2 { Some(&aux::release_check_mmz) } else { None };
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsSetCard,count,false,spcheck,nil,SET_SUPREME_KING_DRAGON) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(
            d,
            tp,
            Some(&|d, c| card::is_set_card(d, c, &[SET_SUPREME_KING_DRAGON as u16])),
            count as usize,
            count as usize,
            false,
            spcheck,
            NO_EX,
        );
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsSetCard,count,count,false,spcheck,nil,SET_SUPREME_KING_DRAGON)
    let g = duel::select_release_group_cost(
        d,
        tp,
        Some(&|d, c| card::is_set_card(d, c, &[SET_SUPREME_KING_DRAGON as u16])),
        count as usize,
        count as usize,
        false,
        spcheck,
        NO_EX,
    );
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPendulumMonster() and c:IsAttackBelow(1500) and c:IsAbleToHand()
    card::is_pendulum_monster(d, c) && card::is_attack_below(d, c, 1500) && card::is_able_to_hand(d, c, None)
}
// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    let h = effect::get_handler(d, a.e).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.Destroy(c,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::destroy(d, c, REASON_EFFECT, None, None) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
// s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
// s.selfspop(e,tp,eg,ep,ev,re,r,rp)
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// s.exspfilter(c,e,tp)
fn exspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard({SET_SUPREME_KING_DRAGON,SET_SUPREME_KING_GATE}) and c:IsPendulumMonster() and c:IsFaceup() and not c:IsCode(id)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_set_card(d, c, &[SET_SUPREME_KING_DRAGON as u16, SET_SUPREME_KING_GATE as u16])
        && card::is_pendulum_monster(d, c)
        && card::is_faceup(d, c)
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
// function() return Duel.IsBattlePhase() end (e4)
fn e4_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None)
}
// s.exsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn exsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCountFromEx(tp,tp,e:GetHandler())>0
    //     and Duel.IsExistingMatchingCard(s.exspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::get_location_count_from_ex(d, tp, Some(tp), h.into(), duel::ExtraCard::None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| exspfilter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// s.exspop(e,tp,eg,ep,ev,re,r,rp)
fn exspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ft=math.min(Duel.GetLocationCountFromEx(tp),2)
    let mut ft = duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::None, None).0.min(2);
    // if ft==0 then return end
    if ft == 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // ft=math.min(ft,aux.CheckSummonGate(tp) or ft)
    let gate = aux::check_summon_gate_limit(d, tp).unwrap_or(ft as i64);
    ft = std::cmp::min(ft as i64, gate) as i32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.exspfilter,tp,LOCATION_EXTRA,0,1,ft,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| exspfilter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, ft as u16, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
// function(e,tp,eg,ep,ev,re,r,rp,chk,chkc) / e3 target: function(e,c)
fn e3_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return c:IsPendulumMonster() and bc and bc:IsControler(1-e:GetHandlerPlayer())
    if !card::is_pendulum_monster(d, c) {
        return false;
    }
    match bc {
        Some(bc) => {
            let p = 1 - effect::get_handler_player(d, e);
            card::is_controler(d, bc, p)
        }
        None => false,
    }
}
