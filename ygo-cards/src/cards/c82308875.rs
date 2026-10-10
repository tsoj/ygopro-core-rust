//! Number 7: Lucky Straight
use super::prelude::*;

pub const ID: u32 = 82308875;
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, xyz_number: Some(7), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 7 monsters
    // Xyz.AddProcedure(c,nil,7,3)
    xyz::add_procedure(d, c, None, Some(7), 3, None, None, None, None, false, None);
    // Roll a die twice
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON+CATEGORY_DRAW+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON + CATEGORY_DRAW + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 2);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,PLAYER_ALL,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,PLAYER_ALL,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_ALL as u8, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,tp,3)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 3);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_HANDES,nil,0,tp,2)
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 2);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(0xff))
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c)) {
        return;
    }
    // local d1,d2=Duel.TossDice(tp,2)
    let dice = duel::toss_dice(d, a.tp, 2, 0);
    let (mut d1, mut d2) = (dice[0], dice[1]);
    // if d2>d1 then d1,d2=d2,d1 end
    if d2 > d1 {
        std::mem::swap(&mut d1, &mut d2);
    }
    // This card's ATK becomes the larger number rolled x 700 until your opponent's next End Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetValue(d1*700)
    effect::set_value(d, e1, (d1 * 700) as i64);
    // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END,2)
    effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 2);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // if d1+d2~=7 then return end
    if d1 + d2 != 7 {
        return;
    }
    // If the total roll is exactly 7, apply 1 effect
    // local b1=Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    // local b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,LOCATION_GRAVE,1,nil,e,tp)
    let b2 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(
            d,
            Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, a.tp))),
            a.tp,
            LOCATION_HAND | LOCATION_GRAVE,
            LOCATION_GRAVE,
            1,
            NO_EX,
        );
    // local b3=Duel.IsPlayerCanDraw(tp,3)
    let b3 = duel::is_player_can_draw(d, a.tp, 3);
    // if not (b1 or b2 or b3) then return end
    if !(b1 || b2 || b3) {
        return;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)},
    //     {b3,aux.Stringid(id,3)})
    let sel = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]);
    // if op==1 then
    if sel == Some(1) {
        // Send all other cards on the field to the GY
        // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,c)
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    } else if sel == Some(2) {
        // Special Summon 1 monster from your hand or either GY
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil,e,tp)
        let sg = duel::select_matching_card(
            d,
            a.tp,
            Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, a.tp))),
            a.tp,
            LOCATION_HAND | LOCATION_GRAVE,
            LOCATION_GRAVE,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #sg==0 then return end
        if sg.is_empty() {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    } else if sel == Some(3) {
        // Draw 3 cards, then discard 2 cards
        // if not Duel.IsPlayerCanDraw(tp) then return end
        if !duel::is_player_can_draw(d, a.tp, 0) {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.Draw(tp,3,REASON_EFFECT)~=3 then return end
        if duel::draw(d, a.tp, 3, REASON_EFFECT) != 3 {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.DiscardHand(tp,nil,2,2,REASON_EFFECT|REASON_DISCARD)
        duel::discard_hand(d, a.tp, None, 2, 2, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
