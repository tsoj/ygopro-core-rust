//! Materiactor Gigadra
use super::prelude::*;

pub const ID: u32 = 33008376;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MATERIACTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Level 3 Normal Monster, or 1 "Materiactor" monster, from your hand or Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You cannot Special Summon monsters from the Extra Deck, except Xyz Monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_XYZ) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_XYZ))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcostfilter(c,e,tp)`
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsDiscardable() and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,c,e,tp,c:IsType(TYPE_NORMAL))
    card::is_discardable(d, c, None, None) && {
        let normal = card::is_type(d, c, TYPE_NORMAL);
        duel::is_existing_matching_card(
            d,
            Some(&|d: &mut Duel, cc: CardId| spfilter(d, cc, e, tp, normal)),
            tp,
            LOCATION_HAND | LOCATION_DECK,
            0,
            1,
            c.into(),
        )
    }
}
/// `s.spfilter(c,e,tp,cost_chk)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, cost_chk: bool) -> bool {
    // return ((c:IsType(TYPE_NORMAL) and c:IsLevel(3)) or (cost_chk and c:IsSetCard(SET_MATERIACTOR)))
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let normal3 = card::is_type(d, c, TYPE_NORMAL) && card::is_level(d, c, &[3]);
    (normal3 || (cost_chk && card::is_set_card(d, c, &[SET_MATERIACTOR as u16])))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d: &mut Duel, c: CardId| spcostfilter(d, c, e, tp)),
            tp,
            LOCATION_HAND,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp):GetFirst()
    let sg = duel::select_matching_card(
        d,
        tp,
        Some(&|d: &mut Duel, c: CardId| spcostfilter(d, c, e, tp)),
        tp,
        LOCATION_HAND,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = group::get_first(d, &sg).nil();
    // e:GetChainData().banished_normal=sc:IsType(TYPE_NORMAL)
    let normal = card::is_type(d, sc, TYPE_NORMAL);
    chain::set_effect_data(d, e, None, chain::field::CARD, Value::Bool(normal));
    // Duel.SendtoGrave(sc,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, CardSet::from_card(sc), REASON_COST | REASON_DISCARD, None, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp,e:GetChainData().banished_normal)
    let banished = chain::effect_data(d, e, None, chain::field::CARD).truthy();
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d: &mut Duel, c: CardId| spfilter(d, c, e, tp, banished)),
        tp,
        LOCATION_HAND | LOCATION_DECK,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
