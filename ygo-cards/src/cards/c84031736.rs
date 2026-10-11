//! Clown Crew Finale
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 84031736;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you control a Tribute Summoned monster: Discard any number of cards, then target that many face-up cards on the field; destroy them
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(Card.IsTributeSummoned,tp,LOCATION_MZONE,0,1,nil) end)
    effect::set_condition(
        d,
        e1,
        Some(lua_fn(|d, a: Args| duel::is_existing_matching_card(d, Some(&|d, c| card::is_tribute_summoned(d, c)), a.p(1), LOCATION_MZONE, 0, 1, NO_EX))),
    );
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish this card from your GY and Tribute 1 Ritual, Fusion, Synchro, Xyz, Pendulum, or Link Monster from your hand or field; add 1 "Clown Crew" monster from your Deck or GY to your hand with a different original Level/Rank/Link Rating than the Tributed monster's
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(Cost.AND(Cost.SelfBanish,s.thcost))
    effect::set_cost(d, e2, Some(cost::and(vec![cost::self_banish(), cost_target(thcost)])));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.listed_series={SET_CLOWN_CREW}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CLOWN_CREW], ..ScriptInfo::DEFAULT };

/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(-1)
    effect::set_label(d, a.e, &[-1]);
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // local max_target_count=Duel.GetTargetCount(Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let handler = effect::get_handler(d, a.e).nil();
    let max_target_count = duel::get_target_count(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, handler.into());
    // local discard_count=Duel.DiscardHand(tp,nil,1,max_target_count,REASON_COST|REASON_DISCARD,nil)
    let discard_count = duel::discard_hand(d, a.tp, None, 1, max_target_count as u16, REASON_COST | REASON_DISCARD, NO_EX);
    // e:SetLabel(discard_count)
    effect::set_label(d, a.e, &[discard_count as i64]);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsOnField() and chkc:IsFaceup() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_faceup(d, chkc) && chkc != c;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // local cost_chk=e:GetLabel()==-1
        let cost_chk = effect::get_label(d, a.e) == -1;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return cost_chk and Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c)
        return cost_chk && duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // local target_count=e:GetLabel()
    let target_count = effect::get_label(d, a.e);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local tg=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,target_count,target_count,c)
    let tg = duel::select_target(
        d,
        a.tp,
        Some(&|d, x| card::is_faceup(d, x)),
        a.tp,
        LOCATION_ONFIELD,
        LOCATION_ONFIELD,
        target_count as u16,
        target_count as u16,
        false,
        c.into(),
    )
    .nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,#tg,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(tg.clone()), tg.len() as u8, a.tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Destroy(tg,REASON_EFFECT)
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
/// `s.rating(c)`
fn rating(d: &mut Duel, c: CardId) -> u32 {
    // if c:IsLinkMonster() then return c:GetLink()
    if card::is_link_monster(d, c) {
        card::get_link(d, c)
    // elseif c:IsXyzMonster() then return c:GetOriginalRank()
    } else if card::is_xyz_monster(d, c) {
        card::get_original_rank(d, c)
    // else return c:GetOriginalLevel() end
    } else {
        card::get_original_level(d, c)
    }
}
/// `s.thcostfilter(c,tp)`
fn thcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_RITUAL|TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_PENDULUM|TYPE_LINK)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,s.rating(c))
    if !card::is_type(d, c, TYPE_RITUAL | TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_PENDULUM | TYPE_LINK) {
        return false;
    }
    let lv = rating(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, lv)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.thfilter(c,lv)`
fn thfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsSetCard(SET_CLOWN_CREW) and c:IsMonster() and c:IsAbleToHand() and s.rating(c)~=lv
    card::is_set_card(d, c, &[SET_CLOWN_CREW as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None) && rating(d, c) != lv
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(-1)
    effect::set_label(d, a.e, &[-1]);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.thcostfilter,1,1,true,nil,nil,tp) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return aux::check_release_group_cost(d, a.tp, Some(&|d, c| thcostfilter(d, c, tp)), 1, 1, true, None, NO_EX);
    }
    // local sc=Duel.SelectReleaseGroupCost(tp,s.thcostfilter,1,1,true,nil,nil,tp):GetFirst()
    let tp = a.tp;
    let sg = aux::select_release_group_cost(d, a.tp, Some(&|d, c| thcostfilter(d, c, tp)), 1, 1, true, None, NO_EX);
    let sc = group::get_first(d, &sg).nil();
    // Duel.Release(sc,REASON_COST)
    duel::release(d, sc, REASON_COST, None);
    // e:SetLabel(s.rating(sc))
    let lv = rating(d, sc);
    effect::set_label(d, a.e, &[lv as i64]);
    true
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local cost_chk=e:GetLabel()==-1
        let cost_chk = effect::get_label(d, a.e) == -1;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return cost_chk
        return cost_chk;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e:GetLabel()):GetFirst()
    let lv = effect::get_label(d, a.e) as u32;
    let filter = aux::necro_valley_filter(move |d, c| thfilter(d, c, lv));
    let sg = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sg);
    // if sc then
    if let Some(sc) = sc {
        // if sc:IsLocation(LOCATION_GRAVE) then Duel.HintSelection(sc) end
        if card::is_location(d, sc, LOCATION_GRAVE) {
            duel::hint_selection(d, sc, true);
        }
        // Duel.SendtoHand(sc,nil,REASON_EFFECT)
        duel::send_to_hand(d, sc, None, REASON_EFFECT, None);
        // if sc:IsPreviousLocation(LOCATION_DECK) then Duel.ConfirmCards(1-tp,sc) end
        if card::is_previous_location(d, sc, LOCATION_DECK) {
            duel::confirm_cards(d, 1 - a.tp, sc);
        }
    }
}
