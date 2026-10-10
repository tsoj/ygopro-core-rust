//! Cyber Jar
use super::prelude::*;

pub const ID: u32 = 34124316;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // FLIP: Destroy all monsters on the field, then both players reveal the top 5 cards from their Decks, then Special Summon all revealed Level 4 or lower monsters in face-up Attack Position or face-down Defense Position, also add any remaining cards to their hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,LOCATION_MZONE)
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, LOCATION_MZONE);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,0,PLAYER_ALL,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, PLAYER_ALL as u8, LOCATION_DECK as i32);
    true
}
/// `s.spchk(c,e,tp)`: `return c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE)`
fn spchk(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level_below(d, c, 4) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some((POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8), None, None)
}
/// `local function summon(g,e,p,tograve,ft)`
fn summon(d: &mut Duel, mut g: CardSet, p: u8, tograve: &mut CardSet, ft: i32) {
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // if ft==0 then
    if ft == 0 {
        // tograve:Merge(g)
        group::add_card(tograve, &g);
        // return
        return;
    }
    // if ft<#g then
    if ft < g.len() as i32 {
        // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_SPSUMMON as u64);
        // local newg=g:Select(p,ft,ft,nil)
        let newg = group::select(d, &g, p, ft as u16, ft as u16, false, NO_EX).nil();
        // tograve:Merge(g:Sub(newg))
        let mut sub = g.clone();
        group::remove_card(&mut sub, &newg);
        group::add_card(tograve, &sub);
        // g=newg
        g = newg;
    }
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // Duel.SpecialSummonStep(tc,0,p,p,false,false,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE)
        duel::special_summon_step(d, tc, 0, p, p, false, false, (POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8, None);
    }
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,LOCATION_MZONE)
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, LOCATION_MZONE);
    // if Duel.Destroy(g,REASON_EFFECT)==0 then return end
    if duel::destroy(d, g, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // local p=Duel.GetTurnPlayer()
    let p = duel::get_turn_player(d);
    // local summonable1,nonsummonable1=Duel.GetDecktopGroup(p,5):Split(s.spchk,nil,e,p)
    let dk1 = duel::get_decktop_group(d, p, 5);
    let (mut summonable1, mut nonsummonable1) = group::split(d, &dk1, &|d, c| spchk(d, c, e, p), NO_EX);
    // local summonable2,nonsummonable2=Duel.GetDecktopGroup(1-p,5):Split(s.spchk,nil,e,1-p)
    let dk2 = duel::get_decktop_group(d, 1 - p, 5);
    let (mut summonable2, mut nonsummonable2) = group::split(d, &dk2, &|d, c| spchk(d, c, e, 1 - p), NO_EX);
    // local ft1=Duel.GetLocationCount(p,LOCATION_MZONE)
    let ft1 = duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0;
    // if ft1>1 and Duel.IsPlayerAffectedByEffect(p,CARD_BLUEEYES_SPIRIT) and #summonable1>1 then
    if ft1 > 1 && duel::is_player_affected_by_effect(d, p, CARD_BLUEEYES_SPIRIT).is_some() && summonable1.len() > 1 {
        // nonsummonable1:Merge(summonable1)
        group::add_card(&mut nonsummonable1, &summonable1);
        // summonable1:Clear()
        summonable1 = CardSet::new();
    }
    // local ft2=Duel.GetLocationCount(1-p,LOCATION_MZONE)
    let ft2 = duel::get_location_count(d, 1 - p, LOCATION_MZONE as u8, None, None, None).0;
    // if ft2>1 and Duel.IsPlayerAffectedByEffect(1-p,CARD_BLUEEYES_SPIRIT) and #summonable2>1 then
    if ft2 > 1 && duel::is_player_affected_by_effect(d, 1 - p, CARD_BLUEEYES_SPIRIT).is_some() && summonable2.len() > 1 {
        // nonsummonable2:Merge(summonable2)
        group::add_card(&mut nonsummonable2, &summonable2);
        // summonable2:Clear()
        summonable2 = CardSet::new();
    }
    // local tohand,tograve=nonsummonable1:Merge(nonsummonable2):Split(Card.IsAbleToHand,nil)
    group::add_card(&mut nonsummonable1, &nonsummonable2);
    let (tohand, mut tograve) = group::split(d, &nonsummonable1, &|d, c| card::is_able_to_hand(d, c, None), NO_EX);
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // Duel.ConfirmDecktop(p,5)
    duel::confirm_decktop(d, p, 5);
    // summon(summonable1,e,p,tograve,ft1)
    summon(d, summonable1, p, &mut tograve, ft1);
    // Duel.ConfirmDecktop(1-p,5)
    duel::confirm_decktop(d, 1 - p, 5);
    // summon(summonable2,e,1-p,tograve,ft2)
    summon(d, summonable2, 1 - p, &mut tograve, ft2);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // if #tohand>0 then
    if !tohand.is_empty() {
        // Duel.SendtoHand(tohand,nil,REASON_EFFECT)
        duel::send_to_hand(d, tohand, None, REASON_EFFECT, None);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - tp);
    }
    // if #tograve>0 then
    if !tograve.is_empty() {
        // Duel.SendtoGrave(tograve,REASON_EFFECT)
        duel::send_to_grave(d, tograve, REASON_EFFECT, None, None);
    }
    // local fg=Duel.GetMatchingGroup(Card.IsFacedown,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let fg = duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.ShuffleSetCard(fg)
    duel::shuffle_set_card(d, &fg);
}
