//! Dipsea Fiend
//! ディプシーデビル
use super::prelude::*;

pub const ID: u32 = 66472129;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand to either field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
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
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_discardable(d, x, None, None)), tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD,c)
    duel::discard_hand(d, tp, Some(&|d, x| card::is_discardable(d, x, None, None)), 1, 1, REASON_COST | REASON_DISCARD, c.into());
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return (Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false))
    //     or (Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)) end
    if a.chk == Some(0) {
        return (duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None))
            || (duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
                && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None));
    }
    // local att=Duel.AnnounceAttribute(tp,1,ATTRIBUTE_ALL&~ATTRIBUTE_WATER)
    let att = duel::announce_attribute(d, tp, 1, ATTRIBUTE_ALL & !ATTRIBUTE_WATER);
    // e:SetLabel(att)
    effect::set_label(d, a.e, &[att as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.spfilter(c,e,tp,attr)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, attr: u32) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(RACE_FIEND) and not c:IsOriginalAttribute(attr)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 4)
        && card::is_race(d, c, RACE_FIEND)
        && !card::is_original_attribute(d, c, attr)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local attr=e:GetLabel()
    let attr = effect::get_label(d, a.e);
    // if c:IsRelateToEffect(e) and attr>0 then
    if card::is_relate_to_effect(d, c, a.e) && attr > 0 {
        let attr = attr as u32;
        // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
        let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
        // local b2=Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
        let b2 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None);
        // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
        let sel_op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
        // local target_player=op==1 and tp or 1-tp
        let target_player = if sel_op == Some(1) { tp } else { 1 - tp };
        // if Duel.SpecialSummon(c,0,tp,target_player,false,false,POS_FACEUP)>0 then
        if duel::special_summon(d, c, 0, tp, target_player, false, false, POS_FACEUP as u8, None) > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // This card becomes the declared Attribute until the end of this turn
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
            effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
            // e1:SetValue(attr)
            effect::set_value(d, e1, attr as i64);
            // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,nil,e,tp,attr)
            let e = a.e;
            let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp, attr))), tp, LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
            // if c:IsAttribute(attr) and target_player==1-tp and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
            //     and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
            if card::is_attribute(d, c, attr)
                && target_player == 1 - tp
                && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && !g.is_empty()
                && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
            {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                // local sg=g:Select(tp,1,1,nil)
                let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
                // if #sg>0 then
                if !sg.is_empty() {
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                    duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                }
            }
        }
    }
    // Cannot Special Summon from the Extra Deck, except Fiend Monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e2, aux::stringid(ID, 4));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsRace(RACE_FIEND) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, x| card::is_location(d, x, LOCATION_EXTRA) && !card::is_race(d, x, RACE_FIEND))));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
