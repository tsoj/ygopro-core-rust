//! GMX Applied Experiment #55
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 18795635;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GMX],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavate the top cards of your Deck until you have excavated a "GMX" monster and a Dinosaur monster, lose 400 LP
    // for each excavated card, then you can Fusion Summon 1 "GMX" Fusion Monster from your Extra Deck, using any
    // excavated monsters as material, also shuffle the rest into the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.gmxfilter(c,tp)`
fn gmxfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GMX) and c:IsMonster() and Duel.IsExistingMatchingCard(Card.IsRace,tp,LOCATION_DECK,0,1,c,RACE_DINOSAUR)
    card::is_set_card(d, c, &[SET_GMX as u16])
        && card::is_monster(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_race(d, x, RACE_DINOSAUR as u64)), tp, LOCATION_DECK, 0, 1, c.into())
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.gmxfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| gmxfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `g:GetMaxGroup(Card.GetSequence):GetFirst()`
fn max_seq_card(d: &mut Duel, g: &CardSet) -> CardId {
    let max = group::get_max_group(d, g, &|d, c| card::get_sequence(d, c) as i64).nil().0;
    group::get_first(d, &max).nil()
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local deck_count=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
    let deck_count = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
    // if deck_count==0 then return end
    if deck_count == 0 {
        return;
    }
    // local gmx_g=Duel.GetMatchingGroup(s.gmxfilter,tp,LOCATION_DECK,0,nil,tp)
    let gmx_g = duel::get_matching_group(d, Some(&|d, c| gmxfilter(d, c, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // local dino_g=Duel.GetMatchingGroup(Card.IsRace,tp,LOCATION_DECK,0,nil,RACE_DINOSAUR)
    let dino_g = duel::get_matching_group(d, Some(&|d, c| card::is_race(d, c, RACE_DINOSAUR as u64)), tp, LOCATION_DECK, 0, NO_EX);
    // if #gmx_g==0 or #dino_g==0 then
    if gmx_g.is_empty() || dino_g.is_empty() {
        // Duel.ConfirmDecktop(tp,deck_count)
        duel::confirm_decktop(d, tp, deck_count as u32);
        // local excav_g=Duel.GetDecktopGroup(tp,deck_count)
        let excav_g = duel::get_decktop_group(d, tp, deck_count as u32);
        // Duel.RaiseEvent(excav_g,EVENT_CUSTOM+1595137,e,REASON_EFFECT,tp,tp,deck_count)
        duel::raise_event(d, excav_g, EVENT_CUSTOM + 1595137, Some(a.e), REASON_EFFECT, tp, tp, deck_count as u32);
        // Duel.SetLP(tp,Duel.GetLP(tp)-deck_count*400)
        let lp = duel::get_lp(d, tp) - deck_count * 400;
        duel::set_lp(d, tp, lp);
    } else {
        // local gmx_c=gmx_g:GetMaxGroup(Card.GetSequence):GetFirst()
        let gmx_c = max_seq_card(d, &gmx_g);
        // local dino_c=dino_g:GetMaxGroup(Card.GetSequence):GetFirst()
        let dino_c = max_seq_card(d, &dino_g);
        // local excav_count=deck_count-math.min(gmx_c:GetSequence(),dino_c:GetSequence())
        let min_seq = card::get_sequence(d, gmx_c).min(card::get_sequence(d, dino_c)) as i32;
        let excav_count = deck_count - min_seq;
        // Duel.ConfirmDecktop(tp,excav_count)
        duel::confirm_decktop(d, tp, excav_count as u32);
        // local excav_g=Duel.GetDecktopGroup(tp,excav_count)
        let excav_g = duel::get_decktop_group(d, tp, excav_count as u32);
        // Duel.RaiseEvent(excav_g,EVENT_CUSTOM+1595137,e,REASON_EFFECT,tp,tp,excav_count)
        duel::raise_event(d, excav_g, EVENT_CUSTOM + 1595137, Some(a.e), REASON_EFFECT, tp, tp, excav_count as u32);
        // Duel.SetLP(tp,Duel.GetLP(tp)-excav_count*400)
        let lp = duel::get_lp(d, tp) - excav_count * 400;
        duel::set_lp(d, tp, lp);
        // local excav_monsters=Duel.GetDecktopGroup(tp,excav_count):Match(Card.IsMonster,nil)
        let decktop = duel::get_decktop_group(d, tp, excav_count as u32);
        let excav_monsters = group::filter(d, &decktop, &|d, c| card::is_monster(d, c), NO_EX);
        // local fusion_params={
        //     fusfilter=function(c) return c:IsSetCard(SET_GMX) end,
        //     matfilter=aux.FALSE,
        //     extrafil=function(e,tp,mg) return excav_monsters end
        // }
        let params = fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_GMX as u16]))),
            matfilter: Some(fusion::mat_filter_fn(|_d, _c, _e, _tp, _chk| false)),
            extrafil: Some(Arc::new(move |_d, _e, _tp, _mg| (Some(excav_monsters.clone()), None))),
            ..Default::default()
        };
        // if Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,0) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        let mut tg_args = a.clone();
        tg_args.chk = Some(0);
        if fusion::summon_eff_tg_call(d, &params, &tg_args) && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Fusion.SummonEffOP(fusion_params)(e,tp,eg,ep,ev,re,r,rp)
            fusion::summon_eff_op_call(d, &params, a);
        }
    }
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
}
