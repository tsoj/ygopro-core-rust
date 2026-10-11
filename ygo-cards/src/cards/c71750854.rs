//! Bomber Place
use super::prelude::*;

pub const ID: u32 = 71750854;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate one of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(Cost.PayLP(600))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(600))));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.oppfilter(c,tp)
fn oppfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:HasLevel() and c:IsFaceup() and Duel.IsExistingMatchingCard(s.lvrklnkfilter,tp,0,LOCATION_MZONE,1,c,c:GetLevel())
    if !card::has_level(d, c) || !card::is_faceup(d, c) {
        return false;
    }
    let lv = card::get_level(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| lvrklnkfilter(d, x, lv)), tp, 0, LOCATION_MZONE, 1, c.into())
}
// function s.lvrklnkfilter(c,lv)
fn lvrklnkfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsFaceup() and (c:IsLevel(lv) or c:IsRank(lv) or c:IsLink(lv))
    card::is_faceup(d, c) && (card::is_level(d, c, &[lv]) || card::is_rank(d, c, &[lv]) || card::is_link(d, c, &[lv]))
}
// function s.selffilter(c,tp)
fn selffilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:HasLevel() and c:IsFaceup() and Duel.IsExistingMatchingCard(s.columnfilter,tp,0,LOCATION_MZONE,1,nil,c:GetLevel(),c:GetColumnGroup())
    if !card::has_level(d, c) || !card::is_faceup(d, c) {
        return false;
    }
    let lv = card::get_level(d, c);
    let colg = card::get_column_group(d, c, 0, 0);
    duel::is_existing_matching_card(d, Some(&|d, x| columnfilter(d, x, lv, &colg)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
// function s.columnfilter(c,lv,columng)
fn columnfilter(d: &mut Duel, c: CardId, lv: u32, columng: &CardSet) -> bool {
    // return columng:IsContains(c) and s.lvrklnkfilter(c,lv)
    columng.contains(c) && lvrklnkfilter(d, c, lv)
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsLevelBelow,6),tp,LOCATION_MZONE,0,nil):GetClassCount(Card.GetLevel)==6
    //     and Duel.IsExistingMatchingCard(s.oppfilter,tp,0,LOCATION_MZONE,1,nil,tp)
    let lvg = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x) && card::is_level_below(d, x, 6)), tp, LOCATION_MZONE, 0, NO_EX);
    let b1 = group::get_class_count(d, &lvg, &|d, x| card::get_level(d, x) as i64) == 6
        && duel::is_existing_matching_card(d, Some(&|d, x| oppfilter(d, x, tp)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.selffilter,tp,LOCATION_MZONE,0,1,nil,tp)
    let b2 = duel::is_existing_matching_card(d, Some(&|d, x| selffilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    // (SelectEffect always returns a choice here; a nil label behaves like any value other than 1 or 2)
    effect::set_label(d, a.e, &[op.map_or(0, |x| x as i64)]);
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let g = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 1 - tp, 0);
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Choose 1 face-up monster your opponent controls that has a Level, and destroy all other monsters they control with an equal Level/Rank/Link Rating
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_OPPO) --"Select your opponent's card"
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_OPPO as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.oppfilter,tp,0,LOCATION_MZONE,1,1,nil,tp):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, x| oppfilter(d, x, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc then
        if let Some(sc) = sc {
            // Duel.HintSelection(sc)
            duel::hint_selection(d, sc, true);
            // local g=Duel.GetMatchingGroup(s.lvrklnkfilter,tp,0,LOCATION_MZONE,sc,sc:GetLevel())
            let lv = card::get_level(d, sc);
            let g = duel::get_matching_group(d, Some(&|d, x| lvrklnkfilter(d, x, lv)), tp, 0, LOCATION_MZONE, sc.into());
            // if #g>0 then
            if !g.is_empty() {
                // Duel.Destroy(g,REASON_EFFECT)
                duel::destroy(d, g, REASON_EFFECT, None, None);
            }
        }
    // elseif op==2 then
    } else if op == 2 {
        // Choose 1 face-up monster you control that has a Level, and destroy all your opponent's monsters in its column with an equal Level/Rank/Link Rating
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELF) --"Select your card"
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELF as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.selffilter,tp,LOCATION_MZONE,0,1,1,nil,tp):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, x| selffilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc then
        if let Some(sc) = sc {
            // Duel.HintSelection(sc)
            duel::hint_selection(d, sc, true);
            // local g=Duel.GetMatchingGroup(s.columnfilter,tp,0,LOCATION_MZONE,nil,sc:GetLevel(),sc:GetColumnGroup())
            let lv = card::get_level(d, sc);
            let colg = card::get_column_group(d, sc, 0, 0);
            let g = duel::get_matching_group(d, Some(&|d, x| columnfilter(d, x, lv, &colg)), tp, 0, LOCATION_MZONE, NO_EX);
            // if #g>0 then
            if !g.is_empty() {
                // Duel.Destroy(g,REASON_EFFECT)
                duel::destroy(d, g, REASON_EFFECT, None, None);
            }
        }
    }
}
