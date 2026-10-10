//! Inferno Reckless Summon
use super::prelude::*;

pub const ID: u32 = 12247206;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    // return #eg==1 and tc:IsLocation(LOCATION_MZONE) and tc:IsControler(tp)
    //     and tc:IsFaceup() and tc:IsAttackBelow(1500)
    //     and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil)
    if eg.len() != 1 {
        return false;
    }
    let tc = group::get_first(d, &eg).nil();
    card::is_location(d, tc, LOCATION_MZONE)
        && card::is_controler(d, tc, a.tp)
        && card::is_faceup(d, tc)
        && card::is_attack_below(d, tc, 1500)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.nfilter(c,tc)`
fn nfilter(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    // return c:IsCode(tc:GetCode())
    let codes = card::get_code(d, tc);
    card::is_code(d, c, &codes)
}
/// `s.filter(c,tc,e,tp)`
fn filter(d: &mut Duel, c: CardId, tc: CardId, e: EffectId, tp: u8) -> bool {
    // return s.nfilter(c,tc) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    nfilter(d, c, tc) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.selfilter(c,e,tp)`
fn selfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,c,e,tp)
    card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &a.eg.clone().nil());
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.nfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,tc)
        let tcv = tc.nil();
        let g = duel::get_matching_group(d, Some(&|d, x| nfilter(d, x, tcv)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and #g>0 and #g==g:FilterCount(Card.IsCanBeSpecialSummoned,nil,e,0,tp,false,false)
        let (cnt, _) = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None);
        if cnt <= 0 || g.is_empty() {
            return false;
        }
        let fc = group::filter_count(
            d,
            &g,
            &|d, x| card::is_can_be_special_summoned(d, x, a.e, 0, a.tp, false, false, None, None, None),
            NO_EX,
        );
        return g.len() == fc;
    }
    // tc:CreateEffectRelation(e)
    card::create_effect_relation(d, tc.nil(), a.e);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,PLAYER_NONE,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(
        d,
        0,
        CATEGORY_SPECIAL_SUMMON,
        None,
        1,
        PLAYER_NONE as u8,
        (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32,
    );
    true
}
/// `s.sp(g,tp,pos)`: `for sc in aux.Next(g) do Duel.SpecialSummonStep(sc,0,tp,tp,false,false,pos) end`
fn sp(d: &mut Duel, g: &CardSet, tp: u8, pos: u8) {
    for sc in group::iter(d, g) {
        duel::special_summon_step(d, sc, 0, tp, tp, false, false, pos, None);
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &a.eg.clone().nil()).nil();
    // local ft1=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let (mut ft1, _) = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None);
    // if ft1>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft1=1 end
    if ft1 > 1 && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft1 = 1;
    }
    // local gg=Group.CreateGroup()
    let mut gg = CardSet::new();
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.filter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,tc,e,tp)
    let nv = aux::necro_valley_filter(move |d, x| filter(d, x, tc, a.e, a.tp));
    let mut g = duel::get_matching_group(d, Some(&nv), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    // if ft1>0 and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if ft1 > 0 && card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // if #g<=ft1 then
        if g.len() as i32 <= ft1 {
            // s.sp(g,tp,POS_FACEUP_ATTACK)
            sp(d, &g, a.tp, POS_FACEUP_ATTACK as u8);
        } else {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local fg=g:Select(tp,ft1,ft1,nil)
            let fg = group::select(d, &g, a.tp, ft1 as u16, ft1 as u16, false, NO_EX).nil();
            // s.sp(fg,tp,POS_FACEUP_ATTACK)
            sp(d, &fg, a.tp, POS_FACEUP_ATTACK as u8);
            // g:Sub(fg)
            group::remove_card(&mut g, &fg);
            // gg:Merge(g)
            group::add_card(&mut gg, &g);
        }
    }
    // local ft2=Duel.GetLocationCount(1-tp,LOCATION_MZONE)
    let opp = 1 - a.tp;
    let (mut ft2, _) = duel::get_location_count(d, opp, LOCATION_MZONE as u8, None, None, None);
    // if ft2>1 and Duel.IsPlayerAffectedByEffect(1-tp,CARD_BLUEEYES_SPIRIT) then ft2=1 end
    if ft2 > 1 && duel::is_player_affected_by_effect(d, opp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft2 = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, opp, HINTMSG_FACEUP as u64);
    // local sg=Duel.SelectMatchingCard(1-tp,s.selfilter,1-tp,LOCATION_MZONE,0,1,1,nil,e,1-tp)
    let sg = duel::select_matching_card(d, opp, Some(&|d, x| selfilter(d, x, a.e, opp)), opp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if ft2>0 and #sg>0 then
    if ft2 > 0 && !sg.is_empty() {
        // local g2=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.filter),1-tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,sg:GetFirst(),e,1-tp)
        let sg_first = group::get_first(d, &sg).nil();
        let nv2 = aux::necro_valley_filter(move |d, x| filter(d, x, sg_first, a.e, opp));
        let mut g2 = duel::get_matching_group(d, Some(&nv2), opp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
        // if #g2>0 then
        if !g2.is_empty() {
            // if #g2<=ft2 then
            if g2.len() as i32 <= ft2 {
                // s.sp(g2,1-tp,POS_FACEUP)
                sp(d, &g2, opp, POS_FACEUP as u8);
            } else {
                // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, opp, HINTMSG_SPSUMMON as u64);
                // local fg=g2:Select(1-tp,ft2,ft2,nil)
                let fg = group::select(d, &g2, opp, ft2 as u16, ft2 as u16, false, NO_EX).nil();
                // s.sp(fg,1-tp,POS_FACEUP)
                sp(d, &fg, opp, POS_FACEUP as u8);
                // g2:Sub(fg)
                group::remove_card(&mut g2, &fg);
                // gg:Merge(g2)
                group::add_card(&mut gg, &g2);
            }
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // Duel.SendtoGrave(gg,REASON_EFFECT)
    duel::send_to_grave(d, gg, REASON_EFFECT, None, None);
}
