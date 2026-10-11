//! Magical Musket Mastermind Caspar
//! 魔弾の悪魔 カスパール
use super::prelude::*;

pub const ID: u32 = 3287359;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MAGICAL_MUSKET],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2 monsters, including a LIGHT Fiend monster
    // Link.AddProcedure(c,nil,2,2,s.matcheck)
    link::add_procedure(d, c, None, 2, Some(2), Some(std::sync::Arc::new(|d: &mut Duel, g: &CardSet, lc: CardId, st: u64, tp: u8| matcheck(d, g, lc, st, tp))), None, None);
    // Take 2 "Magical Musket" cards from your hand and/or Deck, including a monster, Special Summon 1 of those monsters, and Set the other card to your opponent's field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_link_summoned(d, h)
    })));
    // e1:SetTarget(s.spsettg)
    effect::set_target(d, e1, Some(cost_target(spsettg)));
    // e1:SetOperation(s.spsetop)
    effect::set_operation(d, e1, Some(operation(spsetop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During either player's turn, you can activate "Magical Musket" Spell/Trap Cards from your hand
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD);
    // e2a:SetCode(EFFECT_QP_ACT_IN_NTPHAND)
    effect::set_code(d, e2a, EFFECT_QP_ACT_IN_NTPHAND);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e2a, LOCATION_HAND, 0);
    // e2a:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_MAGICAL_MUSKET))
    effect::set_target(d, e2a, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_MAGICAL_MUSKET as u16]))));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e2b, EFFECT_TRAP_ACT_IN_HAND);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `s.matfilter(c,lc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: CardId, st: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT,lc,sumtype,tp) and c:IsRace(RACE_FIEND,lc,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, Some(lc), st, Some(tp)) && card::is_race_ctx(d, c, RACE_FIEND, Some(lc), st, Some(tp))
}
/// `s.matcheck(g,lc,sumtype,tp)`
fn matcheck(d: &mut Duel, g: &CardSet, lc: CardId, st: u64, tp: u8) -> bool {
    // return g:IsExists(s.matfilter,1,nil,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| matfilter(d, c, lc, st, tp), 1, NO_EX)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    // return sg:IsExists(Card.IsMonster,1,nil) and sg:IsExists(s.spfilter,1,nil,e,tp,sg)
    let ok = group::is_exists(d, sg, &card::is_monster, 1, NO_EX) && group::is_exists(d, sg, &|d, c| spfilter(d, c, e, tp, sg), 1, NO_EX);
    (ok, false)
}
/// `s.spfilter(c,e,tp,sg)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sg: &CardSet) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and sg:IsExists(s.setfilter,1,c,e,tp,1-tp)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) && group::is_exists(d, sg, &|d, x| setfilter(d, x, e, tp, 1 - tp), 1, c.into())
}
/// `s.setfilter(c,e,tp,opp)`
fn setfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, opp: u8) -> bool {
    // return (Duel.GetMZoneCount(opp)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE,opp))
    //     or c:IsSSetable(false,opp)
    (duel::get_m_zone_count(d, opp, NO_EX, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), Some(opp), None))
        || card::is_s_setable(d, c, false, Some(opp))
}
/// `s.spsettg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spsettg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then
    if a.chk == Some(0) {
        // if Duel.GetMZoneCount(tp)<=0 then return false end
        if duel::get_m_zone_count(d, tp, NO_EX, None, None, None).0 <= 0 {
            return false;
        }
        // local g=Duel.GetMatchingGroup(Card.IsSetCard,tp,LOCATION_HAND|LOCATION_DECK,0,nil,SET_MAGICAL_MUSKET)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_set_card(d, c, &[SET_MAGICAL_MUSKET as u16])), tp, LOCATION_HAND + LOCATION_DECK, 0, NO_EX);
        // return aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
        let res = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false);
        return matches!(res, Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND + LOCATION_DECK) as i32);
    true
}
/// `s.spsetop(e,tp,eg,ep,ev,re,r,rp)`
fn spsetop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetMZoneCount(tp)<=0 then return end
    if duel::get_m_zone_count(d, tp, NO_EX, None, None, None).0 <= 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsSetCard,tp,LOCATION_HAND|LOCATION_DECK,0,nil,SET_MAGICAL_MUSKET)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_set_card(d, c, &[SET_MAGICAL_MUSKET as u16])), tp, LOCATION_HAND + LOCATION_DECK, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,aux.Stringid(id,1))
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(aux::stringid(ID, 1)), None, None, false).unwrap_or_default();
    // if #sg~=2 then return end
    if sg.len() != 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local spg=sg:FilterSelect(tp,s.spfilter,1,1,nil,e,tp,sg)
    let spg = group::filter_select(d, &sg, tp, &|d, c| spfilter(d, c, e, tp, &sg), 1, 1, false, NO_EX).nil();
    // if #spg>0 and Duel.SpecialSummon(spg,0,tp,tp,false,false,POS_FACEUP)>0 then
    if spg.len() > 0 && duel::special_summon(d, spg.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local opp=1-tp
        let opp = 1 - tp;
        // local setc=(sg-spg):GetFirst()
        let mut rest = sg.clone();
        group::remove_card(&mut rest, &spg);
        let setc = group::get_first(d, &rest).nil();
        // local b1=Duel.GetMZoneCount(opp)>0 and setc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE,opp)
        let b1 = duel::get_m_zone_count(d, opp, NO_EX, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, setc, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), Some(opp), None);
        // local b2=setc:IsSSetable(false,opp)
        let b2 = card::is_s_setable(d, setc, false, Some(opp));
        // local op=nil
        let op: i64;
        // if b1 and b2 then
        if b1 && b2 {
            // op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
            let sel = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
            op = sel.map_or(0, |x| x as i64);
        } else {
            // op=b1 and 1 or 2
            op = if b1 { 1 } else { 2 };
        }
        // if op==1 then
        if op == 1 {
            // Duel.SpecialSummon(setc,0,tp,opp,false,false,POS_FACEDOWN_DEFENSE)
            duel::special_summon(d, setc, 0, tp, opp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
            // Duel.ConfirmCards(tp,setc)
            duel::confirm_cards(d, tp, CardSet::from_card(setc));
        } else if op == 2 {
            // Duel.SSet(tp,setc,opp)
            duel::s_set(d, tp, setc, Some(opp), true);
        }
    }
}
