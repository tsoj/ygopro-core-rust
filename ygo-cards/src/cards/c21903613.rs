//! Sylvan Dancepione
use super::prelude::*;

pub const ID: u32 = 21903613;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon procedure
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_PLANT),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_PLANT as u64, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // Excavate and Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DECKDES+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DECKDES + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change Level
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e2, CATEGORY_LVCHANGE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.lvtg)
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_PLANT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_PLANT as u64) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, tp, 1);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DECKDES,nil,2,tp,3)
    duel::set_possible_operation_info(d, 0, CATEGORY_DECKDES, None, 2, tp, 3);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if not Duel.IsPlayerCanDiscardDeck(tp,1) then return end
    if !duel::is_player_can_discard_deck(d, tp, 1) {
        return;
    }
    // local ct=math.min(3,Duel.GetFieldGroupCount(tp,LOCATION_DECK,0))
    let ct = duel::get_field_group_count(d, tp, LOCATION_DECK, 0).min(3);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local t={}
    // for i=1,ct do t[i]=i end
    let t: Vec<u64> = (1..=ct as u64).collect();
    // Duel.Hint(HINTMSG_NUMBER,tp,HINT_NUMBER)
    duel::hint(d, HINTMSG_NUMBER as u8, tp, HINT_NUMBER as u64);
    // local ac=Duel.AnnounceNumber(tp,table.unpack(t))
    let ac = duel::announce_number(d, tp, &t).map_or(0, |r| r.0);
    // Duel.ConfirmDecktop(tp,ac)
    duel::confirm_decktop(d, tp, ac as u32);
    // local g=Duel.GetDecktopGroup(tp,ac)
    let mut g = duel::get_decktop_group(d, tp, ac as u32);
    // local pg=g:Filter(s.spfilter,nil,e,tp)
    let pg = group::filter(d, &g, &|d, c| spfilter(d, c, e, tp), NO_EX);
    // local ft=math.min(Duel.GetLocationCount(tp,LOCATION_MZONE),2)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0.min(2);
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // if ft>0 and #pg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if ft > 0 && !pg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=pg:Select(tp,1,ft,nil)
        let sg = group::select(d, &pg, tp, 1, ft as u16, false, NO_EX).nil();
        // if #sg==0 then return end
        if sg.is_empty() {
            return;
        }
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // for sc in sg:Iter() do
        for sc in group::iter(d, &sg) {
            // if Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
            if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
                // Cannot be used as Link Material
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetDescription(3312)
                effect::set_description(d, e1, 3312);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
                // e1:SetCode(EFFECT_CANNOT_BE_LINK_MATERIAL)
                effect::set_code(d, e1, EFFECT_CANNOT_BE_LINK_MATERIAL);
                // e1:SetValue(1)
                effect::set_value(d, e1, 1);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // sc:RegisterEffect(e1,true)
                card::register_effect(d, sc, e1, true);
                // g:RemoveCard(sc)
                group::remove_card(&mut g, &CardSet::from_card(sc));
            }
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
    // Duel.SendtoGrave(g,REASON_EFFECT|REASON_EXCAVATE)
    duel::send_to_grave(d, g, REASON_EFFECT | REASON_EXCAVATE, None, None);
}
/// `s.lvfilter(c,lg)`
fn lvfilter(d: &mut Duel, c: CardId, lg: &CardSet) -> bool {
    // return c:IsRace(RACE_PLANT) and c:HasLevel() and lg:IsExists(aux.NOT(Card.IsLevel),1,nil,c:GetLevel())
    if !(card::is_race(d, c, RACE_PLANT as u64) && card::has_level(d, c)) {
        return false;
    }
    let lv = card::get_level(d, c);
    group::is_exists(d, lg, &|d, cc| !card::is_level(d, cc, &[lv]), 1, NO_EX)
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local lg=e:GetHandler():GetLinkedGroup():Match(aux.AND(Card.IsFaceup,Card.HasLevel),nil)
    let c = effect::get_handler(d, a.e).nil();
    let mut lg = card::get_linked_group(d, c);
    group::match_(d, &mut lg, &|d, cc| card::is_faceup(d, cc) && card::has_level(d, cc), NO_EX);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.lvfilter(chkc,lg) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::get_controler(d, chkc) == tp && lvfilter(d, chkc, &lg);
    }
    // if chk==0 then return #lg>0 and Duel.IsExistingTarget(s.lvfilter,tp,LOCATION_GRAVE,0,1,nil,lg) end
    if a.chk == Some(0) {
        return !lg.is_empty() && duel::is_existing_target(d, Some(&|d, cc| lvfilter(d, cc, &lg)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.lvfilter,tp,LOCATION_GRAVE,0,1,1,nil,lg)
    duel::select_target(d, tp, Some(&|d, cc| lvfilter(d, cc, &lg)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // local lg=c:GetLinkedGroup():Match(aux.AND(Card.IsFaceup,Card.HasLevel),nil)
    let mut lg = card::get_linked_group(d, c);
    group::match_(d, &mut lg, &|d, cc| card::is_faceup(d, cc) && card::has_level(d, cc), NO_EX);
    // if #lg==0 then return end
    if lg.is_empty() {
        return;
    }
    // local lv=tc:GetLevel()
    let lv = card::get_level(d, tc);
    // for lc in lg:Iter() do
    for lc in group::iter(d, &lg) {
        // Change Level
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(lv)
        effect::set_value(d, e1, lv as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // lc:RegisterEffect(e1)
        card::register_effect(d, lc, e1, false);
    }
}
