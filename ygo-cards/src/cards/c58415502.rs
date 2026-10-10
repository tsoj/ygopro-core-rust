//! Royal Straight
use super::prelude::*;

pub const ID: u32 = 58415502;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_JACK_KNIGHT, CARD_QUEEN_KNIGHT, CARD_KING_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send "Jack's Knight", "Queen's Knight", and "King's Knight" to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_JACK_KNIGHT,CARD_QUEEN_KNIGHT,CARD_KING_KNIGHT)
    //     and (c:IsFaceup() or c:IsLocation(LOCATION_HAND)) and c:IsAbleToGrave()
    card::is_code(d, c, &[CARD_JACK_KNIGHT, CARD_QUEEN_KNIGHT, CARD_KING_KNIGHT]) && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND)) && card::is_able_to_grave(d, c, None)
}
/// `s.listfilter` = `aux.OR(Card.ListsCode, Card.ListsCodeAsMaterial)`
fn listfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    card::lists_code(d, c, &[code]) || card::lists_code_as_material(d, c, &[code])
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not (s.listfilter(c,CARD_JACK_KNIGHT) and s.listfilter(c,CARD_QUEEN_KNIGHT) and s.listfilter(c,CARD_KING_KNIGHT)) then return end
    if !(listfilter(d, c, CARD_JACK_KNIGHT) && listfilter(d, c, CARD_QUEEN_KNIGHT) && listfilter(d, c, CARD_KING_KNIGHT)) {
        return false;
    }
    // local code_chk=c:IsCode(11020863) and e:GetHandler():IsCode(id)
    let code_chk = card::is_code(d, c, &[11020863]) && {
        let h = effect::get_handler(d, e).nil();
        card::is_code(d, h, &[ID])
    };
    // return c:IsCanBeSpecialSummoned(e,0,tp,code_chk,code_chk)
    card::is_can_be_special_summoned(d, c, e, 0, tp, code_chk, code_chk, None, None, None)
}
/// `s.spchkfilter(c,sg,tp)`
fn spchkfilter(d: &mut Duel, c: CardId, sg: Option<&CardSet>, tp: u8) -> bool {
    // if c:IsLocation(LOCATION_EXTRA) then
    if card::is_location(d, c, LOCATION_EXTRA) {
        // return Duel.GetLocationCountFromEx(tp,tp,sg,c)>0
        duel::get_location_count_from_ex(d, tp, Some(tp), sg.map_or(NO_EX, |g| g.into()), duel::ExtraCard::Card(c), None).0 > 0
    } else {
        // return Duel.GetMZoneCount(tp,sg)>0
        duel::get_m_zone_count(d, tp, sg.map_or(NO_EX, |g| g.into()), None, None, None).0 > 0
    }
}
/// `s.tgrescon(summg)`: the rescon of the cards to send to the GY
fn tgrescon(summg: CardSet) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, e, tp, mg, c| {
        // local res,stop=aux.dncheck(sg)
        let dn = aux::dncheck();
        let (res, stop) = dn(d, sg, e, tp, mg, c);
        // return res and summg:IsExists(s.spchkfilter,1,nil,sg,tp),stop
        (res && group::is_exists(d, &summg, &|d, x| spchkfilter(d, x, Some(sg), tp), 1, NO_EX), stop)
    }
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local summg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA|LOCATION_GRAVE,0,nil,e,tp)
        let summg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA | LOCATION_GRAVE, 0, NO_EX);
        // if #summg<1 then return false end
        if summg.is_empty() {
            return false;
        }
        // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,nil)
        let g = duel::get_matching_group(d, Some(&tgfilter), a.tp, LOCATION_MZONE | LOCATION_HAND, 0, NO_EX);
        // return #g>2 and aux.SelectUnselectGroup(g,e,tp,3,3,s.tgrescon(summg),0)
        let rc = tgrescon(summg);
        return g.len() > 2 && matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&rc), 0, a.tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,3,tp,LOCATION_MZONE|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 3, a.tp, (LOCATION_MZONE | LOCATION_HAND) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA | LOCATION_GRAVE) as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local summg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA|LOCATION_GRAVE,0,nil,e,tp)
    let summg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA | LOCATION_GRAVE, 0, NO_EX);
    // if #summg<1 then return end
    if summg.is_empty() {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), a.tp, LOCATION_MZONE | LOCATION_HAND, 0, NO_EX);
    // if #g<3 then return end
    if g.len() < 3 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // g=aux.SelectUnselectGroup(g,e,tp,3,3,s.tgrescon(summg),1,tp,HINTMSG_TOGRAVE)
    let rc = tgrescon(summg.clone());
    let g = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&rc), 1, a.tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // if #g==3 and Duel.SendtoGrave(g,REASON_EFFECT)>0 then
    if g.len() == 3 && duel::send_to_grave(d, g, REASON_EFFECT, None, None) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sc=summg:FilterSelect(tp,s.spchkfilter,1,1,nil,nil,tp):GetFirst()
        let sc_g = group::filter_select(d, &summg, a.tp, &|d, c| spchkfilter(d, c, None, a.tp), 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sc_g);
        // if sc then
        if let Some(sc) = sc {
            // local code_chk=sc:IsCode(11020863) and e:GetHandler():IsCode(id)
            let code_chk = card::is_code(d, sc, &[11020863]) && {
                let h = effect::get_handler(d, a.e).nil();
                card::is_code(d, h, &[ID])
            };
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(sc,0,tp,tp,code_chk,code_chk,POS_FACEUP)
            duel::special_summon(d, sc, 0, a.tp, a.tp, code_chk, code_chk, POS_FACEUP as u8, None);
            // if code_chk then sc:CompleteProcedure() end
            if code_chk {
                card::complete_procedure(d, sc);
            }
        }
    }
}
