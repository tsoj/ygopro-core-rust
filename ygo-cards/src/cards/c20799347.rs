//! Virtual World Oto-Hime - Toutou
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 20799347;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you control no monsters, or all monsters you control are Psychic and/or Wyrm monsters, you can Normal Summon this card without Tributing
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetCondition(s.ntcon)
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card as a Tuner, but banish it when it leaves the field, also for the rest of this turn, you can only Special Summon Level/Rank 3 or higher monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.Discard(function(c) return c:IsRace(RACE_PSYCHIC|RACE_WYRM) end))
    effect::set_cost(
        d,
        e2,
        Some(cost::discard(
            Some(std::sync::Arc::new(|d, c, _e, _tp| card::is_race(d, c, RACE_PSYCHIC | RACE_WYRM))),
            false,
            None,
            None,
            None,
        )),
    );
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ntconfilter(c)`: `return c:IsFacedown() or not c:IsRace(RACE_PSYCHIC|RACE_WYRM)`
fn ntconfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) || !card::is_race(d, c, RACE_PSYCHIC | RACE_WYRM)
}
/// `s.ntcon(e,c,minc,zone)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    let minc = a.int(2);
    let zone = a.int(3) as u32;
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return minc==0 and c:IsLevelAbove(5) and Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)>0
    //   and not Duel.IsExistingMatchingCard(s.ntconfilter,tp,LOCATION_MZONE,0,1,nil)
    minc == 0
        && card::is_level_above(d, c, 5)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0 > 0
        && !duel::is_existing_matching_card(d, Some(&ntconfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // c:AssumeProperty(ASSUME_TYPE,c:GetType()|TYPE_TUNER)
    let t = card::get_type(d, c) | TYPE_TUNER;
    card::assume_property(d, c, ASSUME_TYPE, t as u64);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        // c:AssumeProperty(ASSUME_TYPE,c:GetType()|TYPE_TUNER)
        let t = card::get_type(d, c) | TYPE_TUNER;
        card::assume_property(d, c, ASSUME_TYPE, t as u64);
        // if Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // --Treated as a tuner
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_ADD_TYPE)
            effect::set_code(d, e1, EFFECT_ADD_TYPE);
            // e1:SetValue(TYPE_TUNER)
            effect::set_value(d, e1, TYPE_TUNER as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, (RESET_EVENT | RESETS_STANDARD) as u32, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // --Banish it when it leaves the field
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetDescription(3300)
            effect::set_description(d, e2, 3300);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT, 0);
            // e2:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
            effect::set_code(d, e2, EFFECT_LEAVE_FIELD_REDIRECT);
            // e2:SetValue(LOCATION_REMOVED)
            effect::set_value(d, e2, LOCATION_REMOVED as i64);
            // e2:SetReset(RESET_EVENT|RESETS_REDIRECT)
            effect::set_reset(d, e2, (RESET_EVENT | RESETS_REDIRECT) as u32, 1);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
    // For the rest of this turn, you can only Special Summon Level/Rank 3 or higher monsters
    // local e3=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e3 = effect::create_effect(d, h);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
    // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetTarget(function(e,c) return not (c:IsLevelAbove(3) or c:IsRankAbove(3)) end)
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| !(card::is_level_above(d, c, 3) || card::is_rank_above(d, c, 3)))));
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, (RESET_PHASE | PHASE_END) as u32, 1);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
}
