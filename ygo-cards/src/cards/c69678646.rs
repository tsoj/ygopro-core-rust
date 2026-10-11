//! Stellarnova Binding
use super::prelude::*;

pub const ID: u32 = 69678646;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TELLARKNIGHT, SET_CONSTELLAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard({SET_TELLARKNIGHT,SET_CONSTELLAR}) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,c:GetRace()),tp,LOCATION_MZONE,0,1,nil)
    if !(card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16, SET_CONSTELLAR as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
    {
        return false;
    }
    let race = card::get_race(d, c);
    !duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_race(d, x, race)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler(),REASON_EFFECT)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
    let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), tp, LOCATION_HAND, 0, 1, h.into())
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,{SET_TELLARKNIGHT,SET_CONSTELLAR}),tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16, SET_CONSTELLAR as u16])),
        tp,
        LOCATION_MZONE,
        0,
        NO_EX,
    );
    // local b2=ct>0 and Duel.IsExistingTarget(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,1,nil)
    let b2 = ct > 0 && duel::is_existing_target(d, Some(&|d, c| card::is_negatable_monster(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, e, &[op as i64]);
    if op == 1 {
        // e:SetCategory(CATEGORY_HANDES+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_HANDES + CATEGORY_SPECIAL_SUMMON);
        // e:SetProperty(0)
        effect::set_property(d, e, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_DISABLE)
        effect::set_category(d, e, CATEGORY_DISABLE);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
        // local g=Duel.SelectTarget(tp,Card.IsNegatableMonster,tp,0,LOCATION_MZONE,1,ct,nil)
        let g = duel::select_target(d, tp, Some(&|d, c| card::is_negatable_monster(d, c)), tp, 0, LOCATION_MZONE, 1, ct as u16, false, NO_EX).nil();
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,#g,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g.clone()), g.len() as u8, tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    if op == 1 {
        // Discard 1 card, and if you do, Special Summon 1 "tellarknight" or "Constellar" monster from your Deck with a different Type from the monsters you control
        // if Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD,nil)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
        if duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) > 0
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
            let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    } else if op == 2 {
        // Negate the effects of face-up monsters your opponent controls, up to the number of "tellarknight" and "Constellar" monsters you control
        // local tg=Duel.GetTargetCards(e):Match(Card.IsMonster,nil)
        let mut tg = duel::get_target_cards(d, e).nil();
        group::match_(d, &mut tg, &|d, x| card::is_monster(d, x), NO_EX);
        // if #tg>0 then
        if !tg.is_empty() {
            // for tc in tg:Iter() do
            for tc in group::iter(d, &tg) {
                // tc:NegateEffects(c,RESET_PHASE|PHASE_END)
                card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), false, None);
            }
        }
    }
    // You cannot Special Summon from the Extra Deck for the rest of this turn, except Xyz Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
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
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsXyzMonster() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_xyz_monster(d, c))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
