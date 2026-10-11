//! Tri-Brigade Hammer
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 92269002;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_TRI_BRIGADE as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects (but you can only use each effect of "Tri-Brigade Hammer" once per turn)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMING_BATTLE_START|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMING_BATTLE_START | TIMINGS_CHECK_MONSTER_E) as u32));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TRI_BRIGADE) and c:IsSpellTrap() and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_TRI_BRIGADE as u16]) && card::is_spell_trap(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.spcostfilter(c)`
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TRI_BRIGADE) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true,false)
    card::is_set_card(d, c, &[SET_TRI_BRIGADE as u16]) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spfilter(c,e,tp,g,link)`: `g` is always nil in this script (the global `g` of `efftg`).
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, link: Option<u32>) -> bool {
    // return c:IsSetCard(SET_TRI_BRIGADE) and c:IsLinkMonster() and (not link or c:IsLink(link))
    //     and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    //     and Duel.GetLocationCountFromEx(tp,tp,g,c)>0
    card::is_set_card(d, c, &[SET_TRI_BRIGADE as u16])
        && card::is_link_monster(d, c)
        && link.map_or(true, |l| card::is_link(d, c, &[l]))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local b1=not Duel.HasFlagEffect(tp,id) and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = !duel::has_flag_effect(d, tp, ID, None) && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local rg=Duel.GetMatchingGroup(s.spcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&spcostfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local lg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_EXTRA,0,nil,e,tp,g)
    let lg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp, None)), tp, LOCATION_EXTRA, 0, NO_EX);
    // local _,min_link=lg:GetMinGroup(Card.GetLink)
    let min_link: Option<i64> = group::get_min_group(d, &lg, &|d, c| card::get_link(d, c) as i64).map(|x| x.1);
    // local b2=not Duel.HasFlagEffect(tp,id+1) and min_link and min_link<=#rg
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None) && min_link.is_some_and(|m| m <= rg.len() as i64);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local cd=e:GetChainData()
    // cd.choice=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let choice = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    chain::set_effect_data(d, e, None, chain::field::CARD, choice.map_or(Value::Nil, |x| Value::Int(x as i64)));
    // if cd.choice==1 then
    if choice == Some(1) {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // elseif cd.choice==2 then
    } else if choice == Some(2) {
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // local link_map={}
        // for lc in lg:Iter() do link_map[lc:GetLink()]=true end
        let mut link_map: Vec<u32> = Vec::new();
        for lc in group::iter(d, &lg) {
            link_map.push(card::get_link(d, lc));
        }
        // local rescon=function(sg) return link_map[#sg] end
        let rescon = |_d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| (link_map.contains(&(sg.len() as u32)), false);
        let rs: aux::Rescon = &rescon;
        // local sg=aux.SelectUnselectGroup(rg,e,tp,min_link,#rg,rescon,1,tp,HINTMSG_REMOVE,rescon)
        let sg = aux::select_unselect_group(d, &rg, Some(e), tp, Some(min_link.nil() as usize), Some(rg.len()), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), Some(rs), None, false).unwrap_or_default();
        // Duel.Remove(sg,POS_FACEUP,REASON_COST)
        duel::remove(d, sg.clone(), POS_FACEUP as u8, REASON_COST, None, None);
        // cd.banished_amount=#sg
        chain::set_effect_data(d, e, None, chain::field::CARD + 1, Value::Int(sg.len() as i64));
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local cd=e:GetChainData()
    let choice = chain::effect_data(d, e, None, chain::field::CARD).int();
    // if cd.choice==1 then
    if choice == 1 {
        // Add 1 "Tri-Brigade" Spell/Trap from your Deck to your hand, except "Tri-Brigade Hammer"
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    // elseif cd.choice==2 and cd.banished_amount then
    } else if choice == 2 && !chain::effect_data(d, e, None, chain::field::CARD + 1).is_nil() {
        let banished = chain::effect_data(d, e, None, chain::field::CARD + 1).int();
        // Special Summon 1 "Tri-Brigade" Link Monster from your Extra Deck, with Link Rating equal to the number banished, ignoring its Summoning conditions
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,nil,cd.banished_amount)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, Some(banished as u32))), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, true, false, POS_FACEUP as u8, None);
        }
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // You cannot Special Summon from the Extra Deck for the rest of this turn, except Beast, Beast-Warrior, or Winged Beast monsters
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
        // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsRace(RACES_BEAST_BWARRIOR_WINGB) end)
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64))));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // "Clock Lizard" check
        // aux.addTempLizardCheck(c,tp,function(e,c) return not c:IsOriginalRace(RACES_BEAST_BWARRIOR_WINGB) end)
        aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(|d, _e, c| !card::is_original_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64))), None, None, None, None);
    }
}
