//! White Circle Reef
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 21579049;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 Fish monster and search 1 monster with the same name or Special Summon it if you controlled a Synchro monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Target 2 Fish monsters in the GY with the same name to return to the deck and Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.fishsyncfilter(c)`
fn fishsyncfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FISH) and c:IsType(TYPE_SYNCHRO) and c:IsFaceup()
    card::is_race(d, c, RACE_FISH) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_faceup(d, c)
}
/// `s.desfilter(c,e,tp,fishsync)`
fn desfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, fishsync: bool) -> bool {
    // return c:IsRace(RACE_FISH) and c:IsFaceup()
    //   and Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetCode(),fishsync and Duel.GetMZoneCount(tp,c)>0)
    if !(card::is_race(d, c, RACE_FISH) && card::is_faceup(d, c)) {
        return false;
    }
    let code = card::get_code(d, c)[0];
    let mzone = fishsync && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0;
    duel::is_existing_matching_card(d, Some(&|d, x| thspfilter(d, x, e, tp, code, mzone)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.thspfilter(c,e,tp,code,fishsync_mzone)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32, fishsync_mzone: bool) -> bool {
    // return c:IsCode(code) and c:IsMonster() and (c:IsAbleToHand() or
    //   (fishsync_mzone and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_code(d, c, &[code])
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None) || (fishsync_mzone && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local fishsync=e:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsExistingMatchingCard(s.fishsyncfilter,tp,LOCATION_MZONE,0,1,nil)
    let fishsync = effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) && duel::is_existing_matching_card(d, Some(&fishsyncfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.desfilter(c,e,tp,fishsync) end
    if let Some(chkc) = a.chkc() {
        if card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) {
            // `c` is not a parameter here: it is the global `c`, which is nil, and `c:IsRace` raises.
            crate::lua_error("attempt to index a nil value (global 'c')");
        }
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_MZONE,0,1,nil,e,tp,fishsync) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| desfilter(d, c, e, tp, fishsync)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp,fishsync)
    let g = duel::select_target(d, tp, Some(&|d, c| desfilter(d, c, e, tp, fishsync)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    // if fishsync then
    if fishsync {
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    }
    // e:SetLabel(fishsync and 1 or 0)
    effect::set_label(d, e, &[if fishsync { 1 } else { 0 }]);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local code=tc:GetCode()
    let code = card::get_code(d, tc)[0];
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsRace(RACE_FISH) and Duel.Destroy(tc,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) && card::is_race(d, tc, RACE_FISH) && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 {
        // local fishsync_mzone=e:GetLabel()==1 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        let fishsync_mzone = effect::get_label(d, e) == 1 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        // local hint_msg=fishsync_mzone and aux.Stringid(id,2) or HINTMSG_ATOHAND
        let hint_msg = if fishsync_mzone { aux::stringid(ID, 2) } else { HINTMSG_ATOHAND as u64 };
        // Duel.Hint(HINT_SELECTMSG,tp,hint_msg)
        duel::hint(d, HINT_SELECTMSG as u8, tp, hint_msg);
        // local sc=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,code,fishsync_mzone):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, c| thspfilter(d, c, e, tp, code, fishsync_mzone)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &g) else {
            return;
        };
        // if fishsync_mzone and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) then
        if fishsync_mzone && card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, None, None, None) {
            // aux.ToHandOrElse(sc,tp, function(sc) return ... end, function(sc) return Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end, aux.Stringid(id,3))
            aux::to_hand_or_else(
                d,
                sc,
                tp,
                Some(&|d: &mut Duel, c: CardId| fishsync_mzone && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)),
                Some(&|d: &mut Duel, g: &CardSet| duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None)),
                Some(aux::stringid(ID, 3)),
            );
        } else {
            // Duel.SendtoHand(sc,nil,REASON_EFFECT)
            duel::send_to_hand(d, sc, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - tp, sc);
        }
    }
}
/// `s.fishfilter(c,e,tp)`
fn fishfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_FISH) and c:IsCanBeEffectTarget(e)
    //   and (c:IsAbleToDeck() or c:IsCanBeSpecialSummoned(e,0,tp,false,false))
    card::is_race(d, c, RACE_FISH)
        && card::is_can_be_effect_target(d, c, Some(e))
        && (card::is_able_to_deck(d, c) || card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetCode)==1 and #sg==2 and sg:IsExists(s.spcheck,1,nil,sg,e,tp)
    let res = group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == 1
        && sg.len() == 2
        && group::is_exists(d, sg, &|d, c| spcheck(d, c, sg, e, tp), 1, NO_EX);
    (res, false)
}
/// `s.spcheck(c,sg,e,tp)`
fn spcheck(d: &mut Duel, c: CardId, sg: &CardSet, e: Option<EffectId>, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and (sg-c):GetFirst():IsAbleToDeck()
    let e = e.nil();
    if !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) {
        return false;
    }
    let mut rest = sg.clone();
    group::remove_card(&mut rest, &CardSet::from_card(c));
    let first = group::get_first(d, &rest).nil();
    card::is_able_to_deck(d, first)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.fishfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| fishfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && g.len() >= 2
            && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(tg.clone()), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg), 1, tp, 0);
    true
}
/// `s.tdop(e,tp,...)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e)
    let mut g = duel::get_target_cards(d, a.e).nil();
    // if #g~=2 then return end
    if g.len() != 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local td=g:FilterSelect(tp,Card.IsAbleToDeck,1,1,nil)
    let td = group::filter_select(d, &g, tp, &|d, c| card::is_able_to_deck(d, c), 1, 1, false, NO_EX).nil();
    // if #td==0 then return end
    if td.is_empty() {
        return;
    }
    // Duel.HintSelection(td,true)
    duel::hint_selection(d, td.clone(), true);
    // g:RemoveCard(td)
    group::remove_card(&mut g, &td);
    // if Duel.SendtoDeck(td,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0 and #g>0
    //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::send_to_deck(d, td, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0
        && !g.is_empty()
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
