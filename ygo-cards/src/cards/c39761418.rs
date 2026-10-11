//! Arcana Force XIX - The Sun
use super::prelude::*;

pub const ID: u32 = 39761418;

pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
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
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsExistingMatchingCard(function(c) return c.toss_coin and c:IsFaceup() end,0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, _a| {
            duel::is_existing_matching_card(
                d,
                Some(&|d, c| crate::card_info(d, c).toss_coin && card::is_faceup(d, c)),
                0,
                LOCATION_ONFIELD,
                LOCATION_ONFIELD,
                1,
                NO_EX,
            )
        })),
    );
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Toss a coin and apply the appropriate effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_COIN+CATEGORY_DESTROY+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_COIN + CATEGORY_DESTROY + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetTarget(s.cointg)
    effect::set_target(d, e2, Some(cost_target(cointg)));
    // e2:SetOperation(s.coinop)
    effect::set_operation(d, e2, Some(operation(coinop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.cointg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cointg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local sg=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let sg = duel::get_matching_group(d, Some(&card::is_spell_trap), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,sg,#sg,tp,0)
    let n = sg.len() as u8;
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, a.tp, 0);
    true
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c.toss_coin and c:IsSpell() and c:IsSSetable()
    crate::card_info(d, c).toss_coin && card::is_spell(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.coinop(e,tp,eg,ep,ev,re,r,rp)`
fn coinop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local coin=nil
    let coin: i32;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_LIGHT_BARRIER) then
    if duel::is_player_affected_by_effect(d, tp, CARD_LIGHT_BARRIER).is_some() {
        // local b1=Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil)
        let b1 = duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
        // local b2=Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,LOCATION_STZONE,LOCATION_STZONE,1,nil)
        let b2 = duel::is_existing_matching_card(d, Some(&card::is_spell_trap), tp, LOCATION_STZONE, LOCATION_STZONE, 1, NO_EX);
        // local op=Duel.SelectEffect(tp,
        //     {b1,aux.GetCoinEffectHintString(COIN_HEADS)},
        //     {b2,aux.GetCoinEffectHintString(COIN_TAILS)})
        let op = duel::select_effect(
            d,
            tp,
            &[
                (b1, aux::get_coin_effect_hint_string(COIN_HEADS as i32).nil()),
                (b2, aux::get_coin_effect_hint_string(COIN_TAILS as i32).nil()),
            ],
        );
        // if not op then return end
        let Some(op) = op else {
            return;
        };
        // coin=op==1 and COIN_HEADS or COIN_TAILS
        coin = if op == 1 { COIN_HEADS as i32 } else { COIN_TAILS as i32 };
    // else
    } else {
        // coin=Duel.TossCoin(tp,1)
        coin = duel::toss_coin(d, tp, 1)[0];
    }
    // if coin==COIN_HEADS then
    if coin == COIN_HEADS as i32 {
        // Heads: Set 1 Spell from your Deck that has a coin tossing effect
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SSet(tp,g)
            duel::s_set(d, tp, &g, None, true);
        }
    // elseif coin==COIN_TAILS then
    } else if coin == COIN_TAILS as i32 {
        // Tails: Destroy all cards in the Spell & Trap Zones
        // local g=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,LOCATION_STZONE,LOCATION_STZONE,nil)
        let g = duel::get_matching_group(d, Some(&card::is_spell_trap), tp, LOCATION_STZONE, LOCATION_STZONE, NO_EX);
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, &g, REASON_EFFECT, None, None);
        }
    }
}
