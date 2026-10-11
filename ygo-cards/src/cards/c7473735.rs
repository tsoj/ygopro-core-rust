//! Harmonic Synchro Fusion
use super::prelude::*;

pub const ID: u32 = 7473735;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
// function s.counterfilter(c)
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    // return not (c:IsSummonLocation(LOCATION_EXTRA) and not c:IsType(TYPE_FUSION|TYPE_SYNCHRO))
    !(card::is_summon_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_FUSION | TYPE_SYNCHRO))
}
// function s.cfilter1(c,e,tp)
fn cfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_TUNER) and c:IsAbleToGraveAsCost() and not c:IsOriginalType(TYPE_TRAP)
    //     and Duel.IsExistingMatchingCard(s.cfilter2,tp,LOCATION_MZONE,0,1,c,e,tp,c)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_TUNER)
        && card::is_able_to_grave_as_cost(d, c)
        && !card::is_original_type(d, c, TYPE_TRAP)
        && duel::is_existing_matching_card(d, Some(&|d, x| cfilter2(d, x, e, tp, c)), tp, LOCATION_MZONE, 0, 1, c.into())
}
// function s.cfilter2(c,e,tp,tun)
fn cfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tun: CardId) -> bool {
    // if not (c:IsFaceup() and not c:IsType(TYPE_TUNER) and c:IsAbleToGraveAsCost() and not c:IsOriginalType(TYPE_TRAP)) then return false end
    if !(card::is_faceup(d, c)
        && !card::is_type(d, c, TYPE_TUNER)
        && card::is_able_to_grave_as_cost(d, c)
        && !card::is_original_type(d, c, TYPE_TRAP))
    {
        return false;
    }
    // local g=Group.FromCards(tun,c)
    let mut g = CardSet::from_card(tun);
    group::add_card(&mut g, &CardSet::from_card(c));
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // tc:AssumeProperty(ASSUME_CODE,tc:GetOriginalCodeRule())
        let code = card::get_original_code_rule(d, tc)[0] as u64;
        card::assume_property(d, tc, ASSUME_CODE, code);
        // tc:AssumeProperty(ASSUME_TYPE,tc:GetOriginalType())
        let ty = card::get_original_type(d, tc) as u64;
        card::assume_property(d, tc, ASSUME_TYPE, ty);
        // tc:AssumeProperty(ASSUME_LEVEL,tc:GetOriginalLevel())
        let lv = card::get_original_level(d, tc) as u64;
        card::assume_property(d, tc, ASSUME_LEVEL, lv);
        // tc:AssumeProperty(ASSUME_ATTRIBUTE,tc:GetOriginalAttribute())
        let attr = card::get_original_attribute(d, tc) as u64;
        card::assume_property(d, tc, ASSUME_ATTRIBUTE, attr);
        // tc:AssumeProperty(ASSUME_RACE,tc:GetOriginalRace())
        let race = card::get_original_race(d, tc);
        card::assume_property(d, tc, ASSUME_RACE, race);
        // tc:AssumeProperty(ASSUME_ATTACK,tc:GetTextAttack())
        let atk = card::get_text_attack(d, tc) as u64;
        card::assume_property(d, tc, ASSUME_ATTACK, atk);
        // tc:AssumeProperty(ASSUME_DEFENSE,tc:GetTextDefense())
        let def = card::get_text_defense(d, tc) as u64;
        card::assume_property(d, tc, ASSUME_DEFENSE, def);
    }
    // local chk=Duel.GetLocationCountFromEx(tp,tp,g,TYPE_FUSION)>=2
    //     and Duel.IsExistingMatchingCard(s.fusfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,g)
    //     and Duel.IsExistingMatchingCard(s.syncfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,g)
    let chk = duel::get_location_count_from_ex(d, tp, Some(tp), (&g).into(), duel::ExtraCard::Type(TYPE_FUSION), None).0 >= 2
        && duel::is_existing_matching_card(d, Some(&|d, x| fusfilter(d, x, e, tp, &g)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, x| syncfilter(d, x, e, tp, &g)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    // Duel.AssumeReset()
    duel::assume_reset(d);
    // return chk
    chk
}
// function s.fusfilter(c,e,tp,mg)
fn fusfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mg: &CardSet) -> bool {
    // if not (c:IsFacedown() and c:IsType(TYPE_FUSION) and (not c.material_location or (c.material_location&LOCATION_GRAVE)>0)) then return false end
    let material_ok = match crate::card_info(d, c).material_location {
        None => true,
        Some(ml) => ml & LOCATION_GRAVE > 0,
    };
    if !(card::is_facedown(d, c) && card::is_type(d, c, TYPE_FUSION) && material_ok) {
        return false;
    }
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:CheckFusionMaterial(mg)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::check_fusion_material(d, c, Some(mg), None, None)
}
// function s.syncfilter(c,e,tp,mg)
fn syncfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mg: &CardSet) -> bool {
    // if not (c:IsFacedown() and c:IsType(TYPE_SYNCHRO)) then return false end
    if !(card::is_facedown(d, c) && card::is_type(d, c, TYPE_SYNCHRO)) {
        return false;
    }
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:IsSynchroSummonable(nil,mg)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_synchro_summonable(d, c, None, Some(mg.clone()), 0, 0)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0
    //     and Duel.IsExistingMatchingCard(s.cfilter1,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0
            && duel::is_existing_matching_card(d, Some(&|d, c| cfilter1(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local tun=Duel.SelectMatchingCard(tp,s.cfilter1,tp,LOCATION_MZONE,0,1,1,nil,e,tp):GetFirst()
    let tun_g = duel::select_matching_card(d, a.tp, Some(&|d, c| cfilter1(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tun = group::get_first(d, &tun_g).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local nt=Duel.SelectMatchingCard(tp,s.cfilter2,tp,LOCATION_MZONE,0,1,1,tun,e,tp,tun):GetFirst()
    let nt_g = duel::select_matching_card(d, a.tp, Some(&|d, c| cfilter2(d, c, a.e, a.tp, tun)), a.tp, LOCATION_MZONE, 0, 1, 1, false, tun.into()).nil();
    let nt = group::get_first(d, &nt_g).nil();
    // local g=Group.FromCards(tun,nt)
    let mut g = CardSet::from_card(tun);
    group::add_card(&mut g, &CardSet::from_card(nt));
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g.clone(), REASON_COST, None, None);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // --Cannot Special Summon from the Extra Deck, except Fusion and Synchro Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(_,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_FUSION|TYPE_SYNCHRO) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_FUSION | TYPE_SYNCHRO))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // --Clock Lizard check
    // aux.addTempLizardCheck(c,tp,function(_,c) return not c:IsOriginalType(TYPE_FUSION|TYPE_SYNCHRO) end)
    aux::add_temp_lizard_check(
        d,
        c,
        a.tp,
        Some(effect_card_fn(|d, _e, c| !card::is_original_type(d, c, TYPE_FUSION | TYPE_SYNCHRO))),
        None,
        None,
        None,
        None,
    );
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none();
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local mg=Duel.GetTargetCards(e)
    let mg = duel::get_target_cards(d, a.e).nil();
    // if #mg~=2 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or Duel.GetLocationCountFromEx(tp,tp,mg,TYPE_FUSION)<2 then return end
    if mg.len() != 2
        || duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
        || duel::get_location_count_from_ex(d, a.tp, Some(a.tp), (&mg).into(), duel::ExtraCard::Type(TYPE_FUSION), None).0 < 2
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local fc=Duel.SelectMatchingCard(tp,s.fusfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,mg):GetFirst()
    let fc_g = duel::select_matching_card(d, a.tp, Some(&|d, x| fusfilter(d, x, a.e, a.tp, &mg)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if not fc then return end
    let Some(fc) = group::get_first(d, &fc_g) else {
        return;
    };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.syncfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,mg):GetFirst()
    let sc_g = duel::select_matching_card(d, a.tp, Some(&|d, x| syncfilter(d, x, a.e, a.tp, &mg)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &sc_g) else {
        return;
    };
    // local g=Group.FromCards(fc,sc)
    let mut g = CardSet::from_card(fc);
    group::add_card(&mut g, &CardSet::from_card(sc));
    // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
