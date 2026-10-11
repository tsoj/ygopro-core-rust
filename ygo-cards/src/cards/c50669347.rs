//! Nephthys, the Sacred Preserver
//! 守護神-ネフティス
use super::prelude::*;

pub const ID: u32 = 50669347;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NEPHTHYS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2 "Nephthys" monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_NEPHTHYS),2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_NEPHTHYS as u16], sc, st, Some(tp)))), 2, None, None, None, None);
    // During your Main Phase, if this card was Link Summoned: You can activate 1 of these effects;
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_link_summoned(d, h)
    })));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.deckthfilter(c)`
fn deckthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(8) and c:IsRace(RACE_WINGEDBEAST) and c:IsAbleToHand()
    card::is_level(d, c, &[8]) && card::is_race(d, c, RACE_WINGEDBEAST) && card::is_able_to_hand(d, c, None)
}
/// `s.desfilter(c,e,tp)`
fn desfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NEPHTHYS) and c:IsMonster() and c:IsFaceup() and Duel.GetMZoneCount(tp,c)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,c:GetOriginalCodeRule())
    if !(card::is_set_card(d, c, &[SET_NEPHTHYS as u16]) && card::is_monster(d, c) && card::is_faceup(d, c)) {
        return false;
    }
    if duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 <= 0 {
        return false;
    }
    let code = card::get_original_code_rule(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp, code)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsSetCard(SET_NEPHTHYS) and not c:IsOriginalCodeRule(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_NEPHTHYS as u16]) && !card::is_original_code_rule(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=Duel.IsExistingMatchingCard(s.deckthfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&deckthfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local lg=e:GetHandler():GetLinkedGroup()
    let h = effect::get_handler(d, a.e).nil();
    let lg = card::get_linked_group(d, h);
    // local b2=lg:IsExists(s.desfilter,1,nil,e,tp)
    let (e, tp) = (a.e, a.tp);
    let b2 = group::is_exists(d, &lg, &|d, c| desfilter(d, c, e, tp), 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //   {b1,aux.Stringid(id,1)},
    //   {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, e, &[op.map_or(0, |v| v as i64)]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,lg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(lg), 1, tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    }
    true
}
/// `s.gythfilter(c)`
fn gythfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRitualSpell() and c:IsAbleToHand()
    card::is_ritual_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local dc=Duel.SelectMatchingCard(tp,s.deckthfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, a.tp, Some(&deckthfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let dc = group::get_first(d, &g);
        // if dc and Duel.SendtoHand(dc,nil,REASON_EFFECT)>0 and dc:IsLocation(LOCATION_HAND) then
        if let Some(dc) = dc {
            if duel::send_to_hand(d, dc, None, REASON_EFFECT, None) > 0 && card::is_location(d, dc, LOCATION_HAND) {
                // Duel.ConfirmCards(1-tp,dc)
                duel::confirm_cards(d, 1 - a.tp, dc);
                // Duel.ShuffleHand(tp)
                duel::shuffle_hand(d, a.tp);
                // if Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.gythfilter),tp,LOCATION_GRAVE,0,1,nil)
                //   and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
                if duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(gythfilter)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3)) {
                    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
                    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
                    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.gythfilter),tp,LOCATION_GRAVE,0,1,1,nil)
                    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(gythfilter)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
                    // if #g>0 then
                    if !g.is_empty() {
                        // Duel.HintSelection(g)
                        duel::hint_selection(d, g.clone(), true);
                        // Duel.BreakEffect()
                        duel::break_effect(d);
                        // Duel.SendtoHand(g,nil,REASON_EFFECT)
                        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
                    }
                }
            }
        }
    } else if op == 2 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if not c:IsRelateToEffect(e) then return end
        if !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        // local lg=c:GetLinkedGroup()
        let lg = card::get_linked_group(d, c);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local dc=lg:FilterSelect(tp,s.desfilter,1,1,nil,e,tp):GetFirst()
        let (e, tp) = (a.e, a.tp);
        let sel = group::filter_select(d, &lg, tp, &|d, cc| desfilter(d, cc, e, tp), 1, 1, false, NO_EX).nil();
        let dc = group::get_first(d, &sel);
        // if not dc then return end
        let Some(dc) = dc else { return };
        // Duel.HintSelection(dc)
        duel::hint_selection(d, dc, true);
        // if Duel.Destroy(dc,REASON_EFFECT)>0 then
        if duel::destroy(d, dc, REASON_EFFECT, None, None) > 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp,dc:GetOriginalCodeRule()):GetFirst()
            let code = card::get_original_code_rule(d, dc)[0];
            let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, cc| spfilter(d, cc, e, tp, code))), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            let sc = group::get_first(d, &g);
            // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
            if let Some(sc) = sc {
                if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
                    // Negate its effects
                    // sc:NegateEffects(c)
                    card::negate_effects(d, sc, c, None, false, None);
                }
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    }
}
