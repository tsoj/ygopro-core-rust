//! Convex Knight
use super::prelude::*;

pub const ID: u32 = 43471513;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If a Machine monster is on the field and this card is in your hand: You can Special Summon this card in Defense Position, then you can make its Level become equal to the Level/Rank of 1 Machine monster on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON | CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_MACHINE),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE)), a.tp, LOCATION_MZONE as u32, LOCATION_MZONE as u32, 1, NO_EX)
    })));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // During your Main Phase: You can send 1 EARTH Machine monster from your Deck to the GY, then this card gains ATK equal to the sent monster's Level x 100 until the end of this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE | CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_LVCHANGE,c,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_LVCHANGE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn lvfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return ((c:HasLevel() and not c:IsLevel(lv)) or (c:HasRank() and not c:IsRank(lv)))
    //     and c:IsRace(RACE_MACHINE) and c:IsFaceup()
    ((card::has_level(d, c) && !card::is_level(d, c, &[lv])) || (card::has_rank(d, c) && !card::is_rank(d, c, &[lv])))
        && card::is_race(d, c, RACE_MACHINE)
        && card::is_faceup(d, c)
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP_DEFENSE)>0
    //     and Duel.IsExistingMatchingCard(s.lvfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,lv)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) > 0
        && duel::is_existing_matching_card(d, Some(&|d, sc| lvfilter(d, sc, lv)), a.tp, LOCATION_MZONE as u32, LOCATION_MZONE as u32, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.lvfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,lv):GetFirst()
        let g = duel::select_matching_card(d, a.tp, Some(&|d, sc| lvfilter(d, sc, lv)), a.tp, LOCATION_MZONE as u32, LOCATION_MZONE as u32, 1, 1, false, NO_EX).nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &g) else {
            return;
        };
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
        // local lv_rnk=(sc:HasLevel() and sc:GetLevel()) or (sc:HasRank() and sc:GetRank())
        let lv_rnk = if card::has_level(d, sc) {
            card::get_level(d, sc)
        } else if card::has_rank(d, sc) {
            card::get_rank(d, sc)
        } else {
            0
        };
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(lv_rnk)
        effect::set_value(d, e1, lv_rnk as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_MACHINE) and c:IsAbleToGrave()
    card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_race(d, c, RACE_MACHINE) && card::is_able_to_grave(d, c, None)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK as u32, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,c,1,tp,0)
    // (the Lua `c` here is not a local of this function: it is the global nil)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, None, 1, a.tp, 0);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK as u32, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SendtoGrave(sc,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_GRAVE)
    //     and c:IsRelateToEffect(e) and c:IsFaceup() then
    if let Some(sc) = sc {
        if duel::send_to_grave(d, sc, REASON_EFFECT, None, None) > 0
            && card::is_location(d, sc, LOCATION_GRAVE)
            && card::is_relate_to_effect(d, c, a.e)
            && card::is_faceup(d, c)
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // c:UpdateAttack(sc:GetLevel()*100,RESETS_STANDARD_DISABLE_PHASE_END)
            let lv = card::get_level(d, sc) as i64;
            card::update_attack(d, c, lv * 100, Some(RESETS_STANDARD_DISABLE_PHASE_END), None, None);
        }
    }
}
