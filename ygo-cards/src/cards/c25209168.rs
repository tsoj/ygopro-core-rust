//! Exchange of Despair and Hope
use super::prelude::*;

pub const ID: u32 = 25209168;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_EXCHANGE_SPIRIT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send all monsters to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.tgcon)
    effect::set_condition(d, e1, Some(condition(tgcon)));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgconfilter(c)`
fn tgconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_FAIRY)
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_race(d, c, RACE_FAIRY)
}
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.tgconfilter,tp,LOCATION_MZONE,0,3,nil)
    duel::is_existing_matching_card(d, Some(&tgconfilter), a.tp, LOCATION_MZONE, 0, 3, NO_EX)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,PLAYER_ALL,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, PLAYER_ALL as u8, LOCATION_GRAVE as i32);
    true
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTrap() and c:IsSSetable()
    card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.spselect(p,e,ct)`
fn spselect(d: &mut Duel, p: u8, e: EffectId, ct: usize) -> Option<CardSet> {
    // if ct<1 then return end
    if ct < 1 {
        return None;
    }
    // local ft=math.min(ct,Duel.GetLocationCount(p,LOCATION_MZONE))
    let ft = (ct as i32).min(duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0);
    // if ft>0 and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(Card.IsCanBeSpecialSummoned),p,0,LOCATION_GRAVE,1,nil,e,0,p,false,false)
    //     and Duel.SelectYesNo(p,aux.Stringid(id,1)) then
    if ft > 0
        && duel::is_existing_matching_card(
            d,
            Some(&aux::necro_valley_filter(|d, c| card::is_can_be_special_summoned(d, c, e, 0, p, false, false, None, None, None))),
            p,
            0,
            LOCATION_GRAVE,
            1,
            NO_EX,
        )
        && duel::select_yes_no(d, p, aux::stringid(ID, 1))
    {
        // if Duel.IsPlayerAffectedByEffect(p,CARD_BLUEEYES_SPIRIT) then ft=1 end
        // (ft is not used after this point in the Lua script)
        let _ = duel::is_player_affected_by_effect(d, p, CARD_BLUEEYES_SPIRIT);
        // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_SPSUMMON as u64);
        // return Duel.SelectMatchingCard(p,aux.NecroValleyFilter(Card.IsCanBeSpecialSummoned),p,0,LOCATION_GRAVE,1,ct,nil,e,0,p,false,false)
        return duel::select_matching_card(
            d,
            p,
            Some(&aux::necro_valley_filter(|d, c| card::is_can_be_special_summoned(d, c, e, 0, p, false, false, None, None, None))),
            p,
            0,
            LOCATION_GRAVE,
            1,
            ct as u16,
            false,
            NO_EX,
        );
    }
    None
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local tg=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let mut tg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #tg<1 or Duel.SendtoGrave(tg,REASON_EFFECT)<1 or #tg:Match(Card.IsLocation,nil,LOCATION_GRAVE)<1 then return end
    if tg.is_empty() {
        return;
    }
    if duel::send_to_grave(d, tg.clone(), REASON_EFFECT, None, None) < 1 {
        return;
    }
    // (Group.Match filters tg in place)
    group::match_(d, &mut tg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    if tg.is_empty() {
        return;
    }
    // local p=Duel.GetTurnPlayer()
    let p = duel::get_turn_player(d);
    // local ct=tg:FilterCount(Card.IsControler,nil,1-p)
    let ct = group::filter_count(d, &tg, &|d, c| card::is_controler(d, c, 1 - p), NO_EX);
    // local g1=s.spselect(p,e,ct)
    let g1 = spselect(d, p, e, ct);
    // local spcount=0
    let mut spcount: i32 = 0;
    // if g1 and #g1>0 then
    if let Some(g1) = g1 {
        if !g1.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // spcount=spcount+Duel.SpecialSummon(g1,0,p,p,false,false,POS_FACEUP)
            spcount += duel::special_summon(d, g1, 0, p, p, false, false, POS_FACEUP as u8, None);
        }
    }
    // local g2=s.spselect(1-p,e,#tg-ct)
    let g2 = spselect(d, 1 - p, e, tg.len() - ct);
    // if g2 and #g2>0 then
    if let Some(g2) = g2 {
        if !g2.is_empty() {
            // if spcount==0 then Duel.BreakEffect() end
            if spcount == 0 {
                duel::break_effect(d);
            }
            // spcount=spcount+Duel.SpecialSummon(g2,0,1-p,1-p,false,false,POS_FACEUP)
            spcount += duel::special_summon(d, g2, 0, 1 - p, 1 - p, false, false, POS_FACEUP as u8, None);
        }
    }
    // if spcount>0
    //     and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,CARD_EXCHANGE_SPIRIT)
    //     and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if spcount > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_EXCHANGE_SPIRIT])), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g<1 then return end
        if g.is_empty() {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.SSet(tp,g)>0 then
        if duel::s_set(d, tp, g.clone(), None, true) > 0 {
            // Set Trap can be activated this turn
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(aux.Stringid(id,3))
            effect::set_description(d, e1, aux::stringid(ID, 3));
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
            effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
            // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
            effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // g:GetFirst():RegisterEffect(e1)
            let first = group::get_first(d, &g).nil();
            card::register_effect(d, first, e1, false);
        }
    }
}
