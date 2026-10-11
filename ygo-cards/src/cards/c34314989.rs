//! Dark-Eyes Illusionist Faceless Mage
use super::prelude::*;

pub const ID: u32 = 34314989;
const CARD_MIND_SCAN: u32 = 34298391;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_MIND_SCAN, CARD_TOON_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card battles a monster, neither can be destroyed by that battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(s.indestg)
    effect::set_target(d, e1, Some(effect_card_fn(indestg)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can discard this card, then activate 1 of these effects (but you can only use each of these effects of "Dark-Eyes Illusionist Faceless Mage" once per turn);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e2, Some(cost::self_discard()));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.indestg(e,c)
fn indestg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local handler=e:GetHandler()
    let handler = effect::get_handler(d, e).nil();
    // return c==handler or c==handler:GetBattleTarget()
    c == handler || Some(c) == card::get_battle_target(d, handler)
}
// function s.plfilter(c,tp)
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(CARD_MIND_SCAN) and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_code(d, c, &[CARD_MIND_SCAN]) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:ListsCode(CARD_TOON_WORLD) and c:IsAbleToHand()
    card::is_monster(d, c) && card::lists_code(d, c, &[CARD_TOON_WORLD]) && card::is_able_to_hand(d, c, None)
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local b1=Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,tp)
    //     and not Duel.HasFlagEffect(tp,id)
    let b1 = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| plfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
        && !duel::has_flag_effect(d, tp, ID, None);
    // local b2=Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,e:GetHandler())
    //     and not Duel.HasFlagEffect(tp,id+1)
    let h = effect::get_handler(d, e).nil();
    let b2 = duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, h.into()) && !duel::has_flag_effect(d, tp, ID + 1, None);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, e, &[op.unwrap_or(0) as i64]);
    if op == Some(1) {
        // e:SetCategory(0)
        effect::set_category(d, e, 0);
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    } else if op == Some(2) {
        // e:GetHandler():CreateEffectRelation(e)
        card::create_effect_relation(d, h, e);
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    if op == 1 {
        // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,tp):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, c| plfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if sc then
        if let Some(sc) = group::get_first(d, &sel) {
            // Duel.MoveToField(sc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
            duel::move_to_field(d, sc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
        }
    } else if op == 2 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // local exc=c:IsRelateToEffect(e) and c or nil
        let exc: Exclude = if card::is_relate_to_effect(d, c, e) { c.into() } else { NO_EX };
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,exc):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, exc).nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &sel) else {
            return;
        };
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
        // if Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and sc:IsCanBeSpecialSummoned(e,0,tp,true,false) then
        if duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0
            && card::is_location(d, sc, LOCATION_HAND)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, sc, e, 0, tp, true, false, None, None, None)
        {
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // if Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
            if duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SpecialSummon(sc,0,tp,tp,true,false,POS_FACEUP)
                duel::special_summon(d, sc, 0, tp, tp, true, false, POS_FACEUP as u8, None);
            }
        }
    }
}
