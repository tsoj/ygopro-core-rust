//! Meklord Astro the Eradicator
use super::prelude::*;

pub const ID: u32 = 2992036;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MEKLORD, SET_MEKLORD_ASTRO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add to hand or Special Summon 3 "Meklord" monsters with different names in your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE as u32));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 Synchro Monster the opponent controls
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE as u32));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.descond)
    effect::set_condition(d, e2, Some(condition(descond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tgfilter(c,e,tp,ft)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_MEKLORD) and c:IsCanBeEffectTarget(e)
    //     and (c:IsAbleToHand() or (ft>0 and c:IsCanBeSpecialSummoned(e,0,tp,true,false)))
    card::is_monster(d, c)
        && card::is_set_card(d, c, &[SET_MEKLORD as u16])
        && card::is_can_be_effect_target(d, c, Some(e))
        && (card::is_able_to_hand(d, c, None) || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tgfilter(chkc,e,tp,ft) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc, e, tp, ft);
    }
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_GRAVE,0,nil,e,tp,ft)
    let g = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, e, tp, ft)), tp, LOCATION_GRAVE, 0, NO_EX);
    let dn = aux::dncheck();
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,3,3,aux.dncheck,0) end
    if a.chk == Some(0) {
        let res = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&dn), 0, tp, None, None, None, false);
        return matches!(res, Err(true));
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tg=aux.SelectUnselectGroup(g,e,tp,3,3,aux.dncheck,1,tp,HINTMSG_TARGET)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&dn), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tg,#tg,tp,0)
    let n = tg.len();
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg.clone()), n as u8, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,tg,#tg,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, Some(tg), n as u8, tp, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, e).nil();
    // local ct=#tg
    let ct = tg.len();
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // if ct>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return false end
    if ct > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // aux.ToHandOrElse(tg,tp,
    //     function(tc)
    //         return Duel.GetLocationCount(tp,LOCATION_MZONE)>=ct and tc:IsCanBeSpecialSummoned(e,0,tp,true,false)
    //     end,
    //     function(tc)
    //         Duel.SpecialSummon(tc,0,tp,tp,true,false,POS_FACEUP)
    //     end,
    //     aux.Stringid(id,2)
    // )
    aux::to_hand_or_else(
        d,
        tg,
        tp,
        Some(&|d, tc| {
            duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= ct as i32
                && card::is_can_be_special_summoned(d, tc, e, 0, tp, true, false, None, None, None)
        }),
        Some(&|d, tc| {
            duel::special_summon(d, tc.clone(), 0, tp, tp, true, false, POS_FACEUP as u8, None);
            0
        }),
        Some(aux::stringid(ID, 2)),
    );
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local reset_ct=Duel.IsTurnPlayer(tp) and 2 or 1
    let reset_ct = if duel::is_turn_player(d, tp) { 2 } else { 1 };
    // Cannot Special Summon, except Machine monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(_,c) return not c:IsRace(RACE_MACHINE) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_MACHINE as u64))));
    // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,reset_ct)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, reset_ct);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.descond(e,tp,eg,ep,ev,re,r,rp)`
fn descond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_MEKLORD_ASTRO),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_MEKLORD_ASTRO as u16]))), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsType,TYPE_SYNCHRO),tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_SYNCHRO))), tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsType,TYPE_SYNCHRO),tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_SYNCHRO))), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if not tc then return end
    let Some(tc) = tc else {
        return;
    };
    // Duel.HintSelection(tc,true)
    duel::hint_selection(d, tc, true);
    // if Duel.Destroy(tc,REASON_EFFECT)>0 then
    if duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 {
        // local value=tc:GetBaseAttack()
        let value = card::get_base_attack(d, tc);
        // if value>0 then
        if value > 0 {
            // Duel.Damage(1-tp,value,REASON_EFFECT)
            duel::damage(d, 1 - tp, value as i64, REASON_EFFECT, false, None);
        }
    }
}
