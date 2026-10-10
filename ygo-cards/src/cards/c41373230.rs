//! Titaniklad the Ash Dragon
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 41373230;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_ALBAZ],
    listed_series: &[SET_DOGMATIKA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: "Fallen of Albaz" + 1 monster with 2500 or more ATK
    // Fusion.AddProcMix(c,true,true,CARD_ALBAZ,aux.FilterBoolFunctionEx(Card.IsAttackAbove,2500))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(CARD_ALBAZ),
            fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| card::is_attack_above(d, c, 2500))),
        ],
    );
    // Gains ATK equal to the combined original Levels of the monsters used for its Fusion Summon x 100
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    // e1:SetValue(s.valcheck)
    effect::set_value_fn(d, e1, effect_card_value(valcheck));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // After this card is Fusion Summoned, for the rest of this turn, it is unaffected by the activated effects of any other monsters Special Summoned from the Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add to your hand, or Special Summon, 1 "Dogmatika" monster or 1 "Fallen of Albaz" from your Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(function(e) local c=e:GetHandler() return c:GetTurnID()==Duel.GetTurnCount() and not c:IsReason(REASON_RETURN) end)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetTarget(s.thsptg)
    effect::set_target(d, e3, Some(cost_target(thsptg)));
    // e3:SetOperation(s.thspop)
    effect::set_operation(d, e3, Some(operation(thspop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local atk=c:GetMaterial():GetSum(Card.GetOriginalLevel)*100
    let mat = card::get_material(d, c);
    let atk = group::get_sum(d, &mat, &|d, x| card::get_original_level(d, x) as i64) * 100;
    // if atk==0 then return end
    if atk == 0 {
        return 0;
    }
    // Gains ATK equal to the combined original Levels of the monsters used for its Fusion Summon x 100
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(atk)
    effect::set_value(d, e1, atk);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE&~RESET_TOFIELD)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    0
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsFusionSummoned() then return end
    if !card::is_fusion_summoned(d, c) {
        return;
    }
    // After this card is Fusion Summoned, for the rest of this turn, it is unaffected by the activated effects of any other monsters Special Summoned from the Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetValue(s.immval)
    effect::set_value_fn(d, e1, lua_fn(immval));
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.immval(e,te)`
fn immval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    // local tc=te:GetHandler()
    let tc = effect::get_handler(d, te);
    // local trig_loc,trig_sum_loc=Duel.GetChainInfo(0,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_SUMMON_LOCATION)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_SUMMON_LOCATION]);
    let trig_loc = match info.first() {
        Some(Value::Nil) | None => None,
        Some(v) => Some(v.int()),
    };
    let trig_sum_loc = match info.get(1) {
        Some(Value::Nil) | None => None,
        Some(v) => Some(v.int()),
    };
    // if not (te:IsActivated() and te:IsMonsterEffect() and trig_loc==LOCATION_MZONE and tc~=e:GetHandler()) then return false end
    if !(effect::is_activated(d, te) && effect::is_monster_effect(d, te) && trig_loc == Some(LOCATION_MZONE as i64) && tc != effect::get_handler(d, e)) {
        return false;
    }
    // if not Duel.IsChainSolving() or (tc:IsRelateToEffect(te) and tc:IsFaceup() and tc:IsLocation(trig_loc)) then
    let tc = tc.nil();
    if !duel::is_chain_solving(d) || (card::is_relate_to_effect(d, tc, te) && card::is_faceup(d, tc) && card::is_location(d, tc, LOCATION_MZONE)) {
        // return tc:IsSummonLocation(LOCATION_EXTRA)
        card::is_summon_location(d, tc, LOCATION_EXTRA)
    } else {
        // return trig_sum_loc&LOCATION_EXTRA>0
        (trig_sum_loc.nil() & LOCATION_EXTRA as i64) > 0
    }
}
/// `s.thspfilter(c,e,tp,mmz_chk)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // return (c:IsSetCard(SET_DOGMATIKA) or c:IsCode(CARD_ALBAZ)) and c:IsMonster() and (c:IsAbleToHand()
    //     or (mmz_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    (card::is_set_card(d, c, &[SET_DOGMATIKA as u16]) || card::is_code(d, c, &[CARD_ALBAZ]))
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None) || (mmz_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.thsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE)>0) end
    if a.chk == Some(0) {
        let mmz_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        return duel::is_existing_matching_card(d, Some(&|d, c| thspfilter(d, c, a.e, a.tp, mmz_chk)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thspop(e,tp,eg,ep,ev,re,r,rp)`
fn thspop(d: &mut Duel, a: &EffArgs) {
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,mmz_chk):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thspfilter(d, c, a.e, a.tp, mmz_chk)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &g) else {
        return;
    };
    // aux.ToHandOrElse(sc,tp,
    //     function() return mmz_chk and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
    //     function() Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end,
    //     aux.Stringid(id,3))
    let (e, tp) = (a.e, a.tp);
    aux::to_hand_or_else(
        d,
        sc,
        a.tp,
        Some(&|d, _c| mmz_chk && card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, None, None, None)),
        Some(&|d, _g| duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None)),
        Some(aux::stringid(ID, 3)),
    );
}
/// `function(e) local c=e:GetHandler() return c:GetTurnID()==Duel.GetTurnCount() and not c:IsReason(REASON_RETURN) end`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_turn_id(d, c) as i32 == duel::get_turn_count(d, None) && !card::is_reason(d, c, REASON_RETURN)
}
