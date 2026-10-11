//! Gimmick Puppet Chimera Doll
use super::prelude::*;

pub const ID: u32 = 97520532;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon Procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_MACHINE),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_MACHINE, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // Take 1 "Gimmick Puppet" monster from your Deck, and either add it to your hand or send it to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DECKDES+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DECKDES + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(lua_fn(linkcond)));
    // e1:SetTarget(s.thtgtg)
    effect::set_target(d, e1, Some(cost_target(thtgtg)));
    // e1:SetOperation(s.thtgop)
    effect::set_operation(d, e1, Some(operation(thtgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.listed_series={SET_GIMMICK_PUPPET}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GIMMICK_PUPPET], ..ScriptInfo::DEFAULT };
/// `function(e) return e:GetHandler():IsLinkSummoned() end`
fn linkcond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_link_summoned(d, h)
}
/// `s.thtgfilter(c)`
fn thtgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GIMMICK_PUPPET) and c:IsMonster() and (c:IsAbleToHand() or c:IsAbleToGrave())
    card::is_set_card(d, c, &[SET_GIMMICK_PUPPET as u16])
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None) || card::is_able_to_grave(d, c, None))
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GIMMICK_PUPPET) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GIMMICK_PUPPET as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(0xff))
}
/// `s.thtgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thtgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thtgfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.thtgop(e,tp,eg,ep,ev,re,r,rp)`
fn thtgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local sc=Duel.SelectMatchingCard(tp,s.thtgfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sc_sel = duel::select_matching_card(d, tp, Some(&thtgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sc_sel);
    // if sc and aux.ToHandOrElse(sc,tp)>0 then
    if let Some(sc) = sc {
        if aux::to_hand_or_else(d, sc, tp, None, None, None) > 0 {
            // if sc:IsLocation(LOCATION_HAND) then Duel.ShuffleHand(tp) end
            if card::is_location(d, sc, LOCATION_HAND) {
                duel::shuffle_hand(d, tp);
            }
            // local fg=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
            let fg = duel::get_field_group(d, tp, LOCATION_MZONE, 0);
            // local hg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
            let hg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, NO_EX);
            // if #fg>0 and fg:FilterCount(aux.FaceupFilter(Card.IsSetCard,SET_GIMMICK_PUPPET),nil)==#fg
            //   and #hg>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if !fg.is_empty()
                && group::filter_count(d, &fg, &|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GIMMICK_PUPPET as u16]), NO_EX) == fg.len()
                && !hg.is_empty()
                && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
            {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                // local sg=hg:Select(tp,1,1,nil)
                let sg = group::select(d, &hg, tp, 1, 1, false, NO_EX).nil();
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Cannot Special Summon monsters from the Extra Deck, except Machine Xyz Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not (c:IsType(TYPE_XYZ) and c:IsRace(RACE_MACHINE)) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| {
        card::is_location(d, c, LOCATION_EXTRA) && !(card::is_type(d, c, TYPE_XYZ) && card::is_race(d, c, RACE_MACHINE))
    })));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Clock Lizard check
    // aux.addTempLizardCheck(c,tp,function(e,c) return not (c:IsOriginalType(TYPE_XYZ) and c:IsOriginalRace(RACE_MACHINE)) end)
    aux::add_temp_lizard_check(
        d,
        c,
        tp,
        Some(effect_card_fn(|d, _e, c| !(card::is_original_type(d, c, TYPE_XYZ) && card::is_original_race(d, c, RACE_MACHINE)))),
        None,
        None,
        None,
        None,
    );
}
