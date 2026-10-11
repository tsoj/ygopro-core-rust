//! Clown Crew Soiree Operations
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 70058649;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects (but you can only use each of these effects of "Clown Crew "Soiree"" once per turn)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.effcost)
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During the Main Phase: You can banish this card from your GY; immediately after this effect resolves, Tribute Summon 1 "Clown Crew" monster face-up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e2, CATEGORY_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(lua_fn(|d, _a: Args| duel::is_main_phase(d, None))));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tribsumtg)
    effect::set_target(d, e2, Some(cost_target(tribsumtg)));
    // e2:SetOperation(s.tribsumop)
    effect::set_operation(d, e2, Some(operation(tribsumop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.listed_series={SET_CLOWN_CREW}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CLOWN_CREW], ..ScriptInfo::DEFAULT };

/// `s.spfilter(c,e,tp,mmz_chk)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // if not (c:IsSetCard(SET_CLOWN_CREW) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,true,false)) then return false end
    if !(card::is_set_card(d, c, &[SET_CLOWN_CREW as u16])
        && card::is_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None))
    {
        return false;
    }
    // if c:IsLocation(LOCATION_DECK) then
    if card::is_location(d, c, LOCATION_DECK) {
        // return mmz_chk
        mmz_chk
    // elseif c:IsLocation(LOCATION_EXTRA) then
    } else if card::is_location(d, c, LOCATION_EXTRA) {
        // return Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
        duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    } else {
        // (the Lua function falls through and returns nil)
        false
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CLOWN_CREW) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_CLOWN_CREW as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(-100)
    effect::set_label(d, a.e, &[-100]);
    // local b1=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE)>0)
    let b1 = !duel::has_flag_effect(d, a.tp, ID, None) && {
        let mmz = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, mmz)), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX)
    };
    // local b2=not Duel.HasFlagEffect(tp,id+1)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b2 = !duel::has_flag_effect(d, a.tp, ID + 1, None) && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    true
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local cost_skip=e:GetLabel()~=-100
    let cost_skip = effect::get_label(d, a.e) != -100;
    // local b1=(cost_skip or not Duel.HasFlagEffect(tp,id))
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE)>0)
    let b1 = (cost_skip || !duel::has_flag_effect(d, a.tp, ID, None)) && {
        let mmz = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, mmz)), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX)
    };
    // local b2=(cost_skip or not Duel.HasFlagEffect(tp,id+1))
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b2 = (cost_skip || !duel::has_flag_effect(d, a.tp, ID + 1, None)) && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then e:SetLabel(0) return b1 or b2 end
    if a.chk == Some(0) {
        effect::set_label(d, a.e, &[0]);
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,2)},
    //     {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |x| x as i64)]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    // elseif op==2 then
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, a.tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    }
    true
}
/// `s.pendlinkfilter(c)`
fn pendlinkfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsLocation(LOCATION_EXTRA) and c:IsFaceup()) or c:IsLinkMonster()
    (card::is_location(d, c, LOCATION_EXTRA) && card::is_faceup(d, c)) || card::is_link_monster(d, c)
}
/// `s.rescon(mmz_ct,linkmz_ct)`: the returned function(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, mmz_ct: usize, linkmz_ct: usize) -> bool {
    // return sg:FilterCount(Card.IsLocation,nil,LOCATION_DECK)<=mmz_ct
    //     and sg:FilterCount(s.pendlinkfilter,nil)<=linkmz_ct
    group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX) <= mmz_ct
        && group::filter_count(d, sg, &pendlinkfilter, NO_EX) <= linkmz_ct
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // You cannot activate the effects of monsters Special Summoned from the Deck or Extra Deck until the end of the next turn after this card resolves
        // local e1=Effect.CreateEffect(e:GetHandler())
        let owner = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, owner);
        // e1:SetDescription(aux.Stringid(id,4))
        effect::set_description(d, e1, aux::stringid(ID, 4));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetValue(s.aclimit)
        effect::set_value_fn(d, e1, lua_fn(aclimit));
        // e1:SetReset(RESET_PHASE|PHASE_END,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // --● Special Summon up to 2 "Clown Crew" monsters from your Deck and/or Extra Deck, ignoring their Summoning conditions
        // local mmz_ct=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let mmz_ct = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        // local emz_ct=Duel.GetLocationCountFromEx(tp,tp,nil,nil,ZONES_EMZ)
        let emz_ct = duel::get_location_count_from_ex(d, a.tp, Some(a.tp), NO_EX, duel::ExtraCard::None, Some(ZONES_EMZ)).0;
        // local linkmz_ct=Duel.GetLocationCountFromEx(tp,tp)
        let linkmz_ct = duel::get_location_count_from_ex(d, a.tp, Some(a.tp), NO_EX, duel::ExtraCard::None, None).0;
        // local ft=math.min(mmz_ct+emz_ct,2)
        let mut ft = (mmz_ct + emz_ct).min(2);
        // if ft<=0 then return end
        if ft <= 0 {
            return;
        }
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,nil,e,tp,mmz_ct>0)
        let mmz = mmz_ct > 0;
        let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, mmz)), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, NO_EX);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // local sg=aux.SelectUnselectGroup(g,e,tp,1,ft,s.rescon(mmz_ct,linkmz_ct),1,tp,HINTMSG_SPSUMMON)
        let (mmz_u, linkmz_u) = (mmz_ct.max(0) as usize, linkmz_ct.max(0) as usize);
        let rc = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| -> (bool, bool) {
            (rescon(d, sg, mmz_u, linkmz_u), false)
        };
        let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(ft as usize), Some(&rc), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
            .unwrap_or_default();
        // if #sg==0 then return end
        if sg.is_empty() {
            return;
        }
        // local fup,fdown=sg:Split(s.pendlinkfilter,nil)
        let (fup, fdown) = group::split(d, &sg, &pendlinkfilter, NO_EX);
        // local fdown_main,fdown_ex=fdown:Split(Card.IsLocation,nil,LOCATION_DECK)
        let (fdown_main, fdown_ex) = group::split(d, &fdown, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
        // local priority_0,priority_1
        // if linkmz_ct<mmz_ct then
        let (priority_0, priority_1) = if linkmz_ct < mmz_ct {
            // priority_0=fup
            // priority_1=fdown_main
            (fup, fdown_main)
        } else {
            // priority_0=fdown_main
            // priority_1=fup
            (fdown_main, fup)
        };
        // for prio0_c in priority_0:Iter() do
        for prio0_c in group::iter(d, &priority_0) {
            // Duel.SpecialSummonStep(prio0_c,0,tp,tp,true,false,POS_FACEUP)
            duel::special_summon_step(d, prio0_c, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
        }
        // for prio1_c in priority_1:Iter() do
        for prio1_c in group::iter(d, &priority_1) {
            // Duel.SpecialSummonStep(prio1_c,0,tp,tp,true,false,POS_FACEUP)
            duel::special_summon_step(d, prio1_c, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
        }
        // for fdown_ex_c in fdown_ex:Iter() do
        for fdown_ex_c in group::iter(d, &fdown_ex) {
            // Duel.SpecialSummonStep(fdown_ex_c,0,tp,tp,true,false,POS_FACEUP)
            duel::special_summon_step(d, fdown_ex_c, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    // elseif op==2 then
    } else if op == 2 {
        // --● Add 1 "Clown Crew" monster from your Deck to your hand
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g);
        }
    }
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let rc = effect::get_handler(d, re).nil();
    // return re:IsMonsterEffect() and rc:IsOnField() and rc:IsSummonLocation(LOCATION_DECK|LOCATION_EXTRA)
    effect::is_monster_effect(d, re) && card::is_on_field(d, rc) && card::is_summon_location(d, rc, LOCATION_DECK | LOCATION_EXTRA)
}
/// `s.tribsumfilter(c)`
fn tribsumfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CLOWN_CREW) and c:IsSummonable(true,nil,1)
    card::is_set_card(d, c, &[SET_CLOWN_CREW as u16]) && card::is_summonable(d, c, true, None, 1, None)
}
/// `s.tribsumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tribsumtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tribsumfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tribsumfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.tribsumop(e,tp,eg,ep,ev,re,r,rp)`
fn tribsumop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.tribsumfilter,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&tribsumfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // Duel.Summon(tp,sc,true,nil,1)
        duel::summon(d, a.tp, sc, true, None, 1, None);
    }
}
