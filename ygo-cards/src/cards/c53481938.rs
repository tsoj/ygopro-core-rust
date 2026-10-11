//! Rokket Barrage
//! ヴァレット・バラージュ
use super::prelude::*;

pub const ID: u32 = 53481938;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_ROKKET], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can send 1 DARK Dragon monster from your Deck to the GY, except a Level 5 monster; Special Summon this card from your hand, also you cannot Special Summon from the Extra Deck for the rest of this turn, except DARK monsters
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
    // e1:SetCost(s.selfspcost)
    effect::set_cost(d, e1, Some(cost_target(selfspcost)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(operation(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // When a Link Monster's effect is activated that targets this card on the field (Quick Effect): You can destroy this card, then Special Summon up to 1 "Rokket" monster each from your hand, Deck, GY, and/or banishment, except "Rokket Barrage"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.selfspcostfilter(c)`
fn selfspcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsRace(RACE_DRAGON) and not c:IsLevel(5) and c:IsAbleToGraveAsCost()
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_race(d, c, RACE_DRAGON as u64) && !card::is_level(d, c, &[5]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.selfspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.selfspcostfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&selfspcostfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.selfspcostfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&selfspcostfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
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
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    // You cannot Special Summon from the Extra Deck for the rest of this turn, except DARK monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and c:IsAttributeExcept(ATTRIBUTE_DARK) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && card::is_attribute_except(d, c, ATTRIBUTE_DARK, None, 0, None))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local re ... if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).remove(0).as_group().cloned();
    // return tg and tg:IsContains(e:GetHandler()) and re:GetHandler():IsLinkMonster() and re:IsMonsterEffect()
    match tg {
        Some(tg) => {
            let h = effect::get_handler(d, a.e).nil();
            if !group::is_contains(&tg, h) {
                return false;
            }
            let rh = effect::get_handler(d, re).nil();
            card::is_link_monster(d, rh) && effect::is_monster_effect(d, re)
        }
        None => false,
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ROKKET) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and not c:IsCode(id)
    //     and (not c:IsLocation(LOCATION_REMOVED) or c:IsFaceup())
    card::is_set_card(d, c, &[SET_ROKKET as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && !card::is_code(d, c, &[ID])
        && (!card::is_location(d, c, LOCATION_REMOVED) || card::is_faceup(d, c))
}
/// `local LOCATION_HAND_DECK_GRAVE_REMOVED=LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED`
const LOCATION_HAND_DECK_GRAVE_REMOVED: u32 = LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED;
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetMZoneCount(tp,c)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND_DECK_GRAVE_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_HAND_DECK_GRAVE_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND_DECK_GRAVE_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND_DECK_GRAVE_REMOVED as i32);
    true
}
/// `aux.dpcheck(Card.GetLocation)` (the value of the card's location)
fn location_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_location(d, c) as i64
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local ft=Duel.GetMZoneCount(tp,c)
    let mut ft = duel::get_m_zone_count(d, tp, c.into(), None, None, None).0;
    // if c:IsRelateToEffect(e) and Duel.Destroy(c,REASON_EFFECT)>0 and ft>0 then
    if card::is_relate_to_effect(d, c, e) && duel::destroy(d, c, REASON_EFFECT, None, None) > 0 && ft > 0 {
        // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND_DECK_GRAVE_REMOVED,0,nil,e,tp)
        let nv = aux::necro_valley_filter(move |d, x| spfilter(d, x, e, tp));
        let g = duel::get_matching_group(d, Some(&nv), tp, LOCATION_HAND_DECK_GRAVE_REMOVED, 0, NO_EX);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // ft=math.min(ft,4)
        ft = ft.min(4);
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        // local sg=aux.SelectUnselectGroup(g,e,tp,1,ft,aux.dpcheck(Card.GetLocation),1,tp,HINTMSG_SPSUMMON)
        let dp = aux::dpcheck(location_value);
        let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(ft as usize), Some(&dp), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
            .unwrap_or_default();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
