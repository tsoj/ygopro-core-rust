//! Dark Renewal
use super::prelude::*;

pub const ID: u32 = 9287078;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSummonPlayer(1-tp) and c:IsLocation(LOCATION_MZONE) and c:IsCanBeEffectTarget(e)
    //   and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsRace,RACE_SPELLCASTER),tp,LOCATION_MZONE,0,1,c)
    card::is_summon_player(d, c, 1 - tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_can_be_effect_target(d, c, Some(e))
        && duel::is_existing_target(
            d,
            Some(&aux::faceup_filter(|d, x| card::is_race(d, x, RACE_SPELLCASTER))),
            tp,
            LOCATION_MZONE,
            0,
            1,
            c.into(),
        )
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsAttribute(ATTRIBUTE_DARK) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_SPELLCASTER)
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return eg and eg:IsExists(s.filter1,1,nil,e,tp) end
    if a.chk == Some(0) {
        return match &a.eg {
            Some(eg) => group::is_exists(d, eg, &|d, c| filter1(d, c, e, tp), 1, NO_EX),
            None => false,
        };
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g1=eg:FilterSelect(tp,s.filter1,1,1,nil,e,tp)
    let eg = a.eg.clone().nil();
    let mut g1 = group::filter_select(d, &eg, tp, &|d, c| filter1(d, c, e, tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g1)
    duel::set_target_card(d, g1.clone());
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g2=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsRace,RACE_SPELLCASTER),tp,LOCATION_MZONE,0,1,1,g1:GetFirst())
    let first = group::get_first(d, &g1);
    let ex = match first {
        Some(c) => c.into(),
        None => NO_EX,
    };
    let g2 = duel::select_target(
        d,
        tp,
        Some(&aux::faceup_filter(|d, x| card::is_race(d, x, RACE_SPELLCASTER))),
        tp,
        LOCATION_MZONE,
        0,
        1,
        1,
        false,
        ex,
    )
    .nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g1,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g1), 2, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,0,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, e).nil();
    // if #g==2 and Duel.SendtoGrave(g,REASON_EFFECT)==2 and g:IsExists(Card.IsLocation,2,nil,LOCATION_GRAVE) then
    if g.len() == 2
        && duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) == 2
        && group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_GRAVE), 2, NO_EX)
    {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // local sg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp)
        let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp));
        let sg = duel::get_matching_group(d, Some(&nv), tp, LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
        // if #sg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if !sg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local tg=sg:Select(tp,1,1,nil)
            let tg = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
            // Duel.SpecialSummon(tg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
