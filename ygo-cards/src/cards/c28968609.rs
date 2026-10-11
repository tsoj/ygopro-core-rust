//! Garbage Golem
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 28968609;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you control a monster with 0 ATK or DEF: You can Special Summon this card from your hand
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
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(aux.OR(Card.IsAttack,Card.IsDefense),0),tp,LOCATION_MZONE,0,1,nil) end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e1, Some(cost_target(e1_target)));
    // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e1, Some(operation(e1_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can reveal 3 Xyz Monsters with different Ranks in your Extra Deck, then target 1 other card on the field;
    // destroy both it and this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_cost(d, e2, Some(cost_target(e2_cost)));
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk,chkc) ... end)
    effect::set_target(d, e2, Some(cost_target(e2_target)));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e2, Some(operation(e2_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // You can banish this card from your GY; detach 1 material from a monster you control, then you can Special Summon
    // 1 monster with 0 ATK or DEF from your hand or GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e3, Some(cost_target(e3_target)));
    // e3:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e3, Some(operation(e3_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `aux.FaceupFilter(aux.OR(Card.IsAttack,Card.IsDefense),0)`
fn faceup_zero_stat(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && (card::is_attack(d, c, &[0]) || card::is_defense(d, c, &[0]))
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsAttack(0) or c:IsDefense(0)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    (card::is_attack(d, c, &[0]) || card::is_defense(d, c, &[0]))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// The condition of e1: `function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsExistingMatchingCard(...) end`
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&faceup_zero_stat), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// The target of e1: `function(e,tp,eg,ep,ev,re,r,rp,chk)`
fn e1_target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// The operation of e1: `function(e,tp,eg,ep,ev,re,r,rp)`
fn e1_op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// The cost of e2: `function(e,tp,eg,ep,ev,re,r,rp,chk)`
fn e2_cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.AND(Card.IsXyzMonster,aux.NOT(Card.IsPublic)),tp,LOCATION_EXTRA,0,nil)
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_xyz_monster(d, c) && !card::is_public(d, c)),
        tp,
        LOCATION_EXTRA,
        0,
        NO_EX,
    );
    // if chk==0 then return #g>=3 and aux.SelectUnselectGroup(g,e,tp,3,3,aux.dpcheck(Card.GetRank),0) end
    if a.chk == Some(0) {
        return g.len() >= 3
            && matches!(
                aux::select_unselect_group(
                    d,
                    &g,
                    Some(a.e),
                    tp,
                    Some(3),
                    Some(3),
                    Some(&aux::dpcheck(|d, c| card::get_rank(d, c) as i64)),
                    0,
                    tp,
                    None,
                    None,
                    None,
                    false
                ),
                Err(true)
            );
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,aux.dncheck,1,tp,HINTMSG_CONFIRM)
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        tp,
        Some(3),
        Some(3),
        Some(&aux::dncheck()),
        1,
        tp,
        Some(HINTMSG_CONFIRM as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.ConfirmCards(1-tp,sg)
    duel::confirm_cards(d, 1 - tp, sg);
    // Duel.ShuffleExtra(tp)
    duel::shuffle_extra(d, tp);
    true
}
/// The target of e2: `function(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn e2_target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsOnField() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,c)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g+c,2,tp,0)
    let mut targets = g;
    group::add_card(&mut targets, &CardSet::from_card(c));
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(targets), 2, tp, 0);
    true
}
/// The operation of e2: `function(e,tp,eg,ep,ev,re,r,rp)`
fn e2_op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        // Duel.Destroy(Group.FromCards(c,tc),REASON_EFFECT)
        let mut g = CardSet::from_card(c);
        group::add_card(&mut g, &CardSet::from_card(tc.nil()));
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// The target of e3: `function(e,tp,eg,ep,ev,re,r,rp,chk)`
fn e3_target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckRemoveOverlayCard(tp,1,0,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return duel::check_remove_overlay_card(d, a.tp, 1, 0, 1, REASON_EFFECT, None);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// The operation of e3: `function(e,tp,eg,ep,ev,re,r,rp)`
fn e3_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.RemoveOverlayCard(tp,1,0,1,1,REASON_EFFECT)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if duel::remove_overlay_card(d, tp, 1, 0, 1, 1, REASON_EFFECT, None) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(
            d,
            Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp))),
            tp,
            LOCATION_HAND | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        )
        && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp))),
            tp,
            LOCATION_HAND | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
